// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.PortType

/// PortType defines the Port number.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PortType {
    /// EndPort defines the ending port number of the FirewallServiceType.
    pub end_port: Option<i64>,

    /// StartPort defines the starting port number of the FirewallServiceType.
    pub start_port: Option<i64>,
}

impl k8s_openapi::DeepMerge for PortType {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.end_port, other.end_port);
        k8s_openapi::DeepMerge::merge_from(&mut self.start_port, other.start_port);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for PortType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_end_port,
            Key_start_port,
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
                            "endPort" => Field::Key_end_port,
                            "startPort" => Field::Key_start_port,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = PortType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PortType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_end_port: Option<i64> = None;
                let mut value_start_port: Option<i64> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_end_port => value_end_port = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_start_port => value_start_port = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PortType {
                    end_port: value_end_port,
                    start_port: value_start_port,
                })
            }
        }

        deserializer.deserialize_struct(
            "PortType",
            &[
                "endPort",
                "startPort",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for PortType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PortType",
            self.end_port.as_ref().map_or(0, |_| 1) +
            self.start_port.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.end_port {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "endPort", value)?;
        }
        if let Some(value) = &self.start_port {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "startPort", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PortType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.PortType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PortType defines the Port number.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "endPort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("EndPort defines the ending port number of the FirewallServiceType.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "startPort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("StartPort defines the starting port number of the FirewallServiceType.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
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
