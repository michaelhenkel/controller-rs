// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckSpec

/// ServiceHealthCheckSpec defines the desired state of ServiceHealthCheck
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceHealthCheckSpec {
    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    pub service_health_check_properties: crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceHealthCheckProperties,
}

impl k8s_openapi::DeepMerge for ServiceHealthCheckSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.service_health_check_properties, other.service_health_check_properties);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for ServiceHealthCheckSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fq_name,
            Key_service_health_check_properties,
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
                            "serviceHealthCheckProperties" => Field::Key_service_health_check_properties,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceHealthCheckSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServiceHealthCheckSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_service_health_check_properties: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceHealthCheckProperties> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_health_check_properties => value_service_health_check_properties = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceHealthCheckSpec {
                    fq_name: value_fq_name,
                    service_health_check_properties: value_service_health_check_properties.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceHealthCheckSpec",
            &[
                "fqName",
                "serviceHealthCheckProperties",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for ServiceHealthCheckSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceHealthCheckSpec",
            1 +
            self.fq_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceHealthCheckProperties", &self.service_health_check_properties)?;
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ServiceHealthCheckSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ServiceHealthCheckSpec defines the desired state of ServiceHealthCheck".to_owned()),
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
                        "serviceHealthCheckProperties".to_owned(),
                        __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceHealthCheckProperties>(),
                    ),
                ].into(),
                required: [
                    "serviceHealthCheckProperties".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
