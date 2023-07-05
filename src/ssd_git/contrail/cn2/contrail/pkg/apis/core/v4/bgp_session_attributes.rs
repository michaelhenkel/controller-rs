// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPSessionAttributes

/// BGPSessionAttributes defines the BGP session parameters configuration.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BGPSessionAttributes {
    /// BGP address families supported on this session. If not specified these address families are enabled: "inet, inet-labeled, inet-vpn, e-vpn, erm-vpn, route-target, inet6, inet-mvpn, inet6-vpn"
    pub address_families: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AddressFamilies>,

    /// Administratively mark this session down.
    pub admin_down: Option<bool>,

    /// ASOverride flag is used to replace the AS number of the control node with the AS number of the tenant VM.
    pub as_override: Option<bool>,

    /// Authentication related configuration for this session like type, keys. Only md5 authentication is supported.
    pub auth_data: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AuthenticationData>,

    /// When the parameters are uni-directional the bgp-router element specifies to which node the configuration applies. If missing the attributes apply to both ends of the session.
    pub bgp_router: Option<String>,

    /// Session attributes over ride per BGP address family. Attributes like address family, loop-count and prefix-limit.
    pub family_attributes: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPFamilyAttributes>>,

    /// A non-zero hold-time overrides the hold-time inherited from the bgp-router configuration. BGP hold time in seconds \[0-65535\], Max time to detect liveliness of peer.
    pub hold_time: Option<i32>,

    /// Local autonomous system number used for this particular session. If configured, this overrides autonomous-system number and local-autonomous-system number configured under BgpRouterParams
    pub local_autonomous_system: Option<i32>,

    /// For routing loop detection, loop-count is the number of times the local AS is allowed in the AS_PATH attribute.
    pub loop_count: Option<i32>,

    /// This is passive session. It will not initiated connection. This is not relevant when session attributes represent common part. It is recommended that it should not be set to true in current release.
    pub passive: Option<bool>,

    /// Remove or replace private ASes from AS Path attributes advertised to this session.
    pub private_as_action: Option<String>,

    /// User defined route origin value to override
    pub route_origin_override: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RouteOriginOverride>,
}

