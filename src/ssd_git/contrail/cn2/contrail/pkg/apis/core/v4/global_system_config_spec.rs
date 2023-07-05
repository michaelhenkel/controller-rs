// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfigSpec

/// GlobalSystemConfigSpec defines the desired state of the GlobalSystemConfig.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlobalSystemConfigSpec {
    /// AutonomousSystem number for the cluster. By default, this number is 16-bits and has a maximum value of 65535 unless Enable4bytesAS is true, in which case the maximum value is 4294967295.
    pub autonomous_system: Option<i32>,

    /// BGPRouterReferences is the list of references to all BGPRouter instances in the cluster.
    pub bgp_router_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,

    /// DefaultEnableSNAT will enable FabricSNAT by default on all VNs if true. VirtualNetworkSpec.FabricSNAT will override this value.
    pub default_enable_snat: Option<bool>,

    /// Enable4bytesAS enables 32-bit Autonomous System number support when true. When false (the default), ASN is 16-bit.
    pub enable4bytes_as: Option<bool>,

    /// FastConvergenceParameters is an enable/disable knob for all Fast-Convergence parameters to take effect
    pub fast_convergence_parameters: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FastConvergenceParametersType>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// GracefulRestartParameters lets enable the GR/LLGR feature for bgp_helper and xmpp_helper. We need to enable the feature overall to be able to enable the bgp_helper or xmpp_helper or both.
    pub graceful_restart_parameters: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::GracefulRestartParametersType>,

    /// IBGPAutoMesh will automatically create an iBGP mesh if set to true (default).
    pub ibgp_auto_mesh: Option<bool>,
}

impl k8s_openapi::DeepMerge for GlobalSystemConfigSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.autonomous_system, other.autonomous_system);
        k8s_openapi::merge_strategies::list::atomic(&mut self.bgp_router_references, other.bgp_router_references);
        k8s_openapi::DeepMerge::merge_from(&mut self.default_enable_snat, other.default_enable_snat);
        k8s_openapi::DeepMerge::merge_from(&mut self.enable4bytes_as, other.enable4bytes_as);
        k8s_openapi::DeepMerge::merge_from(&mut self.fast_convergence_parameters, other.fast_convergence_parameters);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.graceful_restart_parameters, other.graceful_restart_parameters);
        k8s_openapi::DeepMerge::merge_from(&mut self.ibgp_auto_mesh, other.ibgp_auto_mesh);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for GlobalSystemConfigSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_autonomous_system,
            Key_bgp_router_references,
            Key_default_enable_snat,
            Key_enable4bytes_as,
            Key_fast_convergence_parameters,
            Key_fq_name,
            Key_graceful_restart_parameters,
            Key_ibgp_auto_mesh,
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
                            "autonomousSystem" => Field::Key_autonomous_system,
                            "bgpRouterReferences" => Field::Key_bgp_router_references,
                            "defaultEnableSNAT" => Field::Key_default_enable_snat,
                            "enable4bytesAS" => Field::Key_enable4bytes_as,
                            "fastConvergenceParameters" => Field::Key_fast_convergence_parameters,
                            "fqName" => Field::Key_fq_name,
                            "gracefulRestartParameters" => Field::Key_graceful_restart_parameters,
                            "ibgpAutoMesh" => Field::Key_ibgp_auto_mesh,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = GlobalSystemConfigSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GlobalSystemConfigSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_autonomous_system: Option<i32> = None;
                let mut value_bgp_router_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;
                let mut value_default_enable_snat: Option<bool> = None;
                let mut value_enable4bytes_as: Option<bool> = None;
                let mut value_fast_convergence_parameters: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FastConvergenceParametersType> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_graceful_restart_parameters: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::GracefulRestartParametersType> = None;
                let mut value_ibgp_auto_mesh: Option<bool> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_autonomous_system => value_autonomous_system = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_bgp_router_references => value_bgp_router_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_default_enable_snat => value_default_enable_snat = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_enable4bytes_as => value_enable4bytes_as = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fast_convergence_parameters => value_fast_convergence_parameters = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_graceful_restart_parameters => value_graceful_restart_parameters = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ibgp_auto_mesh => value_ibgp_auto_mesh = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GlobalSystemConfigSpec {
                    autonomous_system: value_autonomous_system,
                    bgp_router_references: value_bgp_router_references,
                    default_enable_snat: value_default_enable_snat,
                    enable4bytes_as: value_enable4bytes_as,
                    fast_convergence_parameters: value_fast_convergence_parameters,
                    fq_name: value_fq_name,
                    graceful_restart_parameters: value_graceful_restart_parameters,
                    ibgp_auto_mesh: value_ibgp_auto_mesh,
                })
            }
        }

        deserializer.deserialize_struct(
            "GlobalSystemConfigSpec",
            &[
                "autonomousSystem",
                "bgpRouterReferences",
                "defaultEnableSNAT",
                "enable4bytesAS",
                "fastConvergenceParameters",
                "fqName",
                "gracefulRestartParameters",
                "ibgpAutoMesh",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for GlobalSystemConfigSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GlobalSystemConfigSpec",
            self.autonomous_system.as_ref().map_or(0, |_| 1) +
            self.bgp_router_references.as_ref().map_or(0, |_| 1) +
            self.default_enable_snat.as_ref().map_or(0, |_| 1) +
            self.enable4bytes_as.as_ref().map_or(0, |_| 1) +
            self.fast_convergence_parameters.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.graceful_restart_parameters.as_ref().map_or(0, |_| 1) +
            self.ibgp_auto_mesh.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.autonomous_system {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "autonomousSystem", value)?;
        }
        if let Some(value) = &self.bgp_router_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "bgpRouterReferences", value)?;
        }
        if let Some(value) = &self.default_enable_snat {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultEnableSNAT", value)?;
        }
        if let Some(value) = &self.enable4bytes_as {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "enable4bytesAS", value)?;
        }
        if let Some(value) = &self.fast_convergence_parameters {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fastConvergenceParameters", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.graceful_restart_parameters {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "gracefulRestartParameters", value)?;
        }
        if let Some(value) = &self.ibgp_auto_mesh {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "ibgpAutoMesh", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for GlobalSystemConfigSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfigSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("GlobalSystemConfigSpec defines the desired state of the GlobalSystemConfig.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "autonomousSystem".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AutonomousSystem number for the cluster. By default, this number is 16-bits and has a maximum value of 65535 unless Enable4bytesAS is true, in which case the maximum value is 4294967295.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "bgpRouterReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGPRouterReferences is the list of references to all BGPRouter instances in the cluster.".to_owned()),
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
                    (
                        "defaultEnableSNAT".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("DefaultEnableSNAT will enable FabricSNAT by default on all VNs if true. VirtualNetworkSpec.FabricSNAT will override this value.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "enable4bytesAS".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Enable4bytesAS enables 32-bit Autonomous System number support when true. When false (the default), ASN is 16-bit.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fastConvergenceParameters".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FastConvergenceParametersType>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FastConvergenceParameters is an enable/disable knob for all Fast-Convergence parameters to take effect".to_owned()),
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
                        "gracefulRestartParameters".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::GracefulRestartParametersType>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("GracefulRestartParameters lets enable the GR/LLGR feature for bgp_helper and xmpp_helper. We need to enable the feature overall to be able to enable the bgp_helper or xmpp_helper or both.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "ibgpAutoMesh".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IBGPAutoMesh will automatically create an iBGP mesh if set to true (default).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
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
