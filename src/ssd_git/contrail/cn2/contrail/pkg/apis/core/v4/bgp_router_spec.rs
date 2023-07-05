// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterSpec

/// BGPRouterSpec defines the desired state of BGPRouter
#[derive(Clone, Debug, PartialEq)]
pub struct BGPRouterSpec {
    /// BGP router configuration parameters like IP address, AS number, hold time etc.
    pub bgp_router_parameters: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPRouterParameters>,

    /// BGPRouterReferences list of references to all bgp routers in the cluster.
    pub bgp_router_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPRouterReference>>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// Object reference to routing-instance parent
    pub parent: k8s_openapi::api::core::v1::ObjectReference,
}

impl k8s_openapi::DeepMerge for BGPRouterSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.bgp_router_parameters, other.bgp_router_parameters);
        k8s_openapi::merge_strategies::list::atomic(&mut self.bgp_router_references, other.bgp_router_references);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.parent, other.parent);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for BGPRouterSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_bgp_router_parameters,
            Key_bgp_router_references,
            Key_fq_name,
            Key_parent,
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
                            "bgpRouterParameters" => Field::Key_bgp_router_parameters,
                            "bgpRouterReferences" => Field::Key_bgp_router_references,
                            "fqName" => Field::Key_fq_name,
                            "parent" => Field::Key_parent,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = BGPRouterSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BGPRouterSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_bgp_router_parameters: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPRouterParameters> = None;
                let mut value_bgp_router_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPRouterReference>> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_parent: Option<k8s_openapi::api::core::v1::ObjectReference> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_bgp_router_parameters => value_bgp_router_parameters = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_bgp_router_references => value_bgp_router_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parent => value_parent = Some(k8s_openapi::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BGPRouterSpec {
                    bgp_router_parameters: value_bgp_router_parameters,
                    bgp_router_references: value_bgp_router_references,
                    fq_name: value_fq_name,
                    parent: value_parent.ok_or_else(|| k8s_openapi::serde::de::Error::missing_field("parent"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "BGPRouterSpec",
            &[
                "bgpRouterParameters",
                "bgpRouterReferences",
                "fqName",
                "parent",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for BGPRouterSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BGPRouterSpec",
            1 +
            self.bgp_router_parameters.as_ref().map_or(0, |_| 1) +
            self.bgp_router_references.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.bgp_router_parameters {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "bgpRouterParameters", value)?;
        }
        if let Some(value) = &self.bgp_router_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "bgpRouterReferences", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "parent", &self.parent)?;
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BGPRouterSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("BGPRouterSpec defines the desired state of BGPRouter".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "bgpRouterParameters".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPRouterParameters>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGP router configuration parameters like IP address, AS number, hold time etc.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "bgpRouterReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGPRouterReferences list of references to all bgp routers in the cluster.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPRouterReference>()))),
                                ..Default::default()
                            })),
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
                                description: Some("Object reference to routing-instance parent".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "parent".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