impl k8s_openapi::DeepMerge for BGPSessionAttributes {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.address_families, other.address_families);
        k8s_openapi::DeepMerge::merge_from(&mut self.admin_down, other.admin_down);
        k8s_openapi::DeepMerge::merge_from(&mut self.as_override, other.as_override);
        k8s_openapi::DeepMerge::merge_from(&mut self.auth_data, other.auth_data);
        k8s_openapi::DeepMerge::merge_from(&mut self.bgp_router, other.bgp_router);
        k8s_openapi::merge_strategies::list::atomic(&mut self.family_attributes, other.family_attributes);
        k8s_openapi::DeepMerge::merge_from(&mut self.hold_time, other.hold_time);
        k8s_openapi::DeepMerge::merge_from(&mut self.local_autonomous_system, other.local_autonomous_system);
        k8s_openapi::DeepMerge::merge_from(&mut self.loop_count, other.loop_count);
        k8s_openapi::DeepMerge::merge_from(&mut self.passive, other.passive);
        k8s_openapi::DeepMerge::merge_from(&mut self.private_as_action, other.private_as_action);
        k8s_openapi::DeepMerge::merge_from(&mut self.route_origin_override, other.route_origin_override);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for BGPSessionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_address_families,
            Key_admin_down,
            Key_as_override,
            Key_auth_data,
            Key_bgp_router,
            Key_family_attributes,
            Key_hold_time,
            Key_local_autonomous_system,
            Key_loop_count,
            Key_passive,
            Key_private_as_action,
            Key_route_origin_override,
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
                            "addressFamilies" => Field::Key_address_families,
                            "adminDown" => Field::Key_admin_down,
                            "asOverride" => Field::Key_as_override,
                            "authData" => Field::Key_auth_data,
                            "bgpRouter" => Field::Key_bgp_router,
                            "familyAttributes" => Field::Key_family_attributes,
                            "holdTime" => Field::Key_hold_time,
                            "localAutonomousSystem" => Field::Key_local_autonomous_system,
                            "loopCount" => Field::Key_loop_count,
                            "passive" => Field::Key_passive,
                            "privateAsAction" => Field::Key_private_as_action,
                            "routeOriginOverride" => Field::Key_route_origin_override,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = BGPSessionAttributes;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BGPSessionAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_address_families: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AddressFamilies> = None;
                let mut value_admin_down: Option<bool> = None;
                let mut value_as_override: Option<bool> = None;
                let mut value_auth_data: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AuthenticationData> = None;
                let mut value_bgp_router: Option<String> = None;
                let mut value_family_attributes: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPFamilyAttributes>> = None;
                let mut value_hold_time: Option<i32> = None;
                let mut value_local_autonomous_system: Option<i32> = None;
                let mut value_loop_count: Option<i32> = None;
                let mut value_passive: Option<bool> = None;
                let mut value_private_as_action: Option<String> = None;
                let mut value_route_origin_override: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RouteOriginOverride> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_address_families => value_address_families = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_admin_down => value_admin_down = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_as_override => value_as_override = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_auth_data => value_auth_data = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_bgp_router => value_bgp_router = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_family_attributes => value_family_attributes = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_hold_time => value_hold_time = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_local_autonomous_system => value_local_autonomous_system = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_loop_count => value_loop_count = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_passive => value_passive = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_private_as_action => value_private_as_action = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_route_origin_override => value_route_origin_override = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BGPSessionAttributes {
                    address_families: value_address_families,
                    admin_down: value_admin_down,
                    as_override: value_as_override,
                    auth_data: value_auth_data,
                    bgp_router: value_bgp_router,
                    family_attributes: value_family_attributes,
                    hold_time: value_hold_time,
                    local_autonomous_system: value_local_autonomous_system,
                    loop_count: value_loop_count,
                    passive: value_passive,
                    private_as_action: value_private_as_action,
                    route_origin_override: value_route_origin_override,
                })
            }
        }

        deserializer.deserialize_struct(
            "BGPSessionAttributes",
            &[
                "addressFamilies",
                "adminDown",
                "asOverride",
                "authData",
                "bgpRouter",
                "familyAttributes",
                "holdTime",
                "localAutonomousSystem",
                "loopCount",
                "passive",
                "privateAsAction",
                "routeOriginOverride",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for BGPSessionAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BGPSessionAttributes",
            self.address_families.as_ref().map_or(0, |_| 1) +
            self.admin_down.as_ref().map_or(0, |_| 1) +
            self.as_override.as_ref().map_or(0, |_| 1) +
            self.auth_data.as_ref().map_or(0, |_| 1) +
            self.bgp_router.as_ref().map_or(0, |_| 1) +
            self.family_attributes.as_ref().map_or(0, |_| 1) +
            self.hold_time.as_ref().map_or(0, |_| 1) +
            self.local_autonomous_system.as_ref().map_or(0, |_| 1) +
            self.loop_count.as_ref().map_or(0, |_| 1) +
            self.passive.as_ref().map_or(0, |_| 1) +
            self.private_as_action.as_ref().map_or(0, |_| 1) +
            self.route_origin_override.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.address_families {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "addressFamilies", value)?;
        }
        if let Some(value) = &self.admin_down {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "adminDown", value)?;
        }
        if let Some(value) = &self.as_override {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "asOverride", value)?;
        }
        if let Some(value) = &self.auth_data {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "authData", value)?;
        }
        if let Some(value) = &self.bgp_router {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "bgpRouter", value)?;
        }
        if let Some(value) = &self.family_attributes {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "familyAttributes", value)?;
        }
        if let Some(value) = &self.hold_time {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "holdTime", value)?;
        }
        if let Some(value) = &self.local_autonomous_system {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "localAutonomousSystem", value)?;
        }
        if let Some(value) = &self.loop_count {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "loopCount", value)?;
        }
        if let Some(value) = &self.passive {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "passive", value)?;
        }
        if let Some(value) = &self.private_as_action {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "privateAsAction", value)?;
        }
        if let Some(value) = &self.route_origin_override {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "routeOriginOverride", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BGPSessionAttributes {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPSessionAttributes".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("BGPSessionAttributes defines the BGP session parameters configuration.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "addressFamilies".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AddressFamilies>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGP address families supported on this session. If not specified these address families are enabled: \"inet, inet-labeled, inet-vpn, e-vpn, erm-vpn, route-target, inet6, inet-mvpn, inet6-vpn\"".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "adminDown".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Administratively mark this session down.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "asOverride".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ASOverride flag is used to replace the AS number of the control node with the AS number of the tenant VM.".to_owned()),
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
                                description: Some("Authentication related configuration for this session like type, keys. Only md5 authentication is supported.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "bgpRouter".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("When the parameters are uni-directional the bgp-router element specifies to which node the configuration applies. If missing the attributes apply to both ends of the session.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "familyAttributes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Session attributes over ride per BGP address family. Attributes like address family, loop-count and prefix-limit.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPFamilyAttributes>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "holdTime".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A non-zero hold-time overrides the hold-time inherited from the bgp-router configuration. BGP hold time in seconds [0-65535], Max time to detect liveliness of peer.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "localAutonomousSystem".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Local autonomous system number used for this particular session. If configured, this overrides autonomous-system number and local-autonomous-system number configured under BgpRouterParams".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "loopCount".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("For routing loop detection, loop-count is the number of times the local AS is allowed in the AS_PATH attribute.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "passive".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This is passive session. It will not initiated connection. This is not relevant when session attributes represent common part. It is recommended that it should not be set to true in current release.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "privateAsAction".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Remove or replace private ASes from AS Path attributes advertised to this session.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "routeOriginOverride".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RouteOriginOverride>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("User defined route origin value to override".to_owned()),
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
