// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.KeyValuePair

/// KeyValuePair is attribute associated with VirtualNetwork
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KeyValuePair {
    pub virtual_network_virtual_network_router_name: Option<String>,
}

impl k8s_openapi::DeepMerge for KeyValuePair {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_network_virtual_network_router_name, other.virtual_network_virtual_network_router_name);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for KeyValuePair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_virtual_network_virtual_network_router_name,
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
                            "virtualNetworkVirtualNetworkRouterName" => Field::Key_virtual_network_virtual_network_router_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = KeyValuePair;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("KeyValuePair")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_virtual_network_virtual_network_router_name: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_virtual_network_virtual_network_router_name => value_virtual_network_virtual_network_router_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(KeyValuePair {
                    virtual_network_virtual_network_router_name: value_virtual_network_virtual_network_router_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "KeyValuePair",
            &[
                "virtualNetworkVirtualNetworkRouterName",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for KeyValuePair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "KeyValuePair",
            self.virtual_network_virtual_network_router_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.virtual_network_virtual_network_router_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualNetworkVirtualNetworkRouterName", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for KeyValuePair {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.KeyValuePair".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("KeyValuePair is attribute associated with VirtualNetwork".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "virtualNetworkVirtualNetworkRouterName".to_owned(),
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
