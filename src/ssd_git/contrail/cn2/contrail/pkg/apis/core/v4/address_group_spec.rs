// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupSpec

/// AddressGroupSpec describes the set of CIDRs associated with the FirewallRule resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AddressGroupSpec {
    /// AddressGroupPrefixes defines list of subnets used to match a group of workloads.
    pub address_group_prefixes: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AddressGroupPrefix>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,
}

impl k8s_openapi::DeepMerge for AddressGroupSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.address_group_prefixes, other.address_group_prefixes);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for AddressGroupSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_address_group_prefixes,
            Key_fq_name,
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
                            "addressGroupPrefixes" => Field::Key_address_group_prefixes,
                            "fqName" => Field::Key_fq_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = AddressGroupSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AddressGroupSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_address_group_prefixes: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AddressGroupPrefix> = None;
                let mut value_fq_name: Option<Vec<String>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_address_group_prefixes => value_address_group_prefixes = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AddressGroupSpec {
                    address_group_prefixes: value_address_group_prefixes,
                    fq_name: value_fq_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "AddressGroupSpec",
            &[
                "addressGroupPrefixes",
                "fqName",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for AddressGroupSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AddressGroupSpec",
            self.address_group_prefixes.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.address_group_prefixes {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "addressGroupPrefixes", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AddressGroupSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("AddressGroupSpec describes the set of CIDRs associated with the FirewallRule resource.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "addressGroupPrefixes".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AddressGroupPrefix>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AddressGroupPrefixes defines list of subnets used to match a group of workloads.".to_owned()),
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
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
