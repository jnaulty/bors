use bors::Service;
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Server,
};

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = Service::new();

    // The closure inside `make_service_fn` is run for each connection,
    // creating a 'service' to handle requests for that specific connection.
    let make_service = make_service_fn(move |socket: &AddrStream| {
        println!("remote address: {:?}", socket.remote_addr());

        // While the state was moved into the make_service closure,
        // we need to clone it here because this closure is called
        // once for every connection.
        let service = service.clone();

        async move {
            // This is the `Service` that will handle the connection.
            Ok::<_, hyper::Error>(service_fn(move |request| {
                let service = service.clone();
                service.serve(request)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
