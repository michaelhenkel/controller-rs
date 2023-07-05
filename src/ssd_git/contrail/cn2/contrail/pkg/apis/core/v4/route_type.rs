// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.RouteType

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RouteType {
    /// CommunityAttributes bgp community attributes for the route entry in VRF
    pub community_attributes: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::CommunityAttributes>,

    /// NextHop next-hop for matching prefix, must be another VMI Ip address
    pub next_hop: Option<String>,

    /// NextHopType will be "ip-address"
    pub next_hop_type: Option<String>,

    /// Prefix CIDR value for routing, when associated VMI
    pub prefix: Option<String>,
}

impl k8s_openapi::DeepMerge for RouteType {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.community_attributes, other.community_attributes);
        k8s_openapi::DeepMerge::merge_from(&mut self.next_hop, other.next_hop);
        k8s_openapi::DeepMerge::merge_from(&mut self.next_hop_type, other.next_hop_type);
        k8s_openapi::DeepMerge::merge_from(&mut self.prefix, other.prefix);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for RouteType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_community_attributes,
            Key_next_hop,
            Key_next_hop_type,
            Key_prefix,
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
                            "communityAttributes" => Field::Key_community_attributes,
                            "nextHop" => Field::Key_next_hop,
                            "nextHopType" => Field::Key_next_hop_type,
                            "prefix" => Field::Key_prefix,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = RouteType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RouteType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_community_attributes: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::CommunityAttributes> = None;
                let mut value_next_hop: Option<String> = None;
                let mut value_next_hop_type: Option<String> = None;
                let mut value_prefix: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_community_attributes => value_community_attributes = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_next_hop => value_next_hop = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_next_hop_type => value_next_hop_type = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_prefix => value_prefix = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RouteType {
                    community_attributes: value_community_attributes,
                    next_hop: value_next_hop,
                    next_hop_type: value_next_hop_type,
                    prefix: value_prefix,
                })
            }
        }

        deserializer.deserialize_struct(
            "RouteType",
            &[
                "communityAttributes",
                "nextHop",
                "nextHopType",
                "prefix",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for RouteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RouteType",
            self.community_attributes.as_ref().map_or(0, |_| 1) +
            self.next_hop.as_ref().map_or(0, |_| 1) +
            self.next_hop_type.as_ref().map_or(0, |_| 1) +
            self.prefix.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.community_attributes {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "communityAttributes", value)?;
        }
        if let Some(value) = &self.next_hop {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "nextHop", value)?;
        }
        if let Some(value) = &self.next_hop_type {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "nextHopType", value)?;
        }
        if let Some(value) = &self.prefix {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "prefix", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for RouteType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.RouteType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "communityAttributes".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::CommunityAttributes>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("CommunityAttributes bgp community attributes for the route entry in VRF".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "nextHop".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NextHop next-hop for matching prefix, must be another VMI Ip address".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nextHopType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NextHopType will be \"ip-address\"".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "prefix".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Prefix CIDR value for routing, when associated VMI".to_owned()),
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
