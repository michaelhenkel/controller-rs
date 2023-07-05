// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyReference

/// FirewallPolicyReference is a ResourceReference to a FirewallPolicy with APSAttributes containing the Sequence number.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyReference {
    /// API version of the referent.
    pub api_version: Option<String>,

    /// Attribute defines the sequence number of a FirewallPolicy in \[a given|the\] ApplicationPolicySet.
    pub attributes: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::APSAttribute>,

    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers\[2\]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers\[2\]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object.
    pub field_path: Option<String>,

    /// FqName is the list of resource names that fully qualify a Contrail resource.
    pub fq_name: Option<Vec<String>>,

    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: Option<String>,

    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    pub namespace: Option<String>,

    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    pub resource_version: Option<String>,

    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    pub uid: Option<String>,
}

impl k8s_openapi::DeepMerge for FirewallPolicyReference {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.api_version, other.api_version);
        k8s_openapi::DeepMerge::merge_from(&mut self.attributes, other.attributes);
        k8s_openapi::DeepMerge::merge_from(&mut self.field_path, other.field_path);
        k8s_openapi::merge_strategies::list::atomic(&mut self.fq_name, other.fq_name);
        k8s_openapi::DeepMerge::merge_from(&mut self.kind, other.kind);
        k8s_openapi::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi::DeepMerge::merge_from(&mut self.namespace, other.namespace);
        k8s_openapi::DeepMerge::merge_from(&mut self.resource_version, other.resource_version);
        k8s_openapi::DeepMerge::merge_from(&mut self.uid, other.uid);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FirewallPolicyReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_attributes,
            Key_field_path,
            Key_fq_name,
            Key_kind,
            Key_name,
            Key_namespace,
            Key_resource_version,
            Key_uid,
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
                            "apiVersion" => Field::Key_api_version,
                            "attributes" => Field::Key_attributes,
                            "fieldPath" => Field::Key_field_path,
                            "fqName" => Field::Key_fq_name,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "namespace" => Field::Key_namespace,
                            "resourceVersion" => Field::Key_resource_version,
                            "uid" => Field::Key_uid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallPolicyReference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FirewallPolicyReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_attributes: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::APSAttribute> = None;
                let mut value_field_path: Option<String> = None;
                let mut value_fq_name: Option<Vec<String>> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespace: Option<String> = None;
                let mut value_resource_version: Option<String> = None;
                let mut value_uid: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_attributes => value_attributes = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_field_path => value_field_path = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fq_name => value_fq_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace => value_namespace = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_version => value_resource_version = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FirewallPolicyReference {
                    api_version: value_api_version,
                    attributes: value_attributes,
                    field_path: value_field_path,
                    fq_name: value_fq_name,
                    kind: value_kind,
                    name: value_name,
                    namespace: value_namespace,
                    resource_version: value_resource_version,
                    uid: value_uid,
                })
            }
        }

        deserializer.deserialize_struct(
            "FirewallPolicyReference",
            &[
                "apiVersion",
                "attributes",
                "fieldPath",
                "fqName",
                "kind",
                "name",
                "namespace",
                "resourceVersion",
                "uid",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for FirewallPolicyReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FirewallPolicyReference",
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.attributes.as_ref().map_or(0, |_| 1) +
            self.field_path.as_ref().map_or(0, |_| 1) +
            self.fq_name.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.namespace.as_ref().map_or(0, |_| 1) +
            self.resource_version.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.attributes {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "attributes", value)?;
        }
        if let Some(value) = &self.field_path {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fieldPath", value)?;
        }
        if let Some(value) = &self.fq_name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "fqName", value)?;
        }
        if let Some(value) = &self.kind {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.name {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.namespace {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", value)?;
        }
        if let Some(value) = &self.resource_version {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceVersion", value)?;
        }
        if let Some(value) = &self.uid {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FirewallPolicyReference {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyReference".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FirewallPolicyReference is a ResourceReference to a FirewallPolicy with APSAttributes containing the Sequence number.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "apiVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("API version of the referent.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "attributes".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::APSAttribute>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Attribute defines the sequence number of a FirewallPolicy in [a given|the] ApplicationPolicySet.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "fieldPath".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: \"spec.containers{name}\" (where \"name\" refers to the name of the container that triggered the event) or if no container name is specified \"spec.containers[2]\" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object.".to_owned()),
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
                        "kind".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "namespace".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resourceVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "uid".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids".to_owned()),
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
