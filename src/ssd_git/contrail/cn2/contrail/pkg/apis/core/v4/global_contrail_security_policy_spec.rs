// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicySpec

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlobalContrailSecurityPolicySpec {
    /// Action
    pub action: Option<String>,

    /// Rules
    pub rules: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ContrailSecurityPolicyRule>>,

    pub secondary_actions: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SecondaryActionList>,

    /// Selector for Pod and namespace
    pub selectors: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::GlobalContrailSecurityPolicyPodSelector>>,
}

impl k8s_openapi::DeepMerge for GlobalContrailSecurityPolicySpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.action, other.action);
        k8s_openapi::merge_strategies::list::atomic(&mut self.rules, other.rules);
        k8s_openapi::DeepMerge::merge_from(&mut self.secondary_actions, other.secondary_actions);
        k8s_openapi::merge_strategies::list::atomic(&mut self.selectors, other.selectors);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for GlobalContrailSecurityPolicySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_action,
            Key_rules,
            Key_secondary_actions,
            Key_selectors,
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
                            "action" => Field::Key_action,
                            "rules" => Field::Key_rules,
                            "secondaryActions" => Field::Key_secondary_actions,
                            "selectors" => Field::Key_selectors,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = GlobalContrailSecurityPolicySpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GlobalContrailSecurityPolicySpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_action: Option<String> = None;
                let mut value_rules: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ContrailSecurityPolicyRule>> = None;
                let mut value_secondary_actions: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SecondaryActionList> = None;
                let mut value_selectors: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::GlobalContrailSecurityPolicyPodSelector>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_action => value_action = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secondary_actions => value_secondary_actions = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selectors => value_selectors = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GlobalContrailSecurityPolicySpec {
                    action: value_action,
                    rules: value_rules,
                    secondary_actions: value_secondary_actions,
                    selectors: value_selectors,
                })
            }
        }

        deserializer.deserialize_struct(
            "GlobalContrailSecurityPolicySpec",
            &[
                "action",
                "rules",
                "secondaryActions",
                "selectors",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for GlobalContrailSecurityPolicySpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GlobalContrailSecurityPolicySpec",
            self.action.as_ref().map_or(0, |_| 1) +
            self.rules.as_ref().map_or(0, |_| 1) +
            self.secondary_actions.as_ref().map_or(0, |_| 1) +
            self.selectors.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.action {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "action", value)?;
        }
        if let Some(value) = &self.rules {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", value)?;
        }
        if let Some(value) = &self.secondary_actions {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "secondaryActions", value)?;
        }
        if let Some(value) = &self.selectors {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "selectors", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for GlobalContrailSecurityPolicySpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicySpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "action".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Action".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "rules".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Rules".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ContrailSecurityPolicyRule>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "secondaryActions".to_owned(),
                        __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SecondaryActionList>(),
                    ),
                    (
                        "selectors".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Selector for Pod and namespace".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::GlobalContrailSecurityPolicyPodSelector>()))),
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
