// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceSpec

/// BGPAsAServiceSpec defines the desired state of a BGPAsAService.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BGPAsAServiceSpec {
    /// AutonomousSystem is 16-bit BGP Autonomous System number for the cluster.
    pub autonomous_system: Option<i32>,

    /// BGPAsAServiceSessionAttributes defines session attributes such as hold time, route origin and loop count.
    pub bgp_as_a_service_session_attributes: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPSessionAttributes>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// IPAddress specifies the source-address of a BGPaaS VM/pod.
    pub ip_address: Option<String>,

    /// IPv4MappedIPv6NextHop indicates if the client bgp implementation expects to receive a ipv4-mapped ipv6 address (as opposed to regular ipv6 address) as the bgp nexthop for ipv6 routes.
    pub ipv4_mapped_i_pv6_next_hop: Option<bool>,

    /// ServiceHealthCheck determines which ServiceHealthCheck object to be used to performing healthcheck
    pub service_health_check_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,

    /// Shared is enabled to link all VMIs with the common bgp-router object. When false (default), each virtual machine interface individually links to its own bgp-router object.
    pub shared: Option<bool>,

    /// SuppressRouteAdvertisement indicates that the server should not advertise any routes to the client i.e. the client has static routes (typically a default) configured, default set to false.
    pub suppress_route_advertisement: Option<bool>,

    /// VirtualMachineInterfaceReferences determines the VirtualMachineInterfaces on which BGPaaS BGP peering will happen.
    pub virtual_machine_interface_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,

    /// VirtualMachineInterfacesSelector selects VirtualMachineInterfaces using the 'core.juniper.net/bgpaasVN' label defined on pods. BGPAsAService will be configured on the union of VMIs selected by label and VMI specified through VirtualMachineInterfaceReferences.
    pub virtual_machine_interfaces_selector: Option<Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>>,
}

