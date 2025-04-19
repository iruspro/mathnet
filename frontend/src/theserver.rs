//use hyper::{Body, Request, Response};
//use crate::tokio::net::windows::named_pipe::PipeEnd::Server;
//use hyper::service::{make_service_fn, service_fn};
//use std::convert::Infallible;
//
//async fn serve(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//    let path = req.uri().path();
//    // Serve index.html for any route
//    if path != "/index.html" && path != "/" {
//        return Ok(Response::new(Body::from(include_str!("./index.html"))));
//    }
//
//    Ok(Response::new(Body::from("Not Found")))
//}
//
//pub async fn start_server() {
//    let make_svc = make_service_fn(|_conn| async {
//        Ok::<_, Infallible>(service_fn(serve))
//    });
//
//    let addr = ([127, 0, 0, 1], 3000).into(); // Server address
//    let servert = hyper::Server::bind(&addr).serve(make_svc);
//
//    println!("Server running at http://{}", addr);
//    if let Err(e) = servert.await {
//        eprintln!("Server error: {}", e);
//    }
//}


