// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.LinklocalServiceEntryType

/// LinklocalServiceEntryType specifies parameters for configurable LinkLocalServices.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LinklocalServiceEntryType {
    /// DNS name to which link local service will be proxied.
    pub ip_fabric_dns_service_name: Option<String>,

    /// Destination ip address to which link local traffic will forwarded.
    pub ip_fabric_service_ip: Option<Vec<String>>,

    /// Destination TCP port number to which link local traffic will forwarded.
    pub ip_fabric_service_port: Option<i32>,

    /// ip address of the link local service.
    pub linklocal_service_ip: Option<String>,

    /// Name of the link local service. VM name resolution of this name will result in
    ///  link local ip address
    pub linklocal_service_name: Option<String>,

    /// Destination TCP port number of link local service
    pub linklocal_service_port: Option<i32>,
}

impl k8s_openapi::DeepMerge for LinklocalServiceEntryType {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.ip_fabric_dns_service_name, other.ip_fabric_dns_service_name);
        k8s_openapi::merge_strategies::list::atomic(&mut self.ip_fabric_service_ip, other.ip_fabric_service_ip);
        k8s_openapi::DeepMerge::merge_from(&mut self.ip_fabric_service_port, other.ip_fabric_service_port);
        k8s_openapi::DeepMerge::merge_from(&mut self.linklocal_service_ip, other.linklocal_service_ip);
        k8s_openapi::DeepMerge::merge_from(&mut self.linklocal_service_name, other.linklocal_service_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.linklocal_service_port, other.linklocal_service_port);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for LinklocalServiceEntryType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ip_fabric_dns_service_name,
            Key_ip_fabric_service_ip,
            Key_ip_fabric_service_port,
            Key_linklocal_service_ip,
            Key_linklocal_service_name,
            Key_linklocal_service_port,
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
                            "ipFabricDNSServiceName" => Field::Key_ip_fabric_dns_service_name,
                            "ipFabricServiceIP" => Field::Key_ip_fabric_service_ip,
                            "ipFabricServicePort" => Field::Key_ip_fabric_service_port,
                            "linklocalServiceIP" => Field::Key_linklocal_service_ip,
                            "linklocalServiceName" => Field::Key_linklocal_service_name,
                            "linklocalServicePort" => Field::Key_linklocal_service_port,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = LinklocalServiceEntryType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LinklocalServiceEntryType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_ip_fabric_dns_service_name: Option<String> = None;
                let mut value_ip_fabric_service_ip: Option<Vec<String>> = None;
                let mut value_ip_fabric_service_port: Option<i32> = None;
                let mut value_linklocal_service_ip: Option<String> = None;
                let mut value_linklocal_service_name: Option<String> = None;
                let mut value_linklocal_service_port: Option<i32> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ip_fabric_dns_service_name => value_ip_fabric_dns_service_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip_fabric_service_ip => value_ip_fabric_service_ip = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip_fabric_service_port => value_ip_fabric_service_port = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_linklocal_service_ip => value_linklocal_service_ip = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_linklocal_service_name => value_linklocal_service_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_linklocal_service_port => value_linklocal_service_port = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LinklocalServiceEntryType {
                    ip_fabric_dns_service_name: value_ip_fabric_dns_service_name,
                    ip_fabric_service_ip: value_ip_fabric_service_ip,
                    ip_fabric_service_port: value_ip_fabric_service_port,
                    linklocal_service_ip: value_linklocal_service_ip,
                    linklocal_service_name: value_linklocal_service_name,
                    linklocal_service_port: value_linklocal_service_port,
                })
            }
        }

        deserializer.deserialize_struct(
            "LinklocalServiceEntryType",
            &[
                "ipFabricDNSServiceName",
                "ipFabricServiceIP",
                "ipFabricServicePort",
                "linklocalServiceIP",
                "linklocalServiceName",
                "linklocalServicePort",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for LinklocalServiceEntryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LinklocalServiceEntryType",
            self.ip_fabric_dns_service_name.as_ref().map_or(0, |_| 1) +
            self.ip_fabric_service_ip.as_ref().map_or(0, |_| 1) +
            self.ip_fabric_service_port.as_ref().map_or(0, |_| 1) +
            self.linklocal_service_ip.as_ref().map_or(0, |_| 1) +
            self.linklocal_service_name.as_ref().map_or(0, |_| 1) +
            self.linklocal_service_port.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ip_fabric_dns_service_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "ipFabricDNSServiceName", value)?;
        }
        if let Some(value) = &self.ip_fabric_service_ip {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "ipFabricServiceIP", value)?;
        }
        if let Some(value) = &self.ip_fabric_service_port {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "ipFabricServicePort", value)?;
        }
        if let Some(value) = &self.linklocal_service_ip {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "linklocalServiceIP", value)?;
        }
        if let Some(value) = &self.linklocal_service_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "linklocalServiceName", value)?;
        }
        if let Some(value) = &self.linklocal_service_port {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "linklocalServicePort", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LinklocalServiceEntryType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.LinklocalServiceEntryType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("LinklocalServiceEntryType specifies parameters for configurable LinkLocalServices.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "ipFabricDNSServiceName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("DNS name to which link local service will be proxied.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ipFabricServiceIP".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Destination ip address to which link local traffic will forwarded.".to_owned()),
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
                        "ipFabricServicePort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Destination TCP port number to which link local traffic will forwarded.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "linklocalServiceIP".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ip address of the link local service.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "linklocalServiceName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name of the link local service. VM name resolution of this name will result in\n link local ip address".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "linklocalServicePort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Destination TCP port number of link local service".to_owned()),
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
