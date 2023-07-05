// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfigSpec

/// GlobalVrouterConfigSpec defines the desired state of GlobalVrouterConfig
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalVrouterConfigSpec {
    /// EncapsulationPriorities is an ordered list of encapsulation types to be used in priority by the vrouter. Valid encapsulation types include MPLSoGRE, MPLSoUDP, and VXLAN.
    pub encapsulation_priorities: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::EncapsulationPriorities>,

    /// FlowExportRate is the rate at which each vrouter will sample and export flow records to analytics.
    pub flow_export_rate: Option<i32>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    pub linklocal_services: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::LinklocalServices>,

    /// Parent contains the ObjectReference to the parent GlobalSystemConfig.
    pub parent: k8s_openapi::api::core::v1::ObjectReference,

    /// PortTranslationPools contains the defined SNAT port translation pools.
    pub port_translation_pools: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::PortTranslationPools>,
}

impl k8s_openapi::DeepMerge for GlobalVrouterConfigSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.encapsulation_priorities, other.encapsulation_priorities);
        k8s_openapi::DeepMerge::merge_from(&mut self.flow_export_rate, other.flow_export_rate);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.linklocal_services, other.linklocal_services);
        k8s_openapi::DeepMerge::merge_from(&mut self.parent, other.parent);
        k8s_openapi::DeepMerge::merge_from(&mut self.port_translation_pools, other.port_translation_pools);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for GlobalVrouterConfigSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_encapsulation_priorities,
            Key_flow_export_rate,
            Key_fq_name,
            Key_linklocal_services,
            Key_parent,
            Key_port_translation_pools,
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
                            "encapsulationPriorities" => Field::Key_encapsulation_priorities,
                            "flowExportRate" => Field::Key_flow_export_rate,
                            "fqName" => Field::Key_fq_name,
                            "linklocalServices" => Field::Key_linklocal_services,
                            "parent" => Field::Key_parent,
                            "portTranslationPools" => Field::Key_port_translation_pools,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = GlobalVrouterConfigSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GlobalVrouterConfigSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_encapsulation_priorities: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::EncapsulationPriorities> = None;
                let mut value_flow_export_rate: Option<i32> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_linklocal_services: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::LinklocalServices> = None;
                let mut value_parent: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_port_translation_pools: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::PortTranslationPools> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_encapsulation_priorities => value_encapsulation_priorities = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_flow_export_rate => value_flow_export_rate = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_linklocal_services => value_linklocal_services = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parent => value_parent = Some(k8s_openapi::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_port_translation_pools => value_port_translation_pools = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GlobalVrouterConfigSpec {
                    encapsulation_priorities: value_encapsulation_priorities,
                    flow_export_rate: value_flow_export_rate,
                    fq_name: value_fq_name,
                    linklocal_services: value_linklocal_services,
                    parent: value_parent.ok_or_else(|| k8s_openapi::serde::de::Error::missing_field("parent"))?,
                    port_translation_pools: value_port_translation_pools,
                })
            }
        }

        deserializer.deserialize_struct(
            "GlobalVrouterConfigSpec",
            &[
                "encapsulationPriorities",
                "flowExportRate",
                "fqName",
                "linklocalServices",
                "parent",
                "portTranslationPools",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for GlobalVrouterConfigSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GlobalVrouterConfigSpec",
            1 +
            self.encapsulation_priorities.as_ref().map_or(0, |_| 1) +
            self.flow_export_rate.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.linklocal_services.as_ref().map_or(0, |_| 1) +
            self.port_translation_pools.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.encapsulation_priorities {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "encapsulationPriorities", value)?;
        }
        if let Some(value) = &self.flow_export_rate {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "flowExportRate", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.linklocal_services {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "linklocalServices", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "parent", &self.parent)?;
        if let Some(value) = &self.port_translation_pools {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "portTranslationPools", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for GlobalVrouterConfigSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfigSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("GlobalVrouterConfigSpec defines the desired state of GlobalVrouterConfig".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "encapsulationPriorities".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::EncapsulationPriorities>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("EncapsulationPriorities is an ordered list of encapsulation types to be used in priority by the vrouter. Valid encapsulation types include MPLSoGRE, MPLSoUDP, and VXLAN.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "flowExportRate".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FlowExportRate is the rate at which each vrouter will sample and export flow records to analytics.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
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
                        "linklocalServices".to_owned(),
                        __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::LinklocalServices>(),
                    ),
                    (
                        "parent".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<k8s_openapi::api::core::v1::ObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Parent contains the ObjectReference to the parent GlobalSystemConfig.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "portTranslationPools".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::PortTranslationPools>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PortTranslationPools contains the defined SNAT port translation pools.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "parent".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
