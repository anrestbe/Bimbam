mod cli;
mod ops;
mod utils;
mod build_cache; 

#[tokio::main]
async fn main() -> Result<(), String> {
    cli::run_cli().await
}
