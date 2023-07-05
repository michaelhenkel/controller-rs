// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineSpec

/// VirtualMachineSpec defines the desired state of a VirtualMachine.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualMachineSpec {
    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// Deprecated: ServerClusterName is the name of the cluster the VirtualMachine resource is to run on.
    pub server_cluster_name: String,

    /// ServerName is the name of the VirtualMachine resource.
    pub server_name: String,

    /// ServerNamespace is the namespace of the VirtualMachine resource.
    pub server_namespace: String,

    /// ServerType indicates the computational type of the VirtualMachine resource. Valid values for ServerType include virtual-server, baremetal-server, or container.
    pub server_type: Option<String>,
}

impl k8s_openapi::DeepMerge for VirtualMachineSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.server_cluster_name, other.server_cluster_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.server_name, other.server_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.server_namespace, other.server_namespace);
        k8s_openapi::DeepMerge::merge_from(&mut self.server_type, other.server_type);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for VirtualMachineSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fq_name,
            Key_server_cluster_name,
            Key_server_name,
            Key_server_namespace,
            Key_server_type,
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
                            "serverClusterName" => Field::Key_server_cluster_name,
                            "serverName" => Field::Key_server_name,
                            "serverNamespace" => Field::Key_server_namespace,
                            "serverType" => Field::Key_server_type,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualMachineSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VirtualMachineSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_server_cluster_name: Option<String> = None;
                let mut value_server_name: Option<String> = None;
                let mut value_server_namespace: Option<String> = None;
                let mut value_server_type: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_server_cluster_name => value_server_cluster_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_server_name => value_server_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_server_namespace => value_server_namespace = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_server_type => value_server_type = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VirtualMachineSpec {
                    fq_name: value_fq_name,
                    server_cluster_name: value_server_cluster_name.unwrap_or_default(),
                    server_name: value_server_name.unwrap_or_default(),
                    server_namespace: value_server_namespace.unwrap_or_default(),
                    server_type: value_server_type,
                })
            }
        }

        deserializer.deserialize_struct(
            "VirtualMachineSpec",
            &[
                "fqName",
                "serverClusterName",
                "serverName",
                "serverNamespace",
                "serverType",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for VirtualMachineSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VirtualMachineSpec",
            3 +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.server_type.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "serverClusterName", &self.server_cluster_name)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "serverName", &self.server_name)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "serverNamespace", &self.server_namespace)?;
        if let Some(value) = &self.server_type {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "serverType", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VirtualMachineSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VirtualMachineSpec defines the desired state of a VirtualMachine.".to_owned()),
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
                        "serverClusterName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Deprecated: ServerClusterName is the name of the cluster the VirtualMachine resource is to run on.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "serverName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ServerName is the name of the VirtualMachine resource.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "serverNamespace".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ServerNamespace is the namespace of the VirtualMachine resource.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "serverType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ServerType indicates the computational type of the VirtualMachine resource. Valid values for ServerType include virtual-server, baremetal-server, or container.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "serverClusterName".to_owned(),
                    "serverName".to_owned(),
                    "serverNamespace".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
