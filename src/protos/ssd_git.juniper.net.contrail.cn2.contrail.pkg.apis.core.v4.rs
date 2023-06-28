/// APSAttribute defines the sequence number associated with FireWall Policy referred by given APS.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApsAttribute {
    /// Sequence number of a FireWallPolicy
    #[prost(string, optional, tag = "1")]
    pub sequence: ::core::option::Option<::prost::alloc::string::String>,
}
/// AddressFamilies contains a list of BGP address families supported by BGP router
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressFamilies {
    /// BGP address families supported by BGP router
    #[prost(string, repeated, tag = "1")]
    pub family: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// AddressGroup describes the set of CIDRs associated with the FirewallRule resource.
/// +k8s:openapi-gen=true
/// +resource:path=addressgroups,strategy=AddressGroupStrategy,shortname=ag,categories=contrail;security
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressGroup {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the AddressGroup.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<AddressGroupSpec>,
    /// The most recently observed status of the AddressGroup.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<AddressGroupStatus>,
}
/// AddressGroupList is a list of AddressGroup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressGroupList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the AddressGroup instances in the AddressGroupList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<AddressGroup>,
}
/// AddressGroupPrefix describes the CIDR configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressGroupPrefix {
    /// Subnet defines the IP prefix and length.
    #[prost(message, repeated, tag = "1")]
    pub subnet: ::prost::alloc::vec::Vec<FirewallSubnet>,
}
/// AddressGroupSpec describes the set of CIDRs associated with the FirewallRule resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressGroupSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// AddressGroupPrefixes defines list of subnets used to match a group of workloads.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub address_group_prefixes: ::core::option::Option<AddressGroupPrefix>,
}
/// AddressGroupStatus defines the observed state of AddressGroup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressGroupStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
/// AllowedAddressPair allows the additional IP/MAC pairs to the guest
/// interface. The traffic matching to specified value of IP/MAC will be redirected.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowedAddressPair {
    /// IPAddress defines the IPv4 or IPv6 address of the pair.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub ip: ::core::option::Option<AllowedAddressPairSubnet>,
    /// MACAddress defines the media access control ID of the NIC..
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub mac: ::core::option::Option<::prost::alloc::string::String>,
    /// AddressMode active-standby is used for VRRP address.
    /// AddressMode active-active is used for ECMP.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub address_mode: ::core::option::Option<::prost::alloc::string::String>,
}
/// AllowedAddressPairSubnet defines the IP prefix and length.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowedAddressPairSubnet {
    /// Network prefix
    #[prost(string, optional, tag = "1")]
    pub ip_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Network prefix length
    #[prost(message, optional, tag = "2")]
    pub ip_prefix_len: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::util::intstr::IntOrString,
    >,
}
/// AllowedAddressPairs is a list of AllowedAddressPair.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowedAddressPairs {
    #[prost(message, repeated, tag = "1")]
    pub allowed_address_pair: ::prost::alloc::vec::Vec<AllowedAddressPair>,
}
/// +genclient
/// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
/// ApplicationPolicy defines set of firewall policies belonging to a given application tag for project/namespace resources (VN, VM or VMI).
/// +k8s:openapi-gen=true
/// +resource:path=applicationpolicysets,strategy=ApplicationPolicySetStrategy,shortname=aps,categories=contrail;security
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationPolicySet {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the ApplicationPolicySet.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<ApplicationPolicySetSpec>,
    /// The most recently observed status of the ApplicationPolicySet.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<ApplicationPolicySetStatus>,
}
/// ApplicationPolicySetList is a list of ApplicationPolicySet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationPolicySetList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the ApplicationPolicySet instances in the ApplicationPolicySetList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<ApplicationPolicySet>,
}
/// ApplicationPolicySetSpec defines the desired state of a FirewallPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationPolicySetSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// FirewallPolicyReferences contains references to FirewallPolicy associated with the ApplicationPolicySet.
    #[prost(message, repeated, tag = "2")]
    pub firewall_policy_references: ::prost::alloc::vec::Vec<FirewallPolicyReference>,
    /// TagReferences contains references to Tags attached to the ApplicationPolicySet.
    #[prost(message, repeated, tag = "3")]
    pub tag_references: ::prost::alloc::vec::Vec<ResourceReference>,
    /// ApplicationPolicySetGlobalVrouterConfig reference to defailt-global-vrouter-config
    #[prost(message, optional, tag = "6")]
    pub global_vrouter_config_reference: ::core::option::Option<ResourceReference>,
    /// K8sMode defines ApplicationPolicySet in k8s mode
    /// +optional
    #[prost(bool, optional, tag = "4")]
    pub k8smode: ::core::option::Option<bool>,
    /// ApplicatioPolicySet to be applied to all application tags.
    #[prost(bool, optional, tag = "5")]
    pub all_applications: ::core::option::Option<bool>,
}
/// ApplicationPolicySetStatus defines the observed state of ApplicationPolicySet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationPolicySetStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
/// Authentication related configuration for this session like type, keys etc.
/// Only md5 authentication is supported.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationData {
    /// Authentication type for this session. Currently, only MD5 is supported.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub key_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Upto two keys can be specified. Currently, only one key is supported.
    /// +optional
    #[prost(message, repeated, tag = "2")]
    pub key_items: ::prost::alloc::vec::Vec<AuthenticationKeyItem>,
}
/// AuthenticationKeyItem is used for BGP session authentication configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationKeyItem {
    #[prost(int32, optional, tag = "1")]
    pub key_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
}
/// BGPAsAService determines the configuration of BGP peers.
/// All the BGP peers involved in Contrail are present inthe default
/// RoutingInstance of the default VirtualNetwork.
/// +k8s:openapi-gen=true
/// +resource:path=bgpasaservices,strategy=BGPAsAServiceStrategy,shortname=bgpaas,categories=contrail;networking
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpAsAService {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the BGPAsAService.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<BgpAsAServiceSpec>,
    /// The most recently observed status of the BGPAsAService.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<BgpAsAServiceStatus>,
}
/// BGPAsAServiceList is a list of BGPAsAService.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpAsAServiceList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the BGPAsAService instances in the BGPAsAServiceList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<BgpAsAService>,
}
/// BGPAsAServiceSpec defines the desired state of a BGPAsAService.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpAsAServiceSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// Shared is enabled to link all VMIs with the common bgp-router object.
    /// When false (default), each virtual machine interface individually links to
    /// its own bgp-router object.
    /// +optional
    #[prost(bool, optional, tag = "2")]
    pub shared: ::core::option::Option<bool>,
    /// IPAddress specifies the source-address of a BGPaaS VM/pod.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub ip_address: ::core::option::Option<::prost::alloc::string::String>,
    /// AutonomousSystem is 16-bit BGP Autonomous System number for the cluster.
    #[prost(int32, optional, tag = "4")]
    pub autonomous_system: ::core::option::Option<i32>,
    /// SuppressRouteAdvertisement indicates that the server should not advertise
    /// any routes to the client i.e. the client has static routes (typically a
    /// default) configured, default set to false.
    /// +optional
    #[prost(bool, optional, tag = "5")]
    pub suppress_route_advertisement: ::core::option::Option<bool>,
    /// IPv4MappedIPv6NextHop indicates if the client bgp implementation expects
    /// to receive a ipv4-mapped ipv6 address (as opposed to regular ipv6
    /// address) as the bgp nexthop for ipv6 routes.
    /// +optional
    #[prost(bool, optional, tag = "6")]
    pub ipv4_mapped_i_pv6_next_hop: ::core::option::Option<bool>,
    /// BGPAsAServiceSessionAttributes defines session attributes such as hold time,
    /// route origin and loop count.
    /// +optional
    #[prost(message, optional, tag = "7")]
    pub bgp_as_a_service_session_attributes: ::core::option::Option<
        BgpSessionAttributes,
    >,
    /// VirtualMachineInterfaceReferences determines the VirtualMachineInterfaces
    /// on which BGPaaS BGP peering will happen.
    /// +optional
    #[prost(message, repeated, tag = "8")]
    pub virtual_machine_interface_references: ::prost::alloc::vec::Vec<
        ResourceReference,
    >,
    /// VirtualMachineInterfacesSelector selects VirtualMachineInterfaces using
    /// the 'core.juniper.net/bgpaasVN' label defined on pods. BGPAsAService will be configured on
    /// the union of VMIs selected by label and VMI specified through
    /// VirtualMachineInterfaceReferences.
    /// +optional
    #[prost(message, repeated, tag = "9")]
    pub virtual_machine_interfaces_selector: ::prost::alloc::vec::Vec<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::LabelSelector,
    >,
    /// ServiceHealthCheck determines which ServiceHealthCheck object to be used to performing healthcheck
    /// +optional
    #[prost(message, optional, tag = "10")]
    pub service_health_check_reference: ::core::option::Option<ResourceReference>,
}
/// BGPAsAServiceStatus defines the observed state of BGPAsAService.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpAsAServiceStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
    /// BGPRouterReferences contains references to all BGPRouters created
    /// for a for BGPAsAService session.
    /// +optional
    #[prost(message, repeated, tag = "2")]
    pub bgp_router_references: ::prost::alloc::vec::Vec<BgpRouterReference>,
    /// SubnetReferences contains references to all subnets associated with
    /// the selected VirtualMachineInterfaces' VirtualNetwork.
    /// +optional
    #[prost(message, repeated, tag = "3")]
    pub subnet_references: ::prost::alloc::vec::Vec<SubnetReference>,
    /// VMISelectorReferences contains references to all VirtualMachineInterfaces
    /// selected by VirtualMachineInterfacesSelector.
    /// +optional
    #[prost(message, repeated, tag = "4")]
    pub vmi_selector_references: ::prost::alloc::vec::Vec<ResourceReference>,
}
/// This type is used to configure per address-family parameters
/// for a BgpSession.
///    - loop-count is the number of times the local bgp-router's AS is
///      allowed in the AS_PATH attribute.
///    - prefix-limit contains the maximum number of prefixes that are
///      allowed to be received on the session.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpFamilyAttributes {
    /// Address family for which these parameters are applied.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub address_family: ::core::option::Option<::prost::alloc::string::String>,
    /// For routing loop detection, loop-count is the number of times the local bgp-routers AS is
    /// allowed in the AS_PATH attribute.
    /// +optional
    #[prost(int32, optional, tag = "2")]
    pub loop_count: ::core::option::Option<i32>,
    /// PrefixLimit contains the maximum number of prefixes that are allowed to be received on the session for this address family.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub prefix_limit: ::core::option::Option<BgpPrefixLimit>,
    /// Default prioritized tunnel encapsulation list.
    /// +optional
    #[prost(string, repeated, tag = "4")]
    pub default_tunnel_encap: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// This is used to configure the maximum number of received prefixes
