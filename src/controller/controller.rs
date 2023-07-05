use kube::{
    api::{Api, DynamicObject, GroupVersionKind},
    Client,
    runtime::{watcher,{controller::{Controller, Action}}},
};
use futures::StreamExt;
use std::sync::Arc;
use kube::discovery::ApiResource;
use serde::de::DeserializeOwned;
use serde_json;
use std::{fmt::Debug};
use async_trait::async_trait;
use std::error::Error;
use crate::controller::virtual_network;
use super::routing_instance;

const GROUP: &str = "core.contrail.juniper.net";
const VERSION: &str = "v4";

#[derive(Debug)]
pub struct ReconcileError(pub anyhow::Error);
impl Error for ReconcileError {

}
impl std::fmt::Display for ReconcileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"ReconcileError: {}", self.0)
    }
}

pub struct Context{
    pub client: Client,
    pub ar: ApiResource,
    pub api: Api<DynamicObject>,
}

impl Context{
    pub async fn new(group: &str, version: &str, kind: &str, client: Client) -> anyhow::Result<Arc<Context>> {
        let gvk = GroupVersionKind::gvk(&group, &version, &kind);
        let (ar, _caps) = kube::discovery::pinned_kind(&client, &gvk).await?;
    
        let api = Api::<DynamicObject>::all_with(client.clone(),&ar);
        Ok(Arc::new(Context{
            client,
            ar,
            api,
        }))
    }
}

pub async fn start(client: Client) -> anyhow::Result<()> {
    let mut join_handles = Vec::new();
    let cloned_client = client.clone();
    join_handles.push(tokio::spawn(async move{
        virtual_network::VirtualNetworkReconciler::new(
            cloned_client,
            GROUP,
            VERSION,
            "VirtualNetwork"
        ).run().await
    }));
    let cloned_client = client.clone();
    join_handles.push(tokio::spawn(async move {
        routing_instance::RoutingInstanceReconciler::new(
            cloned_client,
            GROUP,
            VERSION,
            "RoutingInstance"
        ).run().await
    }));


    futures::future::join_all(join_handles).await;
    Ok(())
}

pub fn get_spec_status<S, C>(p: &DynamicObject) -> anyhow::Result<(S, C)>
where
    S: Clone + Debug + Send + DeserializeOwned + serde::Serialize + 'static,
    C: Clone + Debug + Send + DeserializeOwned + serde::Serialize + 'static,
{
    let json_value = serde_json::to_value(p)?;
    let spec_value = json_value.get("spec").unwrap().clone();
    let spec: S = serde_json::from_value(spec_value)?;
    let status_value = json_value.get("status").unwrap().clone();
    let status: C = serde_json::from_value(status_value)?;
    Ok((spec, status))
}

#[async_trait]
pub trait Reconciler{
    async fn run(&self) -> anyhow::Result<()> {
        let context = Context::new(self.group(), self.version(), self.kind(), self.client().clone()).await?;
        Controller::new_with(context.as_ref().api.clone(), watcher::Config::default(), context.as_ref().ar.clone())
            .run(Self::reconcile, Self::error_policy, context)
            .for_each(|res| async move {
                match res {
                    Ok(o) => println!("reconciled {}/{}", o.0.namespace.as_deref().unwrap_or_default(), o.0.name),
                    Err(e) => println!("reconcile failed: {:?}",e),
                }
            }).await;
        Ok(())
    }
    fn group(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn kind(&self) -> &'static str;
    fn client(&self) -> Client;
    async fn reconcile(g: Arc<DynamicObject>, _ctx: Arc<Context>) -> Result<Action, ReconcileError>;
    fn error_policy(obj: Arc<DynamicObject>, _error: &ReconcileError, _ctx: Arc<Context>) -> Action;
}

/* 
#[macro_export]
macro_rules! reconcile {
    ($resource:ident) => {{

            let (spec, status) = get_spec_status(g.as_ref()).map_err(|e| ReconcileError(e))?;
            let ri = $resource{
                metadata: Some(g.metadata.clone()),
                spec: Some(spec),
                status: Some(status),
            };
            println!("reconcile {:#?}", ri);
            Ok(Action::requeue(Duration::from_secs(300)))
        
    }};
}
*/



