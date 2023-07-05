// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterReferenceAttributes

/// BGPRouterReferenceAttributes defines the attributes for 1 (typically) or more sessions between two BGP Routers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BGPRouterReferenceAttributes {
    /// Seesion is a list of BGP sessions parameters. There can be multiple BGP sessions between two BGP routers. Currently, only 1 session is supported.
    pub session: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPSession>>,
}

impl k8s_openapi::DeepMerge for BGPRouterReferenceAttributes {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.session, other.session);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for BGPRouterReferenceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_session,
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
                            "session" => Field::Key_session,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = BGPRouterReferenceAttributes;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BGPRouterReferenceAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_session: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPSession>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_session => value_session = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BGPRouterReferenceAttributes {
                    session: value_session,
                })
            }
        }

        deserializer.deserialize_struct(
            "BGPRouterReferenceAttributes",
            &[
                "session",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for BGPRouterReferenceAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BGPRouterReferenceAttributes",
            self.session.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.session {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "session", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BGPRouterReferenceAttributes {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterReferenceAttributes".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("BGPRouterReferenceAttributes defines the attributes for 1 (typically) or more sessions between two BGP Routers.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "session".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Seesion is a list of BGP sessions parameters. There can be multiple BGP sessions between two BGP routers. Currently, only 1 session is supported.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPSession>()))),
                                ..Default::default()
                            })),
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
