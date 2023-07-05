// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.StaticNHType

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StaticNHType {
    pub vni: Option<i32>,

    pub vtep_dst_ip_address: Option<String>,

    pub vtep_dst_mac_address: Option<String>,
}

impl k8s_openapi::DeepMerge for StaticNHType {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.vni, other.vni);
        k8s_openapi::DeepMerge::merge_from(&mut self.vtep_dst_ip_address, other.vtep_dst_ip_address);
        k8s_openapi::DeepMerge::merge_from(&mut self.vtep_dst_mac_address, other.vtep_dst_mac_address);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for StaticNHType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_vni,
            Key_vtep_dst_ip_address,
            Key_vtep_dst_mac_address,
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
                            "vni" => Field::Key_vni,
                            "vtepDstIPAddress" => Field::Key_vtep_dst_ip_address,
                            "vtepDstMacAddress" => Field::Key_vtep_dst_mac_address,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = StaticNHType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StaticNHType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_vni: Option<i32> = None;
                let mut value_vtep_dst_ip_address: Option<String> = None;
                let mut value_vtep_dst_mac_address: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_vni => value_vni = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vtep_dst_ip_address => value_vtep_dst_ip_address = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vtep_dst_mac_address => value_vtep_dst_mac_address = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StaticNHType {
                    vni: value_vni,
                    vtep_dst_ip_address: value_vtep_dst_ip_address,
                    vtep_dst_mac_address: value_vtep_dst_mac_address,
                })
            }
        }

        deserializer.deserialize_struct(
            "StaticNHType",
            &[
                "vni",
                "vtepDstIPAddress",
                "vtepDstMacAddress",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for StaticNHType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StaticNHType",
            self.vni.as_ref().map_or(0, |_| 1) +
            self.vtep_dst_ip_address.as_ref().map_or(0, |_| 1) +
            self.vtep_dst_mac_address.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.vni {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "vni", value)?;
        }
        if let Some(value) = &self.vtep_dst_ip_address {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "vtepDstIPAddress", value)?;
        }
        if let Some(value) = &self.vtep_dst_mac_address {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "vtepDstMacAddress", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StaticNHType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.StaticNHType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "vni".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "vtepDstIPAddress".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "vtepDstMacAddress".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
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
