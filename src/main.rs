use std::net::SocketAddr;
use tokio::net::TcpListener;
use log::info;

mod handlers;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    log4rs::init_file("logconfig.yml", Default::default()).expect("Log config file not found.");

    info!("Starting server...");

    let app = routes::create_router();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;
    
    Ok(())
}