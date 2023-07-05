// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceSpec

/// VirtualMachineInterfaceSpec defines the desired state of VirtualMachineInterface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualMachineInterfaceSpec {
    /// List of (IP address, MAC) other than instance ip on this interface.
    pub allowed_address_pairs: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AllowedAddressPairs>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// Object reference to namespace or virtualrouter parent.
    pub parent: Option<k8s_openapi::api::core::v1::ObjectReference>,

    /// Port security status on the network.
    pub port_security_enabled: Option<bool>,

    /// VirtualMachineInterface properties
    pub properties: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualMachineInterfaceProperties>,

    /// Reference to Tag attached to this Virtual Machine Interface
    pub tag_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,

    /// DisablePolicy disables all policy checks for ingress and egress traffic from this interface. Flow table entries are not created. Features that require policy will not work on this interface, these include security group, floating IP, service chain, linklocal services.
    pub virtual_machine_interface_disable_policy: Option<bool>,

    /// MAC address of the virtual machine interface, automatically assigned by system if not provided.
    pub virtual_machine_interface_mac_addresses: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::MACAddresses>,

    /// VirtualMachineInterfaceReferences determines the VirtualMachineInterface instances that are sub-interfaces.
    pub virtual_machine_interface_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,

    /// VirtualMachineReferences determines the VirtualMachine the interface belongs to.
    pub virtual_machine_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,

    /// VirtualNetworkReference determines the Virtual Network the interface belongs to.
    pub virtual_network_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,
}

