// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.RouteOriginOverride

/// RouteOriginOverride is used to override route origin attribute.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RouteOriginOverride {
    /// User define route origin value
    pub origin: Option<String>,

    /// Set true to override Route origin with the given value
    pub origin_override: Option<bool>,
}

impl k8s_openapi::DeepMerge for RouteOriginOverride {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.origin, other.origin);
        k8s_openapi::DeepMerge::merge_from(&mut self.origin_override, other.origin_override);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for RouteOriginOverride {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_origin,
            Key_origin_override,
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
                            "origin" => Field::Key_origin,
                            "originOverride" => Field::Key_origin_override,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = RouteOriginOverride;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RouteOriginOverride")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_origin: Option<String> = None;
                let mut value_origin_override: Option<bool> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_origin => value_origin = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_origin_override => value_origin_override = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RouteOriginOverride {
                    origin: value_origin,
                    origin_override: value_origin_override,
                })
            }
        }

        deserializer.deserialize_struct(
            "RouteOriginOverride",
            &[
                "origin",
                "originOverride",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for RouteOriginOverride {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RouteOriginOverride",
            self.origin.as_ref().map_or(0, |_| 1) +
            self.origin_override.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.origin {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "origin", value)?;
        }
        if let Some(value) = &self.origin_override {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "originOverride", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for RouteOriginOverride {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.RouteOriginOverride".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("RouteOriginOverride is used to override route origin attribute.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "origin".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("User define route origin value".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "originOverride".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Set true to override Route origin with the given value".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
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
