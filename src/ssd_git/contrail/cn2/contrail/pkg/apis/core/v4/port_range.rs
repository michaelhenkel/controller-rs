// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.PortRange

/// PortRange encapsulates the start and end of a range of IP ports.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PortRange {
    /// EndPort represents the last allocatable port number. It must be greater than or equal to StartPort.
    pub end_port: i32,

    /// StartPort represents the starting port number. It must be less than or equal to EndPort.
    pub start_port: i32,
}

impl k8s_openapi::DeepMerge for PortRange {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.end_port, other.end_port);
        k8s_openapi::DeepMerge::merge_from(&mut self.start_port, other.start_port);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for PortRange {
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
            type Value = PortRange;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PortRange")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_end_port: Option<i32> = None;
                let mut value_start_port: Option<i32> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_end_port => value_end_port = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_start_port => value_start_port = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PortRange {
                    end_port: value_end_port.unwrap_or_default(),
                    start_port: value_start_port.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PortRange",
            &[
                "endPort",
                "startPort",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for PortRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PortRange",
            2,
        )?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "endPort", &self.end_port)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "startPort", &self.start_port)?;
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PortRange {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.PortRange".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PortRange encapsulates the start and end of a range of IP ports.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "endPort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("EndPort represents the last allocatable port number. It must be greater than or equal to StartPort.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "startPort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("StartPort represents the starting port number. It must be less than or equal to EndPort.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "endPort".to_owned(),
                    "startPort".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