impl k8s_openapi::DeepMerge for BGPAsAServiceSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.autonomous_system, other.autonomous_system);
        k8s_openapi::DeepMerge::merge_from(&mut self.bgp_as_a_service_session_attributes, other.bgp_as_a_service_session_attributes);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.ip_address, other.ip_address);
        k8s_openapi::DeepMerge::merge_from(&mut self.ipv4_mapped_i_pv6_next_hop, other.ipv4_mapped_i_pv6_next_hop);
        k8s_openapi::DeepMerge::merge_from(&mut self.service_health_check_reference, other.service_health_check_reference);
        k8s_openapi::DeepMerge::merge_from(&mut self.shared, other.shared);
        k8s_openapi::DeepMerge::merge_from(&mut self.suppress_route_advertisement, other.suppress_route_advertisement);
        k8s_openapi::merge_strategies::list::atomic(&mut self.virtual_machine_interface_references, other.virtual_machine_interface_references);
        k8s_openapi::merge_strategies::list::atomic(&mut self.virtual_machine_interfaces_selector, other.virtual_machine_interfaces_selector);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for BGPAsAServiceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_autonomous_system,
            Key_bgp_as_a_service_session_attributes,
            Key_fq_name,
            Key_ip_address,
            Key_ipv4_mapped_i_pv6_next_hop,
            Key_service_health_check_reference,
            Key_shared,
            Key_suppress_route_advertisement,
            Key_virtual_machine_interface_references,
            Key_virtual_machine_interfaces_selector,
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
                            "bgpAsAServiceSessionAttributes" => Field::Key_bgp_as_a_service_session_attributes,
                            "fqName" => Field::Key_fq_name,
                            "ipAddress" => Field::Key_ip_address,
                            "ipv4MappedIPv6NextHop" => Field::Key_ipv4_mapped_i_pv6_next_hop,
                            "serviceHealthCheckReference" => Field::Key_service_health_check_reference,
                            "shared" => Field::Key_shared,
                            "suppressRouteAdvertisement" => Field::Key_suppress_route_advertisement,
                            "virtualMachineInterfaceReferences" => Field::Key_virtual_machine_interface_references,
                            "virtualMachineInterfacesSelector" => Field::Key_virtual_machine_interfaces_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = BGPAsAServiceSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BGPAsAServiceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_autonomous_system: Option<i32> = None;
                let mut value_bgp_as_a_service_session_attributes: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPSessionAttributes> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_ip_address: Option<String> = None;
                let mut value_ipv4_mapped_i_pv6_next_hop: Option<bool> = None;
                let mut value_service_health_check_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;
                let mut value_shared: Option<bool> = None;
                let mut value_suppress_route_advertisement: Option<bool> = None;
                let mut value_virtual_machine_interface_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;
                let mut value_virtual_machine_interfaces_selector: Option<Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_autonomous_system => value_autonomous_system = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_bgp_as_a_service_session_attributes => value_bgp_as_a_service_session_attributes = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip_address => value_ip_address = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ipv4_mapped_i_pv6_next_hop => value_ipv4_mapped_i_pv6_next_hop = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_health_check_reference => value_service_health_check_reference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_shared => value_shared = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_suppress_route_advertisement => value_suppress_route_advertisement = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_machine_interface_references => value_virtual_machine_interface_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_machine_interfaces_selector => value_virtual_machine_interfaces_selector = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BGPAsAServiceSpec {
                    autonomous_system: value_autonomous_system,
                    bgp_as_a_service_session_attributes: value_bgp_as_a_service_session_attributes,
                    fq_name: value_fq_name,
                    ip_address: value_ip_address,
                    ipv4_mapped_i_pv6_next_hop: value_ipv4_mapped_i_pv6_next_hop,
                    service_health_check_reference: value_service_health_check_reference,
                    shared: value_shared,
                    suppress_route_advertisement: value_suppress_route_advertisement,
                    virtual_machine_interface_references: value_virtual_machine_interface_references,
                    virtual_machine_interfaces_selector: value_virtual_machine_interfaces_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "BGPAsAServiceSpec",
            &[
                "autonomousSystem",
                "bgpAsAServiceSessionAttributes",
                "fqName",
                "ipAddress",
                "ipv4MappedIPv6NextHop",
                "serviceHealthCheckReference",
                "shared",
                "suppressRouteAdvertisement",
                "virtualMachineInterfaceReferences",
                "virtualMachineInterfacesSelector",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for BGPAsAServiceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BGPAsAServiceSpec",
            self.autonomous_system.as_ref().map_or(0, |_| 1) +
            self.bgp_as_a_service_session_attributes.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.ip_address.as_ref().map_or(0, |_| 1) +
            self.ipv4_mapped_i_pv6_next_hop.as_ref().map_or(0, |_| 1) +
            self.service_health_check_reference.as_ref().map_or(0, |_| 1) +
            self.shared.as_ref().map_or(0, |_| 1) +
            self.suppress_route_advertisement.as_ref().map_or(0, |_| 1) +
            self.virtual_machine_interface_references.as_ref().map_or(0, |_| 1) +
            self.virtual_machine_interfaces_selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.autonomous_system {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "autonomousSystem", value)?;
        }
        if let Some(value) = &self.bgp_as_a_service_session_attributes {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "bgpAsAServiceSessionAttributes", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.ip_address {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "ipAddress", value)?;
        }
        if let Some(value) = &self.ipv4_mapped_i_pv6_next_hop {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "ipv4MappedIPv6NextHop", value)?;
        }
        if let Some(value) = &self.service_health_check_reference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceHealthCheckReference", value)?;
        }
        if let Some(value) = &self.shared {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "shared", value)?;
        }
        if let Some(value) = &self.suppress_route_advertisement {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "suppressRouteAdvertisement", value)?;
        }
        if let Some(value) = &self.virtual_machine_interface_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualMachineInterfaceReferences", value)?;
        }
        if let Some(value) = &self.virtual_machine_interfaces_selector {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualMachineInterfacesSelector", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BGPAsAServiceSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("BGPAsAServiceSpec defines the desired state of a BGPAsAService.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "autonomousSystem".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AutonomousSystem is 16-bit BGP Autonomous System number for the cluster.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "bgpAsAServiceSessionAttributes".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPSessionAttributes>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGPAsAServiceSessionAttributes defines session attributes such as hold time, route origin and loop count.".to_owned()),
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
                        "ipAddress".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IPAddress specifies the source-address of a BGPaaS VM/pod.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ipv4MappedIPv6NextHop".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IPv4MappedIPv6NextHop indicates if the client bgp implementation expects to receive a ipv4-mapped ipv6 address (as opposed to regular ipv6 address) as the bgp nexthop for ipv6 routes.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "serviceHealthCheckReference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ServiceHealthCheck determines which ServiceHealthCheck object to be used to performing healthcheck".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "shared".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Shared is enabled to link all VMIs with the common bgp-router object. When false (default), each virtual machine interface individually links to its own bgp-router object.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "suppressRouteAdvertisement".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("SuppressRouteAdvertisement indicates that the server should not advertise any routes to the client i.e. the client has static routes (typically a default) configured, default set to false.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "virtualMachineInterfaceReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualMachineInterfaceReferences determines the VirtualMachineInterfaces on which BGPaaS BGP peering will happen.".to_owned()),
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
                        "virtualMachineInterfacesSelector".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualMachineInterfacesSelector selects VirtualMachineInterfaces using the 'core.juniper.net/bgpaasVN' label defined on pods. BGPAsAService will be configured on the union of VMIs selected by label and VMI specified through VirtualMachineInterfaceReferences.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>()))),
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
