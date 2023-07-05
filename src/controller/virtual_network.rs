use std::sync::Arc;
use crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4 as v4;
use kube::{
    api::DynamicObject,
    Client,
    runtime::controller::Action
};
use crate::controller::controller::{
    ReconcileError,
    Context,
    get_spec_status, 
    Reconciler
};
use tokio::time::Duration;
use async_trait::async_trait;

pub struct VirtualNetworkReconciler{
    client: Client,
    group: &'static str,
    version: &'static str,
    kind: &'static str,
}

impl VirtualNetworkReconciler {
    pub fn new(client: Client, group: &'static str, version: &'static str, kind: &'static str) -> Self {
        VirtualNetworkReconciler{
            client,
            group,
            version,
            kind,
        }
    }
}

#[async_trait]
impl Reconciler for VirtualNetworkReconciler{
    fn group(&self) -> &'static str {
        self.group
    }
    fn version(&self) -> &'static str {
        self.version
    }
    fn kind(&self) -> &'static str {
        self.kind
    }
    fn client(&self) -> Client {
        self.client.clone()
    }
 
    async fn reconcile(g: Arc<DynamicObject>, _ctx: Arc<Context>) -> Result<Action, ReconcileError> {
        let (spec, status) = get_spec_status(g.as_ref()).map_err(|e| ReconcileError(e))?;
        let vn = v4::VirtualNetwork{
            metadata: Some(g.metadata.clone()),
            spec: Some(spec),
            status: Some(status),
        };
        println!("reconcile {:#?}", vn);
        Ok(Action::requeue(Duration::from_secs(300)))
    }
    
    fn error_policy(_obj: Arc<DynamicObject>, _error: &ReconcileError, _ctx: Arc<Context>) -> Action {
        Action::requeue(Duration::from_secs(60))
    }
}