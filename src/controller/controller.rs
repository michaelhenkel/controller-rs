use kube::{
    api::{Api, DynamicObject, GroupVersionKind, Object},
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
use futures::Future;
use crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4 as v4;


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
    pub kind: String,
    pub version: String,
    pub group: String,
}

impl Context{
    pub async fn new(group: String, version: String, kind: String, client: Client) -> anyhow::Result<Arc<Context>> {
        let gvk = GroupVersionKind::gvk(&group, &version, &kind);
        let (ar, _caps) = kube::discovery::pinned_kind(&client, &gvk).await?;
    
        let api = Api::<DynamicObject>::all_with(client.clone(),&ar);
        Ok(Arc::new(Context{
            client,
            ar,
            api,
            kind,
            version,
            group,
        }))
    }
}

pub async fn start(client: Client) -> anyhow::Result<()> {
    let mut join_handles = Vec::new();
    let cloned_client = client.clone();
    join_handles.push(tokio::spawn(async move{
        let reconciler = Reconciler2::new(
            cloned_client,
            GROUP.to_string(),
            VERSION.to_string(),
            "VirtualNetwork".to_string()).await?;
        reconciler.run(virtual_network::VirtualNetworkReconciler::reconcile).await
    }));
    let cloned_client = client.clone();

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

pub struct Reconciler2{
    pub context: Arc<Context>,
}

impl Reconciler2{
    pub async fn new(client: Client, group: String, version: String, kind: String) -> anyhow::Result<Self> 
    {
        let context = Context::new(group, version, kind, client).await?;
        Ok(Self{
            context,
        })
    }
    async fn run<F,Fut>(&self, reconcile: F) -> anyhow::Result<()>
    where
        F: Fn(Arc<DynamicObject>, Arc<Context>) -> Fut + Send + 'static,
        Fut: Future<Output = Result<Action, ReconcileError>> + Send + 'static,
    {
        Controller::new_with(self.context.as_ref().api.clone(), watcher::Config::default(), self.context.as_ref().ar.clone())
            .run(reconcile, Self::error_policy, self.context.clone())
            .for_each(|res| async move {
                match res {
                    Ok(o) => println!("reconciled {}/{}", o.0.namespace.as_deref().unwrap_or_default(), o.0.name),
                    Err(e) => println!("reconcile failed: {:?}",e),
                }
            }).await;
        Ok(())
    }
    fn error_policy(_obj: Arc<DynamicObject>, _error: &ReconcileError, _ctx: Arc<Context>) -> Action {
        Action::requeue(tokio::time::Duration::from_secs(60))
    }
}
/*
#[async_trait]
pub trait Reconciler<S, C>
where
    S: Clone + Debug + Send + DeserializeOwned + serde::Serialize + 'static,
    C: Clone + Debug + Send + DeserializeOwned + serde::Serialize + 'static,
{
    fn new(client: Client, group: &'static str, version: &'static str, kind: &'static str) -> Self;
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
    
    async fn reconcile(g: Arc<DynamicObject>, _ctx: Arc<Context>) -> Result<Action, ReconcileError> {
        let (spec, status) = get_spec_status::<S,C>(g.as_ref()).map_err(|e| ReconcileError(e))?;
        let o = Object{
            types: g.types.clone(),
            metadata: g.metadata.clone(),
            spec: Some(spec),
            status: Some(status),
        };
        println!("reconcile {:#?}", o);
        Ok(Action::requeue(tokio::time::Duration::from_secs(300)))
    }
    fn error_policy(_obj: Arc<DynamicObject>, _error: &ReconcileError, _ctx: Arc<Context>) -> Action {
        Action::requeue(tokio::time::Duration::from_secs(60))
    }
}
*/



