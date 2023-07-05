// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyRule

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContrailSecurityPolicyRule {
    pub dst_ep: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ContrailSecurityPolicyEndPoint>,

    pub ports: Option<Vec<k8s_openapi::api::networking::v1::NetworkPolicyPort>>,

    /// This field selects secondaty action .
    pub secondary_actions: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SecondaryActionList>,

    pub src_ep: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ContrailSecurityPolicyEndPoint>,
}

impl k8s_openapi::DeepMerge for ContrailSecurityPolicyRule {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.dst_ep, other.dst_ep);
        k8s_openapi::merge_strategies::list::atomic(&mut self.ports, other.ports);
        k8s_openapi::DeepMerge::merge_from(&mut self.secondary_actions, other.secondary_actions);
        k8s_openapi::DeepMerge::merge_from(&mut self.src_ep, other.src_ep);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for ContrailSecurityPolicyRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_dst_ep,
            Key_ports,
            Key_secondary_actions,
            Key_src_ep,
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
                            "dstEP" => Field::Key_dst_ep,
                            "ports" => Field::Key_ports,
                            "secondaryActions" => Field::Key_secondary_actions,
                            "srcEP" => Field::Key_src_ep,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = ContrailSecurityPolicyRule;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContrailSecurityPolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_dst_ep: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ContrailSecurityPolicyEndPoint> = None;
                let mut value_ports: Option<Vec<k8s_openapi::api::networking::v1::NetworkPolicyPort>> = None;
                let mut value_secondary_actions: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SecondaryActionList> = None;
                let mut value_src_ep: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ContrailSecurityPolicyEndPoint> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_dst_ep => value_dst_ep = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ports => value_ports = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secondary_actions => value_secondary_actions = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_src_ep => value_src_ep = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContrailSecurityPolicyRule {
                    dst_ep: value_dst_ep,
                    ports: value_ports,
                    secondary_actions: value_secondary_actions,
                    src_ep: value_src_ep,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContrailSecurityPolicyRule",
            &[
                "dstEP",
                "ports",
                "secondaryActions",
                "srcEP",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for ContrailSecurityPolicyRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContrailSecurityPolicyRule",
            self.dst_ep.as_ref().map_or(0, |_| 1) +
            self.ports.as_ref().map_or(0, |_| 1) +
            self.secondary_actions.as_ref().map_or(0, |_| 1) +
            self.src_ep.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.dst_ep {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "dstEP", value)?;
        }
        if let Some(value) = &self.ports {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", value)?;
        }
        if let Some(value) = &self.secondary_actions {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "secondaryActions", value)?;
        }
        if let Some(value) = &self.src_ep {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "srcEP", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContrailSecurityPolicyRule {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyRule".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "dstEP".to_owned(),
                        __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ContrailSecurityPolicyEndPoint>(),
                    ),
                    (
                        "ports".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<k8s_openapi::api::networking::v1::NetworkPolicyPort>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "secondaryActions".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SecondaryActionList>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This field selects secondaty action .".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "srcEP".to_owned(),
                        __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ContrailSecurityPolicyEndPoint>(),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
