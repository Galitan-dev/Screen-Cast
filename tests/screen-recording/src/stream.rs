use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::thread;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

pub async fn main() {

    thread::spawn(|| {

    let d = Display::primary().unwrap();
    let (w, h) = (d.width(), d.height());

        let mut capturer = Capturer::new(d).unwrap();

        loop {
            match capturer.frame() {
                Ok(frame) => {
                    // Write the frame, removing end-of-row padding.
                    let stride = frame.len() / h;
                    let rowlen = 4 * w;
                    for row in frame.chunks(stride) {
                        let row = &row[..rowlen];
                        println!("{}", row.len());
                        // out.write_all(row).unwrap();
                    }
                }
                Err(ref e) if e.kind() == WouldBlock => {
                    // Wait for the frame.
                }
                Err(_) => {
                    // We're done here.
                    break;
                }
            }
        }
    });


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}