// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetReferenceAttributes

/// RouteTargetReferenceAttributes allows the configuration of import/export mode.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RouteTargetReferenceAttributes {
    /// ImportExport determines the import/export mode. By default, this field is empty. When ImportExport is blank, bott import and export are supported. Setting ImportExport to "import" enables import-only mode. Setting it to "export" enables export-only mode.
    pub import_export: Option<String>,
}

impl k8s_openapi::DeepMerge for RouteTargetReferenceAttributes {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.import_export, other.import_export);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for RouteTargetReferenceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_import_export,
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
                            "importExport" => Field::Key_import_export,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = RouteTargetReferenceAttributes;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RouteTargetReferenceAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_import_export: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_import_export => value_import_export = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RouteTargetReferenceAttributes {
                    import_export: value_import_export,
                })
            }
        }

        deserializer.deserialize_struct(
            "RouteTargetReferenceAttributes",
            &[
                "importExport",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for RouteTargetReferenceAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RouteTargetReferenceAttributes",
            self.import_export.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.import_export {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "importExport", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for RouteTargetReferenceAttributes {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetReferenceAttributes".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("RouteTargetReferenceAttributes allows the configuration of import/export mode.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "importExport".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ImportExport determines the import/export mode. By default, this field is empty. When ImportExport is blank, bott import and export are supported. Setting ImportExport to \"import\" enables import-only mode. Setting it to \"export\" enables export-only mode.".to_owned()),
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
