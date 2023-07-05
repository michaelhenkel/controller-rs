// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMirrorActionType

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FirewallMirrorActionType {
    pub analyzer_ip_address: Option<String>,

    pub analyzer_mac_address: Option<String>,

    pub analyzer_name: Option<String>,

    pub encapsulation: Option<String>,

    pub juniper_header: bool,

    pub nh_mode: Option<String>,

    pub nic_assisted_mirroring: bool,

    pub nic_assisted_mirroring_vlan: Option<i32>,

    pub routing_instance: Option<String>,

    pub static_nh_header: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::StaticNHType>,

    pub udp_port: Option<i32>,
}

impl k8s_openapi::DeepMerge for FirewallMirrorActionType {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.analyzer_ip_address, other.analyzer_ip_address);
        k8s_openapi::DeepMerge::merge_from(&mut self.analyzer_mac_address, other.analyzer_mac_address);
        k8s_openapi::DeepMerge::merge_from(&mut self.analyzer_name, other.analyzer_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.encapsulation, other.encapsulation);
        k8s_openapi::DeepMerge::merge_from(&mut self.juniper_header, other.juniper_header);
        k8s_openapi::DeepMerge::merge_from(&mut self.nh_mode, other.nh_mode);
        k8s_openapi::DeepMerge::merge_from(&mut self.nic_assisted_mirroring, other.nic_assisted_mirroring);
        k8s_openapi::DeepMerge::merge_from(&mut self.nic_assisted_mirroring_vlan, other.nic_assisted_mirroring_vlan);
        k8s_openapi::DeepMerge::merge_from(&mut self.routing_instance, other.routing_instance);
        k8s_openapi::DeepMerge::merge_from(&mut self.static_nh_header, other.static_nh_header);
        k8s_openapi::DeepMerge::merge_from(&mut self.udp_port, other.udp_port);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FirewallMirrorActionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_analyzer_ip_address,
            Key_analyzer_mac_address,
            Key_analyzer_name,
            Key_encapsulation,
            Key_juniper_header,
            Key_nh_mode,
            Key_nic_assisted_mirroring,
            Key_nic_assisted_mirroring_vlan,
            Key_routing_instance,
            Key_static_nh_header,
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
                            "analyzerIPAddress" => Field::Key_analyzer_ip_address,
                            "analyzerMacAddress" => Field::Key_analyzer_mac_address,
                            "analyzerName" => Field::Key_analyzer_name,
                            "encapsulation" => Field::Key_encapsulation,
                            "juniperHeader" => Field::Key_juniper_header,
                            "nhMode" => Field::Key_nh_mode,
                            "nicAssistedMirroring" => Field::Key_nic_assisted_mirroring,
                            "nicAssistedMirroringVlan" => Field::Key_nic_assisted_mirroring_vlan,
                            "routingInstance" => Field::Key_routing_instance,
                            "staticNhHeader" => Field::Key_static_nh_header,
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
            type Value = FirewallMirrorActionType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FirewallMirrorActionType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_analyzer_ip_address: Option<String> = None;
                let mut value_analyzer_mac_address: Option<String> = None;
                let mut value_analyzer_name: Option<String> = None;
                let mut value_encapsulation: Option<String> = None;
                let mut value_juniper_header: Option<bool> = None;
                let mut value_nh_mode: Option<String> = None;
                let mut value_nic_assisted_mirroring: Option<bool> = None;
                let mut value_nic_assisted_mirroring_vlan: Option<i32> = None;
                let mut value_routing_instance: Option<String> = None;
                let mut value_static_nh_header: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::StaticNHType> = None;
                let mut value_udp_port: Option<i32> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_analyzer_ip_address => value_analyzer_ip_address = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_analyzer_mac_address => value_analyzer_mac_address = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_analyzer_name => value_analyzer_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_encapsulation => value_encapsulation = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_juniper_header => value_juniper_header = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nh_mode => value_nh_mode = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nic_assisted_mirroring => value_nic_assisted_mirroring = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nic_assisted_mirroring_vlan => value_nic_assisted_mirroring_vlan = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_routing_instance => value_routing_instance = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_static_nh_header => value_static_nh_header = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_udp_port => value_udp_port = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FirewallMirrorActionType {
                    analyzer_ip_address: value_analyzer_ip_address,
                    analyzer_mac_address: value_analyzer_mac_address,
                    analyzer_name: value_analyzer_name,
                    encapsulation: value_encapsulation,
                    juniper_header: value_juniper_header.unwrap_or_default(),
                    nh_mode: value_nh_mode,
                    nic_assisted_mirroring: value_nic_assisted_mirroring.unwrap_or_default(),
                    nic_assisted_mirroring_vlan: value_nic_assisted_mirroring_vlan,
                    routing_instance: value_routing_instance,
                    static_nh_header: value_static_nh_header,
                    udp_port: value_udp_port,
                })
            }
        }

        deserializer.deserialize_struct(
            "FirewallMirrorActionType",
            &[
                "analyzerIPAddress",
                "analyzerMacAddress",
                "analyzerName",
                "encapsulation",
                "juniperHeader",
                "nhMode",
                "nicAssistedMirroring",
                "nicAssistedMirroringVlan",
                "routingInstance",
                "staticNhHeader",
                "udpPort",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for FirewallMirrorActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FirewallMirrorActionType",
            2 +
            self.analyzer_ip_address.as_ref().map_or(0, |_| 1) +
            self.analyzer_mac_address.as_ref().map_or(0, |_| 1) +
            self.analyzer_name.as_ref().map_or(0, |_| 1) +
            self.encapsulation.as_ref().map_or(0, |_| 1) +
            self.nh_mode.as_ref().map_or(0, |_| 1) +
            self.nic_assisted_mirroring_vlan.as_ref().map_or(0, |_| 1) +
            self.routing_instance.as_ref().map_or(0, |_| 1) +
            self.static_nh_header.as_ref().map_or(0, |_| 1) +
            self.udp_port.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.analyzer_ip_address {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "analyzerIPAddress", value)?;
        }
        if let Some(value) = &self.analyzer_mac_address {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "analyzerMacAddress", value)?;
        }
        if let Some(value) = &self.analyzer_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "analyzerName", value)?;
        }
        if let Some(value) = &self.encapsulation {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "encapsulation", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "juniperHeader", &self.juniper_header)?;
        if let Some(value) = &self.nh_mode {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "nhMode", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "nicAssistedMirroring", &self.nic_assisted_mirroring)?;
        if let Some(value) = &self.nic_assisted_mirroring_vlan {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "nicAssistedMirroringVlan", value)?;
        }
        if let Some(value) = &self.routing_instance {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "routingInstance", value)?;
        }
        if let Some(value) = &self.static_nh_header {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "staticNhHeader", value)?;
        }
        if let Some(value) = &self.udp_port {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "udpPort", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FirewallMirrorActionType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMirrorActionType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "analyzerIPAddress".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "analyzerMacAddress".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "analyzerName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "encapsulation".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "juniperHeader".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nhMode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nicAssistedMirroring".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nicAssistedMirroringVlan".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "routingInstance".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "staticNhHeader".to_owned(),
                        __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::StaticNHType>(),
                    ),
                    (
                        "udpPort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "juniperHeader".to_owned(),
                    "nicAssistedMirroring".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
