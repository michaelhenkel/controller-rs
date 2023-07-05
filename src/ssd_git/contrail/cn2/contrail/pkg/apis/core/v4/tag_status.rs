// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.TagStatus

/// TagStatus defines the observed state of Tag.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TagStatus {
    /// Observation provides additional information related to the state of the resource. For example, if a reconciliation error occurs, Observation will contain a brief description of the problem.
    pub observation: String,

    /// State describe the current readiness of a resource after the last reconciliation. The possible states include Pending, Success, and Failure.
    pub state: String,

    /// TagID is an internal representation of Tag object encapsulating tag type and value in hexadecimal format.
    pub tag_id: Option<String>,
}

impl k8s_openapi::DeepMerge for TagStatus {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.observation, other.observation);
        k8s_openapi::DeepMerge::merge_from(&mut self.state, other.state);
        k8s_openapi::DeepMerge::merge_from(&mut self.tag_id, other.tag_id);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for TagStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_observation,
            Key_state,
            Key_tag_id,
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
                            "observation" => Field::Key_observation,
                            "state" => Field::Key_state,
                            "tagId" => Field::Key_tag_id,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = TagStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TagStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_observation: Option<String> = None;
                let mut value_state: Option<String> = None;
                let mut value_tag_id: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_observation => value_observation = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tag_id => value_tag_id = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TagStatus {
                    observation: value_observation.unwrap_or_default(),
                    state: value_state.unwrap_or_default(),
                    tag_id: value_tag_id,
                })
            }
        }

        deserializer.deserialize_struct(
            "TagStatus",
            &[
                "observation",
                "state",
                "tagId",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for TagStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TagStatus",
            2 +
            self.tag_id.as_ref().map_or(0, |_| 1),
        )?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "observation", &self.observation)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "state", &self.state)?;
        if let Some(value) = &self.tag_id {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "tagId", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TagStatus {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.TagStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("TagStatus defines the observed state of Tag.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
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
                        "tagId".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TagID is an internal representation of Tag object encapsulating tag type and value in hexadecimal format.".to_owned()),
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
