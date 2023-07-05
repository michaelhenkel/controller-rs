// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterSpec

/// VirtualNetworkRouterSpec defines the desired state of the VirtualNetworkRouter.
#[derive(Clone, Debug, PartialEq)]
pub struct VirtualNetworkRouterSpec {
    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// VirtualNetworkRouter can import other VirtualNetworkRouters to enable connectivity between the VirtualNetworks selected by this VirtualNetworkRouter and VirtualNetworks selected by the imported VirtualNetworkRouter.
    ///
    /// Specify list of VirtualNetworkRouters to import.
    pub import: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ImportVirtualNetworkRouter>,

    /// L3vxlanNetworkIdentifier is user defined unique L3 VNI. This will be configured on the internal virtual network of  VirtualNetworkRouter. User will able configure the L3vxlanNetworkIdentifier only when EvpnRouting is enabled. Valid range is 4096 - 4294967296.
    pub l3vxlan_network_identifier: Option<i64>,

    /// RouteTarget will be configured in all routing instance of virtual networks connected to this VirtualNetworkRouter.
    pub route_target: Option<String>,

    /// RoutingType is the routing mode for this VirtualNetworkRouter. Supported Options are evpn/l3vpn.
    pub routing_type: Option<String>,

    /// Type of the VirtualNetworkRouter. Supported types are mesh, spoke and hub.
    pub type_: String,

    /// VirtualNetworkSelector is a LabelSelector to identify VirtualNetworks that this VirtualNetworkRouter should connect to. VirtualNetworkRouter shares its RouteTarget to the connected VirtualNetworks.
    pub virtual_network_selector: k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
}

impl k8s_openapi::DeepMerge for VirtualNetworkRouterSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.import, other.import);
        k8s_openapi::DeepMerge::merge_from(&mut self.l3vxlan_network_identifier, other.l3vxlan_network_identifier);
        k8s_openapi::DeepMerge::merge_from(&mut self.route_target, other.route_target);
        k8s_openapi::DeepMerge::merge_from(&mut self.routing_type, other.routing_type);
        k8s_openapi::DeepMerge::merge_from(&mut self.type_, other.type_);
        k8s_openapi::DeepMerge::merge_from(&mut self.virtual_network_selector, other.virtual_network_selector);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for VirtualNetworkRouterSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fq_name,
            Key_import,
            Key_l3vxlan_network_identifier,
            Key_route_target,
            Key_routing_type,
            Key_type_,
            Key_virtual_network_selector,
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
                            "import" => Field::Key_import,
                            "l3vxlanNetworkIdentifier" => Field::Key_l3vxlan_network_identifier,
                            "routeTarget" => Field::Key_route_target,
                            "routingType" => Field::Key_routing_type,
                            "type" => Field::Key_type_,
                            "virtualNetworkSelector" => Field::Key_virtual_network_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualNetworkRouterSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VirtualNetworkRouterSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_import: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ImportVirtualNetworkRouter> = None;
                let mut value_l3vxlan_network_identifier: Option<i64> = None;
                let mut value_route_target: Option<String> = None;
                let mut value_routing_type: Option<String> = None;
                let mut value_type_: Option<String> = None;
                let mut value_virtual_network_selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_import => value_import = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_l3vxlan_network_identifier => value_l3vxlan_network_identifier = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_route_target => value_route_target = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_routing_type => value_routing_type = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_virtual_network_selector => value_virtual_network_selector = Some(k8s_openapi::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VirtualNetworkRouterSpec {
                    fq_name: value_fq_name,
                    import: value_import,
                    l3vxlan_network_identifier: value_l3vxlan_network_identifier,
                    route_target: value_route_target,
                    routing_type: value_routing_type,
                    type_: value_type_.unwrap_or_default(),
                    virtual_network_selector: value_virtual_network_selector.ok_or_else(|| k8s_openapi::serde::de::Error::missing_field("virtualNetworkSelector"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "VirtualNetworkRouterSpec",
            &[
                "fqName",
                "import",
                "l3vxlanNetworkIdentifier",
                "routeTarget",
                "routingType",
                "type",
                "virtualNetworkSelector",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for VirtualNetworkRouterSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VirtualNetworkRouterSpec",
            2 +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.import.as_ref().map_or(0, |_| 1) +
            self.l3vxlan_network_identifier.as_ref().map_or(0, |_| 1) +
            self.route_target.as_ref().map_or(0, |_| 1) +
            self.routing_type.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.import {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "import", value)?;
        }
        if let Some(value) = &self.l3vxlan_network_identifier {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "l3vxlanNetworkIdentifier", value)?;
        }
        if let Some(value) = &self.route_target {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "routeTarget", value)?;
        }
        if let Some(value) = &self.routing_type {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "routingType", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "virtualNetworkSelector", &self.virtual_network_selector)?;
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VirtualNetworkRouterSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VirtualNetworkRouterSpec defines the desired state of the VirtualNetworkRouter.".to_owned()),
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
                        "import".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ImportVirtualNetworkRouter>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualNetworkRouter can import other VirtualNetworkRouters to enable connectivity between the VirtualNetworks selected by this VirtualNetworkRouter and VirtualNetworks selected by the imported VirtualNetworkRouter.\n\nSpecify list of VirtualNetworkRouters to import.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "l3vxlanNetworkIdentifier".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("L3vxlanNetworkIdentifier is user defined unique L3 VNI. This will be configured on the internal virtual network of  VirtualNetworkRouter. User will able configure the L3vxlanNetworkIdentifier only when EvpnRouting is enabled. Valid range is 4096 - 4294967296.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "routeTarget".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("RouteTarget will be configured in all routing instance of virtual networks connected to this VirtualNetworkRouter.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "routingType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("RoutingType is the routing mode for this VirtualNetworkRouter. Supported Options are evpn/l3vpn.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Type of the VirtualNetworkRouter. Supported types are mesh, spoke and hub.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "virtualNetworkSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VirtualNetworkSelector is a LabelSelector to identify VirtualNetworks that this VirtualNetworkRouter should connect to. VirtualNetworkRouter shares its RouteTarget to the connected VirtualNetworks.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "type".to_owned(),
                    "virtualNetworkSelector".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
