use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v4 as v4;
use kube::{
    api::{Api, DynamicObject, GroupVersionKind, Object},
    runtime::watcher, Client,
};
use std::env;
use crate::controller;


pub struct VirtualNetworkReconciler{
    pub tx: mpsc::Sender<Object<v4::VirtualNetworkSpec,v4::VirtualNetworkStatus>>,
    rx: Arc<Mutex<mpsc::Receiver<Object<v4::VirtualNetworkSpec,v4::VirtualNetworkStatus>>>>,
}

impl VirtualNetworkReconciler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        VirtualNetworkReconciler{
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
    pub async fn send(&self, obj: Object<v4::VirtualNetworkSpec,v4::VirtualNetworkStatus>) -> anyhow::Result<()> {
        self.tx.send(obj).await?;
        Ok(())
    }
    pub async fn recv(&self) -> anyhow::Result<()> {
        let mut rx = self.rx.lock().await;
        while let Some(obj) = rx.recv().await {
            println!("virtual_network_spec: {:#?}", obj.spec);
        }
        Ok(())
    }
}

pub async fn run(client: Client) -> anyhow::Result<Vec<tokio::task::JoinHandle<Result<(), anyhow::Error>>>> {
    let group = env::var("GROUP").unwrap_or_else(|_| "".into());
    let version = env::var("VERSION").unwrap_or_else(|_| "v1".into());
    let kind = env::var("KIND").unwrap_or_else(|_| "Pod".into());
    
    let gvk = GroupVersionKind::gvk(&group, &version, &kind);
    let (ar, _caps) = kube::discovery::pinned_kind(&client, &gvk).await?;

    let api = Api::<DynamicObject>::all_with(client.clone(),&ar);
    let wc = watcher::Config::default();

    let vn_reconciler = VirtualNetworkReconciler::new();
    let vn_tx = vn_reconciler.tx.clone();
    let mut join_handlers = Vec::new();

    join_handlers.push(tokio::spawn(async move {
        vn_reconciler.recv().await
    }));

    join_handlers.push(tokio::spawn(async move {
        controller::controller::handle_events::<DynamicObject,v4::VirtualNetworkSpec,v4::VirtualNetworkStatus>(watcher(api, wc), &ar, vn_tx).await
    }));
    Ok(join_handlers)
}