use anyhow;
use clap::Parser;
use http_fs::{process_http_serve, Args};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let arg = Args::parse();
    process_http_serve(arg.dir_path, arg.port).await
}
