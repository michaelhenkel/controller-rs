// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceStatus

/// BGPAsAServiceStatus defines the observed state of BGPAsAService.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BGPAsAServiceStatus {
    /// BGPRouterReferences contains references to all BGPRouters created for a for BGPAsAService session.
    pub bgp_router_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPRouterReference>>,

    /// Observation provides additional information related to the state of the resource. For example, if a reconciliation error occurs, Observation will contain a brief description of the problem.
    pub observation: String,

    /// State describe the current readiness of a resource after the last reconciliation. The possible states include Pending, Success, and Failure.
    pub state: String,

    /// SubnetReferences contains references to all subnets associated with the selected VirtualMachineInterfaces' VirtualNetwork.
    pub subnet_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SubnetReference>>,

    /// VMISelectorReferences contains references to all VirtualMachineInterfaces selected by VirtualMachineInterfacesSelector.
    pub vmi_selector_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,
}

impl k8s_openapi::DeepMerge for BGPAsAServiceStatus {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.bgp_router_references, other.bgp_router_references);
        k8s_openapi::DeepMerge::merge_from(&mut self.observation, other.observation);
        k8s_openapi::DeepMerge::merge_from(&mut self.state, other.state);
        k8s_openapi::merge_strategies::list::atomic(&mut self.subnet_references, other.subnet_references);
        k8s_openapi::merge_strategies::list::atomic(&mut self.vmi_selector_references, other.vmi_selector_references);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for BGPAsAServiceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_bgp_router_references,
            Key_observation,
            Key_state,
            Key_subnet_references,
            Key_vmi_selector_references,
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
                            "bgpRouterReferences" => Field::Key_bgp_router_references,
                            "observation" => Field::Key_observation,
                            "state" => Field::Key_state,
                            "subnetReferences" => Field::Key_subnet_references,
                            "vmiSelectorReferences" => Field::Key_vmi_selector_references,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = BGPAsAServiceStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BGPAsAServiceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_bgp_router_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPRouterReference>> = None;
                let mut value_observation: Option<String> = None;
                let mut value_state: Option<String> = None;
                let mut value_subnet_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SubnetReference>> = None;
                let mut value_vmi_selector_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_bgp_router_references => value_bgp_router_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observation => value_observation = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subnet_references => value_subnet_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vmi_selector_references => value_vmi_selector_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BGPAsAServiceStatus {
                    bgp_router_references: value_bgp_router_references,
                    observation: value_observation.unwrap_or_default(),
                    state: value_state.unwrap_or_default(),
                    subnet_references: value_subnet_references,
                    vmi_selector_references: value_vmi_selector_references,
                })
            }
        }

        deserializer.deserialize_struct(
            "BGPAsAServiceStatus",
            &[
                "bgpRouterReferences",
                "observation",
                "state",
                "subnetReferences",
                "vmiSelectorReferences",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for BGPAsAServiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BGPAsAServiceStatus",
            2 +
            self.bgp_router_references.as_ref().map_or(0, |_| 1) +
            self.subnet_references.as_ref().map_or(0, |_| 1) +
            self.vmi_selector_references.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.bgp_router_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "bgpRouterReferences", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "observation", &self.observation)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "state", &self.state)?;
        if let Some(value) = &self.subnet_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "subnetReferences", value)?;
        }
        if let Some(value) = &self.vmi_selector_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "vmiSelectorReferences", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BGPAsAServiceStatus {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("BGPAsAServiceStatus defines the observed state of BGPAsAService.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "bgpRouterReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGPRouterReferences contains references to all BGPRouters created for a for BGPAsAService session.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPRouterReference>()))),
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
                        "subnetReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("SubnetReferences contains references to all subnets associated with the selected VirtualMachineInterfaces' VirtualNetwork.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SubnetReference>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "vmiSelectorReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VMISelectorReferences contains references to all VirtualMachineInterfaces selected by VirtualMachineInterfacesSelector.".to_owned()),
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