/// and control the behavior of the session when the maximum is reached.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpPrefixLimit {
    /// Time in seconds after which the session is allowed to re-establish after teardown.
    /// +optional
    #[prost(int32, optional, tag = "1")]
    pub idle_timeout: ::core::option::Option<i32>,
    /// Maximum number of prefixes allowed to be recieved
    /// +optional
    #[prost(int32, optional, tag = "2")]
    pub maximum: ::core::option::Option<i32>,
}
/// BGPRouter object represent configuration of BGP peers.
/// All the BGP peers involved in Contrail system are under default Routing
/// Instance of the default Virtual Network.
/// +k8s:openapi-gen=true
/// +resource:path=bgprouters,strategy=BGPRouterStrategy,shortname=br,categories=contrail;routing
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpRouter {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// BGPRouterSpec defines the desired state of BGPRouter.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<BgpRouterSpec>,
    /// BGPRouterStatus defines the observed state of BGPRouter
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<BgpRouterStatus>,
}
/// BGPRouterList is a list of BGPRouter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpRouterList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the BGPRouter instances in the BGPRouterList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<BgpRouter>,
}
/// BGPRouterParameters contains BGP router configuration parameters like
/// IP address, AS number, hold time etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpRouterParameters {
    /// Administratively up or down BGPRouter, session is not established
    /// for the BGPRouter.
    /// +optional
    #[prost(bool, optional, tag = "1")]
    pub admin_down: ::core::option::Option<bool>,
    /// Vendor name for this BGP router.
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub vendor: ::core::option::Option<::prost::alloc::string::String>,
    /// Cluster ID for this BGP router (between 1 and 4294967295)
    /// when control node is configured as route reflector.
    /// +optional
    #[prost(int64, optional, tag = "3")]
    pub cluster_id: ::core::option::Option<i64>,
    /// Autonomous System number for this BGP router. For contrail control nodes,
    /// this is derived from GlobalSystemConfig AutonomousSystem.
    /// +optional
    #[prost(int32, optional, tag = "4")]
    pub autonomous_system: ::core::option::Option<i32>,
    /// Router ID for this BGP router. Dotted ip notation. For Contrail
    /// control-nodes system will automatically assign value of address field.
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub identifier: ::core::option::Option<::prost::alloc::string::String>,
    /// IP address used to reach this BGP router by the system.
    /// +optional
    #[prost(string, optional, tag = "6")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
    /// TCP port number on which BGP protocol connections are accepted.
    /// Default is port 179
    /// +optional
    #[prost(int32, optional, tag = "7")]
    pub port: ::core::option::Option<i32>,
    /// For system internal use in BGPaaS service.
    /// +optional
    #[prost(int32, optional, tag = "8")]
    pub source_port: ::core::option::Option<i32>,
    /// BGPHoldTime is time in seconds \[0-65535\], maximum time to detect
    /// liveliness to peer. Value 0 will result in default value of 90 seconds.
    /// +optional
    #[prost(int32, optional, tag = "9")]
    pub hold_time: ::core::option::Option<i32>,
    /// BGP address families supported by BGP router. If not specified
    /// these address families are enabled:
    /// "inet, inet-labeled, inet-vpn, e-vpn, erm-vpn, route-target, inet6,
    /// inet-mvpn, inet6-vpn"
    /// +optional
    #[prost(message, optional, tag = "10")]
    pub address_families: ::core::option::Option<AddressFamilies>,
    /// Authentication related configuration like type, keys etc.
    /// +optional
    #[prost(message, optional, tag = "11")]
    pub auth_data: ::core::option::Option<AuthenticationData>,
    /// BGPRouter specific Autonomous System number if different from global AS
    /// number. Typically used when clusters of control nodes in same contrail
    /// system are in different locations.
    /// +optional
    #[prost(int32, optional, tag = "12")]
    pub local_autonomous_system: ::core::option::Option<i32>,
    /// BGPRouter type.
    /// +optional
    #[prost(string, optional, tag = "13")]
    pub router_type: ::core::option::Option<::prost::alloc::string::String>,
    /// GatewayAddress field is used only for router-type bgpaas-client.
    /// It holds the IPv4 gateway address for the IPv4 subnet from which the
    /// client has IP address. The value is used as nexthop when advertising
    /// routes to the client via bgp.
    /// +optional
    #[prost(string, optional, tag = "14")]
    pub gateway_address: ::core::option::Option<::prost::alloc::string::String>,
    /// IPv6GatewayAddress field is used only for router-type bgpaas-client
    /// It holds IPv6 gateway address for IPv6 subnet from which the client has
    /// IP address. The value is used as nexthop when advertising routes to the
    /// client via bgp.
    /// Note that the IPv6GatewayAddress can be a regular IPv6 address or a
    /// ipv4-mapped-ipv6 adddress.
    /// +optional
    #[prost(string, optional, tag = "15")]
    pub ipv6_gateway_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// BGPRouterReference contains BGP Router peering and its session configurations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpRouterReference {
    /// ResourceReference for BGPRouter resource
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub resource_reference: ::core::option::Option<ResourceReference>,
    /// attributes configuration for the BGPRouter reference.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub attributes: ::core::option::Option<BgpRouterReferenceAttributes>,
}
/// BGPRouterReferenceAttributes defines the attributes for 1 (typically) or more sessions between two BGP Routers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpRouterReferenceAttributes {
    /// Seesion is a list of BGP sessions parameters.
    /// There can be multiple BGP sessions between two BGP routers.
    /// Currently, only 1 session is supported.
    /// +optional
    #[prost(message, repeated, tag = "1")]
    pub session: ::prost::alloc::vec::Vec<BgpSession>,
}
/// BGPRouterSpec defines the desired state of BGPRouter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpRouterSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// Object reference to routing-instance parent
    #[prost(message, optional, tag = "2")]
    pub parent: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::api::core::v1::ObjectReference,
    >,
    /// BGPRouterReferences list of references to all bgp routers in the cluster.
    /// +optional
    #[prost(message, repeated, tag = "4")]
    pub bgp_router_references: ::prost::alloc::vec::Vec<BgpRouterReference>,
    /// BGP router configuration parameters like IP address, AS number, hold time etc.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub bgp_router_parameters: ::core::option::Option<BgpRouterParameters>,
}
/// BGPRouterStatus defines the observed state of BGPRouter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpRouterStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
/// BGPSession defines the attributes for 1 (typically) or more sessions between two BGP Routers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpSession {
    /// There can be up to 3 instances BGP session attributes,
    /// representing configuration for both ends and common.
    /// Currently, only 1 instance representing common attributes is supported.
    /// +optional
    #[prost(message, repeated, tag = "1")]
    pub session_attributes: ::prost::alloc::vec::Vec<BgpSessionAttributes>,
}
/// BGPSessionAttributes defines the BGP session parameters configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpSessionAttributes {
    /// When the parameters are uni-directional the bgp-router element specifies
    /// to which node the configuration applies. If missing the attributes apply
    /// to both ends of the session.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub bgp_router: ::core::option::Option<::prost::alloc::string::String>,
    /// Administratively mark this session down.
    /// +optional
    #[prost(bool, optional, tag = "2")]
    pub admin_down: ::core::option::Option<bool>,
    /// This is passive session. It will not initiated connection.
    /// This is not relevant when session attributes represent common part.
    /// It is recommended that it should not be set to true in current release.
    /// +optional
    #[prost(bool, optional, tag = "3")]
    pub passive: ::core::option::Option<bool>,
    /// ASOverride flag is used to replace the AS number of the
    /// control node with the AS number of the tenant VM.
    /// +optional
    #[prost(bool, optional, tag = "4")]
    pub as_override: ::core::option::Option<bool>,
    /// A non-zero hold-time overrides the hold-time inherited from the
    /// bgp-router configuration. BGP hold time in seconds \[0-65535\],
    /// Max time to detect liveliness of peer.
    /// +optional
    #[prost(int32, optional, tag = "5")]
    pub hold_time: ::core::option::Option<i32>,
    /// For routing loop detection, loop-count is the number of times
    /// the local AS is allowed in the AS_PATH attribute.
    /// +optional
    #[prost(int32, optional, tag = "6")]
    pub loop_count: ::core::option::Option<i32>,
    /// Local autonomous system number used for this particular session.
    /// If configured, this overrides autonomous-system number and
    /// local-autonomous-system number configured under BgpRouterParams
    /// +optional
    #[prost(int32, optional, tag = "7")]
    pub local_autonomous_system: ::core::option::Option<i32>,
    /// BGP address families supported on this session. If not specified
    /// these address families are enabled:
    /// "inet, inet-labeled, inet-vpn, e-vpn, erm-vpn, route-target, inet6,
    /// inet-mvpn, inet6-vpn"
    /// +optional
    #[prost(message, optional, tag = "8")]
    pub address_families: ::core::option::Option<AddressFamilies>,
    /// Authentication related configuration for this session like type, keys.
    /// Only md5 authentication is supported.
    /// +optional
    #[prost(message, optional, tag = "9")]
    pub auth_data: ::core::option::Option<AuthenticationData>,
    /// Session attributes over ride per BGP address family. Attributes like address family, loop-count and prefix-limit.
    /// +optional
    #[prost(message, repeated, tag = "10")]
    pub family_attributes: ::prost::alloc::vec::Vec<BgpFamilyAttributes>,
    /// Remove or replace private ASes from AS Path attributes advertised to this session.
    /// +optional
    #[prost(string, optional, tag = "11")]
    pub private_as_action: ::core::option::Option<::prost::alloc::string::String>,
    /// User defined route origin value to override
    /// +optional
    #[prost(message, optional, tag = "12")]
    pub route_origin_override: ::core::option::Option<RouteOriginOverride>,
}
/// BGPSessionIPAttributes contains BGPSession primary and secondary IP addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgpSessionIpAttributes {
    /// BGPaaSPrimaryIP defines the primary IP address used for a BGP session.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub bgpaas_primary_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// BGPaaSSecondaryIP defines the secondary IP address used for a BGP session
    /// when a second control node is present.
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub bgpaas_secondary_ip: ::core::option::Option<::prost::alloc::string::String>,
}
/// CommonSpec contains Contrail resource fields all types must implement in their spec.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonSpec {
    #[prost(message, optional, tag = "1")]
    pub contrail_fq_name: ::core::option::Option<ContrailFqName>,
}
/// CommonStatus contains Contrail resource fields all types must implement in their status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonStatus {
    #[prost(message, optional, tag = "1")]
    pub reconciler_state: ::core::option::Option<ReconcilerState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommunityAttributes {
    #[prost(string, repeated, tag = "1")]
    pub community_attribute: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ContrailFqName contains the specific FqName field necessary for the Contrail
/// Control-node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContrailFqName {
    /// FqName is the list of resource names that fully qualify a Contrail resource.
    /// +optional
    #[prost(string, repeated, tag = "1")]
    pub fq_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ContrailSecurityPolicy contains data for namespace scope contrail security object.
/// +k8s:openapi-gen=true
/// +resource:path=contrailsecuritypolicies,strategy=ContrailSecurityPolicyStrategy,shortname=csp,categories=contrail;security
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContrailSecurityPolicy {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the ContrailSecurityPolicy.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<ContrailSecurityPolicySpec>,
    /// The most recently observed status of the ContrailSecurityPolicy.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<ContrailSecurityPolicyStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContrailSecurityPolicyEndPoint {
    #[prost(message, repeated, tag = "1")]
    pub end_points: ::prost::alloc::vec::Vec<ContrailSecurityPolicySelector>,
}
/// ContrailSecurityPolicyList is a list of ContrailSecurityPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContrailSecurityPolicyList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the ContrailSecurityPolicy instances in the ContrailSecurityPolicyList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<ContrailSecurityPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContrailSecurityPolicyPodSelector {
    /// This is a label selector which selects Pods. This field follows standard label
    /// selector semantics; if present but empty, it selects all pods.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub pod_selector: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::LabelSelector,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContrailSecurityPolicyRule {
    #[prost(message, optional, tag = "1")]
    pub src_ep: ::core::option::Option<ContrailSecurityPolicyEndPoint>,
    #[prost(message, optional, tag = "2")]
    pub dst_ep: ::core::option::Option<ContrailSecurityPolicyEndPoint>,
    #[prost(message, repeated, tag = "3")]
    pub ports: ::prost::alloc::vec::Vec<
        super::super::super::super::super::super::super::super::super::super::k8s::io::api::networking::v1::NetworkPolicyPort,
    >,
    /// This field selects secondaty action .
    #[prost(message, optional, tag = "4")]
    pub secondary_actions: ::core::option::Option<SecondaryActionList>,
}
/// ContrailSecurityPolicySelector describes endpoint for contrail security policy.
/// A endpoint could be LabelSelector or an IpBlock. Only certain combinations of
/// fields are allowed
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContrailSecurityPolicySelector {
    /// This is a label selector which selects Pods. This field follows standard label
    /// selector semantics; if present but empty, it selects all pods.
    ///
    /// If NamespaceSelector is also set, then the ContrailSecurityPolicySelector as a whole selects
    /// the Pods matching PodSelector in the Namespaces selected by NamespaceSelector.
    /// Otherwise it selects the Pods matching PodSelector in the policy's own Namespace.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub pod_selector: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::LabelSelector,
    >,
    /// Selects Namespaces using cluster-scoped labels. This field follows standard label
    /// selector semantics; if present but empty, it selects all namespaces.
    ///
    /// If PodSelector is also set, then the ContrailSecurityPolicySelector as a whole selects
    /// the Pods matching PodSelector in the Namespaces selected by NamespaceSelector.
    /// Otherwise it selects all Pods in the Namespaces selected by NamespaceSelector.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub namespace_selector: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::LabelSelector,
    >,
    /// IPBlock defines policy on a particular IPBlock. If this field is set then
    /// neither of the other fields can be.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub ip_block: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::api::networking::v1::IpBlock,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContrailSecurityPolicySpec {
    /// Selectors is a list of Podselector and NamespaceSelector on ContrailSecurityPolicy
    #[prost(message, repeated, tag = "2")]
    pub selectors: ::prost::alloc::vec::Vec<ContrailSecurityPolicyPodSelector>,
    /// Endpoint
    #[prost(message, repeated, tag = "3")]
    pub rules: ::prost::alloc::vec::Vec<ContrailSecurityPolicyRule>,
    /// Action
    #[prost(string, optional, tag = "4")]
    pub action: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub secondary_actions: ::core::option::Option<SecondaryActionList>,
}
/// ContrailSecurityPolicySpec defines the observed state of ContrailSecurityPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContrailSecurityPolicyStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
/// EncapsulationPriorities is an ordered list of encapsulation types to be
/// used in priority by the vrouter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncapsulationPriorities {
    /// Encapsulation is an ordered list of encapsulation types to be used in
    /// priority by the vrouter. Valid encapsulation types include MPLSoGRE, MPLSoUDP,
    /// and VXLAN.
    #[prost(string, repeated, tag = "1")]
    pub encapsulation: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// FastConvergenceParametersType contains fast convergence configuration parameters like
