// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.Subnet

/// Subnet represents a block of IP addresses and its configuration. IPAM allocates and releases IP address from that block on demand. It can be used by different VirtualNetwork in the mean time.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Subnet {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired state of the Subnet. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub spec: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SubnetSpec>,

    /// The most recently observed status of the Subnet. This data may not be up-to-date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub status: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SubnetStatus>,
}

// Begin core.contrail.juniper.net/v4/Subnet

// Generated from operation createCoreContrailJuniperNetV4NamespacedSubnet

impl Subnet {
    /// create a Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateSubnetResponse`]`>` constructor, or [`CreateSubnetResponse`] directly, to parse the HTTP response.
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
        body: &crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet,
        optional: CreateSubnetOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<CreateSubnetResponse>), crate::RequestError> {
        let CreateSubnetOptional {
            dry_run,
            field_manager,
            field_validation,
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/subnets?",
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

/// Optional parameters of [`Subnet::create`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateSubnetOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields, provided that the `ServerSideFieldValidation` feature gate is also enabled. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23 and is the default behavior when the `ServerSideFieldValidation` feature gate is disabled. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default when the `ServerSideFieldValidation` feature gate is enabled. - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.
    pub field_validation: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateSubnetResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::create`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateSubnetResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Created(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Accepted(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for CreateSubnetResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateSubnetResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::CREATED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateSubnetResponse::Created(result), buf.len()))
            },
            crate::http::StatusCode::ACCEPTED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateSubnetResponse::Accepted(result), buf.len()))
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
                Ok((CreateSubnetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation createCoreContrailJuniperNetV4NamespacedSubnetStatus

impl Subnet {
    /// create status of a Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateSubnetStatusResponse`]`>` constructor, or [`CreateSubnetStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Subnet
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
        body: &crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet,
        optional: CreateSubnetStatusOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<CreateSubnetStatusResponse>), crate::RequestError> {
        let CreateSubnetStatusOptional {
            dry_run,
            field_manager,
            field_validation,
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/subnets/{name}/status?",
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

/// Optional parameters of [`Subnet::create_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateSubnetStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields, provided that the `ServerSideFieldValidation` feature gate is also enabled. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23 and is the default behavior when the `ServerSideFieldValidation` feature gate is disabled. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default when the `ServerSideFieldValidation` feature gate is enabled. - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.
    pub field_validation: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<CreateSubnetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::create_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateSubnetStatusResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Created(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Accepted(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for CreateSubnetStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateSubnetStatusResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::CREATED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateSubnetStatusResponse::Created(result), buf.len()))
            },
            crate::http::StatusCode::ACCEPTED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateSubnetStatusResponse::Accepted(result), buf.len()))
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
                Ok((CreateSubnetStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteCoreContrailJuniperNetV4CollectionNamespacedSubnet

impl Subnet {
    /// delete collection of Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteCollectionSubnetResponse`]`>` constructor, or [`DeleteCollectionSubnetResponse`] directly, to parse the HTTP response.
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
        optional: DeleteCollectionSubnetOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<DeleteCollectionSubnetResponse>), crate::RequestError> {
        let DeleteCollectionSubnetOptional {
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
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/subnets?",
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

/// Optional parameters of [`Subnet::delete_collection`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteCollectionSubnetOptional<'a> {
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

/// Use `<DeleteCollectionSubnetResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::delete_collection`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteCollectionSubnetResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteCollectionSubnetResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteCollectionSubnetResponse::Ok(result), buf.len()))
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
                Ok((DeleteCollectionSubnetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteCoreContrailJuniperNetV4NamespacedSubnet

impl Subnet {
    /// delete a Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteSubnetResponse`]`>` constructor, or [`DeleteSubnetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Subnet
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
        optional: DeleteSubnetOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<DeleteSubnetResponse>), crate::RequestError> {
        let DeleteSubnetOptional {
            body,
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/subnets/{name}?",
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

/// Optional parameters of [`Subnet::delete`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteSubnetOptional<'a> {
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

/// Use `<DeleteSubnetResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::delete`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteSubnetResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::Status),
    Accepted(k8s_openapi::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteSubnetResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteSubnetResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::ACCEPTED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteSubnetResponse::Accepted(result), buf.len()))
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
                Ok((DeleteSubnetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation deleteCoreContrailJuniperNetV4NamespacedSubnetStatus

impl Subnet {
    /// delete status of a Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`DeleteSubnetStatusResponse`]`>` constructor, or [`DeleteSubnetStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Subnet
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
        optional: DeleteSubnetStatusOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<DeleteSubnetStatusResponse>), crate::RequestError> {
        let DeleteSubnetStatusOptional {
            body,
            dry_run,
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/subnets/{name}/status?",
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

/// Optional parameters of [`Subnet::delete_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeleteSubnetStatusOptional<'a> {
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

/// Use `<DeleteSubnetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::delete_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteSubnetStatusResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::Status),
    Accepted(k8s_openapi::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for DeleteSubnetStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteSubnetStatusResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::ACCEPTED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteSubnetStatusResponse::Accepted(result), buf.len()))
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
                Ok((DeleteSubnetStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listCoreContrailJuniperNetV4NamespacedSubnet

impl Subnet {
    /// list or watch objects of kind Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListSubnetResponse`]`>` constructor, or [`ListSubnetResponse`] directly, to parse the HTTP response.
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
        optional: ListSubnetOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ListSubnetResponse>), crate::RequestError> {
        let ListSubnetOptional {
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
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/subnets?",
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

/// Optional parameters of [`Subnet::list`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ListSubnetOptional<'a> {
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

/// Use `<ListSubnetResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::list`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListSubnetResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SubnetList),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListSubnetResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListSubnetResponse::Ok(result), buf.len()))
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
                Ok((ListSubnetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation listCoreContrailJuniperNetV4SubnetForAllNamespaces

impl Subnet {
    /// list or watch objects of kind Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ListSubnetForAllNamespacesResponse`]`>` constructor, or [`ListSubnetForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_for_all_namespaces(
        optional: ListSubnetForAllNamespacesOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ListSubnetForAllNamespacesResponse>), crate::RequestError> {
        let ListSubnetForAllNamespacesOptional {
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
        let __url = "/apis/core.contrail.juniper.net/v4/subnets?".to_owned();
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

/// Optional parameters of [`Subnet::list_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ListSubnetForAllNamespacesOptional<'a> {
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

/// Use `<ListSubnetForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::list_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ListSubnetForAllNamespacesResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SubnetList),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ListSubnetForAllNamespacesResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListSubnetForAllNamespacesResponse::Ok(result), buf.len()))
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
                Ok((ListSubnetForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchCoreContrailJuniperNetV4NamespacedSubnet

impl Subnet {
    /// partially update the specified Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchSubnetResponse`]`>` constructor, or [`PatchSubnetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Subnet
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
        optional: PatchSubnetOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<PatchSubnetResponse>), crate::RequestError> {
        let PatchSubnetOptional {
            dry_run,
            field_manager,
            field_validation,
            force,
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/subnets/{name}?",
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

/// Optional parameters of [`Subnet::patch`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchSubnetOptional<'a> {
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

/// Use `<PatchSubnetResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::patch`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchSubnetResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Created(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchSubnetResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchSubnetResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::CREATED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchSubnetResponse::Created(result), buf.len()))
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
                Ok((PatchSubnetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation patchCoreContrailJuniperNetV4NamespacedSubnetStatus

impl Subnet {
    /// partially update status of the specified Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`PatchSubnetStatusResponse`]`>` constructor, or [`PatchSubnetStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Subnet
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
        optional: PatchSubnetStatusOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<PatchSubnetStatusResponse>), crate::RequestError> {
        let PatchSubnetStatusOptional {
            dry_run,
            field_manager,
            field_validation,
            force,
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/subnets/{name}/status?",
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

/// Optional parameters of [`Subnet::patch_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct PatchSubnetStatusOptional<'a> {
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

/// Use `<PatchSubnetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::patch_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum PatchSubnetStatusResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Created(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for PatchSubnetStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchSubnetStatusResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::CREATED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchSubnetStatusResponse::Created(result), buf.len()))
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
                Ok((PatchSubnetStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readCoreContrailJuniperNetV4NamespacedSubnet

impl Subnet {
    /// read the specified Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadSubnetResponse`]`>` constructor, or [`ReadSubnetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Subnet
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
        optional: ReadSubnetOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadSubnetResponse>), crate::RequestError> {
        let ReadSubnetOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/subnets/{name}?",
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

/// Optional parameters of [`Subnet::read`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadSubnetOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadSubnetResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::read`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadSubnetResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadSubnetResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadSubnetResponse::Ok(result), buf.len()))
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
                Ok((ReadSubnetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readCoreContrailJuniperNetV4NamespacedSubnetStatus

impl Subnet {
    /// read status of the specified Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadSubnetStatusResponse`]`>` constructor, or [`ReadSubnetStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Subnet
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
        optional: ReadSubnetStatusOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadSubnetStatusResponse>), crate::RequestError> {
        let ReadSubnetStatusOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/subnets/{name}/status?",
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

/// Optional parameters of [`Subnet::read_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadSubnetStatusOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReadSubnetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::read_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadSubnetStatusResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadSubnetStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadSubnetStatusResponse::Ok(result), buf.len()))
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
                Ok((ReadSubnetStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceCoreContrailJuniperNetV4NamespacedSubnet

impl Subnet {
    /// replace the specified Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceSubnetResponse`]`>` constructor, or [`ReplaceSubnetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Subnet
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
        body: &crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet,
        optional: ReplaceSubnetOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReplaceSubnetResponse>), crate::RequestError> {
        let ReplaceSubnetOptional {
            dry_run,
            field_manager,
            field_validation,
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/subnets/{name}?",
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

/// Optional parameters of [`Subnet::replace`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceSubnetOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields, provided that the `ServerSideFieldValidation` feature gate is also enabled. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23 and is the default behavior when the `ServerSideFieldValidation` feature gate is disabled. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default when the `ServerSideFieldValidation` feature gate is enabled. - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.
    pub field_validation: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceSubnetResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::replace`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceSubnetResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Created(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReplaceSubnetResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceSubnetResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::CREATED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceSubnetResponse::Created(result), buf.len()))
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
                Ok((ReplaceSubnetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceCoreContrailJuniperNetV4NamespacedSubnetStatus

impl Subnet {
    /// replace status of the specified Subnet
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReplaceSubnetStatusResponse`]`>` constructor, or [`ReplaceSubnetStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Subnet
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
        body: &crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet,
        optional: ReplaceSubnetStatusOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReplaceSubnetStatusResponse>), crate::RequestError> {
        let ReplaceSubnetStatusOptional {
            dry_run,
            field_manager,
            field_validation,
            pretty,
        } = optional;
        let __url = format!("/apis/core.contrail.juniper.net/v4/namespaces/{namespace}/subnets/{name}/status?",
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

/// Optional parameters of [`Subnet::replace_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReplaceSubnetStatusOptional<'a> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    pub dry_run: Option<&'a str>,
    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    pub field_manager: Option<&'a str>,
    /// fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields, provided that the `ServerSideFieldValidation` feature gate is also enabled. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23 and is the default behavior when the `ServerSideFieldValidation` feature gate is disabled. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default when the `ServerSideFieldValidation` feature gate is enabled. - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.
    pub field_validation: Option<&'a str>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Use `<ReplaceSubnetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::replace_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceSubnetStatusResponse {
    Ok(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Created(crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::Subnet),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReplaceSubnetStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceSubnetStatusResponse::Ok(result), buf.len()))
            },
            crate::http::StatusCode::CREATED => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceSubnetStatusResponse::Created(result), buf.len()))
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
                Ok((ReplaceSubnetStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchCoreContrailJuniperNetV4NamespacedSubnet

impl Subnet {
    /// watch changes to an object of kind Subnet. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchSubnetResponse`]`>` constructor, or [`WatchSubnetResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Subnet
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
        optional: WatchSubnetOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<WatchSubnetResponse>), crate::RequestError> {
        let WatchSubnetOptional {
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
        let __url = format!("/apis/core.contrail.juniper.net/v4/watch/namespaces/{namespace}/subnets/{name}?",
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

/// Optional parameters of [`Subnet::watch`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchSubnetOptional<'a> {
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

/// Use `<WatchSubnetResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::watch`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchSubnetResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchSubnetResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((WatchSubnetResponse::Ok(result), buf.len()))
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
                Ok((WatchSubnetResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchCoreContrailJuniperNetV4NamespacedSubnetList

impl Subnet {
    /// watch individual changes to a list of Subnet. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchSubnetListResponse`]`>` constructor, or [`WatchSubnetListResponse`] directly, to parse the HTTP response.
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
        optional: WatchSubnetListOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<WatchSubnetListResponse>), crate::RequestError> {
        let WatchSubnetListOptional {
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
        let __url = format!("/apis/core.contrail.juniper.net/v4/watch/namespaces/{namespace}/subnets?",
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

/// Optional parameters of [`Subnet::watch_list`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchSubnetListOptional<'a> {
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

/// Use `<WatchSubnetListResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::watch_list`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchSubnetListResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchSubnetListResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((WatchSubnetListResponse::Ok(result), buf.len()))
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
                Ok((WatchSubnetListResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchCoreContrailJuniperNetV4NamespacedSubnetStatus

impl Subnet {
    /// watch changes to status of an object of kind Subnet. deprecated: use the 'watch' parameter with a list operation instead, filtered to a single item with the 'fieldSelector' parameter.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchSubnetStatusResponse`]`>` constructor, or [`WatchSubnetStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Subnet
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
        optional: WatchSubnetStatusOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<WatchSubnetStatusResponse>), crate::RequestError> {
        let WatchSubnetStatusOptional {
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
        let __url = format!("/apis/core.contrail.juniper.net/v4/watch/namespaces/{namespace}/subnets/{name}/status?",
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

/// Optional parameters of [`Subnet::watch_status`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchSubnetStatusOptional<'a> {
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

/// Use `<WatchSubnetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::watch_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchSubnetStatusResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchSubnetStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((WatchSubnetStatusResponse::Ok(result), buf.len()))
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
                Ok((WatchSubnetStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation watchCoreContrailJuniperNetV4SubnetListForAllNamespaces

impl Subnet {
    /// watch individual changes to a list of Subnet. deprecated: use the 'watch' parameter with a list operation instead.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`WatchSubnetListForAllNamespacesResponse`]`>` constructor, or [`WatchSubnetListForAllNamespacesResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_list_for_all_namespaces(
        optional: WatchSubnetListForAllNamespacesOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<WatchSubnetListForAllNamespacesResponse>), crate::RequestError> {
        let WatchSubnetListForAllNamespacesOptional {
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
        let __url = "/apis/core.contrail.juniper.net/v4/watch/subnets?".to_owned();
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

/// Optional parameters of [`Subnet::watch_list_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WatchSubnetListForAllNamespacesOptional<'a> {
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

/// Use `<WatchSubnetListForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`Subnet::watch_list_for_all_namespaces`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchSubnetListForAllNamespacesResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Other(Result<Option<k8s_openapi::serde_json::Value>, k8s_openapi::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for WatchSubnetListForAllNamespacesResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match k8s_openapi::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((WatchSubnetListForAllNamespacesResponse::Ok(result), buf.len()))
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
                Ok((WatchSubnetListForAllNamespacesResponse::Other(result), read))
            },
        }
    }
}

// End core.contrail.juniper.net/v4/Subnet

impl k8s_openapi::Resource for Subnet {
    const API_VERSION: &'static str = "core.contrail.juniper.net/v4";
    const GROUP: &'static str = "core.contrail.juniper.net";
    const KIND: &'static str = "Subnet";
    const VERSION: &'static str = "v4";
    const URL_PATH_SEGMENT: &'static str = "subnets";
    type Scope = k8s_openapi::NamespaceResourceScope;
}

impl k8s_openapi::DeepMerge for Subnet {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi::DeepMerge::merge_from(&mut self.spec, other.spec);
        k8s_openapi::DeepMerge::merge_from(&mut self.status, other.status);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for Subnet {
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
            type Value = Subnet;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as k8s_openapi::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: k8s_openapi::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SubnetSpec> = None;
                let mut value_status: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SubnetStatus> = None;

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

                Ok(Subnet {
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

impl k8s_openapi::serde::Serialize for Subnet {
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
impl crate::schemars::JsonSchema for Subnet {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.Subnet".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Subnet represents a block of IP addresses and its configuration. IPAM allocates and releases IP address from that block on demand. It can be used by different VirtualNetwork in the mean time.".to_owned()),
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
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SubnetSpec>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specification of the desired state of the Subnet. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "status".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::SubnetStatus>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The most recently observed status of the Subnet. This data may not be up-to-date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".to_owned()),
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
