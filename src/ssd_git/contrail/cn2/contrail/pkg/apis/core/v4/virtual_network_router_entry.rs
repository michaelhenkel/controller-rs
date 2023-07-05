// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterEntry

/// VirtualNetworkRouterEntry is a combination of VirtualNetworkRouterSelector and NamespaceSelector. Together, these two LabelSelectors identify a VirtualNetworkRouter.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualNetworkRouterEntry {
    /// LabelSelector to identify the Namespace of the VirtualNetworkRouter.
    pub namespace_selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// LabelSelector to identify the VirtualNetworkRouter
    pub virtual_network_router_selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

impl k8s_openapi::DeepMerge for VirtualNetworkRouterEntry {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.namespace_selector, other.namespace_selector);
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_network_router_selector, other.virtual_network_router_selector);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for VirtualNetworkRouterEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_namespace_selector,
            Key_virtual_network_router_selector,
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
                            "namespaceSelector" => Field::Key_namespace_selector,
                            "virtualNetworkRouterSelector" => Field::Key_virtual_network_router_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualNetworkRouterEntry;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VirtualNetworkRouterEntry")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_namespace_selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_virtual_network_router_selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_namespace_selector => value_namespace_selector = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_network_router_selector => value_virtual_network_router_selector = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VirtualNetworkRouterEntry {
                    namespace_selector: value_namespace_selector,
                    virtual_network_router_selector: value_virtual_network_router_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "VirtualNetworkRouterEntry",
            &[
                "namespaceSelector",
                "virtualNetworkRouterSelector",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for VirtualNetworkRouterEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VirtualNetworkRouterEntry",
            self.namespace_selector.as_ref().map_or(0, |_| 1) +
            self.virtual_network_router_selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.namespace_selector {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaceSelector", value)?;
        }
        if let Some(value) = &self.virtual_network_router_selector {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualNetworkRouterSelector", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VirtualNetworkRouterEntry {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterEntry".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VirtualNetworkRouterEntry is a combination of VirtualNetworkRouterSelector and NamespaceSelector. Together, these two LabelSelectors identify a VirtualNetworkRouter.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "namespaceSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("LabelSelector to identify the Namespace of the VirtualNetworkRouter.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "virtualNetworkRouterSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("LabelSelector to identify the VirtualNetworkRouter".to_owned()),
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
