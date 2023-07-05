// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicySelector

/// ContrailSecurityPolicySelector describes endpoint for contrail security policy. A endpoint could be LabelSelector or an IpBlock. Only certain combinations of fields are allowed
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContrailSecurityPolicySelector {
    /// IPBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.
    pub ip_block: Option<k8s_openapi::api::networking::v1::IPBlock>,

    /// Selects Namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces.
    ///
    /// If PodSelector is also set, then the ContrailSecurityPolicySelector as a whole selects the Pods matching PodSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects all Pods in the Namespaces selected by NamespaceSelector.
    pub namespace_selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// This is a label selector which selects Pods. This field follows standard label selector semantics; if present but empty, it selects all pods.
    ///
    /// If NamespaceSelector is also set, then the ContrailSecurityPolicySelector as a whole selects the Pods matching PodSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the Pods matching PodSelector in the policy's own Namespace.
    pub pod_selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

impl k8s_openapi::DeepMerge for ContrailSecurityPolicySelector {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.ip_block, other.ip_block);
        k8s_openapi::DeepMerge::merge_from(&mut self.namespace_selector, other.namespace_selector);
        k8s_openapi::DeepMerge::merge_from(&mut self.pod_selector, other.pod_selector);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for ContrailSecurityPolicySelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ip_block,
            Key_namespace_selector,
            Key_pod_selector,
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
                            "ipBlock" => Field::Key_ip_block,
                            "namespaceSelector" => Field::Key_namespace_selector,
                            "podSelector" => Field::Key_pod_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = ContrailSecurityPolicySelector;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContrailSecurityPolicySelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_ip_block: Option<k8s_openapi::api::networking::v1::IPBlock> = None;
                let mut value_namespace_selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_pod_selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ip_block => value_ip_block = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace_selector => value_namespace_selector = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_selector => value_pod_selector = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContrailSecurityPolicySelector {
                    ip_block: value_ip_block,
                    namespace_selector: value_namespace_selector,
                    pod_selector: value_pod_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContrailSecurityPolicySelector",
            &[
                "ipBlock",
                "namespaceSelector",
                "podSelector",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for ContrailSecurityPolicySelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContrailSecurityPolicySelector",
            self.ip_block.as_ref().map_or(0, |_| 1) +
            self.namespace_selector.as_ref().map_or(0, |_| 1) +
            self.pod_selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ip_block {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "ipBlock", value)?;
        }
        if let Some(value) = &self.namespace_selector {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaceSelector", value)?;
        }
        if let Some(value) = &self.pod_selector {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "podSelector", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContrailSecurityPolicySelector {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicySelector".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ContrailSecurityPolicySelector describes endpoint for contrail security policy. A endpoint could be LabelSelector or an IpBlock. Only certain combinations of fields are allowed".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "ipBlock".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<k8s_openapi::api::networking::v1::IPBlock>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IPBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "namespaceSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Selects Namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces.\n\nIf PodSelector is also set, then the ContrailSecurityPolicySelector as a whole selects the Pods matching PodSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects all Pods in the Namespaces selected by NamespaceSelector.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "podSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This is a label selector which selects Pods. This field follows standard label selector semantics; if present but empty, it selects all pods.\n\nIf NamespaceSelector is also set, then the ContrailSecurityPolicySelector as a whole selects the Pods matching PodSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the Pods matching PodSelector in the policy's own Namespace.".to_owned()),
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