/// xmpp hold time and nh reachability check
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FastConvergenceParametersType {
    #[prost(bool, optional, tag = "1")]
    pub enable: ::core::option::Option<bool>,
    /// Enable next-hop reachability checks in control plane for routes in underlay for faster convergence
    #[prost(bool, optional, tag = "2")]
    pub nh_reachability_check: ::core::option::Option<bool>,
    /// The negotiated XMPP hold-time (in seconds) for sessions between the control and data plane
    /// Default value is 90. Range 1~90
    #[prost(int32, optional, tag = "3")]
    pub xmpp_hold_time: ::core::option::Option<i32>,
}
/// FirewallActionListType defines types of actions performed by FirewallRule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallActionListType {
    /// SimpleAction defines allow(i.e. pass) or deny action for traffic matching this FirewallRule.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub simple_action: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub mirror_to: ::core::option::Option<FirewallMirrorActionType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallMatchEprList {
    #[prost(message, repeated, tag = "1")]
    pub expr_list: ::prost::alloc::vec::Vec<FirewallMatchExpr>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallMatchExpr {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub oper: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, packed = "false", tag = "4")]
    pub values_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "7")]
    pub except_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, packed = "false", tag = "6")]
    pub except_ids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallMirrorActionType {
    #[prost(string, optional, tag = "1")]
    pub analyzer_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub encapsulation: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub analyzer_ip_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub analyzer_mac_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub routing_instance: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "6")]
    pub udp_port: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "7")]
    pub juniper_header: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "8")]
    pub nh_mode: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "9")]
    pub static_nh_header: ::core::option::Option<StaticNhType>,
    #[prost(bool, optional, tag = "10")]
    pub nic_assisted_mirroring: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "11")]
    pub nic_assisted_mirroring_vlan: ::core::option::Option<i32>,
}
/// FirewallPolicy contains references to ordered FirewallRule objects.
/// +k8s:openapi-gen=true
/// +resource:path=firewallpolicies,strategy=FirewallPolicyStrategy,shortname=fp,categories=contrail;security
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallPolicy {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the FirewallPolicy.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<FirewallPolicySpec>,
    /// The most recently observed status of the FirewallPolicy.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<FirewallPolicyStatus>,
}
/// FirewallPolicyAttribute defines the actual Sequence number of a FirewallRule referenced by a FirewallPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallPolicyAttribute {
    /// Sequence defines the position of a referenced FirewallRule within the evaluation order of a FirewallPolicy.
    /// FirewallRules are evaluated in descending order.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub sequence: ::core::option::Option<::prost::alloc::string::String>,
}
/// FirewallPolicyList is a list of FirewallPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallPolicyList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the FirewallPolicy instances in the FirewallPolicyList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<FirewallPolicy>,
}
/// FirewallPolicyReference is a ResourceReference to a FirewallPolicy with APSAttributes containing the Sequence number.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallPolicyReference {
    /// ResourceReference to FirewallPolicy from ApplicationPolicySet.
    #[prost(message, optional, tag = "1")]
    pub resource_reference: ::core::option::Option<ResourceReference>,
    /// Attribute defines the sequence number of a FirewallPolicy in [a given|the] ApplicationPolicySet.
    #[prost(message, optional, tag = "2")]
    pub attributes: ::core::option::Option<ApsAttribute>,
}
/// FirewallPolicySpec defines the desired state of FirewallPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallPolicySpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// FirewallRuleReferences contains references to FirewallRule instances attached to this FirewallPolicy.
    /// +optional
    #[prost(message, repeated, tag = "2")]
    pub firewall_rule: ::prost::alloc::vec::Vec<FirewallRuleReference>,
    /// MatchExpr defines an endpoint type which provides kubernates style match expression
    /// for labels/tags.
    #[prost(message, optional, tag = "3")]
    pub vmi_selector: ::core::option::Option<FirewallPolicyVmiSelector>,
}
/// FirewallPolicyStatus defines the observed state of FirewallPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallPolicyStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallPolicyVmiSelector {
    #[prost(message, repeated, tag = "1")]
    pub match_expr_list: ::prost::alloc::vec::Vec<FirewallMatchEprList>,
}
/// FirewallRule defines how traffic is allowed or blocked based on rules
/// with help of various match and action fields.
/// +k8s:openapi-gen=true
/// +resource:path=firewallrules,strategy=FirewallRuleStrategy,shortname=fr,categories=contrail;security
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallRule {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the FirewallRule.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<FirewallRuleSpec>,
    /// The most recently observed status of the FirewallRule.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<FirewallRuleStatus>,
}
/// FirewallRuleEndpointType defines the EndpointType.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallRuleEndpointType {
    /// Subnet defines the IP prefix and length.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub subnet: ::core::option::Option<FirewallSubnet>,
    /// Addressgroup defines what CIDR FirewallRule can be applied on.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub address_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Tags define tags for the FirewallRule.
    /// +optional
    #[prost(string, repeated, tag = "4")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// TagIds define IDs created for referred Tags.
    /// +optional
    #[prost(int64, repeated, packed = "false", tag = "5")]
    pub tag_ids: ::prost::alloc::vec::Vec<i64>,
    /// Any defines matching to "any" value in an FirewallRuleEndpointType . ie "*"
    /// +optional
    #[prost(bool, optional, tag = "6")]
    pub any: ::core::option::Option<bool>,
    /// MatchExpr defines an endpoint type which provides kubernates style match expression
    /// for labels/tags.
    #[prost(message, repeated, tag = "7")]
    pub match_expr: ::prost::alloc::vec::Vec<FirewallMatchExpr>,
    #[prost(message, repeated, tag = "8")]
    pub match_expr_list: ::prost::alloc::vec::Vec<FirewallMatchEprList>,
}
/// FirewallRuleList is a list of FirewallRule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallRuleList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the FirewallRule instances in the FirewallRuleList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<FirewallRule>,
}
/// FirewallRuleReference is a ResourceReference to a FirewallRule with a FirewallPolicyAttribute containing the Sequence Number.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallRuleReference {
    /// ResourceReference to FirewallRule from FirewallPolicy.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub resource_reference: ::core::option::Option<ResourceReference>,
    /// Attribute defines the Sequence Number of a FirewallRule in [a given|the] FirewallPolicy.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub attributes: ::core::option::Option<FirewallPolicyAttribute>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallRuleSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// ServiceGroupReference refers to an ServiceGroup defining the list of port/proto of a FirewallRule.
    /// +optional
    #[prost(message, optional, tag = "11")]
    pub service_group_reference: ::core::option::Option<ResourceReference>,
    /// AddressGroupReference refers to an AddressGroup defining the CIDR of a FirewallRule.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub address_group_reference: ::core::option::Option<ResourceReference>,
    /// ActionList defines actions to be performed if packets match condition.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub action_list: ::core::option::Option<FirewallActionListType>,
    /// Service defines the service (port, protocol) for packets match condition.
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub service: ::core::option::Option<FirewallServiceType>,
    /// Endpoint1 defines match condition for source traffic.
    /// +optional
    #[prost(message, optional, tag = "5")]
    pub endpoint1: ::core::option::Option<FirewallRuleEndpointType>,
    /// Endpoint2 defines match condition for destination traffic.
    /// +optional
    #[prost(message, optional, tag = "6")]
    pub endpoint2: ::core::option::Option<FirewallRuleEndpointType>,
    /// MatchTags defines matching tags for source and destination endpoints.
    /// +optional
    #[prost(string, repeated, tag = "7")]
    pub match_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// MatchTagsTypes defines matching tags ids for source and destination endpoints.
    /// +optional
    #[prost(int64, repeated, packed = "false", tag = "8")]
    pub match_tags_types: ::prost::alloc::vec::Vec<i64>,
    /// Direction defines direction in the FirewallRule.
    /// +optional
    #[prost(string, optional, tag = "9")]
    pub direction: ::core::option::Option<::prost::alloc::string::String>,
    /// TagReferences refers to Tags associated with the FirewallRule.
    /// +optional
    #[prost(message, repeated, tag = "10")]
    pub tag_references: ::prost::alloc::vec::Vec<ResourceReference>,
    #[prost(bool, optional, tag = "12")]
    pub k8s_mode: ::core::option::Option<bool>,
}
/// FirewallRuleStatus defines the observed state of FirewallRule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallRuleStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
/// FirewallServiceGroupType describes the list of proto:port configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallServiceGroupType {
    /// List of protocol:port definitions
    #[prost(message, repeated, tag = "1")]
    pub firewall_service: ::prost::alloc::vec::Vec<FirewallServiceType>,
}
/// FirewallServiceType defines the Port related info.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallServiceType {
    /// Protocol defines Layer 4 protocol in IP packet.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub protocol: ::core::option::Option<::prost::alloc::string::String>,
    /// ProtocolId defines Layer 4 protocol ID in IP packet.
    /// +optional
    #[prost(int64, optional, tag = "2")]
    pub protocol_id: ::core::option::Option<i64>,
    /// SourcePorts defines the range of source port numbers for Layer 4 Protocol.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub source_ports: ::core::option::Option<PortType>,
    /// DestinationPorts defines the range of destination port numbers for Layer 4 Protocol.
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub destination_ports: ::core::option::Option<PortType>,
}
/// FirewallSubnet defines the IP prefix and length.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallSubnet {
    /// Network prefix
    #[prost(string, optional, tag = "1")]
    pub ip_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Network prefix length
    #[prost(message, optional, tag = "2")]
    pub ip_prefix_len: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::util::intstr::IntOrString,
    >,
}
/// FloatingIP defines a floating IP address.
/// +k8s:openapi-gen=true
/// +resource:path=floatingips,strategy=FloatingIPStrategy,shortname=fip,categories=contrail;ipam;networking
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatingIp {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the FloatingIP.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<FloatingIpSpec>,
    /// The most recently observed status of the FloatingIP.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<FloatingIpStatus>,
}
/// FloatingIPList is a list of FloatingIP.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatingIpList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the FloatingIP instances in the FloatingIPList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<FloatingIp>,
}
/// FloatingIPPortMappings is the list port mappings associated with a given FloatingIP.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatingIpPortMappings {
    /// PortMappings is a list of FloatingIPPortPortMapping instances.
    #[prost(message, repeated, tag = "1")]
    pub port_mappings: ::prost::alloc::vec::Vec<FloatingIpPortPortMapping>,
}
/// FloatingIPPortPortMapping indicates ports exposed by the service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatingIpPortPortMapping {
    /// Source Port
    /// +optional
    #[prost(int32, optional, tag = "1")]
    pub src_port: ::core::option::Option<i32>,
    /// Destination Port
    /// +optional
    #[prost(int32, optional, tag = "2")]
    pub dst_port: ::core::option::Option<i32>,
    /// Network protocol
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub protocol: ::core::option::Option<::prost::alloc::string::String>,
}
/// FloatingIPSpec defines the FloatingIP parameters, used for natting pod IPAddress.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatingIpSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// IP address value for floating IP.
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub floating_ip_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub floating_ip_port_mappings: ::core::option::Option<FloatingIpPortMappings>,
    /// Defines traffic flow direction,(ingress,egress or both),default = both
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub floating_ip_traffic_direction: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    /// Parent refers to the InstanceIP associated with the FloatingIP.
    /// +optional
    #[prost(message, optional, tag = "5")]
    pub parent: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::api::core::v1::ObjectReference,
    >,
    /// VirtualMachineInterfaceReferences determines the VirtualMachineInterface
    /// of the VirtualMachine associated with this floating IP.
    /// +optional
    #[prost(message, repeated, tag = "7")]
    pub virtual_machine_interface_references: ::prost::alloc::vec::Vec<
        ResourceReference,
    >,
    /// FloatingIPEnablePortMapping controls which ports FloatingIP NAT is applied to.
    /// If false, FloatingIP NAT is performed for all Ports.
    /// If true, FloatingIP NAT is limited to the defined list of PortMaps.
    #[prost(bool, optional, tag = "8")]
    pub floating_ip_port_mappings_enable: ::core::option::Option<bool>,
}
/// FloatingIPStatus defines the observed state of the FloatingIP.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatingIpStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
    /// SubnetReference the floating IP address belongs to.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub subnet_reference: ::core::option::Option<ResourceReference>,
}
/// GlobalContrailSecurityPolicy contains data for global contrail security policy object.
/// +k8s:openapi-gen=true
/// +resource:path=globalcontrailsecuritypolicies,strategy=GlobalContrailSecurityPolicyStrategy,shortname=gcsp,categories=contrail;security
/// +genclient:nonNamespaced
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalContrailSecurityPolicy {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the GlobalContrailSecurityPolicy.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "5")]
    pub spec: ::core::option::Option<GlobalContrailSecurityPolicySpec>,
    /// The most recently observed status of the GlobalContrailSecurityPolicy.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<GlobalContrailSecurityPolicyStatus>,
}
/// GlobalContrailSecurityPolicyList is a list of GlobalContrailSecurityPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalContrailSecurityPolicyList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the GlobalContrailSecurityPolicy instances in the GlobalContrailSecurityPolicyList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<GlobalContrailSecurityPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalContrailSecurityPolicyPodSelector {
    /// This is a label selector which selects Pods. This field follows standard label
    /// selector semantics; if present but empty, it selects all pods.
    ///
    /// If NamespaceSelector is also set, then the ContrailSecurityPolicySelector as a whole selects
    /// the Pods matching PodSelector in the Namespaces selected by NamespaceSelector.
    /// Otherwise it selects the Pods matching PodSelector in the policy's own Namespace.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub pod_selector: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::LabelSelector,
    >,
    /// Selects Namespaces using cluster-scoped labels. This field follows standard label
    /// selector semantics; if present but empty, it selects all namespaces.
    ///
    /// If PodSelector is also set, then the ContrailSecurityPolicySelector as a whole selects
    /// the Pods matching PodSelector in the Namespaces selected by NamespaceSelector.
    /// Otherwise it selects all Pods in the Namespaces selected by NamespaceSelector.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub namespace_selector: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::LabelSelector,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalContrailSecurityPolicySpec {
    /// Selector for Pod and namespace
    #[prost(message, repeated, tag = "1")]
    pub selectors: ::prost::alloc::vec::Vec<GlobalContrailSecurityPolicyPodSelector>,
    /// Rules
    #[prost(message, repeated, tag = "2")]
    pub rules: ::prost::alloc::vec::Vec<ContrailSecurityPolicyRule>,
    /// Action
    #[prost(string, optional, tag = "3")]
    pub action: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub secondary_actions: ::core::option::Option<SecondaryActionList>,
}
/// GlobalContrailSecurityPolicySpec defines the observed state of GlobalContrailSecurityPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalContrailSecurityPolicyStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
/// GlobalSystemConfig defines all the global Contrail configurations. This object
/// must be unique for a Contrail deployment with the name
/// 'default-global-system-config'.
/// +k8s:openapi-gen=true
/// +resource:path=globalsystemconfigs,strategy=GlobalSystemConfigStrategy,shortname=gsc,categories=contrail;contrail-cluster
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalSystemConfig {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the GlobalSystemConfig.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<GlobalSystemConfigSpec>,
    /// The most recently observed status of the GlobalSystemConfig.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<GlobalSystemConfigStatus>,
}
/// GlobalSystemConfigList is a list of GlobalSystemConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalSystemConfigList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the GlobalSystemConfig instances in the GlobalSystemConfigList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<GlobalSystemConfig>,
}
/// GlobalSystemConfigSpec defines the desired state of the GlobalSystemConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalSystemConfigSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// Enable4bytesAS enables 32-bit Autonomous System number support when true.
    /// When false (the default), ASN is 16-bit.
    /// +optional
    #[prost(bool, optional, tag = "2")]
    pub enable4bytes_as: ::core::option::Option<bool>,
    /// AutonomousSystem number for the cluster. By default, this number is 16-bits and has
    /// a maximum value of 65535 unless Enable4bytesAS is true, in which case the maximum
    /// value is 4294967295.
    /// +optional
    #[prost(int32, optional, tag = "3")]
    pub autonomous_system: ::core::option::Option<i32>,
    /// BGPRouterReferences is the list of references to all BGPRouter instances
    /// in the cluster.
    /// +optional
    #[prost(message, repeated, tag = "4")]
    pub bgp_router_references: ::prost::alloc::vec::Vec<ResourceReference>,
    /// IBGPAutoMesh will automatically create an iBGP mesh if set to true (default).
    /// +optional
    #[prost(bool, optional, tag = "5")]
    pub ibgp_auto_mesh: ::core::option::Option<bool>,
    /// DefaultEnableSNAT will enable FabricSNAT by default on all VNs if true.
    /// VirtualNetworkSpec.FabricSNAT will override this value.
    /// +optional
    #[prost(bool, optional, tag = "6")]
    pub default_enable_snat: ::core::option::Option<bool>,
    /// GracefulRestartParameters lets enable the GR/LLGR feature for bgp_helper and xmpp_helper. We need to enable the feature
    /// overall to be able to enable the bgp_helper or xmpp_helper or both.
    /// +optional
    #[prost(message, optional, tag = "7")]
    pub graceful_restart_parameters: ::core::option::Option<
        GracefulRestartParametersType,
    >,
    /// FastConvergenceParameters is an enable/disable knob for all Fast-Convergence parameters to take effect
    /// +optional
    #[prost(message, optional, tag = "8")]
    pub fast_convergence_parameters: ::core::option::Option<
        FastConvergenceParametersType,
    >,
}
/// GlobalSystemConfigStatus defines the observed state of the GlobalSystemConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalSystemConfigStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
/// GlobalVrouterConfig defines all the vrouter Contrail configurations. This
/// object must be unique for a Contrail deployment with the name
/// 'default-global-vrouter-config'.
/// +k8s:openapi-gen=true
/// +resource:path=globalvrouterconfigs,strategy=GlobalVrouterConfigStrategy,shortname=gvc,categories=contrail;contrail-cluster
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalVrouterConfig {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the GlobalVrouterConfig.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<GlobalVrouterConfigSpec>,
    /// The most recently observed status of the GlobalVrouterConfig.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<GlobalVrouterConfigStatus>,
}
/// GlobalVrouterConfigList is a list of GlobalVrouterConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalVrouterConfigList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the GlobalVrouterConfig instances in the GlobalVrouterConfigList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<GlobalVrouterConfig>,
}
/// GlobalVrouterConfigSpec defines the desired state of GlobalVrouterConfig
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalVrouterConfigSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// Parent contains the ObjectReference to the parent GlobalSystemConfig.
    #[prost(message, optional, tag = "2")]
    pub parent: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::api::core::v1::ObjectReference,
    >,
    /// EncapsulationPriorities is an ordered list of encapsulation types to be
    /// used in priority by the vrouter. Valid encapsulation types include MPLSoGRE,
    /// MPLSoUDP, and VXLAN.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub encapsulation_priorities: ::core::option::Option<EncapsulationPriorities>,
    #[prost(message, optional, tag = "4")]
    pub linklocal_services: ::core::option::Option<LinklocalServices>,
    /// PortTranslationPools contains the defined SNAT port translation pools.
    /// +optional
    #[prost(message, optional, tag = "5")]
    pub port_translation_pools: ::core::option::Option<PortTranslationPools>,
    /// FlowExportRate is the rate at which each vrouter will sample and export
    /// flow records to analytics.
    /// +optional
    #[prost(int32, optional, tag = "6")]
    pub flow_export_rate: ::core::option::Option<i32>,
}
/// GlobalVrouterConfigStatus defines the observed state of GlobalVrouterConfig
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalVrouterConfigStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GracefulRestartParametersType {
    #[prost(bool, optional, tag = "1")]
    pub bgp_helper_enable: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub enable: ::core::option::Option<bool>,
    /// EndofRibTimeout value. Default value of 90 seconds
    #[prost(int32, optional, tag = "3")]
    pub end_of_rib_timeout: ::core::option::Option<i32>,
    /// LongLivedRestartTime value. Default value of 1800 seconds
    #[prost(int32, optional, tag = "4")]
    pub long_lived_restart_time: ::core::option::Option<i32>,
    /// GracefulRestartTime value. Default value of 60 seconds
    #[prost(int32, optional, tag = "5")]
    pub restart_time: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "6")]
    pub xmpp_helper_enable: ::core::option::Option<bool>,
}
/// IPRange specifies the start and end for a range of IP addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpRange {
    /// From indicates beginning IP address for the allocation range.
    #[prost(string, optional, tag = "1")]
    pub from: ::core::option::Option<::prost::alloc::string::String>,
    /// To indicates last IP address for the allocation range.
    #[prost(string, optional, tag = "2")]
    pub to: ::core::option::Option<::prost::alloc::string::String>,
}
/// ImportVirtualNetworkRouter is a list of other VirtualNetworkRouters imported
/// by the current VirtualNetworkRouter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportVirtualNetworkRouter {
    /// VirtualNetworkRouters is a list of VirtualNetworkRouterEntry.
    #[prost(message, repeated, tag = "1")]
    pub virtual_network_routers: ::prost::alloc::vec::Vec<VirtualNetworkRouterEntry>,
}
/// InstanceIP represents an IP address and its configuration used for interfaces.
/// +k8s:openapi-gen=true
/// +resource:path=instanceips,strategy=InstanceIPStrategy,shortname=iip,categories=contrail;ipam;networking
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceIp {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the InstanceIP.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<InstanceIpSpec>,
    /// The most recently observed status of the InstanceIP.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<InstanceIpStatus>,
}
/// InstanceIPList is a list of InstanceIP.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceIpList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the InstanceIP instances in the InstanceIPList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<InstanceIp>,
}
/// InstanceIPSpec defines the desired state of the InstanceIP.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceIpSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// IP address value for InstanceIP.
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub instance_ip_address: ::core::option::Option<::prost::alloc::string::String>,
    /// IP address family for the InstanceIP: "v4" or "v6" for IPv4 or IPv6.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub instance_ip_family: ::core::option::Option<::prost::alloc::string::String>,
    /// Subnet is the CIDR the InstanceIP belongs to.
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub cidr: ::core::option::Option<::prost::alloc::string::String>,
    /// VirtualNetworkReference determines the VirtualNetwork the InstanceIP belongs to.
    /// +optional
    #[prost(message, optional, tag = "5")]
    pub virtual_network_reference: ::core::option::Option<ResourceReference>,
    /// VirtualMachineInterfaceReferences determines the VirtualMachineInterface
    /// the InstanceIP belongs to.
    /// +optional
    #[prost(message, repeated, tag = "6")]
    pub virtual_machine_interface_references: ::prost::alloc::vec::Vec<
        ResourceReference,
    >,
    /// IPRangeKeys is used to identify the subnet range for IP allocation.
    /// +optional
    #[prost(string, repeated, tag = "7")]
    pub ip_range_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// InstanceIPStatus defines the observed state of the InstanceIP.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceIpStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
    /// SubnetReference refers to the Subnet this InstanceIP belongs to.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub subnet_reference: ::core::option::Option<ResourceReference>,
}
/// Interface route table is mechanism to add static routes pointing to this interface. On successful addition, vRouter
/// adds prefix to VRF of VMI, with next-hop configured to IpAddress of the associated VMI iteself.
/// +k8s:openapi-gen=true
/// +resource:path=interfaceroutetables,strategy=InterfaceRouteTableStrategy,shortname=irt
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceRouteTable {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<InterfaceRouteTableSpec>,
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<InterfaceRouteTableStatus>,
}
/// InterfaceRouteTableList is a list of InterfaceRouteTable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceRouteTableList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the InterfaceRouteTable instances in the InterfaceRouteTableList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<InterfaceRouteTable>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceRouteTableSpec {
    #[prost(message, optional, tag = "9")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// Route information such as: Prefix to be associated with VMI object, on successful reconciliation, the prefix
    /// is added VRF of the VMI, vRouter will populate next-hop for prefix with Ip address of VMI iteself
    #[prost(message, optional, tag = "2")]
    pub interface_route_table_routes: ::core::option::Option<InterfaceRouteTableType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceRouteTableStatus {
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceRouteTableType {
    #[prost(message, repeated, tag = "1")]
    pub route: ::prost::alloc::vec::Vec<InterfaceRouteType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceRouteType {
    /// CommunityAttributes bgp community attributes for the route entry in VRF
    #[prost(message, optional, tag = "1")]
    pub community_attributes: ::core::option::Option<CommunityAttributes>,
    /// Prefix CIDR value for routing, when associated VMI, there will matching entry in VRF of the VMI, with next-hop
    /// configured to same VMI that is associated
    #[prost(string, optional, tag = "4")]
    pub prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// NextHopType will be "ip-address"
    #[prost(string, optional, tag = "3")]
    pub next_hop_type: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAddresses {
    #[prost(string, repeated, tag = "1")]
    pub target_ip_address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// KeyValuePair is attribute associated with VirtualNetwork
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValuePair {
    #[prost(string, optional, tag = "1")]
    pub virtual_network_virtual_network_router_name: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
/// LinklocalServiceEntryType specifies parameters for configurable LinkLocalServices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinklocalServiceEntryType {
    /// DNS name to which link local service will be proxied.
    #[prost(string, optional, tag = "1")]
    pub ip_fabric_dns_service_name: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    /// Destination ip address to which link local traffic will forwarded.
    #[prost(string, repeated, tag = "2")]
    pub ip_fabric_service_ip: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Destination TCP port number to which link local traffic will forwarded.
    #[prost(int32, optional, tag = "3")]
    pub ip_fabric_service_port: ::core::option::Option<i32>,
    /// ip address of the link local service.
    #[prost(string, optional, tag = "4")]
    pub linklocal_service_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the link local service. VM name resolution of this name will result in
    ///   link local ip address
    #[prost(string, optional, tag = "5")]
    pub linklocal_service_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Destination TCP port number of link local service
    #[prost(int32, optional, tag = "6")]
    pub linklocal_service_port: ::core::option::Option<i32>,
}
/// LinklocalServices is list of LinkLocalServices
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinklocalServices {
    #[prost(message, repeated, tag = "1")]
    pub linklocal_service_entry: ::prost::alloc::vec::Vec<LinklocalServiceEntryType>,
}
/// MACAddresses is a list of MACAddress
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MacAddresses {
    /// MACAddresses is list of unique identifier assigned to network interface.
    #[prost(string, repeated, tag = "1")]
    pub mac_address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MirrorDestination defines parameters associated with mirrored traffic reciever.
/// +k8s:openapi-gen=true
/// +resource:path=mirrordestinations,strategy=MirrorDestinationStrategy,shortname=md
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MirrorDestination {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<MirrorDestinationSpec>,
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<MirrorDestinationStatus>,
}
/// MirrorDestinationList is a list of MirrorDestination.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MirrorDestinationList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the MirrorDestination instances in the MirrorDestinationList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<MirrorDestination>,
}
/// MirrorDestinationSpec defines the parameters associated with mirrored traffic reciever
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MirrorDestinationSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "9")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// TrafficDirection indicates ingress, egress or both direction traffic to be mirrored
    /// default is both
    #[prost(string, optional, tag = "2")]
    pub traffic_destination: ::core::option::Option<::prost::alloc::string::String>,
    /// UdpPort specifies the destination udp port for mirrored traffic ip header
    #[prost(int32, optional, tag = "3")]
    pub udp_port: ::core::option::Option<i32>,
    /// JuniperHeader indicates adding juniper header to the mirrored traffic.
    /// default is true, indicating adding juniper header
    #[prost(bool, optional, tag = "4")]
    pub juniper_header: ::core::option::Option<bool>,
    /// NextHopMode indicates using static or dynamic nexthop for mirrored traffic
    /// default is dynamic nexthop
    #[prost(string, optional, tag = "5")]
    pub next_hop_mode: ::core::option::Option<::prost::alloc::string::String>,
    /// StaticNextHopHeader specifies vxlan parameters for static nexthop used to forward to the mirror destination
    /// StaticNextHopHeader is required when NextHopMode is set to static
    #[prost(message, optional, tag = "6")]
    pub static_next_hop_header: ::core::option::Option<NhHeaderType>,
    /// NicAssistedMirroring indictaes to enable NicAssistedMirroring
    /// default is false
    #[prost(bool, optional, tag = "7")]
    pub nic_assisted_mirroring: ::core::option::Option<bool>,
    /// NicAssistedVlanID indicates vlanID when NicAssistedMirroring is enabled
    #[prost(int32, optional, tag = "8")]
    pub nic_assisted_vlan_id: ::core::option::Option<i32>,
    /// ExternalAnalyzer indicates to use external analyzer
    #[prost(bool, optional, tag = "10")]
    pub external_analyzer: ::core::option::Option<bool>,
    /// AnalyzerIP indicates the external analyzer IP address
    #[prost(string, optional, tag = "11")]
    pub analyzer_ip: ::core::option::Option<::prost::alloc::string::String>,
}
/// MirrorDestinationStatus defines the observed state of MirrorDestination
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MirrorDestinationStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
    /// VirtualMachineInterfaceReference is VirtualMachineInterface used to mirror
    /// traffic to when using this MirrorDestination
    #[prost(message, optional, tag = "2")]
    pub virtual_machine_interface_reference: ::core::option::Option<ResourceReference>,
    /// InstanceIPReference is used to infer destination IPAdrress for mirroring traffic
    /// when using this MirrorDestination
    #[prost(message, optional, tag = "3")]
    pub instance_ip_reference: ::core::option::Option<ResourceReference>,
}
/// NHHeaderType specifies vxlan forwarding parameters for forwarding mirrored
/// traffic towards the mirror destination
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NhHeaderType {
    /// VTEPDestinationIP indicates vtep tunnel ipaddress
    #[prost(string, optional, tag = "1")]
    pub v_tep_destination_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// VTEPDestinationMac indicates vtep tunnel destination mac address
    #[prost(string, optional, tag = "2")]
    pub v_tep_destination_mac: ::core::option::Option<::prost::alloc::string::String>,
    /// VxlanID indicates the vxlan identifier
    #[prost(int32, optional, tag = "3")]
    pub vxlan_id: ::core::option::Option<i32>,
}
/// PolicyBasedForwardingRule is the automatically generated Forwarding policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyBasedForwardingRule {
    /// Direction specifies traffic direction allowed for PolicyBasedForwardingRule.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub direction: ::core::option::Option<::prost::alloc::string::String>,
}
/// PortRange encapsulates the start and end of a range of IP ports.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortRange {
    /// StartPort represents the starting port number.
    /// It must be less than or equal to EndPort.
    #[prost(int32, optional, tag = "1")]
    pub start_port: ::core::option::Option<i32>,
    /// EndPort represents the last allocatable port number.
    /// It must be greater than or equal to StartPort.
    #[prost(int32, optional, tag = "2")]
    pub end_port: ::core::option::Option<i32>,
}
/// PortTranslationPool represents a range or quantity of available ports for a
/// given protocol. Protocol is required. PortRange is optional defaults to nil.
/// PortCount is optional defaults to 0. Only one of PortRange and PortCount may
/// be set for the PortTranslationPool to be valid.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortTranslationPool {
    /// Protocol specifies the protocol this pools is for.
    #[prost(string, optional, tag = "1")]
    pub protocol: ::core::option::Option<::prost::alloc::string::String>,
    /// PortRange defines the range from which port numbers may be allocated.
    /// If PortRange is set, PortCount must be 0.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub port_range: ::core::option::Option<PortRange>,
    /// PortCount defines the maximum amount of port numbers to be allocated.
    /// If PortCount is greater than 0, PortRange must be empty.
    /// +optional
    #[prost(int32, optional, tag = "3")]
    pub port_count: ::core::option::Option<i32>,
}
/// PortTranslationPools represents a collection of PortTranslationPool
/// configurations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortTranslationPools {
    /// Pools is the list of PortTranslationPool instances.
    /// +optional
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<PortTranslationPool>,
}
/// PortType defines the Port number.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortType {
    /// StartPort defines the starting port number of the FirewallServiceType.
    /// +optional
    #[prost(int64, optional, tag = "1")]
    pub start_port: ::core::option::Option<i64>,
    /// EndPort defines the ending port number of the FirewallServiceType.
    /// +optional
    #[prost(int64, optional, tag = "2")]
    pub end_port: ::core::option::Option<i64>,
}
/// Range is a list of IPRanges associated with a given key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    /// Key is a text string defining the Range collection. Setting a Range with
    /// an existing key will overwrite the exiting Range.
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// IPRanges lists one or more IPRange instance.
    #[prost(message, repeated, tag = "2")]
    pub ip_ranges: ::prost::alloc::vec::Vec<IpRange>,
}
/// ReconcilerState describes a resource's reconciliation status including the State
/// of the reconciliation as well as an Observation with additional information about
/// the State.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconcilerState {
    /// State describe the current readiness of a resource after the last reconciliation.
    /// The possible states include Pending, Success, and Failure.
    #[prost(string, optional, tag = "1")]
    pub state: ::core::option::Option<::prost::alloc::string::String>,
    /// Observation provides additional information related to the state of the
    /// resource. For example, if a reconciliation error occurs, Observation will
    /// contain a brief description of the problem.
    #[prost(string, optional, tag = "2")]
    pub observation: ::core::option::Option<::prost::alloc::string::String>,
}
/// ResourceReference is an ObjectReference to a Contrail resource that contains
/// the ContrailFqName of the resource being referenced.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceReference {
    #[prost(message, optional, tag = "1")]
    pub object_reference: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::api::core::v1::ObjectReference,
    >,
    #[prost(message, optional, tag = "2")]
    pub contrail_fq_name: ::core::option::Option<ContrailFqName>,
}
/// RouteOriginOverride is used to override route origin attribute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteOriginOverride {
    /// Set true to override Route origin with the given value
    /// +optional
    #[prost(bool, optional, tag = "1")]
    pub origin_override: ::core::option::Option<bool>,
    /// User define route origin value
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub origin: ::core::option::Option<::prost::alloc::string::String>,
}
/// Network route table is mechanism of adding static routes in the virtual network's routing table
/// +k8s:openapi-gen=true
/// +resource:path=routetables,strategy=RouteTableStrategy,shortname=srt
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTable {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<RouteTableSpec>,
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<RouteTableStatus>,
}
/// RouteTableList is a list of RouteTable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTableList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the RouteTable instances in the RouteTableList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<RouteTable>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTableSpec {
    #[prost(message, optional, tag = "9")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// Routes in the route table are configured in following way.
    #[prost(message, optional, tag = "2")]
    pub routes: ::core::option::Option<RouteTableType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTableStatus {
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTableType {
    #[prost(message, repeated, tag = "1")]
    pub route: ::prost::alloc::vec::Vec<RouteType>,
}
/// RouteTarget is a route-target extended community, a type of BGP extended
/// community that used to define VPN membership. The route target appears in a
/// field in route update.
/// +k8s:openapi-gen=true
/// +resource:path=routetargets,strategy=RouteTargetStrategy,shortname=rt,categories=contrail;routing
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTarget {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the RouteTarget.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<RouteTargetSpec>,
    /// The most recently observed status of the RouteTarget.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<RouteTargetStatus>,
}
/// RouteTargetList is a list of RouteTarget.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTargetList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the RouteTarget instances in the RouteTargetList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<RouteTarget>,
}
/// RouteTargetReference contains a RouteTarget reference and the
/// import/export mode in their attributes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTargetReference {
    #[prost(message, optional, tag = "1")]
    pub resource_reference: ::core::option::Option<ResourceReference>,
    /// Attributes contains the ImportExport flag. When ImportExport is blank (the default),
    /// both import and export are supported. Setting ImportExport to "import" enables
    /// import-only mode. Setting it to "export" enables export-only mode.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub attributes: ::core::option::Option<RouteTargetReferenceAttributes>,
}
/// RouteTargetReferenceAttributes allows the configuration of import/export mode.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTargetReferenceAttributes {
    /// ImportExport determines the import/export mode. By default, this field is
    /// empty. When ImportExport is blank, bott import and export are supported.
    /// Setting ImportExport to "import" enables import-only mode. Setting it to
    /// "export" enables export-only mode.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub import_export: ::core::option::Option<::prost::alloc::string::String>,
}
/// RouteTargetSpec defines the desired state of a RouteTarget.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTargetSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
}
/// RouteTargetStatus defines the observed state of a RouteTarget.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTargetStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteType {
    /// CommunityAttributes bgp community attributes for the route entry in VRF
    #[prost(message, optional, tag = "1")]
    pub community_attributes: ::core::option::Option<CommunityAttributes>,
    /// Prefix CIDR value for routing, when associated VMI
    #[prost(string, optional, tag = "4")]
    pub prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// NextHop next-hop for matching prefix, must be another VMI Ip address
    #[prost(string, optional, tag = "2")]
    pub next_hop: ::core::option::Option<::prost::alloc::string::String>,
    /// NextHopType will be "ip-address"
    #[prost(string, optional, tag = "3")]
    pub next_hop_type: ::core::option::Option<::prost::alloc::string::String>,
}
/// RoutingInstance is a group of customer attachment points with the same
/// connectivity policies. Corresponding to the VRF in L3VPN/EVPN.
/// +k8s:openapi-gen=true
/// +resource:path=routinginstances,strategy=RoutingInstanceStrategy,shortname=ri,categories=contrail;routing
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingInstance {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the RoutingInstance.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<RoutingInstanceSpec>,
    /// The most recently observed status of the RoutingInstance.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<RoutingInstanceStatus>,
}
/// RoutingInstanceList is a list of RoutingInstance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingInstanceList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the RoutingInstance instances in the RoutingInstanceList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<RoutingInstance>,
}
/// RoutingInstanceReference contains a RoutingInstance reference and an Attributes
/// instance defining a policy forwarding rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingInstanceReference {
    #[prost(message, optional, tag = "1")]
    pub resource_reference: ::core::option::Option<ResourceReference>,
    /// Attributes contains a policy forwarding rule which specifies the traffic
    /// direction (ingress, egress or both) for the routes to an optimal gateway.
    #[prost(message, optional, tag = "2")]
    pub attributes: ::core::option::Option<PolicyBasedForwardingRule>,
}
/// RoutingInstanceSpec defines the desired state of the RoutingInstance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingInstanceSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// Object reference to the parent VirtualNetwork.
    #[prost(message, optional, tag = "2")]
    pub parent: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::api::core::v1::ObjectReference,
    >,
    /// RouteTargetReferences contains RouteTarget references assigned by the user.
    /// The RoutingInstance's default RouteTarget is defined in RoutingInstanceStatus.
    /// +optional
    #[prost(message, repeated, tag = "3")]
    pub route_target_references: ::prost::alloc::vec::Vec<RouteTargetReference>,
    /// StaticRouteEntries contains user provided Routes to be populated in RI
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub static_route_entries: ::core::option::Option<StaticRouteEntriesType>,
}
/// RoutingInstanceStatus defines the observed state of the RoutingInstance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingInstanceStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
    /// Is this the default routing instance for the VirtualNetwork? This
    /// field contains internal service chaining information, and should not be
    /// modified.
    /// +optional
    #[prost(bool, optional, tag = "2")]
    pub is_default: ::core::option::Option<bool>,
    /// FabricSNAT toggles connectivity to underlay network by port mapping.
    /// +optional
    #[prost(bool, optional, tag = "3")]
    pub routing_instance_fabric_snat: ::core::option::Option<bool>,
    /// DefaultRouteTargetReference contains a reference to the default RouteTarget
    /// and the import/export mode in their attributes.
    /// Only set by the system as user must pass by higher level resources to
    /// add remove Route Target.
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub default_route_target_reference: ::core::option::Option<RouteTargetReference>,
    /// VirtualNetworkRouterRouteTargetReferences are RouteTarget references of VirtualNetworkRouters
    /// selecting this RoutingInstance's parent VirtualNetwork, as well as those of
    /// imported VirtualNetworkRouters.
    /// +optional
    #[prost(map = "string, message", tag = "5")]
    pub virtual_network_router_route_target_references: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        VirtualNetworkRouteTargetReferenceList,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecondaryActionList {
    /// Action
    #[prost(string, optional, tag = "1")]
    pub mirror_to: ::core::option::Option<::prost::alloc::string::String>,
}
/// ServiceGroup describes the list of protocol:port associated with the FirewallRule resource.
/// +k8s:openapi-gen=true
/// +resource:path=servicegroups,strategy=ServiceGroupStrategy,shortname=sg,categories=contrail;security
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceGroup {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the ServiceGroup.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<ServiceGroupSpec>,
    /// The most recently observed status of the ServiceGroup.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<ServiceGroupStatus>,
}
/// ServiceGroupList is a list of ServiceGroup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceGroupList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the ServiceGroup instances in the ServiceGroupList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<ServiceGroup>,
}
/// ServiceGroupSpec describes the set of protocol and port  associated with the FirewallRule resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceGroupSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// ServiceGroupPrefixes defines list of protocol:port used to match a group of workloads.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub service_group_firewall_service_list: ::core::option::Option<
        FirewallServiceGroupType,
    >,
}
/// ServiceGroupStatus defines the observed state of ServiceGroup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceGroupStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
/// +genclient
/// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
/// +k8s:openapi-gen=true
/// +resource:path=servicehealthchecks,strategy=ServiceHealthCheckStrategy,shortname=hc,categories=contrail;networking
/// ServiceHealthCheck is the representation of the ServiceHealthCheck resource.
/// The service instance health check is used to determine the liveliness of a service provided by a Pod, checking whether the service is operationally up or down.
/// The vRouter agent uses ping and an HTTP URL to the link-local address to check the liveliness of the interface.
/// If the health check determines that a service is no longer operational, it removes the routes for the Pod, thereby disabling packet forwarding to the Pod.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceHealthCheck {
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<ServiceHealthCheckSpec>,
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<ServiceHealthCheckStatus>,
}
/// ServiceHealthCheckList is a list of ServiceHealthCheck.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceHealthCheckList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the ServiceHealthCheck instances in the ServiceHealthCheckList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<ServiceHealthCheck>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceHealthCheckProperties {
    /// Delay The delay, in seconds, to repeat the health check.
    #[prost(uint32, optional, tag = "1")]
    pub delay: ::core::option::Option<u32>,
    /// +optional
    /// DelayUSec The delayUsecs, in micro-seconds, to repeat the health check. If Specified total delay is calulated as
    /// delay = Delay + (DelayUSec/1000000)
    #[prost(uint32, optional, tag = "2")]
    pub delay_usecs: ::core::option::Option<u32>,
    /// Enabled Indicates that health check is enabled. The default is False.
    #[prost(bool, optional, tag = "3")]
    pub enabled: ::core::option::Option<bool>,
    /// +optional
    /// ExpectedCodes When the monitor protocol is HTTP, the expected return code for HTTP operations the value must be 2xx.
    #[prost(uint32, optional, tag = "4")]
    pub expected_codes: ::core::option::Option<u32>,
    /// HealthCheckType Indicates the health check type: link-local, end-to-end.
    #[prost(string, optional, tag = "5")]
    pub health_check_type: ::core::option::Option<::prost::alloc::string::String>,
    /// +optional
    /// HttpMethod When the monitor protocol is HTTP, the type of HTTP method used, such as GET, PUT, POST, and so on
    #[prost(string, optional, tag = "6")]
    pub http_method: ::core::option::Option<::prost::alloc::string::String>,
    /// MaxRetries The number of retries to attempt before declaring an instance health down.
    #[prost(uint32, optional, tag = "7")]
    pub max_retries: ::core::option::Option<u32>,
    /// MonitorType The protocol type to be used: PING or HTTP or BFD
    #[prost(string, optional, tag = "8")]
    pub monitor_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Timeout The number of seconds to wait for a response.
    #[prost(uint32, optional, tag = "9")]
    pub timeout: ::core::option::Option<u32>,
    /// +optional
    #[prost(uint32, optional, tag = "10")]
    pub timeout_usecs: ::core::option::Option<u32>,
    /// +optional
    /// UrlPath When the monitor protocol is HTTP, the URL to be used.
    #[prost(string, optional, tag = "11")]
    pub url_path: ::core::option::Option<::prost::alloc::string::String>,
    /// +optional
    /// Run BFD for all IP Addresses learnt by mac-ip learning, when enabled
    #[prost(bool, optional, tag = "12")]
    pub target_ip_all: ::core::option::Option<bool>,
    /// +optional
    /// Any address that belongs to the list has to run BFD health check when target-ip-all is false.
    #[prost(message, optional, tag = "13")]
    pub target_ip_list: ::core::option::Option<IpAddresses>,
}
/// ServiceHealthCheckSpec defines the desired state of ServiceHealthCheck
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceHealthCheckSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    #[prost(message, optional, tag = "2")]
    pub service_health_check_properties: ::core::option::Option<
        ServiceHealthCheckProperties,
    >,
}
/// ServiceHealthCheckStatus defines the observed state of ServiceHealthCheck
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceHealthCheckStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticNhType {
    #[prost(string, optional, tag = "1")]
    pub vtep_dst_ip_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub vtep_dst_mac_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub vni: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticRouteEntriesType {
    /// Route Entries such as: Prefix, NextHop,Communities
    #[prost(message, repeated, tag = "1")]
    pub route: ::prost::alloc::vec::Vec<StaticRouteType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticRouteType {
    /// Community string that needs to be associated with Route
    #[prost(string, repeated, tag = "1")]
    pub community: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// NextHop Ip address of NextHop, it should point to the valid VMI object
    #[prost(string, optional, tag = "2")]
    pub next_hop: ::core::option::Option<::prost::alloc::string::String>,
    /// Prefix for the route
    #[prost(string, optional, tag = "3")]
    pub prefix: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub route_target: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Subnet represents a block of IP addresses and its configuration.
/// IPAM allocates and releases IP address from that block on demand.
/// It can be used by different VirtualNetwork in the mean time.
/// +k8s:openapi-gen=true
/// +resource:path=subnets,strategy=SubnetStrategy,shortname=sn,categories=contrail;ipam;networking
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subnet {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the Subnet.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<SubnetSpec>,
    /// The most recently observed status of the Subnet.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<SubnetStatus>,
}
/// SubnetList is a list of Subnet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubnetList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the Subnet instances in the SubnetList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Subnet>,
}
/// SubnetReference contains a Subnet reference and a BGPSessionIPAttributes
/// instance defining the BGPaaSPrimaryIP and BGPaaSSecondaryIP addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubnetReference {
    #[prost(message, optional, tag = "1")]
    pub resource_reference: ::core::option::Option<ResourceReference>,
    #[prost(message, optional, tag = "2")]
    pub attributes: ::core::option::Option<BgpSessionIpAttributes>,
}
/// SubnetSpec defines the desired state of a Subnet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubnetSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// Subnet range in CIDR notation.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub cidr: ::core::option::Option<::prost::alloc::string::String>,
    /// Default Gateway IP address in the subnet.
    /// If not provided, one is auto-generated by the system.
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub default_gateway: ::core::option::Option<::prost::alloc::string::String>,
    /// List of DNS servers associated with the subnet.
    /// +optional
    #[prost(string, repeated, tag = "5")]
    pub dns_nameservers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Ranges, when present, define the IP allocation ranges corresponding to
    /// a given key.
    /// If not provided, IP allocation is determined by the CIDR.
    /// +optional
    #[prost(message, repeated, tag = "6")]
    pub ranges: ::prost::alloc::vec::Vec<Range>,
    /// Disables auto allocation of BGPaaSPrimaryIP and BGPaaSecondaryIP. False by
    /// default, automatic allocation is enabled. IPs are auto allocated when at
    /// least one BGPAsAService is configured under this subnet. If DisableBGPaaSIPAutoAllocation
    /// is set to true, BGPaaSPrimaryIP and BGPaaSSecondaryIP must be specified.
    /// Leave this flag false if the BGPAsAService feature is not required.
    /// +optional
    #[prost(bool, optional, tag = "7")]
    pub disable_bg_paa_sip_auto_allocation: ::core::option::Option<bool>,
    /// Primary IP address used for the BGP as a service session.
    /// +optional
    #[prost(string, optional, tag = "8")]
    pub bgpaas_primary_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// Secondary IP address used for the BGP as a service session when the
    /// second control node is present.
    /// +optional
    #[prost(string, optional, tag = "9")]
    pub bgpaas_secondary_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// Enables or disables vrouter DHCP
    /// +optional
    #[prost(bool, optional, tag = "10")]
    pub enable_dhcp: ::core::option::Option<bool>,
}
/// SubnetStatus defines the observed state of a Subnet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubnetStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
    /// IPCount is the current number of allocated IP addresses in the Subnet.
    /// +optional
    #[prost(int64, optional, tag = "2")]
    pub ip_count: ::core::option::Option<i64>,
    /// AllocationUsage is current percentage of allocated addresses in the Subnet.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub allocation_usage: ::core::option::Option<::prost::alloc::string::String>,
}
/// Tag is the representation of the Tag resource.
/// +k8s:openapi-gen=true
/// +resource:path=tags,strategy=TagStrategy,shortname=t,categories=contrail;security
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of Tag resource.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<TagSpec>,
    /// The most recently observed status of the Tag object.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<TagStatus>,
}
/// TagList is a list of Tag.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the Tag instances in the TagList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Tag>,
}
/// TagSpec defines the desired state of Tag
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// TagTypeName defines name of Tag Type object in string format.
    #[prost(string, optional, tag = "2")]
    pub tag_type_name: ::core::option::Option<::prost::alloc::string::String>,
    /// TagValue defines namee of Tag Value object in string format.
    #[prost(string, optional, tag = "3")]
    pub tag_value: ::core::option::Option<::prost::alloc::string::String>,
    /// TagTypeReference is reference to Tagtype object attachd to this Tag object.
    #[prost(message, optional, tag = "4")]
    pub tag_type_reference: ::core::option::Option<ResourceReference>,
}
/// TagStatus defines the observed state of Tag.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagStatus {
    /// Common status fields.
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
    /// TagID is an internal representation of Tag object encapsulating
    /// tag type and value in hexadecimal format.
    #[prost(string, optional, tag = "2")]
    pub tag_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// TagType is the representation of the TagType resource.
