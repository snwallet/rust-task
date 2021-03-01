#![deny(warnings)]

mod controller;
mod model;
mod lib;

use crate::app::controller::*;

use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};


pub async fn run(port: u16) {
    let addr = ([127, 0, 0, 1], port).into();
    let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(router)) });
    let server = Server::bind(&addr).serve(make_svc);
    println!("server run at http://{}", addr);
    if let Err(e) = server.await { eprintln!("server error: {}", e); }
}

async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => file_controller::main(req).await,
        (&Method::POST, "/test") => test_controller::main(req).await,
        (&Method::POST, "/param") => param_controller::main(req).await,
        //user
        (&Method::POST, "/user/login") => user::login::main(req).await,
        (&Method::POST, "/user/repwd") => user::repwd::main(req).await,
        (&Method::POST, "/user/insert") => user::insert::main(req).await,
        (&Method::POST, "/user/select") => user::select::main(req).await,
        (&Method::POST, "/user/update") => user::update::main(req).await,
        (&Method::POST, "/user/delete") => user::delete::main(req).await,

        //task
        (&Method::POST, "/task/add_user") => task::add_user::main(req).await,
        (&Method::POST, "/task/end_user") => task::end_user::main(req).await,
        (&Method::POST, "/task/insert") => task::insert::main(req).await,
        (&Method::POST, "/task/select") => task::select::main(req).await,
        (&Method::POST, "/task/update") => task::update::main(req).await,
        (&Method::POST, "/task/delete") => task::delete::main(req).await,

        _ => Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("".to_string())).unwrap()),
    }
}
