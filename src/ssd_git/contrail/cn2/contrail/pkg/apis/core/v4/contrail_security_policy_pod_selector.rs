// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyPodSelector

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContrailSecurityPolicyPodSelector {
    /// This is a label selector which selects Pods. This field follows standard label selector semantics; if present but empty, it selects all pods.
    pub pod_selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

impl k8s_openapi::DeepMerge for ContrailSecurityPolicyPodSelector {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.pod_selector, other.pod_selector);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for ContrailSecurityPolicyPodSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
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
            type Value = ContrailSecurityPolicyPodSelector;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContrailSecurityPolicyPodSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_pod_selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_pod_selector => value_pod_selector = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContrailSecurityPolicyPodSelector {
                    pod_selector: value_pod_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContrailSecurityPolicyPodSelector",
            &[
                "podSelector",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for ContrailSecurityPolicyPodSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContrailSecurityPolicyPodSelector",
            self.pod_selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.pod_selector {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "podSelector", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContrailSecurityPolicyPodSelector {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyPodSelector".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "podSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This is a label selector which selects Pods. This field follows standard label selector semantics; if present but empty, it selects all pods.".to_owned()),
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
