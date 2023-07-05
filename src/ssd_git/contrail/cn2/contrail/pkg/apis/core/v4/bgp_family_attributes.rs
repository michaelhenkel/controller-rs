// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPFamilyAttributes

/// This type is used to configure per address-family parameters for a BgpSession.
///   - loop-count is the number of times the local bgp-router's AS is
///     allowed in the AS_PATH attribute.
///   - prefix-limit contains the maximum number of prefixes that are
///     allowed to be received on the session.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BGPFamilyAttributes {
    /// Address family for which these parameters are applied.
    pub address_family: Option<String>,

    /// Default prioritized tunnel encapsulation list.
    pub default_tunnel_encap: Option<Vec<String>>,

    /// For routing loop detection, loop-count is the number of times the local bgp-routers AS is allowed in the AS_PATH attribute.
    pub loop_count: Option<i32>,

    /// PrefixLimit contains the maximum number of prefixes that are allowed to be received on the session for this address family.
    pub prefix_limit: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPPrefixLimit>,
}

impl k8s_openapi::DeepMerge for BGPFamilyAttributes {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.address_family, other.address_family);
        k8s_openapi::merge_strategies::list::atomic(&mut self.default_tunnel_encap, other.default_tunnel_encap);
        k8s_openapi::DeepMerge::merge_from(&mut self.loop_count, other.loop_count);
        k8s_openapi::DeepMerge::merge_from(&mut self.prefix_limit, other.prefix_limit);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for BGPFamilyAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_address_family,
            Key_default_tunnel_encap,
            Key_loop_count,
            Key_prefix_limit,
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
                            "addressFamily" => Field::Key_address_family,
                            "defaultTunnelEncap" => Field::Key_default_tunnel_encap,
                            "loopCount" => Field::Key_loop_count,
                            "prefixLimit" => Field::Key_prefix_limit,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = BGPFamilyAttributes;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BGPFamilyAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_address_family: Option<String> = None;
                let mut value_default_tunnel_encap: Option<Vec<String>> = None;
                let mut value_loop_count: Option<i32> = None;
                let mut value_prefix_limit: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPPrefixLimit> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_address_family => value_address_family = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_default_tunnel_encap => value_default_tunnel_encap = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_loop_count => value_loop_count = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_prefix_limit => value_prefix_limit = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BGPFamilyAttributes {
                    address_family: value_address_family,
                    default_tunnel_encap: value_default_tunnel_encap,
                    loop_count: value_loop_count,
                    prefix_limit: value_prefix_limit,
                })
            }
        }

        deserializer.deserialize_struct(
            "BGPFamilyAttributes",
            &[
                "addressFamily",
                "defaultTunnelEncap",
                "loopCount",
                "prefixLimit",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for BGPFamilyAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BGPFamilyAttributes",
            self.address_family.as_ref().map_or(0, |_| 1) +
            self.default_tunnel_encap.as_ref().map_or(0, |_| 1) +
            self.loop_count.as_ref().map_or(0, |_| 1) +
            self.prefix_limit.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.address_family {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "addressFamily", value)?;
        }
        if let Some(value) = &self.default_tunnel_encap {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultTunnelEncap", value)?;
        }
        if let Some(value) = &self.loop_count {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "loopCount", value)?;
        }
        if let Some(value) = &self.prefix_limit {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "prefixLimit", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BGPFamilyAttributes {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPFamilyAttributes".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("This type is used to configure per address-family parameters for a BgpSession.\n  - loop-count is the number of times the local bgp-router's AS is\n    allowed in the AS_PATH attribute.\n  - prefix-limit contains the maximum number of prefixes that are\n    allowed to be received on the session.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "addressFamily".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Address family for which these parameters are applied.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "defaultTunnelEncap".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Default prioritized tunnel encapsulation list.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "loopCount".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("For routing loop detection, loop-count is the number of times the local bgp-routers AS is allowed in the AS_PATH attribute.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "prefixLimit".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPPrefixLimit>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PrefixLimit contains the maximum number of prefixes that are allowed to be received on the session for this address family.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
