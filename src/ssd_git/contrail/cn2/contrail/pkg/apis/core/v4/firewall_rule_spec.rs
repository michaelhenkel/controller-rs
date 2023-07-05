// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleSpec

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FirewallRuleSpec {
    /// ActionList defines actions to be performed if packets match condition.
    pub action_list: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallActionListType>,

    /// AddressGroupReference refers to an AddressGroup defining the CIDR of a FirewallRule.
    pub address_group_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,

    /// Direction defines direction in the FirewallRule.
    pub direction: Option<String>,

    /// Endpoint1 defines match condition for source traffic.
    pub endpoint1: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallRuleEndpointType>,

    /// Endpoint2 defines match condition for destination traffic.
    pub endpoint2: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallRuleEndpointType>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    pub k8s_mode: Option<bool>,

    /// MatchTags defines matching tags for source and destination endpoints.
    pub match_tags: Option<Vec<String>>,

    /// MatchTagsTypes defines matching tags ids for source and destination endpoints.
    pub match_tags_types: Option<Vec<i64>>,

    /// Service defines the service (port, protocol) for packets match condition.
    pub service: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallServiceType>,

    /// ServiceGroupReference refers to an ServiceGroup defining the list of port/proto of a FirewallRule.
    pub service_group_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,

    /// TagReferences refers to Tags associated with the FirewallRule.
    pub tag_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,
}

