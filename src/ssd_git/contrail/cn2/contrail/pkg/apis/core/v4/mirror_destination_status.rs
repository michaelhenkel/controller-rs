// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationStatus

/// MirrorDestinationStatus defines the observed state of MirrorDestination
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MirrorDestinationStatus {
    /// InstanceIPReference is used to infer destination IPAdrress for mirroring traffic when using this MirrorDestination
    pub instanceipference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,

    /// Observation provides additional information related to the state of the resource. For example, if a reconciliation error occurs, Observation will contain a brief description of the problem.
    pub observation: String,

    /// State describe the current readiness of a resource after the last reconciliation. The possible states include Pending, Success, and Failure.
    pub state: String,

    /// VirtualMachineInterfaceReference is VirtualMachineInterface used to mirror traffic to when using this MirrorDestination
    pub virtualmachineinterfacereference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,
}

impl k8s_openapi::DeepMerge for MirrorDestinationStatus {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.instanceipference, other.instanceipference);
        k8s_openapi::DeepMerge::merge_from(&mut self.observation, other.observation);
        k8s_openapi::DeepMerge::merge_from(&mut self.state, other.state);
        k8s_openapi::DeepMerge::merge_from(&mut self.virtualmachineinterfacereference, other.virtualmachineinterfacereference);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for MirrorDestinationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_instanceipference,
            Key_observation,
            Key_state,
            Key_virtualmachineinterfacereference,
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
                            "instanceipference" => Field::Key_instanceipference,
                            "observation" => Field::Key_observation,
                            "state" => Field::Key_state,
                            "virtualmachineinterfacereference" => Field::Key_virtualmachineinterfacereference,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = MirrorDestinationStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("MirrorDestinationStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_instanceipference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;
                let mut value_observation: Option<String> = None;
                let mut value_state: Option<String> = None;
                let mut value_virtualmachineinterfacereference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_instanceipference => value_instanceipference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observation => value_observation = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtualmachineinterfacereference => value_virtualmachineinterfacereference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(MirrorDestinationStatus {
                    instanceipference: value_instanceipference,
                    observation: value_observation.unwrap_or_default(),
                    state: value_state.unwrap_or_default(),
                    virtualmachineinterfacereference: value_virtualmachineinterfacereference,
                })
            }
        }

        deserializer.deserialize_struct(
            "MirrorDestinationStatus",
            &[
                "instanceipference",
                "observation",
                "state",
                "virtualmachineinterfacereference",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for MirrorDestinationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "MirrorDestinationStatus",
            2 +
            self.instanceipference.as_ref().map_or(0, |_| 1) +
            self.virtualmachineinterfacereference.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.instanceipference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "instanceipference", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "observation", &self.observation)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "state", &self.state)?;
        if let Some(value) = &self.virtualmachineinterfacereference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualmachineinterfacereference", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for MirrorDestinationStatus {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("MirrorDestinationStatus defines the observed state of MirrorDestination".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "instanceipference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("InstanceIPReference is used to infer destination IPAdrress for mirroring traffic when using this MirrorDestination".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
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
                        "virtualmachineinterfacereference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualMachineInterfaceReference is VirtualMachineInterface used to mirror traffic to when using this MirrorDestination".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
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
