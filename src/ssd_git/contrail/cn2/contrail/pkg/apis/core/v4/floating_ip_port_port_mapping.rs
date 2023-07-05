// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPPortPortMapping

/// FloatingIPPortPortMapping indicates ports exposed by the service.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FloatingIPPortPortMapping {
    /// Destination Port
    pub dst_port: Option<i32>,

    /// Network protocol
    pub protocol: Option<String>,

    /// Source Port
    pub src_port: Option<i32>,
}

impl k8s_openapi::DeepMerge for FloatingIPPortPortMapping {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.dst_port, other.dst_port);
        k8s_openapi::DeepMerge::merge_from(&mut self.protocol, other.protocol);
        k8s_openapi::DeepMerge::merge_from(&mut self.src_port, other.src_port);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FloatingIPPortPortMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_dst_port,
            Key_protocol,
            Key_src_port,
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
                            "dstPort" => Field::Key_dst_port,
                            "protocol" => Field::Key_protocol,
                            "srcPort" => Field::Key_src_port,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = FloatingIPPortPortMapping;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FloatingIPPortPortMapping")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_dst_port: Option<i32> = None;
                let mut value_protocol: Option<String> = None;
                let mut value_src_port: Option<i32> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_dst_port => value_dst_port = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol => value_protocol = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_src_port => value_src_port = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FloatingIPPortPortMapping {
                    dst_port: value_dst_port,
                    protocol: value_protocol,
                    src_port: value_src_port,
                })
            }
        }

        deserializer.deserialize_struct(
            "FloatingIPPortPortMapping",
            &[
                "dstPort",
                "protocol",
                "srcPort",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for FloatingIPPortPortMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FloatingIPPortPortMapping",
            self.dst_port.as_ref().map_or(0, |_| 1) +
            self.protocol.as_ref().map_or(0, |_| 1) +
            self.src_port.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.dst_port {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "dstPort", value)?;
        }
        if let Some(value) = &self.protocol {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", value)?;
        }
        if let Some(value) = &self.src_port {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "srcPort", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FloatingIPPortPortMapping {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPPortPortMapping".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FloatingIPPortPortMapping indicates ports exposed by the service.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "dstPort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Destination Port".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "protocol".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Network protocol".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "srcPort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Source Port".to_owned()),
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
