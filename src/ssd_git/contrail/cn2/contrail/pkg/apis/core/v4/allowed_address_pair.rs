// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPair

/// AllowedAddressPair allows the additional IP/MAC pairs to the guest interface. The traffic matching to specified value of IP/MAC will be redirected.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AllowedAddressPair {
    /// AddressMode active-standby is used for VRRP address. AddressMode active-active is used for ECMP.
    pub address_mode: Option<String>,

    /// IPAddress defines the IPv4 or IPv6 address of the pair.
    pub ip: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AllowedAddressPairSubnet>,

    /// MACAddress defines the media access control ID of the NIC..
    pub mac: Option<String>,
}

impl k8s_openapi::DeepMerge for AllowedAddressPair {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.address_mode, other.address_mode);
        k8s_openapi::DeepMerge::merge_from(&mut self.ip, other.ip);
        k8s_openapi::DeepMerge::merge_from(&mut self.mac, other.mac);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for AllowedAddressPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_address_mode,
            Key_ip,
            Key_mac,
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
                            "addressMode" => Field::Key_address_mode,
                            "ip" => Field::Key_ip,
                            "mac" => Field::Key_mac,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = AllowedAddressPair;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AllowedAddressPair")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_address_mode: Option<String> = None;
                let mut value_ip: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AllowedAddressPairSubnet> = None;
                let mut value_mac: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_address_mode => value_address_mode = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip => value_ip = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_mac => value_mac = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AllowedAddressPair {
                    address_mode: value_address_mode,
                    ip: value_ip,
                    mac: value_mac,
                })
            }
        }

        deserializer.deserialize_struct(
            "AllowedAddressPair",
            &[
                "addressMode",
                "ip",
                "mac",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for AllowedAddressPair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AllowedAddressPair",
            self.address_mode.as_ref().map_or(0, |_| 1) +
            self.ip.as_ref().map_or(0, |_| 1) +
            self.mac.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.address_mode {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "addressMode", value)?;
        }
        if let Some(value) = &self.ip {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "ip", value)?;
        }
        if let Some(value) = &self.mac {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "mac", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AllowedAddressPair {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPair".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("AllowedAddressPair allows the additional IP/MAC pairs to the guest interface. The traffic matching to specified value of IP/MAC will be redirected.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "addressMode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AddressMode active-standby is used for VRRP address. AddressMode active-active is used for ECMP.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ip".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AllowedAddressPairSubnet>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IPAddress defines the IPv4 or IPv6 address of the pair.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "mac".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("MACAddress defines the media access control ID of the NIC..".to_owned()),
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
