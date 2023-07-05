// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.NHHeaderType

/// NHHeaderType specifies vxlan forwarding parameters for forwarding mirrored traffic towards the mirror destination
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NHHeaderType {
    /// VTEPDestinationIP indicates vtep tunnel ipaddress
    pub v_tep_destination_ip: Option<String>,

    /// VTEPDestinationMac indicates vtep tunnel destination mac address
    pub v_tep_destination_mac: Option<String>,

    /// VxlanID indicates the vxlan identifier
    pub vxlan_id: Option<i32>,
}

impl k8s_openapi::DeepMerge for NHHeaderType {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.v_tep_destination_ip, other.v_tep_destination_ip);
        k8s_openapi::DeepMerge::merge_from(&mut self.v_tep_destination_mac, other.v_tep_destination_mac);
        k8s_openapi::DeepMerge::merge_from(&mut self.vxlan_id, other.vxlan_id);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for NHHeaderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_v_tep_destination_ip,
            Key_v_tep_destination_mac,
            Key_vxlan_id,
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
                            "vTEPDestinationIP" => Field::Key_v_tep_destination_ip,
                            "vTEPDestinationMac" => Field::Key_v_tep_destination_mac,
                            "vxlanID" => Field::Key_vxlan_id,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = NHHeaderType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NHHeaderType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_v_tep_destination_ip: Option<String> = None;
                let mut value_v_tep_destination_mac: Option<String> = None;
                let mut value_vxlan_id: Option<i32> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_v_tep_destination_ip => value_v_tep_destination_ip = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_v_tep_destination_mac => value_v_tep_destination_mac = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vxlan_id => value_vxlan_id = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NHHeaderType {
                    v_tep_destination_ip: value_v_tep_destination_ip,
                    v_tep_destination_mac: value_v_tep_destination_mac,
                    vxlan_id: value_vxlan_id,
                })
            }
        }

        deserializer.deserialize_struct(
            "NHHeaderType",
            &[
                "vTEPDestinationIP",
                "vTEPDestinationMac",
                "vxlanID",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for NHHeaderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NHHeaderType",
            self.v_tep_destination_ip.as_ref().map_or(0, |_| 1) +
            self.v_tep_destination_mac.as_ref().map_or(0, |_| 1) +
            self.vxlan_id.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.v_tep_destination_ip {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "vTEPDestinationIP", value)?;
        }
        if let Some(value) = &self.v_tep_destination_mac {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "vTEPDestinationMac", value)?;
        }
        if let Some(value) = &self.vxlan_id {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "vxlanID", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NHHeaderType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.NHHeaderType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("NHHeaderType specifies vxlan forwarding parameters for forwarding mirrored traffic towards the mirror destination".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "vTEPDestinationIP".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VTEPDestinationIP indicates vtep tunnel ipaddress".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "vTEPDestinationMac".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VTEPDestinationMac indicates vtep tunnel destination mac address".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "vxlanID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VxlanID indicates the vxlan identifier".to_owned()),
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
