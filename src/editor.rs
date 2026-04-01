use tungstenite::{connect, WebSocket, stream::MaybeTlsStream, Message};
use std::net::TcpStream;
use serde_json::json;
use thiserror::Error;

use crate::gameobject::{GameObject};

#[derive(Error, Debug)]
pub enum LiveEditorError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Std error: {0}")]
    Sts(#[from] Box<dyn std::error::Error>),

    #[error("Websocket error: {0}")]
    WebSocket(#[from] tungstenite::Error),

    #[error("Only editor action")]
    OnlyEditorAction,

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Unknown err: {0}")]
    Unknown(String),

    #[error("Sending message err")]
    SendingMessage(),
}

enum ResponseStatus {
    Error,
    Successful
}

struct Response {
    pub status: ResponseStatus,
    pub message: String
}

pub struct Editor {
    ws_server: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    objs: Vec<GameObject>,
    pub add_debug_group: bool,
    pub clear_debug_objs: bool,
}

impl Editor {
    pub fn load_ws() -> Result<Self, tungstenite::Error> {
        let ret = connect("ws://localhost:1313")?;
        let (socket, _response) = ret;

        Ok(Self {
            ws_server: Some(socket),
            objs: vec![],
            add_debug_group: true,
            clear_debug_objs: true,
        })
    }

    pub fn add_objects(&mut self, objs: Vec<GameObject>) {
        for obj in objs {
            if self.add_debug_group {
                let mut ret = obj.clone();
                ret.groups.insert(9999);
                self.objs.push(ret);
            } else {
                self.objs.push(obj.clone());
            }
        }
    }

    pub fn level_string(&mut self) -> Result<String, LiveEditorError> {
        let obj = json!({
            "action": "GET_LEVEL_STRING",
        });
        let json = serde_json::to_string(&obj)?;
        let response = self.send_and_receive(&json)?;

        return match response.status {
            ResponseStatus::Successful => Ok(response.message),
            ResponseStatus::Error => Err(self.error_handler(response.message))
        }
    }

    fn get_save_string(&self) -> String {
        let mut ret = String::new();
        for obj in &self.objs {
            ret.push_str(obj.serialize().as_str());
        }
        ret
    }
    
    pub fn remove_objs(&mut self, group: u16) -> Result<(), LiveEditorError> {
        let obj = json!({
            "action": "REMOVE_OBJECTS",
            "group": group
        });

        let json = serde_json::to_string(&obj)?;
        let response = self.send_and_receive(&json)?;

        return match response.status {
            ResponseStatus::Successful => Ok(()),
            ResponseStatus::Error => Err(self.error_handler(response.message))
        }
    }

    pub fn save(&mut self) -> Result<(), LiveEditorError> {
        if self.clear_debug_objs {
            if let Err(e) = self.remove_objs(9999) {
                return Err(e)
            }
        }
        
        let obj = json!({
            "action": "ADD_OBJECTS",
            "objects": self.get_save_string()
        });

        let json = serde_json::to_string(&obj)?;
        let response = self.send_and_receive(&json)?;
        
        return match response.status {
            ResponseStatus::Successful => Ok(()),
            ResponseStatus::Error => Err(self.error_handler(response.message))
        }
    }

    fn error_handler(&self, err: String) -> LiveEditorError {
        return match err.as_str() {
            "Enter the level editor to run this action" => LiveEditorError::OnlyEditorAction,
            _ => LiveEditorError::Unknown(err)
        }
    }

    fn send_and_receive(&mut self, msg: &str) -> Result<Response, LiveEditorError> {
        let socket = self.ws_server.as_mut().ok_or(tungstenite::Error::ConnectionClosed)?;
        socket.send(Message::Text(msg.to_string().into()))?;
        let response = socket.read()?;
        match response {
            Message::Text(text) => {
                let mut value: serde_json::Value = serde_json::from_str(&text)?;

                let status = match value["status"]
                    .take()
                    .as_str()
                    .ok_or_else(|| LiveEditorError::InvalidResponse("status is not a string".to_string()))?
                {
                    "successful" => ResponseStatus::Successful,
                    _ => ResponseStatus::Error
                };

                let message: String = match status {
                    ResponseStatus::Successful => {
                        value["response"]
                            .take()
                            .as_str()
                            .unwrap_or_default()
                            .to_string()
                    },
                    ResponseStatus::Error => {
                        value["message"]
                            .take()
                            .as_str()
                            .ok_or_else(|| LiveEditorError::InvalidResponse("message is not a string".to_string()))?
                            .to_string()
                    }
                };
                Ok(Response { status: status, message })
            },
            _ => Err(LiveEditorError::SendingMessage()),
        }
    }
}