// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkType

/// Advanced properties of the VirtualNetwork.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualNetworkType {
    /// ForwardingMode is the Packet forwarding mode for this VirtualNetwork. Supported Options are l2, l3 and l2_l3. By Default, ForwardingMode is set to l2_l3.
    pub forwarding_mode: Option<String>,

    /// Rpf property enables or disables unicast Reverse Path Forwarding (RPF) on the VirtualNetwork. By Default, Rpf is set to enabled.
    pub rpf: Option<String>,
}

impl k8s_openapi::DeepMerge for VirtualNetworkType {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.forwarding_mode, other.forwarding_mode);
        k8s_openapi::DeepMerge::merge_from(&mut self.rpf, other.rpf);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for VirtualNetworkType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_forwarding_mode,
            Key_rpf,
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
                            "forwardingMode" => Field::Key_forwarding_mode,
                            "rpf" => Field::Key_rpf,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualNetworkType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VirtualNetworkType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_forwarding_mode: Option<String> = None;
                let mut value_rpf: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_forwarding_mode => value_forwarding_mode = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rpf => value_rpf = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VirtualNetworkType {
                    forwarding_mode: value_forwarding_mode,
                    rpf: value_rpf,
                })
            }
        }

        deserializer.deserialize_struct(
            "VirtualNetworkType",
            &[
                "forwardingMode",
                "rpf",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for VirtualNetworkType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VirtualNetworkType",
            self.forwarding_mode.as_ref().map_or(0, |_| 1) +
            self.rpf.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.forwarding_mode {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "forwardingMode", value)?;
        }
        if let Some(value) = &self.rpf {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "rpf", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VirtualNetworkType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Advanced properties of the VirtualNetwork.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "forwardingMode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ForwardingMode is the Packet forwarding mode for this VirtualNetwork. Supported Options are l2, l3 and l2_l3. By Default, ForwardingMode is set to l2_l3.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "rpf".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Rpf property enables or disables unicast Reverse Path Forwarding (RPF) on the VirtualNetwork. By Default, Rpf is set to enabled.".to_owned()),
                                ..Default::default()
                            })),
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
