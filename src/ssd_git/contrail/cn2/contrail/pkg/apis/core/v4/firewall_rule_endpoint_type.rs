// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleEndpointType

/// FirewallRuleEndpointType defines the EndpointType.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FirewallRuleEndpointType {
    /// Addressgroup defines what CIDR FirewallRule can be applied on.
    pub address_group: Option<String>,

    /// Any defines matching to "any" value in an FirewallRuleEndpointType . ie "*"
    pub any: Option<bool>,

    /// MatchExpr defines an endpoint type which provides kubernates style match expression for labels/tags.
    pub match_expr: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMatchExpr>>,

    pub match_expr_list: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMatchEprList>>,

    /// Subnet defines the IP prefix and length.
    pub subnet: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallSubnet>,

    /// TagIds define IDs created for referred Tags.
    pub tag_ids: Option<Vec<i64>>,

    /// Tags define tags for the FirewallRule.
    pub tags: Option<Vec<String>>,
}

impl k8s_openapi::DeepMerge for FirewallRuleEndpointType {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.address_group, other.address_group);
        k8s_openapi::DeepMerge::merge_from(&mut self.any, other.any);
        k8s_openapi::merge_strategies::list::atomic(&mut self.match_expr, other.match_expr);
        k8s_openapi::merge_strategies::list::atomic(&mut self.match_expr_list, other.match_expr_list);
        k8s_openapi::DeepMerge::merge_from(&mut self.subnet, other.subnet);
        k8s_openapi::merge_strategies::list::atomic(&mut self.tag_ids, other.tag_ids);
        k8s_openapi::merge_strategies::list::atomic(&mut self.tags, other.tags);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FirewallRuleEndpointType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_address_group,
            Key_any,
            Key_match_expr,
            Key_match_expr_list,
            Key_subnet,
            Key_tag_ids,
            Key_tags,
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
                            "addressGroup" => Field::Key_address_group,
                            "any" => Field::Key_any,
                            "matchExpr" => Field::Key_match_expr,
                            "matchExprList" => Field::Key_match_expr_list,
                            "subnet" => Field::Key_subnet,
                            "tagIds" => Field::Key_tag_ids,
                            "tags" => Field::Key_tags,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallRuleEndpointType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FirewallRuleEndpointType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_address_group: Option<String> = None;
                let mut value_any: Option<bool> = None;
                let mut value_match_expr: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMatchExpr>> = None;
                let mut value_match_expr_list: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMatchEprList>> = None;
                let mut value_subnet: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallSubnet> = None;
                let mut value_tag_ids: Option<Vec<i64>> = None;
                let mut value_tags: Option<Vec<String>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_address_group => value_address_group = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_any => value_any = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_expr => value_match_expr = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_expr_list => value_match_expr_list = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subnet => value_subnet = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tag_ids => value_tag_ids = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tags => value_tags = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FirewallRuleEndpointType {
                    address_group: value_address_group,
                    any: value_any,
                    match_expr: value_match_expr,
                    match_expr_list: value_match_expr_list,
                    subnet: value_subnet,
                    tag_ids: value_tag_ids,
                    tags: value_tags,
                })
            }
        }

        deserializer.deserialize_struct(
            "FirewallRuleEndpointType",
            &[
                "addressGroup",
                "any",
                "matchExpr",
                "matchExprList",
                "subnet",
                "tagIds",
                "tags",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for FirewallRuleEndpointType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FirewallRuleEndpointType",
            self.address_group.as_ref().map_or(0, |_| 1) +
            self.any.as_ref().map_or(0, |_| 1) +
            self.match_expr.as_ref().map_or(0, |_| 1) +
            self.match_expr_list.as_ref().map_or(0, |_| 1) +
            self.subnet.as_ref().map_or(0, |_| 1) +
            self.tag_ids.as_ref().map_or(0, |_| 1) +
            self.tags.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.address_group {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "addressGroup", value)?;
        }
        if let Some(value) = &self.any {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "any", value)?;
        }
        if let Some(value) = &self.match_expr {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "matchExpr", value)?;
        }
        if let Some(value) = &self.match_expr_list {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "matchExprList", value)?;
        }
        if let Some(value) = &self.subnet {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "subnet", value)?;
        }
        if let Some(value) = &self.tag_ids {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "tagIds", value)?;
        }
        if let Some(value) = &self.tags {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "tags", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FirewallRuleEndpointType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleEndpointType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FirewallRuleEndpointType defines the EndpointType.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "addressGroup".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Addressgroup defines what CIDR FirewallRule can be applied on.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "any".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Any defines matching to \"any\" value in an FirewallRuleEndpointType . ie \"*\"".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "matchExpr".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("MatchExpr defines an endpoint type which provides kubernates style match expression for labels/tags.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMatchExpr>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
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
                    (
                        "subnet".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallSubnet>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Subnet defines the IP prefix and length.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "tagIds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TagIds define IDs created for referred Tags.".to_owned()),
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
                        "tags".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Tags define tags for the FirewallRule.".to_owned()),
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
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
