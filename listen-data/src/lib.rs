#[cfg(test)]
#[ctor::ctor]
fn init() {
    dotenvy::from_filename(".env").ok();
    std::env::set_var("RUST_LOG", "debug");
    let _ = tracing_subscriber::fmt::try_init();
}

pub mod constants;
pub mod diffs;

#[cfg(feature = "rpc")]
pub mod rpc;

#[cfg(feature = "geyser")]
pub mod geyser;

pub mod db;
pub mod kv_store;
pub mod message_queue;
pub mod metadata;
pub mod metrics;
pub mod price;
pub mod process_swap;
pub mod raydium_intruction_processor;
pub mod raydium_processor;
pub mod sol_price_stream;
pub mod util;

#[cfg(test)]
pub mod debug;
