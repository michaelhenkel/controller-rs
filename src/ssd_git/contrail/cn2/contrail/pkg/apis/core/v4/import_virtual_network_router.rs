// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ImportVirtualNetworkRouter

/// ImportVirtualNetworkRouter is a list of other VirtualNetworkRouters imported by the current VirtualNetworkRouter.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImportVirtualNetworkRouter {
    /// VirtualNetworkRouters is a list of VirtualNetworkRouterEntry.
    pub virtual_network_routers: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualNetworkRouterEntry>>,
}

impl k8s_openapi::DeepMerge for ImportVirtualNetworkRouter {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.virtual_network_routers, other.virtual_network_routers);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for ImportVirtualNetworkRouter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_virtual_network_routers,
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
                            "virtualNetworkRouters" => Field::Key_virtual_network_routers,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = ImportVirtualNetworkRouter;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ImportVirtualNetworkRouter")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_virtual_network_routers: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualNetworkRouterEntry>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_virtual_network_routers => value_virtual_network_routers = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ImportVirtualNetworkRouter {
                    virtual_network_routers: value_virtual_network_routers,
                })
            }
        }

        deserializer.deserialize_struct(
            "ImportVirtualNetworkRouter",
            &[
                "virtualNetworkRouters",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for ImportVirtualNetworkRouter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ImportVirtualNetworkRouter",
            self.virtual_network_routers.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.virtual_network_routers {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualNetworkRouters", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ImportVirtualNetworkRouter {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ImportVirtualNetworkRouter".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ImportVirtualNetworkRouter is a list of other VirtualNetworkRouters imported by the current VirtualNetworkRouter.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "virtualNetworkRouters".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualNetworkRouters is a list of VirtualNetworkRouterEntry.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualNetworkRouterEntry>()))),
                                ..Default::default()
                            })),
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
