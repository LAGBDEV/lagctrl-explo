use ipc_system::ipc_io::IpcIO;
use ipc_system::ipc_protocol::{IpcCommand, IpcResponse};

trait Plugin {
    fn initialize(&self) -> Result<(), String>;

    fn run(&self) -> Result<(), String>;

    fn terminate(&self) -> Result<(), String>;
}

struct PluginData {
    pub name: &'static str,
    pub version: &'static str,
}

impl Plugin for PluginData {
    fn initialize(&self) -> Result<(), String> {
        todo!()
    }

    fn run(&self) -> Result<(), String> {
        todo!()
    }

    fn terminate(&self) -> Result<(), String> {
        todo!()
    }
}

struct PluginContext {
    plugin: Box<dyn Plugin>,
}

impl PluginContext {
    fn execute(&self) -> Result<(), String> {
        self.plugin.initialize()?;
        self.plugin.run()?;
        self.plugin.terminate()?;
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let plugin = Box::new(PluginData {
        name: "ExamplePlugin",
        version: "1.0",
    });

    let context = PluginContext { plugin };
    context.execute()
}
