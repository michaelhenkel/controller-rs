// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicySpec

/// FirewallPolicySpec defines the desired state of FirewallPolicy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicySpec {
    /// FirewallRuleReferences contains references to FirewallRule instances attached to this FirewallPolicy.
    pub firewall_rule_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallRuleReference>>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// MatchExpr defines an endpoint type which provides kubernates style match expression for labels/tags.
    pub vmi_selector: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallPolicyVMISelector>,
}

impl k8s_openapi::DeepMerge for FirewallPolicySpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.firewall_rule_references, other.firewall_rule_references);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.vmi_selector, other.vmi_selector);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FirewallPolicySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_firewall_rule_references,
            Key_fq_name,
            Key_vmi_selector,
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
                            "firewallRuleReferences" => Field::Key_firewall_rule_references,
                            "fqName" => Field::Key_fq_name,
                            "vmiSelector" => Field::Key_vmi_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallPolicySpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FirewallPolicySpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_firewall_rule_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallRuleReference>> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_vmi_selector: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallPolicyVMISelector> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_firewall_rule_references => value_firewall_rule_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vmi_selector => value_vmi_selector = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FirewallPolicySpec {
                    firewall_rule_references: value_firewall_rule_references,
                    fq_name: value_fq_name,
                    vmi_selector: value_vmi_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "FirewallPolicySpec",
            &[
                "firewallRuleReferences",
                "fqName",
                "vmiSelector",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for FirewallPolicySpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FirewallPolicySpec",
            self.firewall_rule_references.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.vmi_selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.firewall_rule_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "firewallRuleReferences", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.vmi_selector {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "vmiSelector", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FirewallPolicySpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicySpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FirewallPolicySpec defines the desired state of FirewallPolicy.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "firewallRuleReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FirewallRuleReferences contains references to FirewallRule instances attached to this FirewallPolicy.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallRuleReference>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fqName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FqName is the list of resource names that fully qualify a Contrail resource.".to_owned()),
                                ..Default::default()
                            })),
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
                        "vmiSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallPolicyVMISelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("MatchExpr defines an endpoint type which provides kubernates style match expression for labels/tags.".to_owned()),
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
