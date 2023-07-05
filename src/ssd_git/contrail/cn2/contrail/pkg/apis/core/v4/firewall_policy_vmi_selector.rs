// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyVMISelector

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyVMISelector {
    pub match_expr_list: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMatchEprList>>,
}

impl k8s_openapi::DeepMerge for FirewallPolicyVMISelector {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.match_expr_list, other.match_expr_list);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FirewallPolicyVMISelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_match_expr_list,
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
                            "matchExprList" => Field::Key_match_expr_list,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallPolicyVMISelector;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FirewallPolicyVMISelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_match_expr_list: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMatchEprList>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_expr_list => value_match_expr_list = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FirewallPolicyVMISelector {
                    match_expr_list: value_match_expr_list,
                })
            }
        }

        deserializer.deserialize_struct(
            "FirewallPolicyVMISelector",
            &[
                "matchExprList",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for FirewallPolicyVMISelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FirewallPolicyVMISelector",
            self.match_expr_list.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.match_expr_list {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "matchExprList", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FirewallPolicyVMISelector {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyVMISelector".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "matchExprList".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMatchEprList>()))),
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
