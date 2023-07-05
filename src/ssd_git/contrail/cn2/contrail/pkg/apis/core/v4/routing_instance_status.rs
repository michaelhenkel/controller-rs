// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceStatus

/// RoutingInstanceStatus defines the observed state of the RoutingInstance.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoutingInstanceStatus {
    /// DefaultRouteTargetReference contains a reference to the default RouteTarget and the import/export mode in their attributes. Only set by the system as user must pass by higher level resources to add remove Route Target.
    pub default_route_target_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RouteTargetReference>,

    /// Is this the default routing instance for the VirtualNetwork? This field contains internal service chaining information, and should not be modified.
    pub is_default: Option<bool>,

    /// Observation provides additional information related to the state of the resource. For example, if a reconciliation error occurs, Observation will contain a brief description of the problem.
    pub observation: String,

    /// FabricSNAT toggles connectivity to underlay network by port mapping.
    pub routing_instance_fabric_snat: Option<bool>,

    /// State describe the current readiness of a resource after the last reconciliation. The possible states include Pending, Success, and Failure.
    pub state: String,

    /// VirtualNetworkRouterRouteTargetReferences are RouteTarget references of VirtualNetworkRouters selecting this RoutingInstance's parent VirtualNetwork, as well as those of imported VirtualNetworkRouters.
    pub virtual_network_router_route_target_references: Option<std::collections::BTreeMap<String, crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualNetworkRouteTargetReferenceList>>,
}

impl k8s_openapi::DeepMerge for RoutingInstanceStatus {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.default_route_target_reference, other.default_route_target_reference);
        k8s_openapi::DeepMerge::merge_from(&mut self.is_default, other.is_default);
        k8s_openapi::DeepMerge::merge_from(&mut self.observation, other.observation);
        k8s_openapi::DeepMerge::merge_from(&mut self.routing_instance_fabric_snat, other.routing_instance_fabric_snat);
        k8s_openapi::DeepMerge::merge_from(&mut self.state, other.state);
        k8s_openapi::merge_strategies::map::granular(&mut self.virtual_network_router_route_target_references, other.virtual_network_router_route_target_references, |current_item, other_item| {
            k8s_openapi::DeepMerge::merge_from(current_item, other_item);
        });
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for RoutingInstanceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_default_route_target_reference,
            Key_is_default,
            Key_observation,
            Key_routing_instance_fabric_snat,
            Key_state,
            Key_virtual_network_router_route_target_references,
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
                            "defaultRouteTargetReference" => Field::Key_default_route_target_reference,
                            "isDefault" => Field::Key_is_default,
                            "observation" => Field::Key_observation,
                            "routingInstanceFabricSNAT" => Field::Key_routing_instance_fabric_snat,
                            "state" => Field::Key_state,
                            "virtualNetworkRouterRouteTargetReferences" => Field::Key_virtual_network_router_route_target_references,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = RoutingInstanceStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RoutingInstanceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_default_route_target_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RouteTargetReference> = None;
                let mut value_is_default: Option<bool> = None;
                let mut value_observation: Option<String> = None;
                let mut value_routing_instance_fabric_snat: Option<bool> = None;
                let mut value_state: Option<String> = None;
                let mut value_virtual_network_router_route_target_references: Option<std::collections::BTreeMap<String, crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualNetworkRouteTargetReferenceList>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default_route_target_reference => value_default_route_target_reference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_is_default => value_is_default = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observation => value_observation = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_routing_instance_fabric_snat => value_routing_instance_fabric_snat = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_network_router_route_target_references => value_virtual_network_router_route_target_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RoutingInstanceStatus {
                    default_route_target_reference: value_default_route_target_reference,
                    is_default: value_is_default,
                    observation: value_observation.unwrap_or_default(),
                    routing_instance_fabric_snat: value_routing_instance_fabric_snat,
                    state: value_state.unwrap_or_default(),
                    virtual_network_router_route_target_references: value_virtual_network_router_route_target_references,
                })
            }
        }

        deserializer.deserialize_struct(
            "RoutingInstanceStatus",
            &[
                "defaultRouteTargetReference",
                "isDefault",
                "observation",
                "routingInstanceFabricSNAT",
                "state",
                "virtualNetworkRouterRouteTargetReferences",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for RoutingInstanceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RoutingInstanceStatus",
            2 +
            self.default_route_target_reference.as_ref().map_or(0, |_| 1) +
            self.is_default.as_ref().map_or(0, |_| 1) +
            self.routing_instance_fabric_snat.as_ref().map_or(0, |_| 1) +
            self.virtual_network_router_route_target_references.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.default_route_target_reference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultRouteTargetReference", value)?;
        }
        if let Some(value) = &self.is_default {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "isDefault", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "observation", &self.observation)?;
        if let Some(value) = &self.routing_instance_fabric_snat {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "routingInstanceFabricSNAT", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "state", &self.state)?;
        if let Some(value) = &self.virtual_network_router_route_target_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualNetworkRouterRouteTargetReferences", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for RoutingInstanceStatus {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("RoutingInstanceStatus defines the observed state of the RoutingInstance.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "defaultRouteTargetReference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RouteTargetReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("DefaultRouteTargetReference contains a reference to the default RouteTarget and the import/export mode in their attributes. Only set by the system as user must pass by higher level resources to add remove Route Target.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "isDefault".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Is this the default routing instance for the VirtualNetwork? This field contains internal service chaining information, and should not be modified.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
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
                        "routingInstanceFabricSNAT".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FabricSNAT toggles connectivity to underlay network by port mapping.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
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
                        "virtualNetworkRouterRouteTargetReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualNetworkRouterRouteTargetReferences are RouteTarget references of VirtualNetworkRouters selecting this RoutingInstance's parent VirtualNetwork, as well as those of imported VirtualNetworkRouters.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualNetworkRouteTargetReferenceList>())),
                                ..Default::default()
                            })),
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
