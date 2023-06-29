use tokio::sync::mpsc;
use std::{sync::Arc, f32::consts::E};
use tokio::sync::Mutex;
use crate::protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v4 as v4;
use kube::{
    api::{Api, DynamicObject, GroupVersionKind, Object},
    runtime::watcher, Client,
};
use std::env;
use crate::controller;
use async_trait::async_trait;

use super::reconciler::Reconciler;

const GROUP: &str = "core.contrail.juniper.net";
const VERSION: &str = "v4";
const KIND: &str = "VirtualNetwork";


pub struct VirtualNetworkReconciler{
    pub tx: mpsc::Sender<Object<v4::VirtualNetworkSpec,v4::VirtualNetworkStatus>>,
    rx: Arc<Mutex<mpsc::Receiver<Object<v4::VirtualNetworkSpec,v4::VirtualNetworkStatus>>>>,
    group: String,
    version: String,
    kind: String,
}

impl VirtualNetworkReconciler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        VirtualNetworkReconciler{
            tx,
            rx: Arc::new(Mutex::new(rx)),
            group: GROUP.to_string(),
            version: VERSION.to_string(),
            kind: KIND.to_string(),
        }
    }

    pub async fn recv(&self) -> anyhow::Result<()> {
        let mut rx = self.rx.lock().await;
        while let Some(obj) = rx.recv().await {
            println!("virtual_network_spec: {:#?}", obj.spec);
        }
        Ok(())
    }
}



#[async_trait]
impl Reconciler for VirtualNetworkReconciler{
    async fn run(&self, client: Client) -> anyhow::Result<()> {
        let (api, wc, ar) = match controller::controller::watch_config(&self.group, &self.version, &self.kind, client).await {
            Ok((api, wc, ar)) => (api, wc, ar),
            Err(e) => {
                return Err(anyhow::anyhow!("Error: {:?}", e));
            }
        };
        let reconciler = VirtualNetworkReconciler::new();
        let tx = reconciler.tx.clone();
        let mut join_handlers = Vec::new();

        join_handlers.push(tokio::spawn(async move {
            reconciler.recv().await
        }));

        join_handlers.push(tokio::spawn(async move {
            controller::controller::handle_events::<DynamicObject,v4::VirtualNetworkSpec,v4::VirtualNetworkStatus>(watcher(api, wc), &ar, tx).await
        }));
        futures::future::join_all(join_handlers).await;
        Ok(())
    }
    fn gvk(&self) -> String {
        format!("{}/{}/{}", self.group, self.version, self.kind)
    }
}