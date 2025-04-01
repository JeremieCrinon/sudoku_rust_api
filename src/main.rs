use std::net::SocketAddr;
use tokio::net::TcpListener;
use log::info;

mod handlers;
mod routes;
mod sudoku;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get the .env infos
    dotenv::dotenv().ok();
    // Get the config file for log4rs
    log4rs::init_file("logconfig.yml", Default::default()).expect("Log config file not found.");

    // Tells in the console that the server is starting
    info!("Starting server...");

    // We call the function create_router from the routes module (./routes/mod.rs) that will return a axum router
    let app = routes::create_router();

    // Bind the adress 0.0.0.0 to accept connexion any IP, and 3000 for the port
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // We make a TCP listener
    let listener = TcpListener::bind(addr).await?;

    // Start serving the app with Axum
    axum::serve(listener, app.into_make_service()).await?;
    
    // Everything is ok
    Ok(())
}