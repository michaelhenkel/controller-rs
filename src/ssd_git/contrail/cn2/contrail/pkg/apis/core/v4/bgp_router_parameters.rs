// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterParameters

/// BGPRouterParameters contains BGP router configuration parameters like IP address, AS number, hold time etc.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BGPRouterParameters {
    /// IP address used to reach this BGP router by the system.
    pub address: Option<String>,

    /// BGP address families supported by BGP router. If not specified these address families are enabled: "inet, inet-labeled, inet-vpn, e-vpn, erm-vpn, route-target, inet6, inet-mvpn, inet6-vpn"
    pub address_families: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AddressFamilies>,

    /// Administratively up or down BGPRouter, session is not established for the BGPRouter.
    pub admin_down: Option<bool>,

    /// Authentication related configuration like type, keys etc.
    pub auth_data: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AuthenticationData>,

    /// Autonomous System number for this BGP router. For contrail control nodes, this is derived from GlobalSystemConfig AutonomousSystem.
    pub autonomous_system: Option<i32>,

    /// Cluster ID for this BGP router (between 1 and 4294967295) when control node is configured as route reflector.
    pub cluster_id: Option<i64>,

    /// GatewayAddress field is used only for router-type bgpaas-client. It holds the IPv4 gateway address for the IPv4 subnet from which the client has IP address. The value is used as nexthop when advertising routes to the client via bgp.
    pub gateway_address: Option<String>,

    /// BGPHoldTime is time in seconds \[0-65535\], maximum time to detect liveliness to peer. Value 0 will result in default value of 90 seconds.
    pub hold_time: Option<i32>,

    /// Router ID for this BGP router. Dotted ip notation. For Contrail control-nodes system will automatically assign value of address field.
    pub identifier: Option<String>,

    /// IPv6GatewayAddress field is used only for router-type bgpaas-client It holds IPv6 gateway address for IPv6 subnet from which the client has IP address. The value is used as nexthop when advertising routes to the client via bgp. Note that the IPv6GatewayAddress can be a regular IPv6 address or a ipv4-mapped-ipv6 adddress.
    pub ipv6_gateway_address: Option<String>,

    /// BGPRouter specific Autonomous System number if different from global AS number. Typically used when clusters of control nodes in same contrail system are in different locations.
    pub local_autonomous_system: Option<i32>,

    /// TCP port number on which BGP protocol connections are accepted. Default is port 179
    pub port: Option<i32>,

    /// BGPRouter type.
    pub router_type: Option<String>,

    /// For system internal use in BGPaaS service.
    pub source_port: Option<i32>,

    /// Vendor name for this BGP router.
    pub vendor: Option<String>,
}

