

use controller_rs::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v4 as v4;
use controller_rs::k8s::io::apimachinery::pkg::apis::meta::v1 as meta_v1;
use futures::{Stream, StreamExt, TryStreamExt};
use kube::{
    api::{Api, DynamicObject, GroupVersionKind, Resource, ResourceExt},
    runtime::{metadata_watcher, watcher, watcher::Event, WatchStreamExt},
};
use k8s_openapi::api::core::v1::Pod;
use kube::discovery::{ApiResource};
use std::{borrow::Cow, collections::HashMap};
use serde::de::DeserializeOwned;
use tracing::*;
use prost::Message;

use std::{env, fmt::Debug};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let client = kube::Client::try_default().await?;
    let vn = v4::VirtualNetwork::default();
    // If set will receive only the metadata for watched resources
    let watch_metadata = env::var("WATCH_METADATA").map(|s| s == "1").unwrap_or(false);

    // Take dynamic resource identifiers:
    let group = env::var("GROUP").unwrap_or_else(|_| "".into());
    let version = env::var("VERSION").unwrap_or_else(|_| "v1".into());
    let kind = env::var("KIND").unwrap_or_else(|_| "Pod".into());
    
    // Turn them into a GVK
    let gvk = GroupVersionKind::gvk(&group, &version, &kind);
    // Use API discovery to identify more information about the type (like its plural)
    let (ar, _caps) = kube::discovery::pinned_kind(&client, &gvk).await?;

    // Use the full resource info to create an Api with the ApiResource as its DynamicType
    let api = Api::<DynamicObject>::all_with(client.clone(),&ar);
    let wc = watcher::Config::default();

    handle_events(watcher(api, wc)).await
}

async fn handle_events<K: Resource + Clone + Debug + Send + DeserializeOwned + serde::Serialize + 'static>(
    stream: impl Stream<Item = watcher::Result<Event<K>>> + Send + 'static,
) -> anyhow::Result<()> {
    let mut items = stream.applied_objects().boxed();
    while let Some(p) = items.try_next().await? {
        
        let mut metadata = meta_v1::ObjectMeta::default();
        if let Some(ns) = p.namespace() {
            metadata.namespace = Some(ns.into());
        }
        metadata.name = Some(p.name_any().into());
        metadata.uid = Some(p.uid().unwrap());
        metadata.resource_version = Some(p.resource_version().unwrap());
        metadata.labels = to_map(p.labels());
        metadata.annotations = to_map(p.annotations());

        let mut object = v4::VirtualNetwork::default();
        object.metadata = Some(metadata);

        let json_value = serde_json::to_value(&p)?;
        println!("json_value: {:#?}", json_value);
        let spec_value = json_value.get("spec").unwrap().clone();
        let spec: v4::VirtualNetworkSpec = serde_json::from_value(spec_value)?;
        println!("spec: {:#?}", spec);
        let status_value = json_value.get("status").unwrap().clone();
        let status: v4::VirtualNetworkStatus = serde_json::from_value(status_value)?;
        println!("status: {:#?}", status);

        object.spec = Some(spec);
        object.status = Some(status);

        println!("object: {:#?}", object);
        trace!("full obj: {p:?}");
    }
    Ok(())
}

// fn to_map converts a BtrreeMap to a HashMap
fn to_map<K: Ord + Clone + std::hash::Hash, V: Clone>(b: &std::collections::BTreeMap<K, V>) -> std::collections::HashMap<K, V> {
    b.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}