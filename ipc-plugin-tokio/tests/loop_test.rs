#[cfg(test)]
mod tests {
    use ipc_plugin_tokio::{
        protocol::{Command, Response},
        run_plugin_loop,
    };
    use tokio::io::BufReader;

    #[tokio::test]
    async fn test_commands() {
        let input = vec![
            serde_json::to_string(&Command::Ping).unwrap(),
            serde_json::to_string(&Command::Echo("hello".into())).unwrap(),
            serde_json::to_string(&Command::Add(2, 3)).unwrap(),
        ]
        .join("\n")
            + "\n";

        let mut output = Vec::new();
        run_plugin_loop(BufReader::new(input.as_bytes()), &mut output)
            .await
            .unwrap();

        let result = String::from_utf8(output).unwrap();
        let lines: Vec<_> = result.lines().collect();

        assert_eq!(
            lines[0],
            serde_json::to_string(&Response::Pong("pong".into())).unwrap()
        );
        assert_eq!(
            lines[1],
            serde_json::to_string(&Response::Text("hello".into())).unwrap()
        );
        assert_eq!(
            lines[2],
            serde_json::to_string(&Response::Number(5)).unwrap()
        );
    }
}
