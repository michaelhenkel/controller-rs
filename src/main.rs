mod controller;
mod protos;
mod reconciler;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let client = kube::Client::try_default().await?;
    reconciler::reconciler::start(client).await?;
    Ok(())
}
