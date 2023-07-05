// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceStatus

/// VirtualMachineInterfaceStatus defines the observed state of VirtualMachineInterface
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualMachineInterfaceStatus {
    /// BGPRouterReference is bgpaas-client BGPRouter reference for BGP neighbor. Holds the corresponding BGPRouterRef from BGPAsAService
    pub bgp_router_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,

    /// Reference to the interface route table attached to this interface.
    pub interface_route_table_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,

    /// Observation provides additional information related to the state of the resource. For example, if a reconciliation error occurs, Observation will contain a brief description of the problem.
    pub observation: String,

    /// RoutingInstanceReferences lists all routing instance the interface is associated to. Should at least contains the reference to its Virtual Network primary Routing Instance.
    pub routing_instance_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RoutingInstanceReference>>,

    /// ServiceHealthCheck is ServiceHealthCheck reference
    pub service_health_check_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,

    /// State describe the current readiness of a resource after the last reconciliation. The possible states include Pending, Success, and Failure.
    pub state: String,
}

impl k8s_openapi::DeepMerge for VirtualMachineInterfaceStatus {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.bgp_router_reference, other.bgp_router_reference);
        k8s_openapi::merge_strategies::list::atomic(&mut self.interface_route_table_references, other.interface_route_table_references);
        k8s_openapi::DeepMerge::merge_from(&mut self.observation, other.observation);
        k8s_openapi::merge_strategies::list::atomic(&mut self.routing_instance_references, other.routing_instance_references);
        k8s_openapi::merge_strategies::list::atomic(&mut self.service_health_check_references, other.service_health_check_references);
        k8s_openapi::DeepMerge::merge_from(&mut self.state, other.state);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for VirtualMachineInterfaceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_bgp_router_reference,
            Key_interface_route_table_references,
            Key_observation,
            Key_routing_instance_references,
            Key_service_health_check_references,
            Key_state,
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
                            "bgpRouterReference" => Field::Key_bgp_router_reference,
                            "interfaceRouteTableReferences" => Field::Key_interface_route_table_references,
                            "observation" => Field::Key_observation,
                            "routingInstanceReferences" => Field::Key_routing_instance_references,
                            "serviceHealthCheckReferences" => Field::Key_service_health_check_references,
                            "state" => Field::Key_state,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualMachineInterfaceStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VirtualMachineInterfaceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_bgp_router_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;
                let mut value_interface_route_table_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;
                let mut value_observation: Option<String> = None;
                let mut value_routing_instance_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RoutingInstanceReference>> = None;
                let mut value_service_health_check_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;
                let mut value_state: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_bgp_router_reference => value_bgp_router_reference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_interface_route_table_references => value_interface_route_table_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observation => value_observation = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_routing_instance_references => value_routing_instance_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_health_check_references => value_service_health_check_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VirtualMachineInterfaceStatus {
                    bgp_router_reference: value_bgp_router_reference,
                    interface_route_table_references: value_interface_route_table_references,
                    observation: value_observation.unwrap_or_default(),
                    routing_instance_references: value_routing_instance_references,
                    service_health_check_references: value_service_health_check_references,
                    state: value_state.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "VirtualMachineInterfaceStatus",
            &[
                "bgpRouterReference",
                "interfaceRouteTableReferences",
                "observation",
                "routingInstanceReferences",
                "serviceHealthCheckReferences",
                "state",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for VirtualMachineInterfaceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VirtualMachineInterfaceStatus",
            2 +
            self.bgp_router_reference.as_ref().map_or(0, |_| 1) +
            self.interface_route_table_references.as_ref().map_or(0, |_| 1) +
            self.routing_instance_references.as_ref().map_or(0, |_| 1) +
            self.service_health_check_references.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.bgp_router_reference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "bgpRouterReference", value)?;
        }
        if let Some(value) = &self.interface_route_table_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "interfaceRouteTableReferences", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "observation", &self.observation)?;
        if let Some(value) = &self.routing_instance_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "routingInstanceReferences", value)?;
        }
        if let Some(value) = &self.service_health_check_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceHealthCheckReferences", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "state", &self.state)?;
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VirtualMachineInterfaceStatus {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VirtualMachineInterfaceStatus defines the observed state of VirtualMachineInterface".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "bgpRouterReference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGPRouterReference is bgpaas-client BGPRouter reference for BGP neighbor. Holds the corresponding BGPRouterRef from BGPAsAService".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "interfaceRouteTableReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Reference to the interface route table attached to this interface.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>()))),
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
                        "routingInstanceReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("RoutingInstanceReferences lists all routing instance the interface is associated to. Should at least contains the reference to its Virtual Network primary Routing Instance.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RoutingInstanceReference>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "serviceHealthCheckReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ServiceHealthCheck is ServiceHealthCheck reference".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>()))),
                                ..Default::default()
                            })),
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
