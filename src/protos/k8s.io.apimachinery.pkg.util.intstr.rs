/// IntOrString is a type that can hold an int32 or a string.  When used in
/// JSON or YAML marshalling and unmarshalling, it produces or consumes the
/// inner type.  This allows you to have, for example, a JSON field that can
/// accept a name or number.
/// TODO: Rename to Int32OrString
///
/// +protobuf=true
/// +protobuf.options.(gogoproto.goproto_stringer)=false
/// +k8s:openapi-gen=true
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntOrString {
    #[prost(int64, optional, tag = "1")]
    pub r#type: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub int_val: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub str_val: ::core::option::Option<::prost::alloc::string::String>,
}
