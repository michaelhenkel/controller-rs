// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationSpec

/// MirrorDestinationSpec defines the parameters associated with mirrored traffic reciever
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MirrorDestinationSpec {
    /// AnalyzerIP indicates the external analyzer IP address
    pub analyzer_ip: Option<String>,

    /// ExternalAnalyzer indicates to use external analyzer
    pub external_analyzer: Option<bool>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// JuniperHeader indicates adding juniper header to the mirrored traffic. default is true, indicating adding juniper header
    pub juniper_header: Option<bool>,

    /// NextHopMode indicates using static or dynamic nexthop for mirrored traffic default is dynamic nexthop
    pub next_hop_mode: Option<String>,

    /// NicAssistedMirroring indictaes to enable NicAssistedMirroring default is false
    pub nic_assisted_mirroring: Option<bool>,

    /// NicAssistedVlanID indicates vlanID when NicAssistedMirroring is enabled
    pub nic_assisted_vlan_id: Option<i32>,

    /// StaticNextHopHeader specifies vxlan parameters for static nexthop used to forward to the mirror destination StaticNextHopHeader is required when NextHopMode is set to static
    pub static_next_hop_header: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::NHHeaderType>,

    /// TrafficDirection indicates ingress, egress or both direction traffic to be mirrored default is both
    pub traffic_direction: Option<String>,

    /// UdpPort specifies the destination udp port for mirrored traffic ip header
    pub udp_port: Option<i32>,
}

