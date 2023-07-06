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
    //Reconciler
};
use tokio::time::Duration;
use async_trait::async_trait;


pub struct RoutingInstanceReconciler{
    client: Client,
    group: &'static str,
    version: &'static str,
    kind: &'static str,
}

/*
#[async_trait]
impl Reconciler<v4::RoutingInstanceSpec,v4::RoutingInstanceStatus> for RoutingInstanceReconciler{
    fn new(client: Client, group: &'static str, version: &'static str, kind: &'static str) -> Self {
        Self{
            client,
            group,
            version,
            kind,
        }
    }
    
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
}
*/

