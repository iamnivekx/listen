pub mod db;
pub mod error;
pub mod redis_client;
pub mod redis_subscriber;
pub mod routes;
pub mod state;
pub mod tls;
pub mod websocket;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    let _ = tracing_subscriber::fmt::try_init();
}
