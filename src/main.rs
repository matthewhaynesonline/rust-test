mod http_server;
use http_server::server;

fn main() {
    // https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
    let addr = "0.0.0.0";
    let port = "8088";

    server(addr, port);
}
