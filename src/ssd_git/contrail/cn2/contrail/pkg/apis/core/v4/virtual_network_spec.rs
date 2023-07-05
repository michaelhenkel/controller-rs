// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkSpec

/// VirtualNetworkSpec defines the desired state of a VirtualNetwork.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualNetworkSpec {
    /// RouteTargetList is a list of route targets that are used as import for this virtual network.
    pub export_route_target_list: Option<Vec<String>>,

    /// FabricForwarding when set to true adds the ip-fabric VN as a provider network to this virtual network. If provider network already has a reference to a different network, it will be overriden to the ip-fabric VN. Both IsProviderNetwork and FabricForwarding cannot be set to true at the same time.
    pub fabric_forwarding: Option<bool>,

    /// FabricSNAT toggles connectivity to underlay network by port mapping.
    pub fabric_snat: Option<bool>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// ImportRouteTargetList is a list of route targets that are used as import for this virtual network.
    pub import_route_target_list: Option<Vec<String>>,

    /// IsProviderNetwork is a flag that needs to be set to true if VN is a Provider Network Cannot be updated from true to false. Both IsProviderNetwork and FabricForwarding cannot be set to true at the same time.
    pub is_provider_network: Option<bool>,

    /// PodNetwork is used to differentiate between regular networks and pod networks. When PodNetwork is set to true, host routes will be added towards the VN's CIDR.
    pub pod_network: Option<bool>,

    /// ProviderNetworkReference is the reference to a provider virtual network, example: ip-fabric.
    pub provider_network_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,

    /// static route table to be associated VN
    pub route_table_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>>,

    /// RouteTargetList is a list of route targets that are used as both import and export for this virtual network.
    pub route_target_list: Option<Vec<String>>,

    /// Reference to the v4 family subnet.
    pub v4_subnet_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,

    /// Reference to the v6 family subnet.
    pub v6_subnet_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,

    /// VirtualNetworkNetworkID is User defined unique 32-bit VxlanId for every virtual network. Valid range is 4096 - 4294967296.
    pub virtual_network_network_id: Option<i64>,

    /// VirtualNetworkProperties defines additional configuration parameters for each virtual network.
    pub virtual_network_properties: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualNetworkType>,

    /// VlanID to use for the virtual network. Valid range is 0 - 4094.
    pub vlan_id: Option<i32>,
}