/// +k8s:openapi-gen=true
/// +resource:path=tagtypes,strategy=TagTypeStrategy,shortname=tt,categories=contrail;security
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagType {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of TagType resource.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<TagTypeSpec>,
    /// The most recently observed status of the TagType object.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<TagTypeStatus>,
}
/// TagTypeList is a list of TagType.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagTypeList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the TagType instances in the TagTypeList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<TagType>,
}
/// TagTypeSpec defines the desired state of TagType
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagTypeSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
}
/// TagTypeStatus defines the observed state of TagType
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagTypeStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
    /// TagTypeID is an internal representation of TagType object, encapsulating
    /// tag type in hexadecimal format.
    #[prost(string, optional, tag = "2")]
    pub tag_type_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// VirtualMachine represents a computational resource, for example, a virtual machine,
/// bare metal server, or container.
/// +k8s:openapi-gen=true
/// +resource:path=virtualmachines,strategy=VirtualMachineStrategy,shortname=vm,categories=contrail;networking
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachine {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the VirtualMachine.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<VirtualMachineSpec>,
    /// The most recently observed status of the VirtualMachine.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<VirtualMachineStatus>,
}
/// VirtualMachineInterface represents an interface(port) into virtual network.
/// It may or may not have corresponding virtual machine. A virtual machine
/// interface has at least a MAC address and an IP address.
/// +k8s:openapi-gen=true
/// +resource:path=virtualmachineinterfaces,strategy=VirtualMachineInterfaceStrategy,shortname=vmi,categories=contrail;networking
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachineInterface {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<VirtualMachineInterfaceSpec>,
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<VirtualMachineInterfaceStatus>,
}
/// VirtualMachineInterfaceList is a list of VirtualMachineInterface.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachineInterfaceList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the VirtualMachineInterface instances in the VirtualMachineInterfaceList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<VirtualMachineInterface>,
}
/// Advanced Properties of the VirtualMachineInterface.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachineInterfaceProperties {
    /// 802.1Q VLAN tag to be used if this interface is a sub-interface of
    /// other interface.
    /// +optional
    #[prost(uint32, optional, tag = "1")]
    pub sub_interface_vlan_tag: ::core::option::Option<u32>,
}
/// VirtualMachineInterfaceSpec defines the desired state of VirtualMachineInterface.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachineInterfaceSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// Object reference to namespace or virtualrouter parent.
    #[prost(message, optional, tag = "2")]
    pub parent: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::api::core::v1::ObjectReference,
    >,
    /// MAC address of the virtual machine interface, automatically assigned by
    /// system if not provided.
    #[prost(message, optional, tag = "3")]
    pub virtual_machine_interface_mac_addresses: ::core::option::Option<MacAddresses>,
    /// VirtualNetworkReference determines the Virtual Network the interface belongs to.
    #[prost(message, optional, tag = "4")]
    pub virtual_network_reference: ::core::option::Option<ResourceReference>,
    /// VirtualMachineReferences determines the VirtualMachine the interface belongs
    /// to.
    /// +optional
    #[prost(message, repeated, tag = "5")]
    pub virtual_machine_references: ::prost::alloc::vec::Vec<ResourceReference>,
    /// DisablePolicy disables all policy checks for ingress and egress traffic
    /// from this interface. Flow table entries are not created. Features that
    /// require policy will not work on this interface, these include security
    /// group, floating IP, service chain, linklocal services.
    #[prost(bool, optional, tag = "6")]
    pub virtual_machine_interface_disable_policy: ::core::option::Option<bool>,
    /// List of (IP address, MAC) other than instance ip on this interface.
    /// +optional
    #[prost(message, optional, tag = "7")]
    pub allowed_address_pairs: ::core::option::Option<AllowedAddressPairs>,
    /// Port security status on the network.
    #[prost(bool, optional, tag = "8")]
    pub port_security_enabled: ::core::option::Option<bool>,
    /// VirtualMachineInterfaceReferences determines the VirtualMachineInterface
    /// instances that are sub-interfaces.
    /// +optional
    #[prost(message, repeated, tag = "9")]
    pub virtual_machine_interface_references: ::prost::alloc::vec::Vec<
        ResourceReference,
    >,
    /// VirtualMachineInterface properties
    /// +optional
    #[prost(message, optional, tag = "10")]
    pub properties: ::core::option::Option<VirtualMachineInterfaceProperties>,
    /// Reference to Tag attached to this Virtual Machine Interface
    /// +optional
    #[prost(message, repeated, tag = "11")]
    pub tag_references: ::prost::alloc::vec::Vec<ResourceReference>,
}
/// VirtualMachineInterfaceStatus defines the observed state of VirtualMachineInterface
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachineInterfaceStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
    /// RoutingInstanceReferences lists all routing instance the interface is associated to.
    /// Should at least contains the reference to its Virtual Network primary Routing Instance.
    #[prost(message, repeated, tag = "3")]
    pub routing_instance_references: ::prost::alloc::vec::Vec<RoutingInstanceReference>,
    /// BGPRouterReference is bgpaas-client BGPRouter reference for BGP neighbor.
    /// Holds the corresponding BGPRouterRef from BGPAsAService
    #[prost(message, optional, tag = "4")]
    pub bgp_router_reference: ::core::option::Option<ResourceReference>,
    /// ServiceHealthCheck is ServiceHealthCheck reference
    #[prost(message, repeated, tag = "12")]
    pub service_health_check: ::prost::alloc::vec::Vec<ResourceReference>,
    /// Reference to the interface route table attached to this interface.
    #[prost(message, repeated, tag = "5")]
    pub interface_route_table_references: ::prost::alloc::vec::Vec<ResourceReference>,
}
/// VirtualMachineList is a list of VirtualMachine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachineList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the VirtualMachine instances in the VirtualMachineList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<VirtualMachine>,
}
/// VirtualMachineSpec defines the desired state of a VirtualMachine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachineSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// ServerType indicates the computational type of the VirtualMachine resource.
    /// Valid values for ServerType include virtual-server, baremetal-server, or
    /// container.
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub server_type: ::core::option::Option<::prost::alloc::string::String>,
    /// ServerName is the name of the VirtualMachine resource.
    #[prost(string, optional, tag = "3")]
    pub server_name: ::core::option::Option<::prost::alloc::string::String>,
    /// ServerNamespace is the namespace of the VirtualMachine resource.
    #[prost(string, optional, tag = "4")]
    pub server_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Deprecated: ServerClusterName is the name of the cluster the VirtualMachine resource
    /// is to run on.
    #[prost(string, optional, tag = "5")]
    pub server_cluster_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// VirtualMachineStatus defines the observed state of the VirtualMachine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachineStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
