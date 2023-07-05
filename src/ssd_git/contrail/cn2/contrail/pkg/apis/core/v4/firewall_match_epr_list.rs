// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMatchEprList

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FirewallMatchEprList {
    pub expr_list: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMatchExpr>>,
}

impl k8s_openapi::DeepMerge for FirewallMatchEprList {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.expr_list, other.expr_list);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FirewallMatchEprList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_expr_list,
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
                            "exprList" => Field::Key_expr_list,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallMatchEprList;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FirewallMatchEprList")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_expr_list: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMatchExpr>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_expr_list => value_expr_list = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FirewallMatchEprList {
                    expr_list: value_expr_list,
                })
            }
        }

        deserializer.deserialize_struct(
            "FirewallMatchEprList",
            &[
                "exprList",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for FirewallMatchEprList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FirewallMatchEprList",
            self.expr_list.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.expr_list {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "exprList", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FirewallMatchEprList {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMatchEprList".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "exprList".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMatchExpr>()))),
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
