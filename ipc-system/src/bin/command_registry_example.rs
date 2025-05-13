use ipc_system::command_registry::CommandRegistry;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let registry = CommandRegistry::new();

    registry
        .register(
            "core",
            "greet",
            "Greets the user",
            "greet name=YourName",
            |args| {
                tokio::spawn(async move {
                    sleep(Duration::from_millis(50)).await;
                    let name = args.get("name").cloned().unwrap_or("stranger".into());
                    format!("Hello, {}!", name)
                })
            },
        )
        .await;

    let mut args = HashMap::new();
    args.insert("name".to_string(), "Alice".to_string());

    if let Some(handle) = registry.run("greet", args).await {
        println!("{}", handle.await.unwrap()); // => Hello, Alice!
    }

    // Help overview
    for (name, desc, owner) in registry.help().await {
        println!("{owner}.{name}: {desc}");
    }

    // Usage info
    if let Some(usage) = registry.usage("greet").await {
        println!("Usage: {usage}");
    }
}
