use tungstenite::{connect, WebSocket, stream::MaybeTlsStream, Message};
use std::net::TcpStream;
use serde_json::json;

use crate::gameobject::{GameObjectTrait}; // уточните путь, если нужно

macro_rules! editor_log {
    ($print:literal) => {
        println!("[GDFABRIC] {}", format!($print))
    };
}

pub struct Editor {
    ws_server: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    objs: Vec<Box<dyn GameObjectTrait>>,
}

impl Editor {
    pub fn load_ws() -> Self {
        let ret = connect("ws://localhost:1313");
        match ret {
            Ok(c) => {
                editor_log!("Websocket connection Ok!");
                let (socket, _response) = c;
                Self {
                    ws_server: Some(socket),
                    objs: vec![],
                }
            }
            Err(e) => {
                editor_log!("Websocket connection Err: {e}");
                Self {
                    ws_server: None,
                    objs: vec![],
                }
            }
        }
    }

    // Принимаем срез объектов, клонируем их и сохраняем в Box
    pub fn add_objects<T: GameObjectTrait + Clone + 'static>(&mut self, objs: &[T]) {
        for obj in objs {
            self.objs.push(Box::new(obj.clone()));
        }
    }

    fn send_and_receive(&mut self, msg: &str) -> Result<String, tungstenite::Error> {
        let socket = self.ws_server.as_mut().ok_or(tungstenite::Error::ConnectionClosed)?;
        socket.send(Message::Text(msg.to_string().into()))?;
        let response = socket.read()?;
        match response {
            Message::Text(text) => Ok(text.to_string()),   // Utf8Bytes -> String
            Message::Binary(data) => Ok(String::from_utf8_lossy(&data).into_owned()),
            _ => Err(tungstenite::Error::AttackAttempt),
        }
    }

    // Получить строку сериализации всех объектов
    fn get_save_string(&self) -> String {
        let mut ret = String::new();
        for obj in &self.objs {
            ret.push_str(obj.serialize().as_str());
        }
        ret
    }

    // Сохранение: отправляем данные на сервер и получаем ответ
    pub fn save(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let data = self.get_save_string();
        let obj = json!({ "objects": data });
        let json = serde_json::to_string(&obj)?;
        let response = self.send_and_receive(&json)?;
        let value: serde_json::Value = serde_json::from_str(&response)?;
        Ok(value)
    }
}