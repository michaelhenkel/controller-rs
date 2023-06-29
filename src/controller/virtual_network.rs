use std::sync::Arc;
use crate::protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v4 as v4;
use kube::{
    api::DynamicObject,
    runtime::watcher, Client
};
use kube::runtime::{controller::{Controller, Action}};
use futures::StreamExt;
use crate::controller;
use tokio::time::Duration;
use thiserror::Error;

const GROUP: &str = "core.contrail.juniper.net";
const VERSION: &str = "v4";
const KIND: &str = "VirtualNetwork";

#[derive(Debug, Error)]
enum Error {}

pub struct VirtualNetworkReconciler{
    client: Client,
}

impl VirtualNetworkReconciler {
    pub fn new(client: Client) -> Self {
        VirtualNetworkReconciler{
            client,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let (api, ar) = match controller::controller::watch_config(GROUP, VERSION, KIND, self.client.clone()).await{
            Ok((api, ar)) => {(api, ar)},
            Err(e) => {
                println!("watch_config failed: {:?}", e);
                return Err(e)
            }
        };
        let context = Arc::new(controller::controller::Context{
            client: self.client.clone(),
            ar: ar.clone(),
        });
        Controller::new_with(api, watcher::Config::default(), ar)
            //.owns(cms, Config::default())
            .run(reconcile, error_policy, context)
            .for_each(|res| async move {
                match res {
                    Ok(o) => println!("reconciled {}/{}", o.0.namespace.as_deref().unwrap_or_default(), o.0.name),
                    Err(e) => println!("reconcile failed: {:?}",e),
                }
            }) .await;
        Ok(())
    }
}

async fn reconcile(g: Arc<DynamicObject>, _ctx: Arc<controller::controller::Context>) -> Result<Action, Error> {
    let g = g.clone();
    let o = controller::controller::convert::<DynamicObject, v4::VirtualNetworkSpec, v4::VirtualNetworkStatus>(g.as_ref(), &_ctx.get_ar());
    println!("reconcile {:#?}", o);
    Ok(Action::requeue(Duration::from_secs(300)))
}

fn error_policy(obj: Arc<DynamicObject>, _error: &Error, _ctx: Arc<controller::controller::Context>) -> Action {
    Action::requeue(Duration::from_secs(60))
}