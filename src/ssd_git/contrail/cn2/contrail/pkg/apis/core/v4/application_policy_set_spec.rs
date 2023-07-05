// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySetSpec

/// ApplicationPolicySetSpec defines the desired state of a FirewallPolicy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ApplicationPolicySetSpec {
    /// ApplicatioPolicySet to be applied to all application tags.
    pub all_applications: Option<bool>,

    /// FirewallPolicyReferences contains references to FirewallPolicy associated with the ApplicationPolicySet.
    pub firewall_policy_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallPolicyReference>>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// ApplicationPolicySetGlobalVrouterConfig reference to defailt-global-vrouter-config
    pub global_vrouter_config_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,

    /// K8sMode defines ApplicationPolicySet in k8s mode
    pub k8smode: Option<bool>,

    /// TagReferences contains references to Tags attached to the ApplicationPolicySet.
    pub tag_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,
}

impl k8s_openapi::DeepMerge for ApplicationPolicySetSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.all_applications, other.all_applications);
        k8s_openapi::merge_strategies::list::atomic(&mut self.firewall_policy_references, other.firewall_policy_references);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.global_vrouter_config_reference, other.global_vrouter_config_reference);
        k8s_openapi::DeepMerge::merge_from(&mut self.k8smode, other.k8smode);
        k8s_openapi::merge_strategies::list::atomic(&mut self.tag_references, other.tag_references);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for ApplicationPolicySetSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_all_applications,
            Key_firewall_policy_references,
            Key_fq_name,
            Key_global_vrouter_config_reference,
            Key_k8smode,
            Key_tag_references,
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
                            "allApplications" => Field::Key_all_applications,
                            "firewallPolicyReferences" => Field::Key_firewall_policy_references,
                            "fqName" => Field::Key_fq_name,
                            "globalVrouterConfigReference" => Field::Key_global_vrouter_config_reference,
                            "k8smode" => Field::Key_k8smode,
                            "tagReferences" => Field::Key_tag_references,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationPolicySetSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ApplicationPolicySetSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_all_applications: Option<bool> = None;
                let mut value_firewall_policy_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallPolicyReference>> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_global_vrouter_config_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;
                let mut value_k8smode: Option<bool> = None;
                let mut value_tag_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_all_applications => value_all_applications = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_firewall_policy_references => value_firewall_policy_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_global_vrouter_config_reference => value_global_vrouter_config_reference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_k8smode => value_k8smode = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tag_references => value_tag_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ApplicationPolicySetSpec {
                    all_applications: value_all_applications,
                    firewall_policy_references: value_firewall_policy_references,
                    fq_name: value_fq_name,
                    global_vrouter_config_reference: value_global_vrouter_config_reference,
                    k8smode: value_k8smode,
                    tag_references: value_tag_references,
                })
            }
        }

        deserializer.deserialize_struct(
            "ApplicationPolicySetSpec",
            &[
                "allApplications",
                "firewallPolicyReferences",
                "fqName",
                "globalVrouterConfigReference",
                "k8smode",
                "tagReferences",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for ApplicationPolicySetSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ApplicationPolicySetSpec",
            self.all_applications.as_ref().map_or(0, |_| 1) +
            self.firewall_policy_references.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.global_vrouter_config_reference.as_ref().map_or(0, |_| 1) +
            self.k8smode.as_ref().map_or(0, |_| 1) +
            self.tag_references.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.all_applications {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "allApplications", value)?;
        }
        if let Some(value) = &self.firewall_policy_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "firewallPolicyReferences", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.global_vrouter_config_reference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "globalVrouterConfigReference", value)?;
        }
        if let Some(value) = &self.k8smode {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "k8smode", value)?;
        }
        if let Some(value) = &self.tag_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "tagReferences", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ApplicationPolicySetSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySetSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ApplicationPolicySetSpec defines the desired state of a FirewallPolicy.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allApplications".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ApplicatioPolicySet to be applied to all application tags.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "firewallPolicyReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FirewallPolicyReferences contains references to FirewallPolicy associated with the ApplicationPolicySet.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallPolicyReference>()))),
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
                        "globalVrouterConfigReference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ApplicationPolicySetGlobalVrouterConfig reference to defailt-global-vrouter-config".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "k8smode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("K8sMode defines ApplicationPolicySet in k8s mode".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "tagReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TagReferences contains references to Tags attached to the ApplicationPolicySet.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>()))),
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
