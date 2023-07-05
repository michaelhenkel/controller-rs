// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouterSpec

/// VirtualRouterSpec defines the desired state of VirtualRouter
#[derive(Clone, Debug, PartialEq)]
pub struct VirtualRouterSpec {
    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// Parent contains the ObjectReference to the parent GlobalSystemConfig.
    pub parent: k8s_openapi::api::core::v1::ObjectReference,

    /// VirtualMachineReferences is the list of all VirtualMachine instances on this vrouter. This link is present for virtual machines associated to Kubernetes Pods.
    pub virtual_machine_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,

    /// This VirtualRouter's data path is using DPDK library. Virtual machine interfaces scheduled on this compute node will be tagged with additional flags so that they are spawned with user space virtio driver. It is only applicable for embedded VirtualRouters.
    pub virtual_router_dpdk_enabled: Option<bool>,

    /// IP address of the VirtualRouter (required).
    pub virtual_router_ip_address: String,

    /// The type of VirtualRouter in the system.
    pub virtual_router_type: Option<String>,
}

impl k8s_openapi::DeepMerge for VirtualRouterSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.parent, other.parent);
        k8s_openapi::merge_strategies::list::map(
            &mut self.virtual_machine_references,
            other.virtual_machine_references,
            &[|lhs, rhs| lhs.uid == rhs.uid],
            |current_item, other_item| {
                k8s_openapi::DeepMerge::merge_from(current_item, other_item);
            },
        );
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_router_dpdk_enabled, other.virtual_router_dpdk_enabled);
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_router_ip_address, other.virtual_router_ip_address);
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_router_type, other.virtual_router_type);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for VirtualRouterSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fq_name,
            Key_parent,
            Key_virtual_machine_references,
            Key_virtual_router_dpdk_enabled,
            Key_virtual_router_ip_address,
            Key_virtual_router_type,
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
                            "parent" => Field::Key_parent,
                            "virtualMachineReferences" => Field::Key_virtual_machine_references,
                            "virtualRouterDpdkEnabled" => Field::Key_virtual_router_dpdk_enabled,
                            "virtualRouterIPAddress" => Field::Key_virtual_router_ip_address,
                            "virtualRouterType" => Field::Key_virtual_router_type,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualRouterSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VirtualRouterSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_parent: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_virtual_machine_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;
                let mut value_virtual_router_dpdk_enabled: Option<bool> = None;
                let mut value_virtual_router_ip_address: Option<String> = None;
                let mut value_virtual_router_type: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parent => value_parent = Some(k8s_openapi::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_virtual_machine_references => value_virtual_machine_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_router_dpdk_enabled => value_virtual_router_dpdk_enabled = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_router_ip_address => value_virtual_router_ip_address = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_router_type => value_virtual_router_type = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VirtualRouterSpec {
                    fq_name: value_fq_name,
                    parent: value_parent.ok_or_else(|| k8s_openapi::serde::de::Error::missing_field("parent"))?,
                    virtual_machine_references: value_virtual_machine_references,
                    virtual_router_dpdk_enabled: value_virtual_router_dpdk_enabled,
                    virtual_router_ip_address: value_virtual_router_ip_address.unwrap_or_default(),
                    virtual_router_type: value_virtual_router_type,
                })
            }
        }

        deserializer.deserialize_struct(
            "VirtualRouterSpec",
            &[
                "fqName",
                "parent",
                "virtualMachineReferences",
                "virtualRouterDpdkEnabled",
                "virtualRouterIPAddress",
                "virtualRouterType",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for VirtualRouterSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VirtualRouterSpec",
            2 +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.virtual_machine_references.as_ref().map_or(0, |_| 1) +
            self.virtual_router_dpdk_enabled.as_ref().map_or(0, |_| 1) +
            self.virtual_router_type.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "parent", &self.parent)?;
        if let Some(value) = &self.virtual_machine_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualMachineReferences", value)?;
        }
        if let Some(value) = &self.virtual_router_dpdk_enabled {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualRouterDpdkEnabled", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualRouterIPAddress", &self.virtual_router_ip_address)?;
        if let Some(value) = &self.virtual_router_type {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualRouterType", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VirtualRouterSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouterSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VirtualRouterSpec defines the desired state of VirtualRouter".to_owned()),
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
                        "virtualMachineReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualMachineReferences is the list of all VirtualMachine instances on this vrouter. This link is present for virtual machines associated to Kubernetes Pods.".to_owned()),
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
                        "virtualRouterDpdkEnabled".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This VirtualRouter's data path is using DPDK library. Virtual machine interfaces scheduled on this compute node will be tagged with additional flags so that they are spawned with user space virtio driver. It is only applicable for embedded VirtualRouters.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "virtualRouterIPAddress".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IP address of the VirtualRouter (required).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "virtualRouterType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The type of VirtualRouter in the system.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "parent".to_owned(),
                    "virtualRouterIPAddress".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
