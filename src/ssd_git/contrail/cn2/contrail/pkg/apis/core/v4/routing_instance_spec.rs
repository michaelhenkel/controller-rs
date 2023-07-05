// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceSpec

/// RoutingInstanceSpec defines the desired state of the RoutingInstance.
#[derive(Clone, Debug, PartialEq)]
pub struct RoutingInstanceSpec {
    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// Object reference to the parent VirtualNetwork.
    pub parent: k8s_openapi::api::core::v1::ObjectReference,

    /// RouteTargetReferences contains RouteTarget references assigned by the user. The RoutingInstance's default RouteTarget is defined in RoutingInstanceStatus.
    pub route_target_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RouteTargetReference>>,

    /// StaticRouteEntries contains user provided Routes to be populated in RI
    pub static_route_entries: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::StaticRouteEntriesType>,
}

impl k8s_openapi::DeepMerge for RoutingInstanceSpec {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.parent, other.parent);
        k8s_openapi::merge_strategies::list::atomic(&mut self.route_target_references, other.route_target_references);
        k8s_openapi::DeepMerge::merge_from(&mut self.static_route_entries, other.static_route_entries);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for RoutingInstanceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fq_name,
            Key_parent,
            Key_route_target_references,
            Key_static_route_entries,
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
                            "routeTargetReferences" => Field::Key_route_target_references,
                            "staticRouteEntries" => Field::Key_static_route_entries,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = RoutingInstanceSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RoutingInstanceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_parent: Option<k8s_openapi::api::core::v1::ObjectReference> = None;
                let mut value_route_target_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RouteTargetReference>> = None;
                let mut value_static_route_entries: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::StaticRouteEntriesType> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parent => value_parent = Some(k8s_openapi::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_route_target_references => value_route_target_references = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_static_route_entries => value_static_route_entries = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RoutingInstanceSpec {
                    fq_name: value_fq_name,
                    parent: value_parent.ok_or_else(|| k8s_openapi::serde::de::Error::missing_field("parent"))?,
                    route_target_references: value_route_target_references,
                    static_route_entries: value_static_route_entries,
                })
            }
        }

        deserializer.deserialize_struct(
            "RoutingInstanceSpec",
            &[
                "fqName",
                "parent",
                "routeTargetReferences",
                "staticRouteEntries",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for RoutingInstanceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RoutingInstanceSpec",
            1 +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.route_target_references.as_ref().map_or(0, |_| 1) +
            self.static_route_entries.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "parent", &self.parent)?;
        if let Some(value) = &self.route_target_references {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "routeTargetReferences", value)?;
        }
        if let Some(value) = &self.static_route_entries {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "staticRouteEntries", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for RoutingInstanceSpec {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("RoutingInstanceSpec defines the desired state of the RoutingInstance.".to_owned()),
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
                                description: Some("Object reference to the parent VirtualNetwork.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "routeTargetReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("RouteTargetReferences contains RouteTarget references assigned by the user. The RoutingInstance's default RouteTarget is defined in RoutingInstanceStatus.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RouteTargetReference>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "staticRouteEntries".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::StaticRouteEntriesType>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("StaticRouteEntries contains user provided Routes to be populated in RI".to_owned()),
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