impl k8s_openapi::DeepMerge for VirtualNetworkSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.export_route_target_list, other.export_route_target_list);
        k8s_openapi::DeepMerge::merge_from(&mut self.fabric_forwarding, other.fabric_forwarding);
        k8s_openapi::DeepMerge::merge_from(&mut self.fabric_snat, other.fabric_snat);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::merge_strategies::list::atomic(&mut self.import_route_target_list, other.import_route_target_list);
        k8s_openapi::DeepMerge::merge_from(&mut self.is_provider_network, other.is_provider_network);
        k8s_openapi::DeepMerge::merge_from(&mut self.pod_network, other.pod_network);
        k8s_openapi::DeepMerge::merge_from(&mut self.provider_network_reference, other.provider_network_reference);
        k8s_openapi::merge_strategies::list::atomic(&mut self.route_table_references, other.route_table_references);
        k8s_openapi::merge_strategies::list::atomic(&mut self.route_target_list, other.route_target_list);
        k8s_openapi::DeepMerge::merge_from(&mut self.v4_subnet_reference, other.v4_subnet_reference);
        k8s_openapi::DeepMerge::merge_from(&mut self.v6_subnet_reference, other.v6_subnet_reference);
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_network_network_id, other.virtual_network_network_id);
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_network_properties, other.virtual_network_properties);
        k8s_openapi::DeepMerge::merge_from(&mut self.vlan_id, other.vlan_id);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for VirtualNetworkSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_export_route_target_list,
            Key_fabric_forwarding,
            Key_fabric_snat,
            Key_fq_name,
            Key_import_route_target_list,
            Key_is_provider_network,
            Key_pod_network,
            Key_provider_network_reference,
            Key_route_table_references,
            Key_route_target_list,
            Key_v4_subnet_reference,
            Key_v6_subnet_reference,
            Key_virtual_network_network_id,
            Key_virtual_network_properties,
            Key_vlan_id,
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
                            "exportRouteTargetList" => Field::Key_export_route_target_list,
                            "fabricForwarding" => Field::Key_fabric_forwarding,
                            "fabricSNAT" => Field::Key_fabric_snat,
                            "fqName" => Field::Key_fq_name,
                            "importRouteTargetList" => Field::Key_import_route_target_list,
                            "isProviderNetwork" => Field::Key_is_provider_network,
                            "podNetwork" => Field::Key_pod_network,
                            "providerNetworkReference" => Field::Key_provider_network_reference,
                            "routeTableReferences" => Field::Key_route_table_references,
                            "routeTargetList" => Field::Key_route_target_list,
                            "v4SubnetReference" => Field::Key_v4_subnet_reference,
                            "v6SubnetReference" => Field::Key_v6_subnet_reference,
                            "virtualNetworkNetworkId" => Field::Key_virtual_network_network_id,
                            "virtualNetworkProperties" => Field::Key_virtual_network_properties,
                            "vlanID" => Field::Key_vlan_id,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualNetworkSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VirtualNetworkSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_export_route_target_list: Option<Vec<String>> = None;
                let mut value_fabric_forwarding: Option<bool> = None;
                let mut value_fabric_snat: Option<bool> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_import_route_target_list: Option<Vec<String>> = None;
                let mut value_is_provider_network: Option<bool> = None;
                let mut value_pod_network: Option<bool> = None;
                let mut value_provider_network_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;
                let mut value_route_table_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>> = None;
                let mut value_route_target_list: Option<Vec<String>> = None;
                let mut value_v4_subnet_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;
                let mut value_v6_subnet_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;
                let mut value_virtual_network_network_id: Option<i64> = None;
                let mut value_virtual_network_properties: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualNetworkType> = None;
                let mut value_vlan_id: Option<i32> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_export_route_target_list => value_export_route_target_list = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fabric_forwarding => value_fabric_forwarding = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fabric_snat => value_fabric_snat = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_import_route_target_list => value_import_route_target_list = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_is_provider_network => value_is_provider_network = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_network => value_pod_network = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_provider_network_reference => value_provider_network_reference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_route_table_references => value_route_table_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_route_target_list => value_route_target_list = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_v4_subnet_reference => value_v4_subnet_reference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_v6_subnet_reference => value_v6_subnet_reference = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_network_network_id => value_virtual_network_network_id = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_network_properties => value_virtual_network_properties = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vlan_id => value_vlan_id = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VirtualNetworkSpec {
                    export_route_target_list: value_export_route_target_list,
                    fabric_forwarding: value_fabric_forwarding,
                    fabric_snat: value_fabric_snat,
                    fq_name: value_fq_name,
                    import_route_target_list: value_import_route_target_list,
                    is_provider_network: value_is_provider_network,
                    pod_network: value_pod_network,
                    provider_network_reference: value_provider_network_reference,
                    route_table_references: value_route_table_references,
                    route_target_list: value_route_target_list,
                    v4_subnet_reference: value_v4_subnet_reference,
                    v6_subnet_reference: value_v6_subnet_reference,
                    virtual_network_network_id: value_virtual_network_network_id,
                    virtual_network_properties: value_virtual_network_properties,
                    vlan_id: value_vlan_id,
                })
            }
        }

        deserializer.deserialize_struct(
            "VirtualNetworkSpec",
            &[
                "exportRouteTargetList",
                "fabricForwarding",
                "fabricSNAT",
                "fqName",
                "importRouteTargetList",
                "isProviderNetwork",
                "podNetwork",
                "providerNetworkReference",
                "routeTableReferences",
                "routeTargetList",
                "v4SubnetReference",
                "v6SubnetReference",
                "virtualNetworkNetworkId",
                "virtualNetworkProperties",
                "vlanID",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for VirtualNetworkSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VirtualNetworkSpec",
            self.export_route_target_list.as_ref().map_or(0, |_| 1) +
            self.fabric_forwarding.as_ref().map_or(0, |_| 1) +
            self.fabric_snat.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.import_route_target_list.as_ref().map_or(0, |_| 1) +
            self.is_provider_network.as_ref().map_or(0, |_| 1) +
            self.pod_network.as_ref().map_or(0, |_| 1) +
            self.provider_network_reference.as_ref().map_or(0, |_| 1) +
            self.route_table_references.as_ref().map_or(0, |_| 1) +
            self.route_target_list.as_ref().map_or(0, |_| 1) +
            self.v4_subnet_reference.as_ref().map_or(0, |_| 1) +
            self.v6_subnet_reference.as_ref().map_or(0, |_| 1) +
            self.virtual_network_network_id.as_ref().map_or(0, |_| 1) +
            self.virtual_network_properties.as_ref().map_or(0, |_| 1) +
            self.vlan_id.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.export_route_target_list {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "exportRouteTargetList", value)?;
        }
        if let Some(value) = &self.fabric_forwarding {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fabricForwarding", value)?;
        }
        if let Some(value) = &self.fabric_snat {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fabricSNAT", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.import_route_target_list {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "importRouteTargetList", value)?;
        }
        if let Some(value) = &self.is_provider_network {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "isProviderNetwork", value)?;
        }
        if let Some(value) = &self.pod_network {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "podNetwork", value)?;
        }
        if let Some(value) = &self.provider_network_reference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "providerNetworkReference", value)?;
        }
        if let Some(value) = &self.route_table_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "routeTableReferences", value)?;
        }
        if let Some(value) = &self.route_target_list {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "routeTargetList", value)?;
        }
        if let Some(value) = &self.v4_subnet_reference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "v4SubnetReference", value)?;
        }
        if let Some(value) = &self.v6_subnet_reference {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "v6SubnetReference", value)?;
        }
        if let Some(value) = &self.virtual_network_network_id {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualNetworkNetworkId", value)?;
        }
        if let Some(value) = &self.virtual_network_properties {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualNetworkProperties", value)?;
        }
        if let Some(value) = &self.vlan_id {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "vlanID", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VirtualNetworkSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VirtualNetworkSpec defines the desired state of a VirtualNetwork.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "exportRouteTargetList".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("RouteTargetList is a list of route targets that are used as import for this virtual network.".to_owned()),
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
                        "fabricForwarding".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FabricForwarding when set to true adds the ip-fabric VN as a provider network to this virtual network. If provider network already has a reference to a different network, it will be overriden to the ip-fabric VN. Both IsProviderNetwork and FabricForwarding cannot be set to true at the same time.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fabricSNAT".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FabricSNAT toggles connectivity to underlay network by port mapping.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
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
                        "importRouteTargetList".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ImportRouteTargetList is a list of route targets that are used as import for this virtual network.".to_owned()),
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
                        "isProviderNetwork".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IsProviderNetwork is a flag that needs to be set to true if VN is a Provider Network Cannot be updated from true to false. Both IsProviderNetwork and FabricForwarding cannot be set to true at the same time.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "podNetwork".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PodNetwork is used to differentiate between regular networks and pod networks. When PodNetwork is set to true, host routes will be added towards the VN's CIDR.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "providerNetworkReference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ProviderNetworkReference is the reference to a provider virtual network, example: ip-fabric.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "routeTableReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("static route table to be associated VN".to_owned()),
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
                        "routeTargetList".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("RouteTargetList is a list of route targets that are used as both import and export for this virtual network.".to_owned()),
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
                        "v4SubnetReference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Reference to the v4 family subnet.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "v6SubnetReference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Reference to the v6 family subnet.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "virtualNetworkNetworkId".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualNetworkNetworkID is User defined unique 32-bit VxlanId for every virtual network. Valid range is 4096 - 4294967296.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "virtualNetworkProperties".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::VirtualNetworkType>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualNetworkProperties defines additional configuration parameters for each virtual network.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "vlanID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VlanID to use for the virtual network. Valid range is 0 - 4094.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
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