impl k8s_openapi::DeepMerge for BGPRouterParameters {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.address, other.address);
        k8s_openapi::DeepMerge::merge_from(&mut self.address_families, other.address_families);
        k8s_openapi::DeepMerge::merge_from(&mut self.admin_down, other.admin_down);
        k8s_openapi::DeepMerge::merge_from(&mut self.auth_data, other.auth_data);
        k8s_openapi::DeepMerge::merge_from(&mut self.autonomous_system, other.autonomous_system);
        k8s_openapi::DeepMerge::merge_from(&mut self.cluster_id, other.cluster_id);
        k8s_openapi::DeepMerge::merge_from(&mut self.gateway_address, other.gateway_address);
        k8s_openapi::DeepMerge::merge_from(&mut self.hold_time, other.hold_time);
        k8s_openapi::DeepMerge::merge_from(&mut self.identifier, other.identifier);
        k8s_openapi::DeepMerge::merge_from(&mut self.ipv6_gateway_address, other.ipv6_gateway_address);
        k8s_openapi::DeepMerge::merge_from(&mut self.local_autonomous_system, other.local_autonomous_system);
        k8s_openapi::DeepMerge::merge_from(&mut self.port, other.port);
        k8s_openapi::DeepMerge::merge_from(&mut self.router_type, other.router_type);
        k8s_openapi::DeepMerge::merge_from(&mut self.source_port, other.source_port);
        k8s_openapi::DeepMerge::merge_from(&mut self.vendor, other.vendor);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for BGPRouterParameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_address,
            Key_address_families,
            Key_admin_down,
            Key_auth_data,
            Key_autonomous_system,
            Key_cluster_id,
            Key_gateway_address,
            Key_hold_time,
            Key_identifier,
            Key_ipv6_gateway_address,
            Key_local_autonomous_system,
            Key_port,
            Key_router_type,
            Key_source_port,
            Key_vendor,
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
                            "address" => Field::Key_address,
                            "addressFamilies" => Field::Key_address_families,
                            "adminDown" => Field::Key_admin_down,
                            "authData" => Field::Key_auth_data,
                            "autonomousSystem" => Field::Key_autonomous_system,
                            "clusterID" => Field::Key_cluster_id,
                            "gatewayAddress" => Field::Key_gateway_address,
                            "holdTime" => Field::Key_hold_time,
                            "identifier" => Field::Key_identifier,
                            "ipv6GatewayAddress" => Field::Key_ipv6_gateway_address,
                            "localAutonomousSystem" => Field::Key_local_autonomous_system,
                            "port" => Field::Key_port,
                            "routerType" => Field::Key_router_type,
                            "sourcePort" => Field::Key_source_port,
                            "vendor" => Field::Key_vendor,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = BGPRouterParameters;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BGPRouterParameters")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_address: Option<String> = None;
                let mut value_address_families: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AddressFamilies> = None;
                let mut value_admin_down: Option<bool> = None;
                let mut value_auth_data: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AuthenticationData> = None;
                let mut value_autonomous_system: Option<i32> = None;
                let mut value_cluster_id: Option<i64> = None;
                let mut value_gateway_address: Option<String> = None;
                let mut value_hold_time: Option<i32> = None;
                let mut value_identifier: Option<String> = None;
                let mut value_ipv6_gateway_address: Option<String> = None;
                let mut value_local_autonomous_system: Option<i32> = None;
                let mut value_port: Option<i32> = None;
                let mut value_router_type: Option<String> = None;
                let mut value_source_port: Option<i32> = None;
                let mut value_vendor: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_address => value_address = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_address_families => value_address_families = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_admin_down => value_admin_down = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_auth_data => value_auth_data = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_autonomous_system => value_autonomous_system = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cluster_id => value_cluster_id = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gateway_address => value_gateway_address = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_hold_time => value_hold_time = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_identifier => value_identifier = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ipv6_gateway_address => value_ipv6_gateway_address = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_local_autonomous_system => value_local_autonomous_system = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_router_type => value_router_type = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_source_port => value_source_port = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vendor => value_vendor = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BGPRouterParameters {
                    address: value_address,
                    address_families: value_address_families,
                    admin_down: value_admin_down,
                    auth_data: value_auth_data,
                    autonomous_system: value_autonomous_system,
                    cluster_id: value_cluster_id,
                    gateway_address: value_gateway_address,
                    hold_time: value_hold_time,
                    identifier: value_identifier,
                    ipv6_gateway_address: value_ipv6_gateway_address,
                    local_autonomous_system: value_local_autonomous_system,
                    port: value_port,
                    router_type: value_router_type,
                    source_port: value_source_port,
                    vendor: value_vendor,
                })
            }
        }

        deserializer.deserialize_struct(
            "BGPRouterParameters",
            &[
                "address",
                "addressFamilies",
                "adminDown",
                "authData",
                "autonomousSystem",
                "clusterID",
                "gatewayAddress",
                "holdTime",
                "identifier",
                "ipv6GatewayAddress",
                "localAutonomousSystem",
                "port",
                "routerType",
                "sourcePort",
                "vendor",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for BGPRouterParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BGPRouterParameters",
            self.address.as_ref().map_or(0, |_| 1) +
            self.address_families.as_ref().map_or(0, |_| 1) +
            self.admin_down.as_ref().map_or(0, |_| 1) +
            self.auth_data.as_ref().map_or(0, |_| 1) +
            self.autonomous_system.as_ref().map_or(0, |_| 1) +
            self.cluster_id.as_ref().map_or(0, |_| 1) +
            self.gateway_address.as_ref().map_or(0, |_| 1) +
            self.hold_time.as_ref().map_or(0, |_| 1) +
            self.identifier.as_ref().map_or(0, |_| 1) +
            self.ipv6_gateway_address.as_ref().map_or(0, |_| 1) +
            self.local_autonomous_system.as_ref().map_or(0, |_| 1) +
            self.port.as_ref().map_or(0, |_| 1) +
            self.router_type.as_ref().map_or(0, |_| 1) +
            self.source_port.as_ref().map_or(0, |_| 1) +
            self.vendor.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.address {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "address", value)?;
        }
        if let Some(value) = &self.address_families {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "addressFamilies", value)?;
        }
        if let Some(value) = &self.admin_down {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "adminDown", value)?;
        }
        if let Some(value) = &self.auth_data {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "authData", value)?;
        }
        if let Some(value) = &self.autonomous_system {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "autonomousSystem", value)?;
        }
        if let Some(value) = &self.cluster_id {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterID", value)?;
        }
        if let Some(value) = &self.gateway_address {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "gatewayAddress", value)?;
        }
        if let Some(value) = &self.hold_time {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "holdTime", value)?;
        }
        if let Some(value) = &self.identifier {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "identifier", value)?;
        }
        if let Some(value) = &self.ipv6_gateway_address {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "ipv6GatewayAddress", value)?;
        }
        if let Some(value) = &self.local_autonomous_system {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "localAutonomousSystem", value)?;
        }
        if let Some(value) = &self.port {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "port", value)?;
        }
        if let Some(value) = &self.router_type {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "routerType", value)?;
        }
        if let Some(value) = &self.source_port {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "sourcePort", value)?;
        }
        if let Some(value) = &self.vendor {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "vendor", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BGPRouterParameters {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterParameters".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("BGPRouterParameters contains BGP router configuration parameters like IP address, AS number, hold time etc.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "address".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IP address used to reach this BGP router by the system.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "addressFamilies".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AddressFamilies>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGP address families supported by BGP router. If not specified these address families are enabled: \"inet, inet-labeled, inet-vpn, e-vpn, erm-vpn, route-target, inet6, inet-mvpn, inet6-vpn\"".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "adminDown".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Administratively up or down BGPRouter, session is not established for the BGPRouter.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "authData".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AuthenticationData>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Authentication related configuration like type, keys etc.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "autonomousSystem".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Autonomous System number for this BGP router. For contrail control nodes, this is derived from GlobalSystemConfig AutonomousSystem.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "clusterID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Cluster ID for this BGP router (between 1 and 4294967295) when control node is configured as route reflector.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "gatewayAddress".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("GatewayAddress field is used only for router-type bgpaas-client. It holds the IPv4 gateway address for the IPv4 subnet from which the client has IP address. The value is used as nexthop when advertising routes to the client via bgp.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "holdTime".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGPHoldTime is time in seconds [0-65535], maximum time to detect liveliness to peer. Value 0 will result in default value of 90 seconds.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "identifier".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Router ID for this BGP router. Dotted ip notation. For Contrail control-nodes system will automatically assign value of address field.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ipv6GatewayAddress".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IPv6GatewayAddress field is used only for router-type bgpaas-client It holds IPv6 gateway address for IPv6 subnet from which the client has IP address. The value is used as nexthop when advertising routes to the client via bgp. Note that the IPv6GatewayAddress can be a regular IPv6 address or a ipv4-mapped-ipv6 adddress.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "localAutonomousSystem".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGPRouter specific Autonomous System number if different from global AS number. Typically used when clusters of control nodes in same contrail system are in different locations.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "port".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TCP port number on which BGP protocol connections are accepted. Default is port 179".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "routerType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGPRouter type.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "sourcePort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("For system internal use in BGPaaS service.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "vendor".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Vendor name for this BGP router.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
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
