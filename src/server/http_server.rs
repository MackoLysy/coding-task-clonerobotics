use super::HttpServer;
use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use log::*;
use std::error::Error;
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::pin::Pin;
use tokio::net::TcpListener;

impl HttpServer {
    pub fn new(host: String, port: u16) -> Self {
        HttpServer {
            host: host,
            port: port,
        }
    }
    fn get_socket_addr(&self) -> Result<SocketAddr, Box<dyn Error>> {
        let ip_addr: IpAddr = match self.host.as_str() {
            "localhost" => IpAddr::V4(Ipv4Addr::LOCALHOST),
            _ => self
                .host
                .parse::<IpAddr>()
                .expect(&format!("Invalid hostaddress {}", self.host)),
        };
        Ok(SocketAddr::new(ip_addr, self.port))
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let socket_addr = self.get_socket_addr()?;
        let listener = TcpListener::bind(socket_addr).await?;
        info!("Listening on http://{}", socket_addr);
        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let server = self.clone();
            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new().serve_connection(io, server).await {
                    error!("Failed to serve connection: {:?}", err);
                }
            });
        }
    }

    fn handle_start() -> Result<Response<Full<Bytes>>, hyper::Error> {
        info!("try to start");
        HttpServer::response("ok".into(), StatusCode::OK)
    }
    fn handle_stop() -> Result<Response<Full<Bytes>>, hyper::Error> {
        info!("try to stop");
        HttpServer::response("ok".into(), StatusCode::OK)
    }
    fn response(s: String, code: StatusCode) -> Result<Response<Full<Bytes>>, hyper::Error> {
        let mut res = Response::new(Full::new(Bytes::from(s)));
        *res.status_mut() = code;
        Ok(res)
    }
}

impl Service<Request<IncomingBody>> for HttpServer {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/start") => return Box::pin(async { HttpServer::handle_start() }),
            (&Method::GET, "/stop") => return Box::pin(async { HttpServer::handle_stop() }),
            (&Method::GET, "/stop") => return Box::pin(async { HttpServer::handle_start() }),
            _ => {
                return Box::pin(async {
                    HttpServer::response("".to_owned(), StatusCode::NOT_FOUND)
                })
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn socket_parser_test_should_pass_localhost() {
        let server = HttpServer::new("localhost".to_string(), 8080);
        let result = server.get_socket_addr();
        assert_eq!(result.is_ok(), true);
    }
    #[test]
    #[should_panic]
    fn socket_parser_test_should_not_pass_badadress() {
        let server = HttpServer::new("bad_address".to_string(), 8080);
        let result = server.get_socket_addr();
    }

    #[test]
    fn socket_parser_test_should_pass_proper_ip() {
        let server = HttpServer::new("127.0.0.1".to_string(), 8080);
        let result = server.get_socket_addr();
        assert_eq!(result.is_ok(), true);
    }
}
