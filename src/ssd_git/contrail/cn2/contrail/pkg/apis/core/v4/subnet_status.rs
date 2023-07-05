// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.SubnetStatus

/// SubnetStatus defines the observed state of a Subnet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubnetStatus {
    /// AllocationUsage is current percentage of allocated addresses in the Subnet.
    pub allocation_usage: Option<String>,

    /// IPCount is the current number of allocated IP addresses in the Subnet.
    pub ip_count: Option<i64>,

    /// Observation provides additional information related to the state of the resource. For example, if a reconciliation error occurs, Observation will contain a brief description of the problem.
    pub observation: String,

    /// State describe the current readiness of a resource after the last reconciliation. The possible states include Pending, Success, and Failure.
    pub state: String,
}

impl k8s_openapi::DeepMerge for SubnetStatus {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.allocation_usage, other.allocation_usage);
        k8s_openapi::DeepMerge::merge_from(&mut self.ip_count, other.ip_count);
        k8s_openapi::DeepMerge::merge_from(&mut self.observation, other.observation);
        k8s_openapi::DeepMerge::merge_from(&mut self.state, other.state);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for SubnetStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocation_usage,
            Key_ip_count,
            Key_observation,
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
                            "allocationUsage" => Field::Key_allocation_usage,
                            "ipCount" => Field::Key_ip_count,
                            "observation" => Field::Key_observation,
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
            type Value = SubnetStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SubnetStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_allocation_usage: Option<String> = None;
                let mut value_ip_count: Option<i64> = None;
                let mut value_observation: Option<String> = None;
                let mut value_state: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocation_usage => value_allocation_usage = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip_count => value_ip_count = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observation => value_observation = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SubnetStatus {
                    allocation_usage: value_allocation_usage,
                    ip_count: value_ip_count,
                    observation: value_observation.unwrap_or_default(),
                    state: value_state.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "SubnetStatus",
            &[
                "allocationUsage",
                "ipCount",
                "observation",
                "state",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for SubnetStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SubnetStatus",
            2 +
            self.allocation_usage.as_ref().map_or(0, |_| 1) +
            self.ip_count.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocation_usage {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "allocationUsage", value)?;
        }
        if let Some(value) = &self.ip_count {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "ipCount", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "observation", &self.observation)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "state", &self.state)?;
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SubnetStatus {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.SubnetStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("SubnetStatus defines the observed state of a Subnet.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allocationUsage".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AllocationUsage is current percentage of allocated addresses in the Subnet.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ipCount".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IPCount is the current number of allocated IP addresses in the Subnet.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
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
