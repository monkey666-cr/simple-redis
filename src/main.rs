use anyhow::Result;
use simple_redis::{stream_handler, Backend};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";
    info!("Simple-Redis-Server is listening on {}", addr);
    let listnener = TcpListener::bind(addr).await?;

    let backend = Backend::new();
    loop {
        let (stream, raddr) = listnener.accept().await?;
        info!("Accepted connection from {}", raddr);
        let backend_clone = backend.clone();
        tokio::spawn(async move {
            match stream_handler(stream, backend_clone).await {
                Ok(_) => info!("Connection from {} closed", raddr),
                Err(e) => info!("Connection from {} closed with error: {}", raddr, e),
            };
        });
    }
}
