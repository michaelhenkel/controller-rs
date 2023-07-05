// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPPrefixLimit

/// This is used to configure the maximum number of received prefixes and control the behavior of the session when the maximum is reached.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BGPPrefixLimit {
    /// Time in seconds after which the session is allowed to re-establish after teardown.
    pub idle_timeout: Option<i32>,

    /// Maximum number of prefixes allowed to be recieved
    pub maximum: Option<i32>,
}

impl k8s_openapi::DeepMerge for BGPPrefixLimit {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.idle_timeout, other.idle_timeout);
        k8s_openapi::DeepMerge::merge_from(&mut self.maximum, other.maximum);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for BGPPrefixLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_idle_timeout,
            Key_maximum,
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
                            "idleTimeout" => Field::Key_idle_timeout,
                            "maximum" => Field::Key_maximum,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = BGPPrefixLimit;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BGPPrefixLimit")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_idle_timeout: Option<i32> = None;
                let mut value_maximum: Option<i32> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_idle_timeout => value_idle_timeout = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_maximum => value_maximum = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BGPPrefixLimit {
                    idle_timeout: value_idle_timeout,
                    maximum: value_maximum,
                })
            }
        }

        deserializer.deserialize_struct(
            "BGPPrefixLimit",
            &[
                "idleTimeout",
                "maximum",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for BGPPrefixLimit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BGPPrefixLimit",
            self.idle_timeout.as_ref().map_or(0, |_| 1) +
            self.maximum.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.idle_timeout {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "idleTimeout", value)?;
        }
        if let Some(value) = &self.maximum {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "maximum", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BGPPrefixLimit {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPPrefixLimit".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("This is used to configure the maximum number of received prefixes and control the behavior of the session when the maximum is reached.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "idleTimeout".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Time in seconds after which the session is allowed to re-establish after teardown.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "maximum".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Maximum number of prefixes allowed to be recieved".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
