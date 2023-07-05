// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIPSpec

/// InstanceIPSpec defines the desired state of the InstanceIP.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InstanceIPSpec {
    /// Subnet is the CIDR the InstanceIP belongs to.
    pub cidr: Option<String>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// IP address value for InstanceIP.
    pub instance_ip_address: Option<String>,

    /// IP address family for the InstanceIP: "v4" or "v6" for IPv4 or IPv6.
    pub instance_ip_family: Option<String>,

    /// IPRangeKeys is used to identify the subnet range for IP allocation.
    pub ip_range_keys: Option<Vec<String>>,

    /// VirtualMachineInterfaceReferences determines the VirtualMachineInterface the InstanceIP belongs to.
    pub virtual_machine_interface_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,

    /// VirtualNetworkReference determines the VirtualNetwork the InstanceIP belongs to.
    pub virtual_network_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,
}

impl k8s_openapi::DeepMerge for InstanceIPSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.cidr, other.cidr);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.instance_ip_address, other.instance_ip_address);
        k8s_openapi::DeepMerge::merge_from(&mut self.instance_ip_family, other.instance_ip_family);
        k8s_openapi::merge_strategies::list::atomic(&mut self.ip_range_keys, other.ip_range_keys);
        k8s_openapi::merge_strategies::list::atomic(&mut self.virtual_machine_interface_references, other.virtual_machine_interface_references);
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_network_reference, other.virtual_network_reference);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for InstanceIPSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cidr,
            Key_fq_name,
            Key_instance_ip_address,
            Key_instance_ip_family,
            Key_ip_range_keys,
            Key_virtual_machine_interface_references,
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
                            "cidr" => Field::Key_cidr,
                            "fqName" => Field::Key_fq_name,
                            "instanceIPAddress" => Field::Key_instance_ip_address,
                            "instanceIPFamily" => Field::Key_instance_ip_family,
                            "ipRangeKeys" => Field::Key_ip_range_keys,
                            "virtualMachineInterfaceReferences" => Field::Key_virtual_machine_interface_references,
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
            type Value = InstanceIPSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("InstanceIPSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_cidr: Option<String> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_instance_ip_address: Option<String> = None;
                let mut value_instance_ip_family: Option<String> = None;
                let mut value_ip_range_keys: Option<Vec<String>> = None;
                let mut value_virtual_machine_interface_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;
                let mut value_virtual_network_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cidr => value_cidr = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_instance_ip_address => value_instance_ip_address = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_instance_ip_family => value_instance_ip_family = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip_range_keys => value_ip_range_keys = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_machine_interface_references => value_virtual_machine_interface_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_network_reference => value_virtual_network_reference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(InstanceIPSpec {
                    cidr: value_cidr,
                    fq_name: value_fq_name,
                    instance_ip_address: value_instance_ip_address,
                    instance_ip_family: value_instance_ip_family,
                    ip_range_keys: value_ip_range_keys,
                    virtual_machine_interface_references: value_virtual_machine_interface_references,
                    virtual_network_reference: value_virtual_network_reference,
                })
            }
        }

        deserializer.deserialize_struct(
            "InstanceIPSpec",
            &[
                "cidr",
                "fqName",
                "instanceIPAddress",
                "instanceIPFamily",
                "ipRangeKeys",
                "virtualMachineInterfaceReferences",
                "virtualNetworkReference",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for InstanceIPSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "InstanceIPSpec",
            self.cidr.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.instance_ip_address.as_ref().map_or(0, |_| 1) +
            self.instance_ip_family.as_ref().map_or(0, |_| 1) +
            self.ip_range_keys.as_ref().map_or(0, |_| 1) +
            self.virtual_machine_interface_references.as_ref().map_or(0, |_| 1) +
            self.virtual_network_reference.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.cidr {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "cidr", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.instance_ip_address {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "instanceIPAddress", value)?;
        }
        if let Some(value) = &self.instance_ip_family {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "instanceIPFamily", value)?;
        }
        if let Some(value) = &self.ip_range_keys {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "ipRangeKeys", value)?;
        }
        if let Some(value) = &self.virtual_machine_interface_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualMachineInterfaceReferences", value)?;
        }
        if let Some(value) = &self.virtual_network_reference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualNetworkReference", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for InstanceIPSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIPSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("InstanceIPSpec defines the desired state of the InstanceIP.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "cidr".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Subnet is the CIDR the InstanceIP belongs to.".to_owned()),
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
                        "instanceIPAddress".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IP address value for InstanceIP.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "instanceIPFamily".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IP address family for the InstanceIP: \"v4\" or \"v6\" for IPv4 or IPv6.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ipRangeKeys".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IPRangeKeys is used to identify the subnet range for IP allocation.".to_owned()),
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
                        "virtualMachineInterfaceReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualMachineInterfaceReferences determines the VirtualMachineInterface the InstanceIP belongs to.".to_owned()),
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
                                description: Some("VirtualNetworkReference determines the VirtualNetwork the InstanceIP belongs to.".to_owned()),
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
