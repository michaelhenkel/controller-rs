// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPSessionIPAttributes

/// BGPSessionIPAttributes contains BGPSession primary and secondary IP addresses.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BGPSessionIPAttributes {
    /// BGPaaSPrimaryIP defines the primary IP address used for a BGP session.
    pub bgpaas_primary_ip: Option<String>,

    /// BGPaaSSecondaryIP defines the secondary IP address used for a BGP session when a second control node is present.
    pub bgpaas_secondary_ip: Option<String>,
}

impl k8s_openapi::DeepMerge for BGPSessionIPAttributes {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.bgpaas_primary_ip, other.bgpaas_primary_ip);
        k8s_openapi::DeepMerge::merge_from(&mut self.bgpaas_secondary_ip, other.bgpaas_secondary_ip);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for BGPSessionIPAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_bgpaas_primary_ip,
            Key_bgpaas_secondary_ip,
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
                            "bgpaasPrimaryIP" => Field::Key_bgpaas_primary_ip,
                            "bgpaasSecondaryIP" => Field::Key_bgpaas_secondary_ip,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = BGPSessionIPAttributes;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BGPSessionIPAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_bgpaas_primary_ip: Option<String> = None;
                let mut value_bgpaas_secondary_ip: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_bgpaas_primary_ip => value_bgpaas_primary_ip = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_bgpaas_secondary_ip => value_bgpaas_secondary_ip = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BGPSessionIPAttributes {
                    bgpaas_primary_ip: value_bgpaas_primary_ip,
                    bgpaas_secondary_ip: value_bgpaas_secondary_ip,
                })
            }
        }

        deserializer.deserialize_struct(
            "BGPSessionIPAttributes",
            &[
                "bgpaasPrimaryIP",
                "bgpaasSecondaryIP",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for BGPSessionIPAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BGPSessionIPAttributes",
            self.bgpaas_primary_ip.as_ref().map_or(0, |_| 1) +
            self.bgpaas_secondary_ip.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.bgpaas_primary_ip {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "bgpaasPrimaryIP", value)?;
        }
        if let Some(value) = &self.bgpaas_secondary_ip {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "bgpaasSecondaryIP", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BGPSessionIPAttributes {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPSessionIPAttributes".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("BGPSessionIPAttributes contains BGPSession primary and secondary IP addresses.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "bgpaasPrimaryIP".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGPaaSPrimaryIP defines the primary IP address used for a BGP session.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "bgpaasSecondaryIP".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGPaaSSecondaryIP defines the secondary IP address used for a BGP session when a second control node is present.".to_owned()),
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
