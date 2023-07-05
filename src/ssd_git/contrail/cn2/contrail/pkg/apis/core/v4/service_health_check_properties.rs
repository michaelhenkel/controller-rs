// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckProperties

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceHealthCheckProperties {
    /// Delay The delay, in seconds, to repeat the health check.
    pub delay: i64,

    /// DelayUSec The delayUsecs, in micro-seconds, to repeat the health check. If Specified total delay is calulated as delay = Delay + (DelayUSec/1000000)
    pub delay_usecs: Option<i64>,

    /// Enabled Indicates that health check is enabled. The default is False.
    pub enabled: bool,

    /// ExpectedCodes When the monitor protocol is HTTP, the expected return code for HTTP operations the value must be 2xx.
    pub expected_codes: Option<i64>,

    /// HealthCheckType Indicates the health check type: link-local, end-to-end.
    pub health_check_type: String,

    /// HttpMethod When the monitor protocol is HTTP, the type of HTTP method used, such as GET, PUT, POST, and so on
    pub http_method: Option<String>,

    /// MaxRetries The number of retries to attempt before declaring an instance health down.
    pub max_retries: i64,

    /// MonitorType The protocol type to be used: PING or HTTP or BFD
    pub monitor_type: String,

    /// Run BFD for all IP Addresses learnt by mac-ip learning, when enabled
    pub target_ip_all: Option<bool>,

    /// Any address that belongs to the list has to run BFD health check when target-ip-all is false.
    pub target_ip_list: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::IpAddresses>,

    /// Timeout The number of seconds to wait for a response.
    pub timeout: i64,

    pub timeout_usecs: Option<i64>,

    /// UrlPath When the monitor protocol is HTTP, the URL to be used.
    pub url_path: Option<String>,
}

