// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FastConvergenceParametersType

/// FastConvergenceParametersType contains fast convergence configuration parameters like xmpp hold time and nh reachability check
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FastConvergenceParametersType {
    pub enable: Option<bool>,

    /// Enable next-hop reachability checks in control plane for routes in underlay for faster convergence
    pub nh_reachability_check: Option<bool>,

    /// The negotiated XMPP hold-time (in seconds) for sessions between the control and data plane Default value is 90. Range 1~90
    pub xmpp_hold_time: Option<i32>,
}

impl k8s_openapi::DeepMerge for FastConvergenceParametersType {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.enable, other.enable);
        k8s_openapi::DeepMerge::merge_from(&mut self.nh_reachability_check, other.nh_reachability_check);
        k8s_openapi::DeepMerge::merge_from(&mut self.xmpp_hold_time, other.xmpp_hold_time);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FastConvergenceParametersType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_enable,
            Key_nh_reachability_check,
            Key_xmpp_hold_time,
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
                            "enable" => Field::Key_enable,
                            "nhReachabilityCheck" => Field::Key_nh_reachability_check,
                            "xmppHoldTime" => Field::Key_xmpp_hold_time,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = FastConvergenceParametersType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FastConvergenceParametersType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_enable: Option<bool> = None;
                let mut value_nh_reachability_check: Option<bool> = None;
                let mut value_xmpp_hold_time: Option<i32> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_enable => value_enable = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nh_reachability_check => value_nh_reachability_check = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_xmpp_hold_time => value_xmpp_hold_time = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FastConvergenceParametersType {
                    enable: value_enable,
                    nh_reachability_check: value_nh_reachability_check,
                    xmpp_hold_time: value_xmpp_hold_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "FastConvergenceParametersType",
            &[
                "enable",
                "nhReachabilityCheck",
                "xmppHoldTime",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for FastConvergenceParametersType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FastConvergenceParametersType",
            self.enable.as_ref().map_or(0, |_| 1) +
            self.nh_reachability_check.as_ref().map_or(0, |_| 1) +
            self.xmpp_hold_time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.enable {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "enable", value)?;
        }
        if let Some(value) = &self.nh_reachability_check {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "nhReachabilityCheck", value)?;
        }
        if let Some(value) = &self.xmpp_hold_time {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "xmppHoldTime", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FastConvergenceParametersType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FastConvergenceParametersType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FastConvergenceParametersType contains fast convergence configuration parameters like xmpp hold time and nh reachability check".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "enable".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nhReachabilityCheck".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Enable next-hop reachability checks in control plane for routes in underlay for faster convergence".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "xmppHoldTime".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The negotiated XMPP hold-time (in seconds) for sessions between the control and data plane Default value is 90. Range 1~90".to_owned()),
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
