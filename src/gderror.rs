use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Std error: {0}")]
    Sts(#[from] Box<dyn std::error::Error>),

    #[error("Websocket error: {0}")]
    WebSocket(#[from] tungstenite::Error),

    #[error("b64 decode err")]
    B64Decode(#[from] base64::DecodeError),

    #[error("Only editor action")]
    OnlyEditorAction,

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Unknown err: {0}")]
    Unknown(String),

    #[error("Sending message err")]
    SendingMessage(),

    #[error("Getting the last level err")]
    GettingLastLevel(),

    #[error("Getting list dict err")]
    GettingListDict(),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}