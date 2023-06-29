use std::collections::HashMap;
use crate::reconciler::reconciler::Reconciler;

pub struct Watcher{
    registry: HashMap<String, Box<dyn Reconciler>>,
    client: kube::Client,
}

impl Watcher{
    pub fn new(client: kube::Client) -> Self {
        Watcher{
            registry: HashMap::new(),
            client,
        }
    }
    pub fn register(&mut self, name: String, reconciler: Box<dyn Reconciler>) {
        self.registry.insert(name, reconciler);
    }
    pub async fn run(self) -> anyhow::Result<()> {
        let client = kube::Client::try_default().await?;
        let mut join_handles = Vec::new();
        for (name, reconciler) in self.registry {
            println!("Watching for {}", reconciler.gvk());
            let client = client.clone();
            join_handles.push(tokio::spawn(async move {
                reconciler.run(client).await
            }));
        }
        println!("setup all watchers");
        futures::future::join_all(join_handles).await;
        println!("failed");
        Ok(())
    }
}