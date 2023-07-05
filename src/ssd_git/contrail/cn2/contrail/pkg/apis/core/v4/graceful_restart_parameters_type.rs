// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.GracefulRestartParametersType

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GracefulRestartParametersType {
    pub bgp_helper_enable: Option<bool>,

    pub enable: Option<bool>,

    /// EndofRibTimeout value. Default value of 90 seconds
    pub end_of_rib_timeout: Option<i32>,

    /// LongLivedRestartTime value. Default value of 1800 seconds
    pub long_lived_restart_time: Option<i32>,

    /// GracefulRestartTime value. Default value of 60 seconds
    pub restart_time: Option<i32>,

    pub xmpp_helper_enable: Option<bool>,
}

impl k8s_openapi::DeepMerge for GracefulRestartParametersType {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.bgp_helper_enable, other.bgp_helper_enable);
        k8s_openapi::DeepMerge::merge_from(&mut self.enable, other.enable);
        k8s_openapi::DeepMerge::merge_from(&mut self.end_of_rib_timeout, other.end_of_rib_timeout);
        k8s_openapi::DeepMerge::merge_from(&mut self.long_lived_restart_time, other.long_lived_restart_time);
        k8s_openapi::DeepMerge::merge_from(&mut self.restart_time, other.restart_time);
        k8s_openapi::DeepMerge::merge_from(&mut self.xmpp_helper_enable, other.xmpp_helper_enable);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for GracefulRestartParametersType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_bgp_helper_enable,
            Key_enable,
            Key_end_of_rib_timeout,
            Key_long_lived_restart_time,
            Key_restart_time,
            Key_xmpp_helper_enable,
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
                            "bgpHelperEnable" => Field::Key_bgp_helper_enable,
                            "enable" => Field::Key_enable,
                            "endOfRibTimeout" => Field::Key_end_of_rib_timeout,
                            "longLivedRestartTime" => Field::Key_long_lived_restart_time,
                            "restartTime" => Field::Key_restart_time,
                            "xmppHelperEnable" => Field::Key_xmpp_helper_enable,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = GracefulRestartParametersType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GracefulRestartParametersType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_bgp_helper_enable: Option<bool> = None;
                let mut value_enable: Option<bool> = None;
                let mut value_end_of_rib_timeout: Option<i32> = None;
                let mut value_long_lived_restart_time: Option<i32> = None;
                let mut value_restart_time: Option<i32> = None;
                let mut value_xmpp_helper_enable: Option<bool> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_bgp_helper_enable => value_bgp_helper_enable = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_enable => value_enable = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_end_of_rib_timeout => value_end_of_rib_timeout = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_long_lived_restart_time => value_long_lived_restart_time = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_restart_time => value_restart_time = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_xmpp_helper_enable => value_xmpp_helper_enable = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GracefulRestartParametersType {
                    bgp_helper_enable: value_bgp_helper_enable,
                    enable: value_enable,
                    end_of_rib_timeout: value_end_of_rib_timeout,
                    long_lived_restart_time: value_long_lived_restart_time,
                    restart_time: value_restart_time,
                    xmpp_helper_enable: value_xmpp_helper_enable,
                })
            }
        }

        deserializer.deserialize_struct(
            "GracefulRestartParametersType",
            &[
                "bgpHelperEnable",
                "enable",
                "endOfRibTimeout",
                "longLivedRestartTime",
                "restartTime",
                "xmppHelperEnable",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for GracefulRestartParametersType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GracefulRestartParametersType",
            self.bgp_helper_enable.as_ref().map_or(0, |_| 1) +
            self.enable.as_ref().map_or(0, |_| 1) +
            self.end_of_rib_timeout.as_ref().map_or(0, |_| 1) +
            self.long_lived_restart_time.as_ref().map_or(0, |_| 1) +
            self.restart_time.as_ref().map_or(0, |_| 1) +
            self.xmpp_helper_enable.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.bgp_helper_enable {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "bgpHelperEnable", value)?;
        }
        if let Some(value) = &self.enable {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "enable", value)?;
        }
        if let Some(value) = &self.end_of_rib_timeout {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "endOfRibTimeout", value)?;
        }
        if let Some(value) = &self.long_lived_restart_time {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "longLivedRestartTime", value)?;
        }
        if let Some(value) = &self.restart_time {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "restartTime", value)?;
        }
        if let Some(value) = &self.xmpp_helper_enable {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "xmppHelperEnable", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for GracefulRestartParametersType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.GracefulRestartParametersType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "bgpHelperEnable".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "enable".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "endOfRibTimeout".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("EndofRibTimeout value. Default value of 90 seconds".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "longLivedRestartTime".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("LongLivedRestartTime value. Default value of 1800 seconds".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "restartTime".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("GracefulRestartTime value. Default value of 60 seconds".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "xmppHelperEnable".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
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
