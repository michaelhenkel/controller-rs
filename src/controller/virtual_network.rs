use std::sync::Arc;
use crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4 as v4;
use kube::{
    api::DynamicObject,
    runtime::controller::Action
};
use crate::controller::controller::{
    ReconcileError,
    Context,
    get_spec_status
};
use tokio::time::Duration;

pub struct VirtualNetworkReconciler{}

impl VirtualNetworkReconciler{
    pub async fn reconcile(g: Arc<DynamicObject>, _ctx: Arc<Context>) -> Result<Action, ReconcileError> {
        let (spec, status) = get_spec_status(g.as_ref()).map_err(|e| ReconcileError(e))?;
        let vn = v4::VirtualNetwork{
            metadata: Some(g.metadata.clone()),
            spec: Some(spec),
            status: Some(status),
        };
        println!("reconcile {:#?}", vn);
        Ok(Action::requeue(Duration::from_secs(300)))
    }
}