impl k8s_openapi::DeepMerge for MirrorDestinationSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.analyzer_ip, other.analyzer_ip);
        k8s_openapi::DeepMerge::merge_from(&mut self.external_analyzer, other.external_analyzer);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.juniper_header, other.juniper_header);
        k8s_openapi::DeepMerge::merge_from(&mut self.next_hop_mode, other.next_hop_mode);
        k8s_openapi::DeepMerge::merge_from(&mut self.nic_assisted_mirroring, other.nic_assisted_mirroring);
        k8s_openapi::DeepMerge::merge_from(&mut self.nic_assisted_vlan_id, other.nic_assisted_vlan_id);
        k8s_openapi::DeepMerge::merge_from(&mut self.static_next_hop_header, other.static_next_hop_header);
        k8s_openapi::DeepMerge::merge_from(&mut self.traffic_direction, other.traffic_direction);
        k8s_openapi::DeepMerge::merge_from(&mut self.udp_port, other.udp_port);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for MirrorDestinationSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_analyzer_ip,
            Key_external_analyzer,
            Key_fq_name,
            Key_juniper_header,
            Key_next_hop_mode,
            Key_nic_assisted_mirroring,
            Key_nic_assisted_vlan_id,
            Key_static_next_hop_header,
            Key_traffic_direction,
            Key_udp_port,
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
                            "analyzerIP" => Field::Key_analyzer_ip,
                            "externalAnalyzer" => Field::Key_external_analyzer,
                            "fqName" => Field::Key_fq_name,
                            "juniperHeader" => Field::Key_juniper_header,
                            "nextHopMode" => Field::Key_next_hop_mode,
                            "nicAssistedMirroring" => Field::Key_nic_assisted_mirroring,
                            "nicAssistedVlanID" => Field::Key_nic_assisted_vlan_id,
                            "staticNextHopHeader" => Field::Key_static_next_hop_header,
                            "trafficDirection" => Field::Key_traffic_direction,
                            "udpPort" => Field::Key_udp_port,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = MirrorDestinationSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("MirrorDestinationSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_analyzer_ip: Option<String> = None;
                let mut value_external_analyzer: Option<bool> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_juniper_header: Option<bool> = None;
                let mut value_next_hop_mode: Option<String> = None;
                let mut value_nic_assisted_mirroring: Option<bool> = None;
                let mut value_nic_assisted_vlan_id: Option<i32> = None;
                let mut value_static_next_hop_header: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::NHHeaderType> = None;
                let mut value_traffic_direction: Option<String> = None;
                let mut value_udp_port: Option<i32> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_analyzer_ip => value_analyzer_ip = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_analyzer => value_external_analyzer = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_juniper_header => value_juniper_header = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_next_hop_mode => value_next_hop_mode = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nic_assisted_mirroring => value_nic_assisted_mirroring = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nic_assisted_vlan_id => value_nic_assisted_vlan_id = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_static_next_hop_header => value_static_next_hop_header = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_traffic_direction => value_traffic_direction = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_udp_port => value_udp_port = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(MirrorDestinationSpec {
                    analyzer_ip: value_analyzer_ip,
                    external_analyzer: value_external_analyzer,
                    fq_name: value_fq_name,
                    juniper_header: value_juniper_header,
                    next_hop_mode: value_next_hop_mode,
                    nic_assisted_mirroring: value_nic_assisted_mirroring,
                    nic_assisted_vlan_id: value_nic_assisted_vlan_id,
                    static_next_hop_header: value_static_next_hop_header,
                    traffic_direction: value_traffic_direction,
                    udp_port: value_udp_port,
                })
            }
        }

        deserializer.deserialize_struct(
            "MirrorDestinationSpec",
            &[
                "analyzerIP",
                "externalAnalyzer",
                "fqName",
                "juniperHeader",
                "nextHopMode",
                "nicAssistedMirroring",
                "nicAssistedVlanID",
                "staticNextHopHeader",
                "trafficDirection",
                "udpPort",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for MirrorDestinationSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "MirrorDestinationSpec",
            self.analyzer_ip.as_ref().map_or(0, |_| 1) +
            self.external_analyzer.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.juniper_header.as_ref().map_or(0, |_| 1) +
            self.next_hop_mode.as_ref().map_or(0, |_| 1) +
            self.nic_assisted_mirroring.as_ref().map_or(0, |_| 1) +
            self.nic_assisted_vlan_id.as_ref().map_or(0, |_| 1) +
            self.static_next_hop_header.as_ref().map_or(0, |_| 1) +
            self.traffic_direction.as_ref().map_or(0, |_| 1) +
            self.udp_port.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.analyzer_ip {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "analyzerIP", value)?;
        }
        if let Some(value) = &self.external_analyzer {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "externalAnalyzer", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.juniper_header {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "juniperHeader", value)?;
        }
        if let Some(value) = &self.next_hop_mode {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "nextHopMode", value)?;
        }
        if let Some(value) = &self.nic_assisted_mirroring {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "nicAssistedMirroring", value)?;
        }
        if let Some(value) = &self.nic_assisted_vlan_id {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "nicAssistedVlanID", value)?;
        }
        if let Some(value) = &self.static_next_hop_header {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "staticNextHopHeader", value)?;
        }
        if let Some(value) = &self.traffic_direction {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "trafficDirection", value)?;
        }
        if let Some(value) = &self.udp_port {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "udpPort", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for MirrorDestinationSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("MirrorDestinationSpec defines the parameters associated with mirrored traffic reciever".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "analyzerIP".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AnalyzerIP indicates the external analyzer IP address".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "externalAnalyzer".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ExternalAnalyzer indicates to use external analyzer".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fqName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FqName is the list of resource names that fully qualify a Contrail resource.".to_owned()),
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
                        "juniperHeader".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("JuniperHeader indicates adding juniper header to the mirrored traffic. default is true, indicating adding juniper header".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nextHopMode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NextHopMode indicates using static or dynamic nexthop for mirrored traffic default is dynamic nexthop".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nicAssistedMirroring".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NicAssistedMirroring indictaes to enable NicAssistedMirroring default is false".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nicAssistedVlanID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NicAssistedVlanID indicates vlanID when NicAssistedMirroring is enabled".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "staticNextHopHeader".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::NHHeaderType>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("StaticNextHopHeader specifies vxlan parameters for static nexthop used to forward to the mirror destination StaticNextHopHeader is required when NextHopMode is set to static".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "trafficDirection".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TrafficDirection indicates ingress, egress or both direction traffic to be mirrored default is both".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "udpPort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("UdpPort specifies the destination udp port for mirrored traffic ip header".to_owned()),
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