impl k8s_openapi::DeepMerge for ServiceHealthCheckProperties {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.delay, other.delay);
        k8s_openapi::DeepMerge::merge_from(&mut self.delay_usecs, other.delay_usecs);
        k8s_openapi::DeepMerge::merge_from(&mut self.enabled, other.enabled);
        k8s_openapi::DeepMerge::merge_from(&mut self.expected_codes, other.expected_codes);
        k8s_openapi::DeepMerge::merge_from(&mut self.health_check_type, other.health_check_type);
        k8s_openapi::DeepMerge::merge_from(&mut self.http_method, other.http_method);
        k8s_openapi::DeepMerge::merge_from(&mut self.max_retries, other.max_retries);
        k8s_openapi::DeepMerge::merge_from(&mut self.monitor_type, other.monitor_type);
        k8s_openapi::DeepMerge::merge_from(&mut self.target_ip_all, other.target_ip_all);
        k8s_openapi::DeepMerge::merge_from(&mut self.target_ip_list, other.target_ip_list);
        k8s_openapi::DeepMerge::merge_from(&mut self.timeout, other.timeout);
        k8s_openapi::DeepMerge::merge_from(&mut self.timeout_usecs, other.timeout_usecs);
        k8s_openapi::DeepMerge::merge_from(&mut self.url_path, other.url_path);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for ServiceHealthCheckProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_delay,
            Key_delay_usecs,
            Key_enabled,
            Key_expected_codes,
            Key_health_check_type,
            Key_http_method,
            Key_max_retries,
            Key_monitor_type,
            Key_target_ip_all,
            Key_target_ip_list,
            Key_timeout,
            Key_timeout_usecs,
            Key_url_path,
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
                            "delay" => Field::Key_delay,
                            "delayUsecs" => Field::Key_delay_usecs,
                            "enabled" => Field::Key_enabled,
                            "expectedCodes" => Field::Key_expected_codes,
                            "healthCheckType" => Field::Key_health_check_type,
                            "httpMethod" => Field::Key_http_method,
                            "maxRetries" => Field::Key_max_retries,
                            "monitorType" => Field::Key_monitor_type,
                            "targetIpAll" => Field::Key_target_ip_all,
                            "targetIpList" => Field::Key_target_ip_list,
                            "timeout" => Field::Key_timeout,
                            "timeoutUsecs" => Field::Key_timeout_usecs,
                            "urlPath" => Field::Key_url_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceHealthCheckProperties;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServiceHealthCheckProperties")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_delay: Option<i64> = None;
                let mut value_delay_usecs: Option<i64> = None;
                let mut value_enabled: Option<bool> = None;
                let mut value_expected_codes: Option<i64> = None;
                let mut value_health_check_type: Option<String> = None;
                let mut value_http_method: Option<String> = None;
                let mut value_max_retries: Option<i64> = None;
                let mut value_monitor_type: Option<String> = None;
                let mut value_target_ip_all: Option<bool> = None;
                let mut value_target_ip_list: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::IpAddresses> = None;
                let mut value_timeout: Option<i64> = None;
                let mut value_timeout_usecs: Option<i64> = None;
                let mut value_url_path: Option<String> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_delay => value_delay = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_delay_usecs => value_delay_usecs = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_enabled => value_enabled = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_expected_codes => value_expected_codes = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_health_check_type => value_health_check_type = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_http_method => value_http_method = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_retries => value_max_retries = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_monitor_type => value_monitor_type = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_ip_all => value_target_ip_all = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_ip_list => value_target_ip_list = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_timeout => value_timeout = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_timeout_usecs => value_timeout_usecs = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_url_path => value_url_path = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceHealthCheckProperties {
                    delay: value_delay.unwrap_or_default(),
                    delay_usecs: value_delay_usecs,
                    enabled: value_enabled.unwrap_or_default(),
                    expected_codes: value_expected_codes,
                    health_check_type: value_health_check_type.unwrap_or_default(),
                    http_method: value_http_method,
                    max_retries: value_max_retries.unwrap_or_default(),
                    monitor_type: value_monitor_type.unwrap_or_default(),
                    target_ip_all: value_target_ip_all,
                    target_ip_list: value_target_ip_list,
                    timeout: value_timeout.unwrap_or_default(),
                    timeout_usecs: value_timeout_usecs,
                    url_path: value_url_path,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceHealthCheckProperties",
            &[
                "delay",
                "delayUsecs",
                "enabled",
                "expectedCodes",
                "healthCheckType",
                "httpMethod",
                "maxRetries",
                "monitorType",
                "targetIpAll",
                "targetIpList",
                "timeout",
                "timeoutUsecs",
                "urlPath",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for ServiceHealthCheckProperties {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceHealthCheckProperties",
            6 +
            self.delay_usecs.as_ref().map_or(0, |_| 1) +
            self.expected_codes.as_ref().map_or(0, |_| 1) +
            self.http_method.as_ref().map_or(0, |_| 1) +
            self.target_ip_all.as_ref().map_or(0, |_| 1) +
            self.target_ip_list.as_ref().map_or(0, |_| 1) +
            self.timeout_usecs.as_ref().map_or(0, |_| 1) +
            self.url_path.as_ref().map_or(0, |_| 1),
        )?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "delay", &self.delay)?;
        if let Some(value) = &self.delay_usecs {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "delayUsecs", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "enabled", &self.enabled)?;
        if let Some(value) = &self.expected_codes {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "expectedCodes", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "healthCheckType", &self.health_check_type)?;
        if let Some(value) = &self.http_method {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "httpMethod", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "maxRetries", &self.max_retries)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "monitorType", &self.monitor_type)?;
        if let Some(value) = &self.target_ip_all {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "targetIpAll", value)?;
        }
        if let Some(value) = &self.target_ip_list {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "targetIpList", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "timeout", &self.timeout)?;
        if let Some(value) = &self.timeout_usecs {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "timeoutUsecs", value)?;
        }
        if let Some(value) = &self.url_path {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "urlPath", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ServiceHealthCheckProperties {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckProperties".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "delay".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Delay The delay, in seconds, to repeat the health check.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "delayUsecs".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("DelayUSec The delayUsecs, in micro-seconds, to repeat the health check. If Specified total delay is calulated as delay = Delay + (DelayUSec/1000000)".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "enabled".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Enabled Indicates that health check is enabled. The default is False.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "expectedCodes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ExpectedCodes When the monitor protocol is HTTP, the expected return code for HTTP operations the value must be 2xx.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "healthCheckType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("HealthCheckType Indicates the health check type: link-local, end-to-end.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "httpMethod".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("HttpMethod When the monitor protocol is HTTP, the type of HTTP method used, such as GET, PUT, POST, and so on".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "maxRetries".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("MaxRetries The number of retries to attempt before declaring an instance health down.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "monitorType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("MonitorType The protocol type to be used: PING or HTTP or BFD".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "targetIpAll".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Run BFD for all IP Addresses learnt by mac-ip learning, when enabled".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "targetIpList".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::IpAddresses>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Any address that belongs to the list has to run BFD health check when target-ip-all is false.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "timeout".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Timeout The number of seconds to wait for a response.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "timeoutUsecs".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "urlPath".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("UrlPath When the monitor protocol is HTTP, the URL to be used.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "delay".to_owned(),
                    "enabled".to_owned(),
                    "healthCheckType".to_owned(),
                    "maxRetries".to_owned(),
                    "monitorType".to_owned(),
                    "timeout".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
