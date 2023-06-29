

use controller_rs::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v4 as v4;
//use controller_rs::k8s::io::apimachinery::pkg::apis::meta::v1 as meta_v1;
use controller_rs::k8s::io::api::core::v1 as core_v1;
use futures::{Stream, StreamExt, TryStreamExt};
use kube::{
    api::{Api, DynamicObject, GroupVersionKind, Resource, Object},
    runtime::{watcher, watcher::Event, WatchStreamExt},
};
use kube::discovery::ApiResource;
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use serde_json;
use std::{env, fmt::Debug};
use controller_rs::reconciler;
use tokio::sync::mpsc;

mod controller;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let client = kube::Client::try_default().await?;

    let group = env::var("GROUP").unwrap_or_else(|_| "".into());
    let version = env::var("VERSION").unwrap_or_else(|_| "v1".into());
    let kind = env::var("KIND").unwrap_or_else(|_| "Pod".into());
    
    let gvk = GroupVersionKind::gvk(&group, &version, &kind);
    let (ar, _caps) = kube::discovery::pinned_kind(&client, &gvk).await?;

    let api = Api::<DynamicObject>::all_with(client.clone(),&ar);
    let wc = watcher::Config::default();

    let vn_reconciler = reconciler::virtual_network::VirtualNetworkReconciler::new();
    let vn_tx = vn_reconciler.tx.clone();
    let mut join_handlers = Vec::new();

    join_handlers.push(tokio::spawn(async move {
        vn_reconciler.recv().await
    }));

    join_handlers.push(tokio::spawn(async move {
        handle_events::<DynamicObject,v4::VirtualNetworkSpec,v4::VirtualNetworkStatus>(watcher(api, wc), &ar, vn_tx).await
    }));
    futures::future::join_all(join_handlers).await;
    Ok(())
}

pub async fn handle_events<K, S, C>(stream: impl Stream<Item = watcher::Result<Event<K>>> + Send + 'static, ar: &ApiResource, tx: mpsc::Sender<Object<S,C>>) -> anyhow::Result<()> 
where
    K: Resource + Clone + Debug + Send + DeserializeOwned + serde::Serialize + 'static,
    S: Clone + Debug + Send + DeserializeOwned + serde::Serialize + 'static,
    C: Clone + Debug + Send + DeserializeOwned + serde::Serialize + 'static,
{
    let mut items = stream.applied_objects().boxed();
    while let Some(p) = items.try_next().await? {

        let o = convert::<K, S, C>(p, ar)?;
        tx.send(o).await;
        //println!("virtual_network_spec: {:#?}", o.spec);
    }
    Ok(())
}

fn convert<K, S, C>(p: K, ar: &ApiResource) -> anyhow::Result<Object<S,C>>
where
    K: Resource + Clone + Debug + Send + DeserializeOwned + serde::Serialize + 'static,
    S: Clone + Debug + Send + DeserializeOwned + serde::Serialize + 'static,
    C: Clone + Debug + Send + DeserializeOwned + serde::Serialize + 'static,
{

    let json_value = serde_json::to_value(&p)?;

    let mut spec_value = json_value.get("spec").unwrap().clone();
    convert_reference(&mut spec_value);
    let spec: S = serde_json::from_value(spec_value)?;
    let status_value = json_value.get("status").unwrap().clone();
    let status: C = serde_json::from_value(status_value)?;

    let o: Object<S,C> = Object{
        types: Some(kube::core::TypeMeta { 
            api_version: ar.api_version.clone(), 
            kind: ar.kind.clone() }),
        metadata: p.meta().clone(),
        spec,
        status: Some(status),
    };
    Ok(o)
}

fn convert_reference(json: &mut serde_json::Value) {
    let mut fq_name = HashMap::new();
    match json {
        serde_json::Value::Object(obj) => {
            obj.iter_mut().for_each(|(key, value)| {
                if key == "fqName" {
                    let common_spec = match value {
                        serde_json::Value::Array(fqn) => {
                            let mut fq_name_list = Vec::new();
                            for f in fqn {
                                fq_name_list.push(f.as_str().unwrap().to_string());
                            }
                            let common_spec = v4::CommonSpec{
                                contrail_fq_name: Some(v4::ContrailFqName { 
                                    fq_name: fq_name_list
                                }),
                            };
                            Some(common_spec)
                        },
                        _ => {
                            None
                        },
                    };
                    fq_name.insert("commonSpec".to_string(), serde_json::to_value(&common_spec).unwrap());
                }
                if key.ends_with("Reference") {
                    let fq_name = match value.get("fqName").unwrap(){
                        serde_json::Value::Array(fqn) => {
                            let mut fq_name_list = Vec::new();
                            for f in fqn {
                                fq_name_list.push(f.as_str().unwrap().to_string());
                            }
                            let fq_name_obj = v4::ContrailFqName { 
                                fq_name: fq_name_list
                            };
                            Some(fq_name_obj)
                        },
                        _ => {
                            None
                        },
                    };
  
                    let resource_reference = v4::ResourceReference {
                        object_reference: Some(core_v1::ObjectReference {
                            name: value.get("name").map(|f| f.as_str().unwrap_or_default().to_string()),
                            namespace: value.get("namespace").map(|f| f.as_str().unwrap_or_default().to_string()),
                            kind: value.get("kind").map(|f| f.as_str().unwrap_or_default().to_string()),
                            uid: value.get("uid").map(|f| f.as_str().unwrap_or_default().to_string()),
                            api_version: value.get("apiVersion").map(|f| f.as_str().unwrap_or_default().to_string()),
                            resource_version: value.get("resourceVersion").map(|f| f.as_str().unwrap_or_default().to_string()),
                            field_path: None,
                        }),
                        contrail_fq_name: fq_name,
                    };
                    *value = serde_json::to_value(&resource_reference).unwrap();
                }
                if key.ends_with("References") {
                    let mut resource_references = Vec::new();
                    match value {
                        serde_json::Value::Array(arr) => {
                            for v in arr {
                                let fq_name = match v.get("fqName").unwrap(){
                                    serde_json::Value::Array(fqn) => {
                                        let mut fq_name_list = Vec::new();
                                        for f in fqn {
                                            fq_name_list.push(f.as_str().unwrap().to_string());
                                        }
                                        let fq_name_obj = v4::ContrailFqName { 
                                            fq_name: fq_name_list
                                        };
                                        Some(fq_name_obj)
                                    },
                                    _ => {
                                        None
                                    },
                                };
                                let resource_reference = v4::ResourceReference {
                                    object_reference: Some(core_v1::ObjectReference {
                                        name: v.get("name").map(|f| f.as_str().unwrap_or_default().to_string()),
                                        namespace: v.get("namespace").map(|f| f.as_str().unwrap_or_default().to_string()),
                                        kind: v.get("kind").map(|f| f.as_str().unwrap_or_default().to_string()),
                                        uid: v.get("uid").map(|f| f.as_str().unwrap_or_default().to_string()),
                                        api_version: v.get("apiVersion").map(|f| f.as_str().unwrap_or_default().to_string()),
                                        resource_version: v.get("resourceVersion").map(|f| f.as_str().unwrap_or_default().to_string()),
                                        field_path: None,
                                    }),
                                    contrail_fq_name: fq_name,
                                };
                                resource_references.push(resource_reference);
                            }
                        },
                        _ => {}
                    }
                    *value = serde_json::to_value(&resource_references).unwrap();
                }
            });
            for (key, value) in fq_name.iter() {
                obj.insert(key.clone(), value.clone());
            }
        },
        _ => {}
    }
}
