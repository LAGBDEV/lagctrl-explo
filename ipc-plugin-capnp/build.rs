// build.rs
fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("src")
        .file("src/protocol.capnp")
        .run()
        .expect("schema compilation failed");
}
