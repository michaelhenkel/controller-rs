// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.TagSpec

/// TagSpec defines the desired state of Tag
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TagSpec {
    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// TagTypeName defines name of Tag Type object in string format.
    pub tag_type_name: Option<String>,

    /// TagTypeReference is reference to Tagtype object attachd to this Tag object.
    pub tag_type_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,

    /// TagValue defines namee of Tag Value object in string format.
    pub tag_value: Option<String>,
}

impl k8s_openapi::DeepMerge for TagSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.tag_type_name, other.tag_type_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.tag_type_reference, other.tag_type_reference);
        k8s_openapi::DeepMerge::merge_from(&mut self.tag_value, other.tag_value);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for TagSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fq_name,
            Key_tag_type_name,
            Key_tag_type_reference,
            Key_tag_value,
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
                            "fqName" => Field::Key_fq_name,
                            "tagTypeName" => Field::Key_tag_type_name,
                            "tagTypeReference" => Field::Key_tag_type_reference,
                            "tagValue" => Field::Key_tag_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = TagSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TagSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_tag_type_name: Option<String> = None;
                let mut value_tag_type_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;
                let mut value_tag_value: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tag_type_name => value_tag_type_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tag_type_reference => value_tag_type_reference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tag_value => value_tag_value = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TagSpec {
                    fq_name: value_fq_name,
                    tag_type_name: value_tag_type_name,
                    tag_type_reference: value_tag_type_reference,
                    tag_value: value_tag_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "TagSpec",
            &[
                "fqName",
                "tagTypeName",
                "tagTypeReference",
                "tagValue",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for TagSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TagSpec",
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.tag_type_name.as_ref().map_or(0, |_| 1) +
            self.tag_type_reference.as_ref().map_or(0, |_| 1) +
            self.tag_value.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.tag_type_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "tagTypeName", value)?;
        }
        if let Some(value) = &self.tag_type_reference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "tagTypeReference", value)?;
        }
        if let Some(value) = &self.tag_value {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "tagValue", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TagSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.TagSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("TagSpec defines the desired state of Tag".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
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
                        "tagTypeName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TagTypeName defines name of Tag Type object in string format.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "tagTypeReference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TagTypeReference is reference to Tagtype object attachd to this Tag object.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "tagValue".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TagValue defines namee of Tag Value object in string format.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
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