impl k8s_openapi::DeepMerge for VirtualMachineInterfaceSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.allowed_address_pairs, other.allowed_address_pairs);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.parent, other.parent);
        k8s_openapi::DeepMerge::merge_from(&mut self.port_security_enabled, other.port_security_enabled);
        k8s_openapi::DeepMerge::merge_from(&mut self.properties, other.properties);
        k8s_openapi::merge_strategies::list::atomic(&mut self.tag_references, other.tag_references);
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_machine_interface_disable_policy, other.virtual_machine_interface_disable_policy);
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_machine_interface_mac_addresses, other.virtual_machine_interface_mac_addresses);
        k8s_openapi::merge_strategies::list::atomic(&mut self.virtual_machine_interface_references, other.virtual_machine_interface_references);
        k8s_openapi::merge_strategies::list::atomic(&mut self.virtual_machine_references, other.virtual_machine_references);
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_network_reference, other.virtual_network_reference);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for VirtualMachineInterfaceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allowed_address_pairs,
            Key_fq_name,
            Key_parent,
            Key_port_security_enabled,
            Key_properties,
            Key_tag_references,
            Key_virtual_machine_interface_disable_policy,
            Key_virtual_machine_interface_mac_addresses,
            Key_virtual_machine_interface_references,
            Key_virtual_machine_references,
            Key_virtual_network_reference,
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
                            "allowedAddressPairs" => Field::Key_allowed_address_pairs,
                            "fqName" => Field::Key_fq_name,
                            "parent" => Field::Key_parent,
                            "portSecurityEnabled" => Field::Key_port_security_enabled,
                            "properties" => Field::Key_properties,
                            "tagReferences" => Field::Key_tag_references,
                            "virtualMachineInterfaceDisablePolicy" => Field::Key_virtual_machine_interface_disable_policy,
                            "virtualMachineInterfaceMacAddresses" => Field::Key_virtual_machine_interface_mac_addresses,
                            "virtualMachineInterfaceReferences" => Field::Key_virtual_machine_interface_references,
                            "virtualMachineReferences" => Field::Key_virtual_machine_references,
                            "virtualNetworkReference" => Field::Key_virtual_network_reference,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualMachineInterfaceSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VirtualMachineInterfaceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_allowed_address_pairs: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AllowedAddressPairs> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_parent: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_port_security_enabled: Option<bool> = None;
                let mut value_properties: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualMachineInterfaceProperties> = None;
                let mut value_tag_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;
                let mut value_virtual_machine_interface_disable_policy: Option<bool> = None;
                let mut value_virtual_machine_interface_mac_addresses: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::MACAddresses> = None;
                let mut value_virtual_machine_interface_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;
                let mut value_virtual_machine_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;
                let mut value_virtual_network_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allowed_address_pairs => value_allowed_address_pairs = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parent => value_parent = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port_security_enabled => value_port_security_enabled = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_properties => value_properties = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tag_references => value_tag_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_machine_interface_disable_policy => value_virtual_machine_interface_disable_policy = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_machine_interface_mac_addresses => value_virtual_machine_interface_mac_addresses = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_machine_interface_references => value_virtual_machine_interface_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_machine_references => value_virtual_machine_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_network_reference => value_virtual_network_reference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VirtualMachineInterfaceSpec {
                    allowed_address_pairs: value_allowed_address_pairs,
                    fq_name: value_fq_name,
                    parent: value_parent,
                    port_security_enabled: value_port_security_enabled,
                    properties: value_properties,
                    tag_references: value_tag_references,
                    virtual_machine_interface_disable_policy: value_virtual_machine_interface_disable_policy,
                    virtual_machine_interface_mac_addresses: value_virtual_machine_interface_mac_addresses,
                    virtual_machine_interface_references: value_virtual_machine_interface_references,
                    virtual_machine_references: value_virtual_machine_references,
                    virtual_network_reference: value_virtual_network_reference,
                })
            }
        }

        deserializer.deserialize_struct(
            "VirtualMachineInterfaceSpec",
            &[
                "allowedAddressPairs",
                "fqName",
                "parent",
                "portSecurityEnabled",
                "properties",
                "tagReferences",
                "virtualMachineInterfaceDisablePolicy",
                "virtualMachineInterfaceMacAddresses",
                "virtualMachineInterfaceReferences",
                "virtualMachineReferences",
                "virtualNetworkReference",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for VirtualMachineInterfaceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VirtualMachineInterfaceSpec",
            self.allowed_address_pairs.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.parent.as_ref().map_or(0, |_| 1) +
            self.port_security_enabled.as_ref().map_or(0, |_| 1) +
            self.properties.as_ref().map_or(0, |_| 1) +
            self.tag_references.as_ref().map_or(0, |_| 1) +
            self.virtual_machine_interface_disable_policy.as_ref().map_or(0, |_| 1) +
            self.virtual_machine_interface_mac_addresses.as_ref().map_or(0, |_| 1) +
            self.virtual_machine_interface_references.as_ref().map_or(0, |_| 1) +
            self.virtual_machine_references.as_ref().map_or(0, |_| 1) +
            self.virtual_network_reference.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allowed_address_pairs {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "allowedAddressPairs", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.parent {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "parent", value)?;
        }
        if let Some(value) = &self.port_security_enabled {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "portSecurityEnabled", value)?;
        }
        if let Some(value) = &self.properties {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "properties", value)?;
        }
        if let Some(value) = &self.tag_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "tagReferences", value)?;
        }
        if let Some(value) = &self.virtual_machine_interface_disable_policy {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualMachineInterfaceDisablePolicy", value)?;
        }
        if let Some(value) = &self.virtual_machine_interface_mac_addresses {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualMachineInterfaceMacAddresses", value)?;
        }
        if let Some(value) = &self.virtual_machine_interface_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualMachineInterfaceReferences", value)?;
        }
        if let Some(value) = &self.virtual_machine_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualMachineReferences", value)?;
        }
        if let Some(value) = &self.virtual_network_reference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualNetworkReference", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VirtualMachineInterfaceSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VirtualMachineInterfaceSpec defines the desired state of VirtualMachineInterface.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allowedAddressPairs".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AllowedAddressPairs>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("List of (IP address, MAC) other than instance ip on this interface.".to_owned()),
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
                        "parent".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<k8s_openapi::api::core::v1::ObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Object reference to namespace or virtualrouter parent.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "portSecurityEnabled".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Port security status on the network.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "properties".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualMachineInterfaceProperties>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualMachineInterface properties".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "tagReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Reference to Tag attached to this Virtual Machine Interface".to_owned()),
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
                        "virtualMachineInterfaceDisablePolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("DisablePolicy disables all policy checks for ingress and egress traffic from this interface. Flow table entries are not created. Features that require policy will not work on this interface, these include security group, floating IP, service chain, linklocal services.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "virtualMachineInterfaceMacAddresses".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::MACAddresses>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("MAC address of the virtual machine interface, automatically assigned by system if not provided.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "virtualMachineInterfaceReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualMachineInterfaceReferences determines the VirtualMachineInterface instances that are sub-interfaces.".to_owned()),
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
                        "virtualMachineReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualMachineReferences determines the VirtualMachine the interface belongs to.".to_owned()),
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
                        "virtualNetworkReference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualNetworkReference determines the Virtual Network the interface belongs to.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
