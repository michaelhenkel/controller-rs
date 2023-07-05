// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkStatus

/// VirtualNetworkStatus defines the observed state of a VirtualNetwork.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualNetworkStatus {
    /// EvpnRoutingVirtualNetworkReferences lists of all internal virtual network of virtual network routers, where this virtual network connected.
    pub evpn_routing_virtual_network_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualNetworkReference>>,

    /// Observation provides additional information related to the state of the resource. For example, if a reconciliation error occurs, Observation will contain a brief description of the problem.
    pub observation: String,

    /// State describe the current readiness of a resource after the last reconciliation. The possible states include Pending, Success, and Failure.
    pub state: String,

    /// System assigned unique 32-bit ID for every virtual network.
    pub virtual_network_network_id: Option<i64>,
}

impl k8s_openapi::DeepMerge for VirtualNetworkStatus {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.evpn_routing_virtual_network_references, other.evpn_routing_virtual_network_references);
        k8s_openapi::DeepMerge::merge_from(&mut self.observation, other.observation);
        k8s_openapi::DeepMerge::merge_from(&mut self.state, other.state);
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_network_network_id, other.virtual_network_network_id);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for VirtualNetworkStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_evpn_routing_virtual_network_references,
            Key_observation,
            Key_state,
            Key_virtual_network_network_id,
            Other,
        }

        impl<'de> k8s_openapi::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: k8s_openapi::serde::de::Error {
                        Ok(match v {
                            "evpnRoutingVirtualNetworkReferences" => Field::Key_evpn_routing_virtual_network_references,
                            "observation" => Field::Key_observation,
                            "state" => Field::Key_state,
                            "virtualNetworkNetworkId" => Field::Key_virtual_network_network_id,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualNetworkStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VirtualNetworkStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_evpn_routing_virtual_network_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualNetworkReference>> = None;
                let mut value_observation: Option<String> = None;
                let mut value_state: Option<String> = None;
                let mut value_virtual_network_network_id: Option<i64> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_evpn_routing_virtual_network_references => value_evpn_routing_virtual_network_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observation => value_observation = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_network_network_id => value_virtual_network_network_id = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VirtualNetworkStatus {
                    evpn_routing_virtual_network_references: value_evpn_routing_virtual_network_references,
                    observation: value_observation.unwrap_or_default(),
                    state: value_state.unwrap_or_default(),
                    virtual_network_network_id: value_virtual_network_network_id,
                })
            }
        }

        deserializer.deserialize_struct(
            "VirtualNetworkStatus",
            &[
                "evpnRoutingVirtualNetworkReferences",
                "observation",
                "state",
                "virtualNetworkNetworkId",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for VirtualNetworkStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VirtualNetworkStatus",
            2 +
            self.evpn_routing_virtual_network_references.as_ref().map_or(0, |_| 1) +
            self.virtual_network_network_id.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.evpn_routing_virtual_network_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "evpnRoutingVirtualNetworkReferences", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "observation", &self.observation)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "state", &self.state)?;
        if let Some(value) = &self.virtual_network_network_id {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualNetworkNetworkId", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VirtualNetworkStatus {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VirtualNetworkStatus defines the observed state of a VirtualNetwork.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "evpnRoutingVirtualNetworkReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("EvpnRoutingVirtualNetworkReferences lists of all internal virtual network of virtual network routers, where this virtual network connected.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualNetworkReference>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "observation".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Observation provides additional information related to the state of the resource. For example, if a reconciliation error occurs, Observation will contain a brief description of the problem.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "state".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("State describe the current readiness of a resource after the last reconciliation. The possible states include Pending, Success, and Failure.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "virtualNetworkNetworkId".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("System assigned unique 32-bit ID for every virtual network.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "observation".to_owned(),
                    "state".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
