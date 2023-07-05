// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroup

/// ServiceGroup describes the list of protocol:port associated with the FirewallRule resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceGroup {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired state of the ServiceGroup. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub spec: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroupSpec>,

    /// The most recently observed status of the ServiceGroup. This data may not be up-to-date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub status: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroupStatus>,
}

// Begin core.contrail.juniper.net/v4/ServiceGroup

// Generated from operation createCoreContrailJuniperNetV4NamespacedServiceGroup

impl ServiceGroup {
    /// create a ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateServiceGroupResponse`]`>` constructor, or [`CreateServiceGroupResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create(
        namespace: &str,
        body: &crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup,
        optional: CreateServiceGroupOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<CreateServiceGroupResponse>), crate::RequestError> {
        let CreateServiceGroupOptional {
            dry_run,
            field_manager,
            field_validation,
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/servicegroups?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(field_validation) = field_validation {
            __query_pairs.append_pair("fieldValidation", field_validation);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::post(__url);
        let __body = k8s_openapi::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::create`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateServiceGroupOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields, provided that the `ServerSideFieldValidation` feature gate is also enabled. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23 and is the default behavior when the `ServerSideFieldValidation` feature gate is disabled. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default when the `ServerSideFieldValidation` feature gate is enabled. - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.
    pub field_validation: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateServiceGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::create`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateServiceGroupResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Created(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Accepted(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for CreateServiceGroupResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateServiceGroupResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::CREATED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateServiceGroupResponse::Created(result), buf.len()))
            },
            crate::http::StatusCode::ACCEPTED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateServiceGroupResponse::Accepted(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((CreateServiceGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation createCoreContrailJuniperNetV4NamespacedServiceGroupStatus

impl ServiceGroup {
    /// create status of a ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateServiceGroupStatusResponse`]`>` constructor, or [`CreateServiceGroupStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceGroup
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create_status(
        name: &str,
        namespace: &str,
        body: &crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup,
        optional: CreateServiceGroupStatusOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<CreateServiceGroupStatusResponse>), crate::RequestError> {
        let CreateServiceGroupStatusOptional {
            dry_run,
            field_manager,
            field_validation,
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/servicegroups/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(field_validation) = field_validation {
            __query_pairs.append_pair("fieldValidation", field_validation);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::post(__url);
        let __body = k8s_openapi::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::create_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateServiceGroupStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields, provided that the `ServerSideFieldValidation` feature gate is also enabled. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23 and is the default behavior when the `ServerSideFieldValidation` feature gate is disabled. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default when the `ServerSideFieldValidation` feature gate is enabled. - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.
    pub field_validation: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateServiceGroupStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::create_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateServiceGroupStatusResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Created(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Accepted(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for CreateServiceGroupStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateServiceGroupStatusResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::CREATED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateServiceGroupStatusResponse::Created(result), buf.len()))
            },
            crate::http::StatusCode::ACCEPTED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateServiceGroupStatusResponse::Accepted(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((CreateServiceGroupStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteCoreContrailJuniperNetV4CollectionNamespacedServiceGroup

impl ServiceGroup {
    /// delete collection of ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionServiceGroupResponse`]`>` constructor, or [`DeleteCollectionServiceGroupResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_collection(
        namespace: &str,
        optional: DeleteCollectionServiceGroupOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<DeleteCollectionServiceGroupResponse>), crate::RequestError> {
        let DeleteCollectionServiceGroupOptional {
            body,
            continue_,
            dry_run,
            field_selector,
            grace_period_seconds,
            label_selector,
            limit,
            orphan_dependents,
            pretty,
            propagation_policy,
            resource_version,
            resource_version_match,
            timeout_seconds,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/servicegroups?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(grace_period_seconds) = grace_period_seconds {
            __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(orphan_dependents) = orphan_dependents {
            __query_pairs.append_pair("orphanDependents", if orphan_dependents { "true" } else { "false" });
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(propagation_policy) = propagation_policy {
            __query_pairs.append_pair("propagationPolicy", propagation_policy);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(resource_version_match) = resource_version_match {
            __query_pairs.append_pair("resourceVersionMatch", resource_version_match);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::delete(__url);
        let __body = 
body.unwrap_or(Ok(vec![]), |value| k8s_openapi::serde_json::to_vec(value).map_err(crate::RequestError::Json))?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::delete_collection`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionServiceGroupOptional<'a> {
    pub body: Option<&'a k8s_openapi::apimachinery::pkg::apis::meta::v1::DeleteOptions>,
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,
    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version: Option<&'a str>,
    /// resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version_match: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
}

/// Use `<DeleteCollectionServiceGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::delete_collection`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteCollectionServiceGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteCollectionServiceGroupResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteCollectionServiceGroupResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((DeleteCollectionServiceGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteCoreContrailJuniperNetV4NamespacedServiceGroup

impl ServiceGroup {
    /// delete a ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteServiceGroupResponse`]`>` constructor, or [`DeleteServiceGroupResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceGroup
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete(
        name: &str,
        namespace: &str,
        optional: DeleteServiceGroupOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<DeleteServiceGroupResponse>), crate::RequestError> {
        let DeleteServiceGroupOptional {
            body,
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/servicegroups/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(grace_period_seconds) = grace_period_seconds {
            __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
        }
        if let Some(orphan_dependents) = orphan_dependents {
            __query_pairs.append_pair("orphanDependents", if orphan_dependents { "true" } else { "false" });
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(propagation_policy) = propagation_policy {
            __query_pairs.append_pair("propagationPolicy", propagation_policy);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::delete(__url);
        let __body = 
body.unwrap_or(Ok(vec![]), |value| k8s_openapi::serde_json::to_vec(value).map_err(crate::RequestError::Json))?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::delete`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteServiceGroupOptional<'a> {
    pub body: Option<&'a k8s_openapi::apimachinery::pkg::apis::meta::v1::DeleteOptions>,
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,
}

/// Use `<DeleteServiceGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::delete`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteServiceGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::Status),
    Accepted(k8s_openapi::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteServiceGroupResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteServiceGroupResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::ACCEPTED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteServiceGroupResponse::Accepted(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((DeleteServiceGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteCoreContrailJuniperNetV4NamespacedServiceGroupStatus

impl ServiceGroup {
    /// delete status of a ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteServiceGroupStatusResponse`]`>` constructor, or [`DeleteServiceGroupStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceGroup
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_status(
        name: &str,
        namespace: &str,
        optional: DeleteServiceGroupStatusOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<DeleteServiceGroupStatusResponse>), crate::RequestError> {
        let DeleteServiceGroupStatusOptional {
            body,
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/servicegroups/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(grace_period_seconds) = grace_period_seconds {
            __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
        }
        if let Some(orphan_dependents) = orphan_dependents {
            __query_pairs.append_pair("orphanDependents", if orphan_dependents { "true" } else { "false" });
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(propagation_policy) = propagation_policy {
            __query_pairs.append_pair("propagationPolicy", propagation_policy);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::delete(__url);
        let __body = 
body.unwrap_or(Ok(vec![]), |value| k8s_openapi::serde_json::to_vec(value).map_err(crate::RequestError::Json))?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::delete_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteServiceGroupStatusOptional<'a> {
    pub body: Option<&'a k8s_openapi::apimachinery::pkg::apis::meta::v1::DeleteOptions>,
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,
}

/// Use `<DeleteServiceGroupStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::delete_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteServiceGroupStatusResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::Status),
    Accepted(k8s_openapi::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteServiceGroupStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteServiceGroupStatusResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::ACCEPTED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteServiceGroupStatusResponse::Accepted(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((DeleteServiceGroupStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listCoreContrailJuniperNetV4NamespacedServiceGroup

impl ServiceGroup {
    /// list or watch objects of kind ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListServiceGroupResponse`]`>` constructor, or [`ListServiceGroupResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list(
        namespace: &str,
        optional: ListServiceGroupOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ListServiceGroupResponse>), crate::RequestError> {
        let ListServiceGroupOptional {
            allow_watch_bookmarks,
            continue_,
            field_selector,
            label_selector,
            limit,
            pretty,
            resource_version,
            resource_version_match,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/servicegroups?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(allow_watch_bookmarks) = allow_watch_bookmarks {
            __query_pairs.append_pair("allowWatchBookmarks", if allow_watch_bookmarks { "true" } else { "false" });
        }
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(resource_version_match) = resource_version_match {
            __query_pairs.append_pair("resourceVersionMatch", resource_version_match);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", if watch { "true" } else { "false" });
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::list`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ListServiceGroupOptional<'a> {
    /// allowWatchBookmarks requests watch events with type "BOOKMARK". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored.
    pub allow_watch_bookmarks: Option<bool>,
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version: Option<&'a str>,
    /// resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version_match: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<ListServiceGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::list`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListServiceGroupResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroupList),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListServiceGroupResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListServiceGroupResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ListServiceGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listCoreContrailJuniperNetV4ServiceGroupForAllNamespaces

impl ServiceGroup {
    /// list or watch objects of kind ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListServiceGroupForAllNamespacesResponse`]`>` constructor, or [`ListServiceGroupForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_for_all_namespaces(
        optional: ListServiceGroupForAllNamespacesOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ListServiceGroupForAllNamespacesResponse>), crate::RequestError> {
        let ListServiceGroupForAllNamespacesOptional {
            allow_watch_bookmarks,
            continue_,
            field_selector,
            label_selector,
            limit,
            pretty,
            resource_version,
            resource_version_match,
            timeout_seconds,
            watch,
        } = optional;
        let __url = "/apis/core.contrail.juniper.net/v4/servicegroups?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(allow_watch_bookmarks) = allow_watch_bookmarks {
            __query_pairs.append_pair("allowWatchBookmarks", if allow_watch_bookmarks { "true" } else { "false" });
        }
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(resource_version_match) = resource_version_match {
            __query_pairs.append_pair("resourceVersionMatch", resource_version_match);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", if watch { "true" } else { "false" });
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::list_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ListServiceGroupForAllNamespacesOptional<'a> {
    /// allowWatchBookmarks requests watch events with type "BOOKMARK". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored.
    pub allow_watch_bookmarks: Option<bool>,
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version: Option<&'a str>,
    /// resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version_match: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<ListServiceGroupForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::list_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListServiceGroupForAllNamespacesResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroupList),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListServiceGroupForAllNamespacesResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListServiceGroupForAllNamespacesResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ListServiceGroupForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchCoreContrailJuniperNetV4NamespacedServiceGroup

impl ServiceGroup {
    /// partially update the specified ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchServiceGroupResponse`]`>` constructor, or [`PatchServiceGroupResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceGroup
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch(
        name: &str,
        namespace: &str,
        body: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchServiceGroupOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<PatchServiceGroupResponse>), crate::RequestError> {
        let PatchServiceGroupOptional {
            dry_run,
            field_manager,
            field_validation,
            force,
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/servicegroups/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(field_validation) = field_validation {
            __query_pairs.append_pair("fieldValidation", field_validation);
        }
        if let Some(force) = force {
            __query_pairs.append_pair("force", if force { "true" } else { "false" });
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::patch(__url);
        let __body = k8s_openapi::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static(match body {
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::patch`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchServiceGroupOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch).
    pub field_manager: Option<&'a str>,
    /// fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields, provided that the `ServerSideFieldValidation` feature gate is also enabled. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23 and is the default behavior when the `ServerSideFieldValidation` feature gate is disabled. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default when the `ServerSideFieldValidation` feature gate is enabled. - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.
    pub field_validation: Option<&'a str>,
    /// Force is going to "force" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests.
    pub force: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchServiceGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::patch`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchServiceGroupResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Created(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchServiceGroupResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchServiceGroupResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::CREATED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchServiceGroupResponse::Created(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((PatchServiceGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchCoreContrailJuniperNetV4NamespacedServiceGroupStatus

impl ServiceGroup {
    /// partially update status of the specified ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchServiceGroupStatusResponse`]`>` constructor, or [`PatchServiceGroupStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceGroup
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_status(
        name: &str,
        namespace: &str,
        body: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchServiceGroupStatusOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<PatchServiceGroupStatusResponse>), crate::RequestError> {
        let PatchServiceGroupStatusOptional {
            dry_run,
            field_manager,
            field_validation,
            force,
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/servicegroups/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(field_validation) = field_validation {
            __query_pairs.append_pair("fieldValidation", field_validation);
        }
        if let Some(force) = force {
            __query_pairs.append_pair("force", if force { "true" } else { "false" });
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::patch(__url);
        let __body = k8s_openapi::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static(match body {
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::patch_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchServiceGroupStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch).
    pub field_manager: Option<&'a str>,
    /// fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields, provided that the `ServerSideFieldValidation` feature gate is also enabled. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23 and is the default behavior when the `ServerSideFieldValidation` feature gate is disabled. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default when the `ServerSideFieldValidation` feature gate is enabled. - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.
    pub field_validation: Option<&'a str>,
    /// Force is going to "force" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests.
    pub force: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<PatchServiceGroupStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::patch_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchServiceGroupStatusResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Created(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchServiceGroupStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchServiceGroupStatusResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::CREATED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchServiceGroupStatusResponse::Created(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((PatchServiceGroupStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readCoreContrailJuniperNetV4NamespacedServiceGroup

impl ServiceGroup {
    /// read the specified ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadServiceGroupResponse`]`>` constructor, or [`ReadServiceGroupResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceGroup
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read(
        name: &str,
        namespace: &str,
        optional: ReadServiceGroupOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadServiceGroupResponse>), crate::RequestError> {
        let ReadServiceGroupOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/servicegroups/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::read`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadServiceGroupOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadServiceGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::read`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadServiceGroupResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadServiceGroupResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadServiceGroupResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadServiceGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readCoreContrailJuniperNetV4NamespacedServiceGroupStatus

impl ServiceGroup {
    /// read status of the specified ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadServiceGroupStatusResponse`]`>` constructor, or [`ReadServiceGroupStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceGroup
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_status(
        name: &str,
        namespace: &str,
        optional: ReadServiceGroupStatusOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadServiceGroupStatusResponse>), crate::RequestError> {
        let ReadServiceGroupStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/servicegroups/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::read_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadServiceGroupStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadServiceGroupStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::read_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadServiceGroupStatusResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadServiceGroupStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadServiceGroupStatusResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadServiceGroupStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceCoreContrailJuniperNetV4NamespacedServiceGroup

impl ServiceGroup {
    /// replace the specified ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceServiceGroupResponse`]`>` constructor, or [`ReplaceServiceGroupResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceGroup
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace(
        name: &str,
        namespace: &str,
        body: &crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup,
        optional: ReplaceServiceGroupOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReplaceServiceGroupResponse>), crate::RequestError> {
        let ReplaceServiceGroupOptional {
            dry_run,
            field_manager,
            field_validation,
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/servicegroups/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(field_validation) = field_validation {
            __query_pairs.append_pair("fieldValidation", field_validation);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::put(__url);
        let __body = k8s_openapi::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::replace`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceServiceGroupOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields, provided that the `ServerSideFieldValidation` feature gate is also enabled. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23 and is the default behavior when the `ServerSideFieldValidation` feature gate is disabled. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default when the `ServerSideFieldValidation` feature gate is enabled. - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.
    pub field_validation: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceServiceGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::replace`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceServiceGroupResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Created(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReplaceServiceGroupResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceServiceGroupResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::CREATED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceServiceGroupResponse::Created(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReplaceServiceGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceCoreContrailJuniperNetV4NamespacedServiceGroupStatus

impl ServiceGroup {
    /// replace status of the specified ServiceGroup
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceServiceGroupStatusResponse`]`>` constructor, or [`ReplaceServiceGroupStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceGroup
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_status(
        name: &str,
        namespace: &str,
        body: &crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup,
        optional: ReplaceServiceGroupStatusOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReplaceServiceGroupStatusResponse>), crate::RequestError> {
        let ReplaceServiceGroupStatusOptional {
            dry_run,
            field_manager,
            field_validation,
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/servicegroups/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(dry_run) = dry_run {
            __query_pairs.append_pair("dryRun", dry_run);
        }
        if let Some(field_manager) = field_manager {
            __query_pairs.append_pair("fieldManager", field_manager);
        }
        if let Some(field_validation) = field_validation {
            __query_pairs.append_pair("fieldValidation", field_validation);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::put(__url);
        let __body = k8s_openapi::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::replace_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceServiceGroupStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields, provided that the `ServerSideFieldValidation` feature gate is also enabled. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23 and is the default behavior when the `ServerSideFieldValidation` feature gate is disabled. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default when the `ServerSideFieldValidation` feature gate is enabled. - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.
    pub field_validation: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceServiceGroupStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::replace_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceServiceGroupStatusResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Created(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroup),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReplaceServiceGroupStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceServiceGroupStatusResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::CREATED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceServiceGroupStatusResponse::Created(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReplaceServiceGroupStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchCoreContrailJuniperNetV4NamespacedServiceGroup

impl ServiceGroup {
    /// watch changes to an object of kind ServiceGroup. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchServiceGroupResponse`]`>` constructor, or [`WatchServiceGroupResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceGroup
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch(
        name: &str,
        namespace: &str,
        optional: WatchServiceGroupOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<WatchServiceGroupResponse>), crate::RequestError> {
        let WatchServiceGroupOptional {
            allow_watch_bookmarks,
            continue_,
            field_selector,
            label_selector,
            limit,
            pretty,
            resource_version,
            resource_version_match,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/watch/namespaces/{namespace}/servicegroups/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(allow_watch_bookmarks) = allow_watch_bookmarks {
            __query_pairs.append_pair("allowWatchBookmarks", if allow_watch_bookmarks { "true" } else { "false" });
        }
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(resource_version_match) = resource_version_match {
            __query_pairs.append_pair("resourceVersionMatch", resource_version_match);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", if watch { "true" } else { "false" });
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::watch`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchServiceGroupOptional<'a> {
    /// allowWatchBookmarks requests watch events with type "BOOKMARK". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored.
    pub allow_watch_bookmarks: Option<bool>,
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version: Option<&'a str>,
    /// resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version_match: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<WatchServiceGroupResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::watch`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchServiceGroupResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchServiceGroupResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((WatchServiceGroupResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((WatchServiceGroupResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchCoreContrailJuniperNetV4NamespacedServiceGroupList

impl ServiceGroup {
    /// watch individual changes to a list of ServiceGroup. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchServiceGroupListResponse`]`>` constructor, or [`WatchServiceGroupListResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_list(
        namespace: &str,
        optional: WatchServiceGroupListOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<WatchServiceGroupListResponse>), crate::RequestError> {
        let WatchServiceGroupListOptional {
            allow_watch_bookmarks,
            continue_,
            field_selector,
            label_selector,
            limit,
            pretty,
            resource_version,
            resource_version_match,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/watch/namespaces/{namespace}/servicegroups?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(allow_watch_bookmarks) = allow_watch_bookmarks {
            __query_pairs.append_pair("allowWatchBookmarks", if allow_watch_bookmarks { "true" } else { "false" });
        }
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(resource_version_match) = resource_version_match {
            __query_pairs.append_pair("resourceVersionMatch", resource_version_match);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", if watch { "true" } else { "false" });
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::watch_list`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchServiceGroupListOptional<'a> {
    /// allowWatchBookmarks requests watch events with type "BOOKMARK". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored.
    pub allow_watch_bookmarks: Option<bool>,
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version: Option<&'a str>,
    /// resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version_match: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<WatchServiceGroupListResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::watch_list`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchServiceGroupListResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchServiceGroupListResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((WatchServiceGroupListResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((WatchServiceGroupListResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchCoreContrailJuniperNetV4NamespacedServiceGroupStatus

impl ServiceGroup {
    /// watch changes to status of an object of kind ServiceGroup. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchServiceGroupStatusResponse`]`>` constructor, or [`WatchServiceGroupStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ServiceGroup
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_status(
        name: &str,
        namespace: &str,
        optional: WatchServiceGroupStatusOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<WatchServiceGroupStatusResponse>), crate::RequestError> {
        let WatchServiceGroupStatusOptional {
            allow_watch_bookmarks,
            continue_,
            field_selector,
            label_selector,
            limit,
            pretty,
            resource_version,
            resource_version_match,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/watch/namespaces/{namespace}/servicegroups/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(allow_watch_bookmarks) = allow_watch_bookmarks {
            __query_pairs.append_pair("allowWatchBookmarks", if allow_watch_bookmarks { "true" } else { "false" });
        }
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(resource_version_match) = resource_version_match {
            __query_pairs.append_pair("resourceVersionMatch", resource_version_match);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", if watch { "true" } else { "false" });
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::watch_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchServiceGroupStatusOptional<'a> {
    /// allowWatchBookmarks requests watch events with type "BOOKMARK". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored.
    pub allow_watch_bookmarks: Option<bool>,
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version: Option<&'a str>,
    /// resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version_match: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<WatchServiceGroupStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::watch_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchServiceGroupStatusResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchServiceGroupStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((WatchServiceGroupStatusResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((WatchServiceGroupStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchCoreContrailJuniperNetV4ServiceGroupListForAllNamespaces

impl ServiceGroup {
    /// watch individual changes to a list of ServiceGroup. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchServiceGroupListForAllNamespacesResponse`]`>` constructor, or [`WatchServiceGroupListForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_list_for_all_namespaces(
        optional: WatchServiceGroupListForAllNamespacesOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<WatchServiceGroupListForAllNamespacesResponse>), crate::RequestError> {
        let WatchServiceGroupListForAllNamespacesOptional {
            allow_watch_bookmarks,
            continue_,
            field_selector,
            label_selector,
            limit,
            pretty,
            resource_version,
            resource_version_match,
            timeout_seconds,
            watch,
        } = optional;
        let __url = "/apis/core.contrail.juniper.net/v4/watch/servicegroups?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(allow_watch_bookmarks) = allow_watch_bookmarks {
            __query_pairs.append_pair("allowWatchBookmarks", if allow_watch_bookmarks { "true" } else { "false" });
        }
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(resource_version_match) = resource_version_match {
            __query_pairs.append_pair("resourceVersionMatch", resource_version_match);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", if watch { "true" } else { "false" });
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`ServiceGroup::watch_list_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchServiceGroupListForAllNamespacesOptional<'a> {
    /// allowWatchBookmarks requests watch events with type "BOOKMARK". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored.
    pub allow_watch_bookmarks: Option<bool>,
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version: Option<&'a str>,
    /// resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.
    ///
    /// Defaults to unset
    pub resource_version_match: Option<&'a str>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Use `<WatchServiceGroupListForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`ServiceGroup::watch_list_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchServiceGroupListForAllNamespacesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchServiceGroupListForAllNamespacesResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((WatchServiceGroupListForAllNamespacesResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match k8s_openapi::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((WatchServiceGroupListForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// End core.contrail.juniper.net/v4/ServiceGroup

impl k8s_openapi::Resource for ServiceGroup {
    const API_VERSION: &'static str = "core.contrail.juniper.net/v4";
    const GROUP: &'static str = "core.contrail.juniper.net";
    const KIND: &'static str = "ServiceGroup";
    const VERSION: &'static str = "v4";
    const URL_PATH_SEGMENT: &'static str = "servicegroups";
    type Scope = k8s_openapi::NamespaceResourceScope;
}

impl k8s_openapi::DeepMerge for ServiceGroup {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi::DeepMerge::merge_from(&mut self.spec, other.spec);
        k8s_openapi::DeepMerge::merge_from(&mut self.status, other.status);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for ServiceGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
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
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceGroup;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroupSpec> = None;
                let mut value_status: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroupStatus> = None;

                while let Some(key) = k8s_openapi::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as k8s_openapi::Resource>::API_VERSION {
                                return Err(k8s_openapi::serde::de::Error::invalid_value(k8s_openapi::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as k8s_openapi::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as k8s_openapi::Resource>::KIND {
                                return Err(k8s_openapi::serde::de::Error::invalid_value(k8s_openapi::serde::de::Unexpected::Str(&value_kind), &<Self::Value as k8s_openapi::Resource>::KIND));
                            }
                        },
                        Field::Key_metadata => value_metadata = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: k8s_openapi::serde::de::IgnoredAny = k8s_openapi::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceGroup {
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
                "status",
            ],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for ServiceGroup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as k8s_openapi::Resource>::KIND,
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as k8s_openapi::Resource>::API_VERSION)?;
        k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as k8s_openapi::Resource>::KIND)?;
        if let Some(value) = &self.metadata {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.spec {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            k8s_openapi::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        k8s_openapi::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ServiceGroup {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroup".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ServiceGroup describes the list of protocol:port associated with the FirewallRule resource.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "apiVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kind".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "metadata".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "spec".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroupSpec>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specification of the desired state of the ServiceGroup. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "status".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ServiceGroupStatus>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The most recently observed status of the ServiceGroup. This data may not be up-to-date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".to_owned()),
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
