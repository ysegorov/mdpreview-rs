use std::env;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

mod cli;
mod handlers;
mod markdown;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = cli::cli();
    let port:u16 = args.value_of("port").unwrap().parse().unwrap();
    let rootdir = env::current_dir()?.to_str().unwrap().to_string();

    let make_service = make_service_fn(move |_conn| {

        let dir = rootdir.clone();
        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handlers::handle_request(req, dir.clone())
            }))
        }
    });

    let addr = ([127, 0, 0, 1], port).into();

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }

    Ok(())
}
