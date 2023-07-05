// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.StaticRouteType

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StaticRouteType {
    /// Community string that needs to be associated with Route
    pub community: Option<Vec<String>>,

    /// NextHop Ip address of NextHop, it should point to the valid VMI object
    pub next_hop: Option<String>,

    /// Prefix for the route
    pub prefix: Option<String>,

    pub route_target: Option<Vec<String>>,
}

impl k8s_openapi::DeepMerge for StaticRouteType {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.community, other.community);
        k8s_openapi::DeepMerge::merge_from(&mut self.next_hop, other.next_hop);
        k8s_openapi::DeepMerge::merge_from(&mut self.prefix, other.prefix);
        k8s_openapi::merge_strategies::list::atomic(&mut self.route_target, other.route_target);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for StaticRouteType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_community,
            Key_next_hop,
            Key_prefix,
            Key_route_target,
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
                            "community" => Field::Key_community,
                            "nextHop" => Field::Key_next_hop,
                            "prefix" => Field::Key_prefix,
                            "routeTarget" => Field::Key_route_target,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = StaticRouteType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StaticRouteType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_community: Option<Vec<String>> = None;
                let mut value_next_hop: Option<String> = None;
                let mut value_prefix: Option<String> = None;
                let mut value_route_target: Option<Vec<String>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_community => value_community = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_next_hop => value_next_hop = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_prefix => value_prefix = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_route_target => value_route_target = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StaticRouteType {
                    community: value_community,
                    next_hop: value_next_hop,
                    prefix: value_prefix,
                    route_target: value_route_target,
                })
            }
        }

        deserializer.deserialize_struct(
            "StaticRouteType",
            &[
                "community",
                "nextHop",
                "prefix",
                "routeTarget",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for StaticRouteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StaticRouteType",
            self.community.as_ref().map_or(0, |_| 1) +
            self.next_hop.as_ref().map_or(0, |_| 1) +
            self.prefix.as_ref().map_or(0, |_| 1) +
            self.route_target.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.community {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "community", value)?;
        }
        if let Some(value) = &self.next_hop {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "nextHop", value)?;
        }
        if let Some(value) = &self.prefix {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "prefix", value)?;
        }
        if let Some(value) = &self.route_target {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "routeTarget", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StaticRouteType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.StaticRouteType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "community".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Community string that needs to be associated with Route".to_owned()),
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
                        "nextHop".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NextHop Ip address of NextHop, it should point to the valid VMI object".to_owned()),
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
                                description: Some("Prefix for the route".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "routeTarget".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
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
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
