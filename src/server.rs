use log::{info, trace};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        trace!("initialize");
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("server initialized");
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
pub async fn start() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use log::info;
    use serde_json::json;
    use tower_lsp::{LspService, Server};

    use super::Backend;

    #[ctor::ctor]
    fn setup() {
        pretty_env_logger::init();
    }

    #[tokio::test]
    async fn it_initializses() {
        let input = tokio_test::io::Builder::new()
            .write(br#"Content-Length: 123\r\n\r\n{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#)
            .build();

        let output = tokio_test::io::Builder::new()
            .read(json!({ "hi": "and stuff" }).to_string().as_bytes())
            .build();

        let (service, socket) = LspService::new(|client| Backend { client });
        let server = Server::new(input, output, socket);

        info!("test start");
        server.serve(service).await;
        info!("test done?");
    }
}
