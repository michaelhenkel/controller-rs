// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallServiceType

/// FirewallServiceType defines the Port related info.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FirewallServiceType {
    /// DestinationPorts defines the range of destination port numbers for Layer 4 Protocol.
    pub dst_ports: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::PortType>,

    /// Protocol defines Layer 4 protocol in IP packet.
    pub protocol: Option<String>,

    /// ProtocolId defines Layer 4 protocol ID in IP packet.
    pub protocol_id: Option<i64>,

    /// SourcePorts defines the range of source port numbers for Layer 4 Protocol.
    pub src_ports: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::PortType>,
}

impl k8s_openapi::DeepMerge for FirewallServiceType {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.dst_ports, other.dst_ports);
        k8s_openapi::DeepMerge::merge_from(&mut self.protocol, other.protocol);
        k8s_openapi::DeepMerge::merge_from(&mut self.protocol_id, other.protocol_id);
        k8s_openapi::DeepMerge::merge_from(&mut self.src_ports, other.src_ports);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FirewallServiceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_dst_ports,
            Key_protocol,
            Key_protocol_id,
            Key_src_ports,
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
                            "dstPorts" => Field::Key_dst_ports,
                            "protocol" => Field::Key_protocol,
                            "protocolId" => Field::Key_protocol_id,
                            "srcPorts" => Field::Key_src_ports,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallServiceType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FirewallServiceType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_dst_ports: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::PortType> = None;
                let mut value_protocol: Option<String> = None;
                let mut value_protocol_id: Option<i64> = None;
                let mut value_src_ports: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::PortType> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_dst_ports => value_dst_ports = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol => value_protocol = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol_id => value_protocol_id = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_src_ports => value_src_ports = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FirewallServiceType {
                    dst_ports: value_dst_ports,
                    protocol: value_protocol,
                    protocol_id: value_protocol_id,
                    src_ports: value_src_ports,
                })
            }
        }

        deserializer.deserialize_struct(
            "FirewallServiceType",
            &[
                "dstPorts",
                "protocol",
                "protocolId",
                "srcPorts",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for FirewallServiceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FirewallServiceType",
            self.dst_ports.as_ref().map_or(0, |_| 1) +
            self.protocol.as_ref().map_or(0, |_| 1) +
            self.protocol_id.as_ref().map_or(0, |_| 1) +
            self.src_ports.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.dst_ports {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "dstPorts", value)?;
        }
        if let Some(value) = &self.protocol {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", value)?;
        }
        if let Some(value) = &self.protocol_id {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "protocolId", value)?;
        }
        if let Some(value) = &self.src_ports {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "srcPorts", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FirewallServiceType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallServiceType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FirewallServiceType defines the Port related info.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "dstPorts".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::PortType>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("DestinationPorts defines the range of destination port numbers for Layer 4 Protocol.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "protocol".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Protocol defines Layer 4 protocol in IP packet.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "protocolId".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ProtocolId defines Layer 4 protocol ID in IP packet.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "srcPorts".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::PortType>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("SourcePorts defines the range of source port numbers for Layer 4 Protocol.".to_owned()),
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
