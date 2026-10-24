#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    tracing::info!("worker scaffold started; durable job adapter is not wired yet");
    tokio::signal::ctrl_c().await?;
    Ok(())
}
