// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.AuthenticationKeyItem

/// AuthenticationKeyItem is used for BGP session authentication configuration.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AuthenticationKeyItem {
    pub key: Option<String>,

    pub key_id: Option<i32>,
}

impl k8s_openapi::DeepMerge for AuthenticationKeyItem {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.key, other.key);
        k8s_openapi::DeepMerge::merge_from(&mut self.key_id, other.key_id);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for AuthenticationKeyItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_key,
            Key_key_id,
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
                            "key" => Field::Key_key,
                            "keyId" => Field::Key_key_id,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = AuthenticationKeyItem;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AuthenticationKeyItem")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_key: Option<String> = None;
                let mut value_key_id: Option<i32> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_key => value_key = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_key_id => value_key_id = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AuthenticationKeyItem {
                    key: value_key,
                    key_id: value_key_id,
                })
            }
        }

        deserializer.deserialize_struct(
            "AuthenticationKeyItem",
            &[
                "key",
                "keyId",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for AuthenticationKeyItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AuthenticationKeyItem",
            self.key.as_ref().map_or(0, |_| 1) +
            self.key_id.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.key {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "key", value)?;
        }
        if let Some(value) = &self.key_id {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "keyId", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AuthenticationKeyItem {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.AuthenticationKeyItem".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("AuthenticationKeyItem is used for BGP session authentication configuration.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "key".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "keyId".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
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
