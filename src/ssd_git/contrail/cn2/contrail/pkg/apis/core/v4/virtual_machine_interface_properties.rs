// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceProperties

/// Advanced Properties of the VirtualMachineInterface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualMachineInterfaceProperties {
    /// 802.1Q VLAN tag to be used if this interface is a sub-interface of other interface.
    pub sub_interface_vlan_tag: Option<i64>,
}

impl k8s_openapi::DeepMerge for VirtualMachineInterfaceProperties {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.sub_interface_vlan_tag, other.sub_interface_vlan_tag);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for VirtualMachineInterfaceProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_sub_interface_vlan_tag,
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
                            "subInterfaceVlanTag" => Field::Key_sub_interface_vlan_tag,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualMachineInterfaceProperties;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VirtualMachineInterfaceProperties")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_sub_interface_vlan_tag: Option<i64> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_sub_interface_vlan_tag => value_sub_interface_vlan_tag = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VirtualMachineInterfaceProperties {
                    sub_interface_vlan_tag: value_sub_interface_vlan_tag,
                })
            }
        }

        deserializer.deserialize_struct(
            "VirtualMachineInterfaceProperties",
            &[
                "subInterfaceVlanTag",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for VirtualMachineInterfaceProperties {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VirtualMachineInterfaceProperties",
            self.sub_interface_vlan_tag.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.sub_interface_vlan_tag {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "subInterfaceVlanTag", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VirtualMachineInterfaceProperties {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceProperties".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Advanced Properties of the VirtualMachineInterface.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "subInterfaceVlanTag".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("802.1Q VLAN tag to be used if this interface is a sub-interface of other interface.".to_owned()),
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
