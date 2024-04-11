mod http_server;
#[derive(Debug, Clone)]
pub struct HttpServer {
    port: u16,
    host: String,
}
