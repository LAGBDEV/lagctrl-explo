use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type ArgMap = HashMap<String, String>;
type CommandFn = Arc<dyn Fn(ArgMap) -> tokio::task::JoinHandle<String> + Send + Sync>;

#[derive(Clone)]
pub struct CommandInfo {
    pub owner: String,
    pub name: String,
    pub description: String,
    pub usage: String,
    pub handler: CommandFn,
}

#[derive(Default, Clone)]
pub struct CommandRegistry {
    commands: Arc<RwLock<HashMap<String, CommandInfo>>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register<F>(
        &self,
        owner: &str,
        name: &str,
        description: &str,
        usage: &str,
        handler: F,
    ) where
        F: Fn(ArgMap) -> tokio::task::JoinHandle<String> + Send + Sync + 'static,
    {
        let mut cmds = self.commands.write().await;
        cmds.insert(
            name.to_string(),
            CommandInfo {
                owner: owner.to_string(),
                name: name.to_string(),
                description: description.to_string(),
                usage: usage.to_string(),
                handler: Arc::new(handler),
            },
        );
    }

    pub async fn run(&self, name: &str, args: ArgMap) -> Option<tokio::task::JoinHandle<String>> {
        let cmds = self.commands.read().await;
        cmds.get(name).map(|cmd| (cmd.handler)(args))
    }

    pub async fn help(&self) -> Vec<(String, String, String)> {
        let cmds = self.commands.read().await;
        cmds.values()
            .map(|c| (c.name.clone(), c.description.clone(), c.owner.clone()))
            .collect()
    }

    pub async fn usage(&self, name: &str) -> Option<String> {
        let cmds = self.commands.read().await;
        cmds.get(name).map(|c| c.usage.clone())
    }
}
