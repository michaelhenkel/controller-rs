use std::sync::Arc;
use crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4 as v4;
use serde::de::DeserializeOwned;
use kube::{Resource, Client};
use kube_runtime::reflector::ObjectRef;
use kube::{
    api::DynamicObject,
    runtime::controller::Action,
};
use crate::controller::controller::{
    ReconcileError,
    Context,
    get_spec_status,
};
use tokio::time::Duration;
use std::fmt::Debug;
use kube::api::GroupVersionKind;
use kube::discovery::ApiResource;
use kube::ResourceExt;
use kube_runtime::watcher::Config;
use kube::Api;



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
 
    pub async fn watch_opts() -> Result<Vec<(&'static str, &'static str, &'static str, Box<impl Fn(DynamicObject) -> Option<ObjectRef<DynamicObject>> + Clone + Send + Sync + 'static>)>, Box<dyn std::error::Error + Send>> 
    {
        let mut res_vec = Vec::new();
        let f = | dyn_obj: DynamicObject|  {
            println!("dyn_obj: {:#?}", dyn_obj);
            match dyn_obj.metadata.owner_references{
                Some(owner_ref) => {
                    for owner_ref in owner_ref{
                        if owner_ref.kind == "VirtualNetwork"{
                            let name = owner_ref.name.as_str();
                            let namespace = dyn_obj.metadata.namespace.unwrap();
                            let ar = ApiResource::from_gvk(&GroupVersionKind::gvk("core.contrail.juniper.net", "v4", "VirtualNetwork"));
                            let obj_ref: Option<ObjectRef<DynamicObject>> = Some(ObjectRef::new_with(name, ar).within(namespace.as_str()));
                            return obj_ref;
                        }
                    }
                    None
                },
                None => { None }
            }
        };
        res_vec.push(("core.contrail.juniper.net","v4","RoutingInstance", Box::new(f)));
        
        Ok(res_vec)
    }

}