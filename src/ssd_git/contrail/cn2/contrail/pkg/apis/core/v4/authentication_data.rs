// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.AuthenticationData

/// Authentication related configuration for this session like type, keys etc. Only md5 authentication is supported.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AuthenticationData {
    /// Upto two keys can be specified. Currently, only one key is supported.
    pub key_items: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AuthenticationKeyItem>>,

    /// Authentication type for this session. Currently, only MD5 is supported.
    pub key_type: Option<String>,
}

impl k8s_openapi::DeepMerge for AuthenticationData {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.key_items, other.key_items);
        k8s_openapi::DeepMerge::merge_from(&mut self.key_type, other.key_type);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for AuthenticationData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_key_items,
            Key_key_type,
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
                            "keyItems" => Field::Key_key_items,
                            "keyType" => Field::Key_key_type,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = AuthenticationData;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AuthenticationData")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_key_items: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AuthenticationKeyItem>> = None;
                let mut value_key_type: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_key_items => value_key_items = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_key_type => value_key_type = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AuthenticationData {
                    key_items: value_key_items,
                    key_type: value_key_type,
                })
            }
        }

        deserializer.deserialize_struct(
            "AuthenticationData",
            &[
                "keyItems",
                "keyType",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for AuthenticationData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AuthenticationData",
            self.key_items.as_ref().map_or(0, |_| 1) +
            self.key_type.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.key_items {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "keyItems", value)?;
        }
        if let Some(value) = &self.key_type {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "keyType", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AuthenticationData {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.AuthenticationData".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Authentication related configuration for this session like type, keys etc. Only md5 authentication is supported.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "keyItems".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Upto two keys can be specified. Currently, only one key is supported.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AuthenticationKeyItem>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "keyType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Authentication type for this session. Currently, only MD5 is supported.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
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
