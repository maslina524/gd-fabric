use tungstenite::{connect, WebSocket, stream::MaybeTlsStream, Message};
use std::{net::TcpStream};
use serde_json::json;

use crate::*;

enum ResponseStatus {
    Error,
    Successful
}

struct Response {
    pub status: ResponseStatus,
    pub message: String
}

/// Editor for Geometry Dash level.
/// 
/// Provides real-time communication with the Geometry Dash editor mod,
/// allowing you to add, modify, and save game objects programmatically.
/// 
/// # Example
/// 
/// ```
/// use gdfabric::{Editor, TextGameObject, Point};
/// 
/// # fn main() -> Result<(), gdfabric::Error> {
/// // Connect to the editor
/// let mut editor = Editor::load_ws()?;
/// 
/// // Create objects
/// let mut obj = TextGameObject::from(String::from("Hello, World!"), 5);
/// obj.pos = Point::new(100.0, 200.0);
/// 
/// // Add and save
/// editor.add_objects(vec![obj]);
/// editor.save()?;
/// # Ok(())
/// # }
/// ```
/// 
/// # Connection
/// 
/// The editor connects to `ws://localhost:1313`.
/// Make sure Geometry Dash is running with the editor mod installed.
pub struct Editor {
    ws_server: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    objs: Vec<GameObject>,
    pub add_debug_group: bool,
    pub clear_debug_objs: bool,
}

impl Editor {
    /// `gdfabric` connects to a local websocket server launched by the `WSLIveEditor` mod
    /// 
    /// Download WSLiveEditor in [Geode](https://geode-sdk.org/mods/iandyhd3.wsliveeditor) or from [GitHub](https://github.com/iAndyHD3/WSLiveEditor)
    /// 
    /// # Examples
    /// 
    /// ```
    /// # #[test]
    /// # fn test() {
    /// use gdfabric::Editor;
    /// 
    /// let result = Editor::load_ws();
    /// 
    /// match result {
    ///     Ok(_editor) => println!("Connection successful!"),
    ///     Err(e) => panic!("{e}")
    /// }
    /// # }
    /// ```
    pub fn load_ws() -> Result<Self, Error> {
        let ret = connect("ws://localhost:1313")?;
        let (socket, _response) = ret;

        Ok(Self {
            ws_server: Some(socket),
            objs: vec![],
            add_debug_group: false,
            clear_debug_objs: false,
        })
    }

    /// Adds objects to a `Editor` (struct)
    /// 
    /// It does not add objects directly to the game. To do this, you need to save the changes using the `save()` method!
    /// 
    /// # Examples
    /// 
    /// ```
    /// use gdfabric::{Editor, GameObject};
    /// 
    /// # fn main() -> Result<(), gdfabric::Error> {
    /// let mut editor = Editor::load_ws()?;
    /// 
    /// let mut obj = GameObject::new();
    /// editor.add_objects(vec![obj]);
    /// 
    /// editor.save()?; // !!!
    /// # Ok(())
    /// # }
    /// ```
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
    /// Returns a level string
    /// 
    /// Look at how the [Level String](https://boomlings.dev/resources/client/level-components/level-string) and the [Level Start String](https://boomlings.dev/resources/client/level-components/level-start) work
    /// 
    /// # Examples
    /// 
    /// ```
    /// use gdfabric::Editor;
    /// 
    /// # fn main() -> Result<(), gdfabric::Error> {
    /// let mut editor = Editor::load_ws()?;
    /// 
    /// match editor.level_string() {
    ///     Ok(c) => assert_eq!(c.starts_with("kS38,"), true),
    ///     Err(e) => panic!("unexpected err {e}"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn level_string(&mut self) -> Result<String, Error> {
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
    
    /// Deletes all objects in the level with the specified group
    /// 
    /// # Examples
    /// 
    ///  ```
    /// use gdfabric::Editor;
    /// 
    /// # fn main() -> Result<(), gdfabric::Error> {
    /// let mut editor = Editor::load_ws()?;
    /// 
    /// editor.remove_objs(9999);
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove_objs(&mut self, group: u16) -> Result<(), Error> {
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

    /// Adds all the objects you previously added via the `add_objects()` method to the level
    /// 
    /// # Examples
    /// 
    /// ```
    /// use gdfabric::{Editor, GameObject};
    /// 
    /// # fn main() -> Result<(), gdfabric::Error> {
    /// let mut editor = Editor::load_ws()?;
    /// 
    /// let mut obj = GameObject::new();
    /// editor.add_objects(vec![obj]);
    /// 
    /// editor.save()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn save(&mut self) -> Result<(), Error> {
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

    fn error_handler(&self, err: String) -> Error {
        return match err.as_str() {
            "Enter the level editor to run this action" => Error::OnlyEditorAction,
            _ => Error::Unknown(err)
        }
    }

    fn send_and_receive(&mut self, msg: &str) -> Result<Response, Error> {
        let socket = self.ws_server.as_mut().ok_or(tungstenite::Error::ConnectionClosed)?;
        socket.send(Message::Text(msg.to_string().into()))?;
        let response = socket.read()?;
        match response {
            Message::Text(text) => {
                let mut value: serde_json::Value = serde_json::from_str(&text)?;

                let status = match value["status"]
                    .take()
                    .as_str()
                    .ok_or_else(|| Error::InvalidResponse("status is not a string".to_string()))?
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
                            .ok_or_else(|| Error::InvalidResponse("message is not a string".to_string()))?
                            .to_string()
                    }
                };
                Ok(Response { status: status, message })
            },
            _ => Err(Error::SendingMessage()),
        }
    }
}