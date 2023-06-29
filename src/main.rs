mod controller;
mod protos;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let client = kube::Client::try_default().await?;
    controller::controller::start(client).await?;
    Ok(())
}
