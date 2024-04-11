// use bytes::Bytes;
// use clap::{parser, Parser};
// use http_body_util::Full;
// use hyper::server::conn::http1;
// use hyper::service::Service;
// use hyper::{body::Incoming as IncomingBody, Request, Response};
// use hyper_util::rt::TokioIo;
// use log::info;
use std::error::Error;
// use std::future::Future;
// use std::pin::Pin;
// use std::sync::{Arc, Mutex};
// use tokio::net::TcpListener;
mod args;
mod errors;
mod logger;
mod server;

use server::HttpServer;

// #[derive(Debug, Clone)]
// struct Svc {
//     counter: Arc<Mutex<i32>>,
// }
// impl Service<Request<IncomingBody>> for Svc {
//     type Response = Response<Full<Bytes>>;
//     type Error = hyper::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

//     fn call(&self, req: Request<IncomingBody>) -> Self::Future {
//         fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
//             Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
//         }

//         if req.uri().path() != "/favicon.ico" {
//             *self.counter.lock().expect("lock poisoned") += 1;
//         }

//         let res = match req.uri().path() {
//             "/" => mk_response(format!("home! counter = {:?}", self.counter)),
//             "/posts" => mk_response(format!("posts, of course! counter = {:?}", self.counter)),
//             "/authors" => mk_response(format!(
//                 "authors extraordinare! counter = {:?}",
//                 self.counter
//             )),
//             // Return the 404 Not Found for other routes, and don't increment counter.
//             _ => return Box::pin(async { mk_response("oh no! not found".into()) }),
//         };

//         Box::pin(async { res })
//     }
// }
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::enable_logger();
    let mut server = HttpServer::new("localhost".to_string(), 3000);
    server.start().await?;
    // let listener = TcpListener::bind("localhost:3000").await?;
    // println!("Listening on http://localhost:3000");
    // // let parser = args::ArgParser::parse();
    // // println!("Hello, world! {}", parser.get_host());
    // let svc = Svc {
    //     counter: Arc::new(Mutex::new(0)),
    // };

    // loop {
    //     let (stream, _) = listener.accept().await?;
    //     let io = TokioIo::new(stream);
    //     let svc_clone = svc.clone();
    //     tokio::task::spawn(async move {
    //         if let Err(err) = http1::Builder::new().serve_connection(io, svc_clone).await {
    //             println!("Failed to serve connection: {:?}", err);
    //         }
    //     });
    // }
    Ok(())
}
