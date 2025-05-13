use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UIComponentTree {
    ui_components: Vec<UIComponentTree>,
}

/// Defines the commands the plugin can handle.
#[derive(Deserialize, Serialize)]
#[serde(tag = "cmd", content = "args")]
pub enum Command {
    Ping,
    Echo(String),
    Add(i64, i64),
    Time,
    GetInitialUI(),
}

/// Defines possible responses the plugin can emit.
#[derive(Serialize)]
#[serde(untagged)]
pub enum Response {
    Pong(String),
    Number(i64),
    Text(String),
    UIInit(UIComponentTree),
}

impl Command {
    /// Executes business logic and returns a Response.
    pub fn execute(self) -> Response {
        match self {
            Command::Ping => Response::Pong("pong".into()),
            Command::Echo(text) => Response::Text(text),
            Command::Add(a, b) => Response::Number(a + b),
            Command::Time => Response::Text(Utc::now().to_rfc3339()),
            Command::GetInitialUI() => Response::UIInit(UIComponentTree {
                ui_components: vec![],
            }),
        }
    }
}
