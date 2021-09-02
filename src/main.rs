use std::net::TcpListener;

use skel::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind address");
   run(listener)?.await
}
