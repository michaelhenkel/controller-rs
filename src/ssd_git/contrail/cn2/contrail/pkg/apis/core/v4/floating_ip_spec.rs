// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPSpec

/// FloatingIPSpec defines the FloatingIP parameters, used for natting pod IPAddress.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FloatingIPSpec {
    /// IP address value for floating IP.
    pub floating_ip_address: Option<String>,

    pub floating_ip_port_mappings: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FloatingIPPortMappings>,

    /// FloatingIPEnablePortMapping controls which ports FloatingIP NAT is applied to. If false, FloatingIP NAT is performed for all Ports. If true, FloatingIP NAT is limited to the defined list of PortMaps.
    pub floating_ip_port_mappings_enable: Option<bool>,

    /// Defines traffic flow direction,(ingress,egress or both),default = both
    pub floating_ip_traffic_direction: Option<String>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// Parent refers to the InstanceIP associated with the FloatingIP.
    pub parent: Option<k8s_openapi::api::core::v1::ObjectReference>,

    /// VirtualMachineInterfaceReferences determines the VirtualMachineInterface of the VirtualMachine associated with this floating IP.
    pub virtual_machine_interface_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,
}

impl k8s_openapi::DeepMerge for FloatingIPSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.floating_ip_address, other.floating_ip_address);
        k8s_openapi::DeepMerge::merge_from(&mut self.floating_ip_port_mappings, other.floating_ip_port_mappings);
        k8s_openapi::DeepMerge::merge_from(&mut self.floating_ip_port_mappings_enable, other.floating_ip_port_mappings_enable);
        k8s_openapi::DeepMerge::merge_from(&mut self.floating_ip_traffic_direction, other.floating_ip_traffic_direction);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.parent, other.parent);
        k8s_openapi::merge_strategies::list::atomic(&mut self.virtual_machine_interface_references, other.virtual_machine_interface_references);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FloatingIPSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_floating_ip_address,
            Key_floating_ip_port_mappings,
            Key_floating_ip_port_mappings_enable,
            Key_floating_ip_traffic_direction,
            Key_fq_name,
            Key_parent,
            Key_virtual_machine_interface_references,
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
                            "floatingIPAddress" => Field::Key_floating_ip_address,
                            "floatingIPPortMappings" => Field::Key_floating_ip_port_mappings,
                            "floatingIPPortMappingsEnable" => Field::Key_floating_ip_port_mappings_enable,
                            "floatingIPTrafficDirection" => Field::Key_floating_ip_traffic_direction,
                            "fqName" => Field::Key_fq_name,
                            "parent" => Field::Key_parent,
                            "virtualMachineInterfaceReferences" => Field::Key_virtual_machine_interface_references,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = FloatingIPSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FloatingIPSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_floating_ip_address: Option<String> = None;
                let mut value_floating_ip_port_mappings: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FloatingIPPortMappings> = None;
                let mut value_floating_ip_port_mappings_enable: Option<bool> = None;
                let mut value_floating_ip_traffic_direction: Option<String> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_parent: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_virtual_machine_interface_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_floating_ip_address => value_floating_ip_address = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_floating_ip_port_mappings => value_floating_ip_port_mappings = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_floating_ip_port_mappings_enable => value_floating_ip_port_mappings_enable = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_floating_ip_traffic_direction => value_floating_ip_traffic_direction = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parent => value_parent = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_machine_interface_references => value_virtual_machine_interface_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FloatingIPSpec {
                    floating_ip_address: value_floating_ip_address,
                    floating_ip_port_mappings: value_floating_ip_port_mappings,
                    floating_ip_port_mappings_enable: value_floating_ip_port_mappings_enable,
                    floating_ip_traffic_direction: value_floating_ip_traffic_direction,
                    fq_name: value_fq_name,
                    parent: value_parent,
                    virtual_machine_interface_references: value_virtual_machine_interface_references,
                })
            }
        }

        deserializer.deserialize_struct(
            "FloatingIPSpec",
            &[
                "floatingIPAddress",
                "floatingIPPortMappings",
                "floatingIPPortMappingsEnable",
                "floatingIPTrafficDirection",
                "fqName",
                "parent",
                "virtualMachineInterfaceReferences",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for FloatingIPSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FloatingIPSpec",
            self.floating_ip_address.as_ref().map_or(0, |_| 1) +
            self.floating_ip_port_mappings.as_ref().map_or(0, |_| 1) +
            self.floating_ip_port_mappings_enable.as_ref().map_or(0, |_| 1) +
            self.floating_ip_traffic_direction.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.parent.as_ref().map_or(0, |_| 1) +
            self.virtual_machine_interface_references.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.floating_ip_address {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "floatingIPAddress", value)?;
        }
        if let Some(value) = &self.floating_ip_port_mappings {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "floatingIPPortMappings", value)?;
        }
        if let Some(value) = &self.floating_ip_port_mappings_enable {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "floatingIPPortMappingsEnable", value)?;
        }
        if let Some(value) = &self.floating_ip_traffic_direction {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "floatingIPTrafficDirection", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.parent {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "parent", value)?;
        }
        if let Some(value) = &self.virtual_machine_interface_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualMachineInterfaceReferences", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FloatingIPSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FloatingIPSpec defines the FloatingIP parameters, used for natting pod IPAddress.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "floatingIPAddress".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IP address value for floating IP.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "floatingIPPortMappings".to_owned(),
                        __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FloatingIPPortMappings>(),
                    ),
                    (
                        "floatingIPPortMappingsEnable".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FloatingIPEnablePortMapping controls which ports FloatingIP NAT is applied to. If false, FloatingIP NAT is performed for all Ports. If true, FloatingIP NAT is limited to the defined list of PortMaps.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "floatingIPTrafficDirection".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Defines traffic flow direction,(ingress,egress or both),default = both".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
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
                        "parent".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<k8s_openapi::api::core::v1::ObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Parent refers to the InstanceIP associated with the FloatingIP.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "virtualMachineInterfaceReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualMachineInterfaceReferences determines the VirtualMachineInterface of the VirtualMachine associated with this floating IP.".to_owned()),
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
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
