use futures::future::ready;
use futures::FutureExt;
use hyper::{
    server::Server,
    service::{make_service_fn, service_fn},
    {Body, Response},
};
use prometheus::{gather, Encoder, TextEncoder};
use std::convert::Infallible;
use tokio::spawn;

pub fn serve(port: u16) {
    let make_service = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(|_req| async {
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            let metric_families = gather();
            encoder.encode(&metric_families, &mut buffer).unwrap();

            Ok::<_, Infallible>(Response::new(Body::from(buffer)))
        }))
    });

    let server = Server::bind(&([0, 0, 0, 0], port).into()).serve(make_service);

    spawn(server.then(|r| {
        if let Err(e) = r {
            eprintln!("[Server] Exit with error: {}", e);
        }
        ready(())
    }));
}
