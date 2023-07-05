// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMatchExpr

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FirewallMatchExpr {
    pub except_ids: Option<Vec<i64>>,

    pub except_list: Option<Vec<String>>,

    pub key: String,

    pub oper: String,

    pub values: Vec<String>,

    pub values_ids: Vec<i64>,
}

impl k8s_openapi::DeepMerge for FirewallMatchExpr {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.except_ids, other.except_ids);
        k8s_openapi::merge_strategies::list::atomic(&mut self.except_list, other.except_list);
        k8s_openapi::DeepMerge::merge_from(&mut self.key, other.key);
        k8s_openapi::DeepMerge::merge_from(&mut self.oper, other.oper);
        k8s_openapi::merge_strategies::list::atomic(&mut self.values, other.values);
        k8s_openapi::merge_strategies::list::atomic(&mut self.values_ids, other.values_ids);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FirewallMatchExpr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_except_ids,
            Key_except_list,
            Key_key,
            Key_oper,
            Key_values,
            Key_values_ids,
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
                            "exceptIds" => Field::Key_except_ids,
                            "exceptList" => Field::Key_except_list,
                            "key" => Field::Key_key,
                            "oper" => Field::Key_oper,
                            "values" => Field::Key_values,
                            "valuesIds" => Field::Key_values_ids,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallMatchExpr;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FirewallMatchExpr")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_except_ids: Option<Vec<i64>> = None;
                let mut value_except_list: Option<Vec<String>> = None;
                let mut value_key: Option<String> = None;
                let mut value_oper: Option<String> = None;
                let mut value_values: Option<Vec<String>> = None;
                let mut value_values_ids: Option<Vec<i64>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_except_ids => value_except_ids = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_except_list => value_except_list = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_key => value_key = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_oper => value_oper = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_values => value_values = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_values_ids => value_values_ids = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FirewallMatchExpr {
                    except_ids: value_except_ids,
                    except_list: value_except_list,
                    key: value_key.unwrap_or_default(),
                    oper: value_oper.unwrap_or_default(),
                    values: value_values.unwrap_or_default(),
                    values_ids: value_values_ids.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "FirewallMatchExpr",
            &[
                "exceptIds",
                "exceptList",
                "key",
                "oper",
                "values",
                "valuesIds",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for FirewallMatchExpr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FirewallMatchExpr",
            4 +
            self.except_ids.as_ref().map_or(0, |_| 1) +
            self.except_list.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.except_ids {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "exceptIds", value)?;
        }
        if let Some(value) = &self.except_list {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "exceptList", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "oper", &self.oper)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "values", &self.values)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "valuesIds", &self.values_ids)?;
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FirewallMatchExpr {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMatchExpr".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "exceptIds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                                        format: Some("int64".to_owned()),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "exceptList".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "key".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "oper".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "values".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "valuesIds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                                        format: Some("int64".to_owned()),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "key".to_owned(),
                    "oper".to_owned(),
                    "values".to_owned(),
                    "valuesIds".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