impl k8s_openapi::DeepMerge for FirewallRuleSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.action_list, other.action_list);
        k8s_openapi::DeepMerge::merge_from(&mut self.address_group_reference, other.address_group_reference);
        k8s_openapi::DeepMerge::merge_from(&mut self.direction, other.direction);
        k8s_openapi::DeepMerge::merge_from(&mut self.endpoint1, other.endpoint1);
        k8s_openapi::DeepMerge::merge_from(&mut self.endpoint2, other.endpoint2);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.k8s_mode, other.k8s_mode);
        k8s_openapi::merge_strategies::list::atomic(&mut self.match_tags, other.match_tags);
        k8s_openapi::merge_strategies::list::atomic(&mut self.match_tags_types, other.match_tags_types);
        k8s_openapi::DeepMerge::merge_from(&mut self.service, other.service);
        k8s_openapi::DeepMerge::merge_from(&mut self.service_group_reference, other.service_group_reference);
        k8s_openapi::merge_strategies::list::atomic(&mut self.tag_references, other.tag_references);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FirewallRuleSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_action_list,
            Key_address_group_reference,
            Key_direction,
            Key_endpoint1,
            Key_endpoint2,
            Key_fq_name,
            Key_k8s_mode,
            Key_match_tags,
            Key_match_tags_types,
            Key_service,
            Key_service_group_reference,
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
                            "actionList" => Field::Key_action_list,
                            "addressGroupReference" => Field::Key_address_group_reference,
                            "direction" => Field::Key_direction,
                            "endpoint1" => Field::Key_endpoint1,
                            "endpoint2" => Field::Key_endpoint2,
                            "fqName" => Field::Key_fq_name,
                            "k8sMode" => Field::Key_k8s_mode,
                            "matchTags" => Field::Key_match_tags,
                            "matchTagsTypes" => Field::Key_match_tags_types,
                            "service" => Field::Key_service,
                            "serviceGroupReference" => Field::Key_service_group_reference,
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
            type Value = FirewallRuleSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FirewallRuleSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_action_list: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallActionListType> = None;
                let mut value_address_group_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;
                let mut value_direction: Option<String> = None;
                let mut value_endpoint1: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallRuleEndpointType> = None;
                let mut value_endpoint2: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallRuleEndpointType> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_k8s_mode: Option<bool> = None;
                let mut value_match_tags: Option<Vec<String>> = None;
                let mut value_match_tags_types: Option<Vec<i64>> = None;
                let mut value_service: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallServiceType> = None;
                let mut value_service_group_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;
                let mut value_tag_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_action_list => value_action_list = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_address_group_reference => value_address_group_reference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_direction => value_direction = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_endpoint1 => value_endpoint1 = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_endpoint2 => value_endpoint2 = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_k8s_mode => value_k8s_mode = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_tags => value_match_tags = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_tags_types => value_match_tags_types = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service => value_service = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_group_reference => value_service_group_reference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tag_references => value_tag_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FirewallRuleSpec {
                    action_list: value_action_list,
                    address_group_reference: value_address_group_reference,
                    direction: value_direction,
                    endpoint1: value_endpoint1,
                    endpoint2: value_endpoint2,
                    fq_name: value_fq_name,
                    k8s_mode: value_k8s_mode,
                    match_tags: value_match_tags,
                    match_tags_types: value_match_tags_types,
                    service: value_service,
                    service_group_reference: value_service_group_reference,
                    tag_references: value_tag_references,
                })
            }
        }

        deserializer.deserialize_struct(
            "FirewallRuleSpec",
            &[
                "actionList",
                "addressGroupReference",
                "direction",
                "endpoint1",
                "endpoint2",
                "fqName",
                "k8sMode",
                "matchTags",
                "matchTagsTypes",
                "service",
                "serviceGroupReference",
                "tagReferences",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for FirewallRuleSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FirewallRuleSpec",
            self.action_list.as_ref().map_or(0, |_| 1) +
            self.address_group_reference.as_ref().map_or(0, |_| 1) +
            self.direction.as_ref().map_or(0, |_| 1) +
            self.endpoint1.as_ref().map_or(0, |_| 1) +
            self.endpoint2.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.k8s_mode.as_ref().map_or(0, |_| 1) +
            self.match_tags.as_ref().map_or(0, |_| 1) +
            self.match_tags_types.as_ref().map_or(0, |_| 1) +
            self.service.as_ref().map_or(0, |_| 1) +
            self.service_group_reference.as_ref().map_or(0, |_| 1) +
            self.tag_references.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.action_list {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "actionList", value)?;
        }
        if let Some(value) = &self.address_group_reference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "addressGroupReference", value)?;
        }
        if let Some(value) = &self.direction {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "direction", value)?;
        }
        if let Some(value) = &self.endpoint1 {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "endpoint1", value)?;
        }
        if let Some(value) = &self.endpoint2 {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "endpoint2", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.k8s_mode {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "k8sMode", value)?;
        }
        if let Some(value) = &self.match_tags {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "matchTags", value)?;
        }
        if let Some(value) = &self.match_tags_types {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "matchTagsTypes", value)?;
        }
        if let Some(value) = &self.service {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "service", value)?;
        }
        if let Some(value) = &self.service_group_reference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceGroupReference", value)?;
        }
        if let Some(value) = &self.tag_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "tagReferences", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FirewallRuleSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "actionList".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallActionListType>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ActionList defines actions to be performed if packets match condition.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "addressGroupReference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AddressGroupReference refers to an AddressGroup defining the CIDR of a FirewallRule.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "direction".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Direction defines direction in the FirewallRule.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "endpoint1".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallRuleEndpointType>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Endpoint1 defines match condition for source traffic.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "endpoint2".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallRuleEndpointType>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Endpoint2 defines match condition for destination traffic.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
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
                        "k8sMode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "matchTags".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("MatchTags defines matching tags for source and destination endpoints.".to_owned()),
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
                        "matchTagsTypes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("MatchTagsTypes defines matching tags ids for source and destination endpoints.".to_owned()),
                                ..Default::default()
                            })),
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
                        "service".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallServiceType>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Service defines the service (port, protocol) for packets match condition.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "serviceGroupReference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ServiceGroupReference refers to an ServiceGroup defining the list of port/proto of a FirewallRule.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "tagReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TagReferences refers to Tags associated with the FirewallRule.".to_owned()),
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
