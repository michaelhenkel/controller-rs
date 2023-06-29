use kube::Client;
use crate::reconciler::virtual_network;
use crate::controller::watcher::Watcher;
use async_trait::async_trait;

pub async fn start(client: Client) -> anyhow::Result<()> {
    let mut watcher = Watcher::new(client.clone());

    let vn_reconciler = virtual_network::VirtualNetworkReconciler::new();
    watcher.register("virtual_network".to_string(), Box::new(vn_reconciler));


    let mut join_handles = Vec::new();
    join_handles.push(tokio::spawn(async move {
        watcher.run().await
    }));
    futures::future::join_all(join_handles).await;
    Ok(())
}

#[async_trait]
pub trait Reconciler: Send + Sync{
    async fn run(&self, client: Client) -> anyhow::Result<()>;
    fn gvk(&self) -> String;
}