/// VirtualNetwork is a collection of endpoints (interface or IP(s) or MAC(s))
/// that can communicate with each other.
/// It is also a collection of subnets whose default gateways are connected by
/// an implicit router.
/// +k8s:openapi-gen=true
/// +resource:path=virtualnetworks,strategy=VirtualNetworkStrategy,shortname=vn,categories=contrail;networking
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNetwork {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the VirtualNetwork.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<VirtualNetworkSpec>,
    /// The most recently observed status of the VirtualNetwork.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<VirtualNetworkStatus>,
}
/// VirtualNetworkList is a list of VirtualNetwork.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNetworkList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the VirtualNetwork instances in the VirtualNetworkList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<VirtualNetwork>,
}
/// VirtualNetworkReference contains a VirtualNetwork reference and an Attributes
/// instance defining a virtual network router.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNetworkReference {
    #[prost(message, optional, tag = "1")]
    pub resource_reference: ::core::option::Option<ResourceReference>,
    /// Attributes contains a different attributes of VirtualNetworkReference
    #[prost(message, optional, tag = "2")]
    pub attributes: ::core::option::Option<KeyValuePair>,
}
/// VirtualNetworkRouteTargetReferenceList contains RouteTarget references and
/// their import/export mode.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNetworkRouteTargetReferenceList {
    /// RouteTargetReferences is the actual list of RouteTargetReferences.
    /// +optional
    #[prost(message, repeated, tag = "1")]
    pub route_target_references: ::prost::alloc::vec::Vec<RouteTargetReference>,
}
/// VirtualNetworkRouter establishes connectivity between two or more
/// VirtualNetworks
/// +k8s:openapi-gen=true
/// +resource:path=virtualnetworkrouters,strategy=VirtualNetworkRouterStrategy,shortname=vnr
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNetworkRouter {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired state of the VirtualNetworkRouter.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<VirtualNetworkRouterSpec>,
    /// The most recently observed status of the VirtualNetworkRouter.
    /// This data may not be up-to-date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<VirtualNetworkRouterStatus>,
}
/// VirtualNetworkRouterEntry is a combination of VirtualNetworkRouterSelector and
/// NamespaceSelector. Together, these two LabelSelectors identify a VirtualNetworkRouter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNetworkRouterEntry {
    /// LabelSelector to identify the VirtualNetworkRouter
    #[prost(message, optional, tag = "1")]
    pub virtual_network_router_selector: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::LabelSelector,
    >,
    /// LabelSelector to identify the Namespace of the VirtualNetworkRouter.
    #[prost(message, optional, tag = "2")]
    pub namespace_selector: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::LabelSelector,
    >,
}
/// VirtualNetworkRouterList is a list of VirtualNetworkRouter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNetworkRouterList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the VirtualNetworkRouter instances in the VirtualNetworkRouterList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<VirtualNetworkRouter>,
}
/// VirtualNetworkRouterSpec defines the desired state of the VirtualNetworkRouter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNetworkRouterSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// Type of the VirtualNetworkRouter.
    /// Supported types are mesh, spoke and hub.
    #[prost(string, optional, tag = "2")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// VirtualNetworkSelector is a LabelSelector to identify VirtualNetworks that
    /// this VirtualNetworkRouter should connect to.
    /// VirtualNetworkRouter shares its RouteTarget to the connected VirtualNetworks.
    #[prost(message, optional, tag = "3")]
    pub virtual_network_selector: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::LabelSelector,
    >,
    /// VirtualNetworkRouter can import other VirtualNetworkRouters to enable
    /// connectivity between the VirtualNetworks selected by this VirtualNetworkRouter and
    /// VirtualNetworks selected by the imported VirtualNetworkRouter.
    ///
    /// Specify list of VirtualNetworkRouters to import.
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub import: ::core::option::Option<ImportVirtualNetworkRouter>,
    /// RouteTarget will be configured in all routing instance of virtual networks connected to this VirtualNetworkRouter.
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub route_target: ::core::option::Option<::prost::alloc::string::String>,
    /// RoutingType is the routing mode for this VirtualNetworkRouter.
    /// Supported Options are evpn/l3vpn.
    /// +optional
    #[prost(string, optional, tag = "6")]
    pub routing_type: ::core::option::Option<::prost::alloc::string::String>,
    /// L3vxlanNetworkIdentifier is user defined unique L3 VNI. This will be configured on the internal virtual network of  VirtualNetworkRouter.
    /// User will able configure the L3vxlanNetworkIdentifier only when EvpnRouting is enabled.
    /// Valid range is 4096 - 4294967296.
    /// +optional
    #[prost(int64, optional, tag = "7")]
    pub l3vxlan_network_identifier: ::core::option::Option<i64>,
}
/// VirtualNetworkRouterStatus defines the observed state of the VirtualNetworkRouter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNetworkRouterStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
    /// EvpnRoutingVirtualNetworkReference is the reference to VirtualNetworkRouter's  internal Virtual Network
    #[prost(message, optional, tag = "2")]
    pub evpn_routing_virtual_network_reference: ::core::option::Option<
        ResourceReference,
    >,
    /// L3vxlanNetworkIdentifier is L3 VNI associated with internal network of VirtualNetworkRouter in case EvpnRouting is enabled.
    /// +optional
    #[prost(int64, optional, tag = "3")]
    pub l3vxlan_network_identifier: ::core::option::Option<i64>,
}
/// VirtualNetworkSpec defines the desired state of a VirtualNetwork.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNetworkSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// FabricSNAT toggles connectivity to underlay network by port mapping.
    /// +optional
    #[prost(bool, optional, tag = "2")]
    pub fabric_snat: ::core::option::Option<bool>,
    /// Reference to the v4 family subnet.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub v4_subnet_reference: ::core::option::Option<ResourceReference>,
    /// Reference to the v6 family subnet.
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub v6_subnet_reference: ::core::option::Option<ResourceReference>,
    /// RouteTargetList is a list of route targets that are used as both import
    /// and export for this virtual network.
    /// +optional
    #[prost(string, repeated, tag = "5")]
    pub route_target_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ImportRouteTargetList is a list of route targets that are used as import
    /// for this virtual network.
    /// +optional
    #[prost(string, repeated, tag = "6")]
    pub import_route_target_list: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// RouteTargetList is a list of route targets that are used as import for
    /// this virtual network.
    /// +optional
    #[prost(string, repeated, tag = "7")]
    pub export_route_target_list: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// VirtualNetworkProperties defines additional configuration parameters for
    /// each virtual network.
    /// +optional
    #[prost(message, optional, tag = "8")]
    pub virtual_network_properties: ::core::option::Option<VirtualNetworkType>,
    /// ProviderNetworkReference is the reference to a provider virtual network,
    /// example: ip-fabric.
    /// +optional
    #[prost(message, optional, tag = "9")]
    pub provider_network_reference: ::core::option::Option<ResourceReference>,
    /// IsProviderNetwork is a flag that needs to be set to true if VN is a Provider Network
    /// Cannot be updated from true to false. Both IsProviderNetwork and
    /// FabricForwarding cannot be set to true at the same time.
    /// +optional
    #[prost(bool, optional, tag = "10")]
    pub is_provider_network: ::core::option::Option<bool>,
    /// FabricForwarding when set to true adds the ip-fabric VN as a provider network
    /// to this virtual network. If provider network already has a reference to a different
    /// network, it will be overriden to the ip-fabric VN. Both IsProviderNetwork and
    /// FabricForwarding cannot be set to true at the same time.
    /// +optional
    #[prost(bool, optional, tag = "11")]
    pub fabric_forwarding: ::core::option::Option<bool>,
    /// VlanID to use for the virtual network. Valid range is 0 - 4094.
    /// +optional
    #[prost(int32, optional, tag = "12")]
    pub vlan_id: ::core::option::Option<i32>,
    /// VirtualNetworkNetworkID is User defined unique 32-bit VxlanId for every virtual network. Valid range is 4096 - 4294967296.
    /// +optional
    #[prost(int64, optional, tag = "13")]
    pub virtual_network_network_id: ::core::option::Option<i64>,
    /// PodNetwork is used to differentiate between regular networks and pod networks.
    /// When PodNetwork is set to true, host routes will be added towards the VN's CIDR.
    #[prost(bool, optional, tag = "14")]
    pub pod_network: ::core::option::Option<bool>,
    /// static route table to be associated VN
    #[prost(message, repeated, tag = "16")]
    pub route_table_references: ::prost::alloc::vec::Vec<ResourceReference>,
}
/// VirtualNetworkStatus defines the observed state of a VirtualNetwork.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNetworkStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
    /// System assigned unique 32-bit ID for every virtual network.
    /// +optional
    #[prost(int64, optional, tag = "2")]
    pub virtual_network_network_id: ::core::option::Option<i64>,
    /// EvpnRoutingVirtualNetworkReferences lists of all internal virtual network of virtual network routers, where this virtual network connected.
    #[prost(message, repeated, tag = "3")]
    pub evpn_routing_virtual_network_references: ::prost::alloc::vec::Vec<
        VirtualNetworkReference,
    >,
}
/// Advanced properties of the VirtualNetwork.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNetworkType {
    /// Rpf property enables or disables unicast Reverse Path Forwarding (RPF) on
    /// the VirtualNetwork.
    /// By Default, Rpf is set to enabled.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub rpf: ::core::option::Option<::prost::alloc::string::String>,
    /// ForwardingMode is the Packet forwarding mode for this VirtualNetwork.
    /// Supported Options are l2, l3 and l2_l3.
    /// By Default, ForwardingMode is set to l2_l3.
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub forwarding_mode: ::core::option::Option<::prost::alloc::string::String>,
}
/// VirtualRouter is packet forwarding system on devices such as compute
/// nodes(servers), TOR(s), routers.
/// +k8s:openapi-gen=true
/// +resource:path=virtualrouters,strategy=VirtualRouterStrategy,shortname=vr,categories=contrail;contrail-cluster
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualRouter {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<VirtualRouterSpec>,
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<VirtualRouterStatus>,
}
/// VirtualRouterList is a list of VirtualRouter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualRouterList {
    /// Standard list's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#lists-and-simple-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items contains all of the VirtualRouter instances in the VirtualRouterList.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<VirtualRouter>,
}
/// VirtualRouterSpec defines the desired state of VirtualRouter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualRouterSpec {
    /// Common spec fields
    #[prost(message, optional, tag = "1")]
    pub common_spec: ::core::option::Option<CommonSpec>,
    /// Parent contains the ObjectReference to the parent GlobalSystemConfig.
    #[prost(message, optional, tag = "2")]
    pub parent: ::core::option::Option<
        super::super::super::super::super::super::super::super::super::super::k8s::io::api::core::v1::ObjectReference,
    >,
    /// This VirtualRouter's data path is using DPDK library. Virtual machine interfaces
    /// scheduled on this compute node will be tagged with additional flags so that they
    /// are spawned with user space virtio driver. It is only applicable for embedded
    /// VirtualRouters.
    /// +optional
    #[prost(bool, optional, tag = "3")]
    pub virtual_router_dpdk_enabled: ::core::option::Option<bool>,
    /// IP address of the VirtualRouter (required).
    #[prost(string, optional, tag = "4")]
    pub virtual_router_ip_address: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    /// The type of VirtualRouter in the system.
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub virtual_router_type: ::core::option::Option<::prost::alloc::string::String>,
    /// VirtualMachineReferences is the list of all VirtualMachine instances on
    /// this vrouter. This link is present for virtual machines associated to
    /// Kubernetes Pods.
    /// +optional
    /// +patchMergeKey=uid
    /// +patchStrategy=merge
    #[prost(message, repeated, tag = "6")]
    pub virtual_machine_references: ::prost::alloc::vec::Vec<ResourceReference>,
}
/// VirtualRouterStatus defines the observed state of VirtualRouter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualRouterStatus {
    /// Common status fields
    #[prost(message, optional, tag = "1")]
    pub common_status: ::core::option::Option<CommonStatus>,
}
