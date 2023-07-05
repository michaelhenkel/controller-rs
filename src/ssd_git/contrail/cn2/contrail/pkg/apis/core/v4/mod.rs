
mod aps_attribute;
pub use self::aps_attribute::APSAttribute;

mod address_families;
pub use self::address_families::AddressFamilies;

mod address_group;
pub use self::address_group::AddressGroup;
#[cfg(feature = "api")] pub use self::address_group::{CreateAddressGroupOptional, CreateAddressGroupResponse};
#[cfg(feature = "api")] pub use self::address_group::{CreateAddressGroupStatusOptional, CreateAddressGroupStatusResponse};
#[cfg(feature = "api")] pub use self::address_group::{DeleteCollectionAddressGroupOptional, DeleteCollectionAddressGroupResponse};
#[cfg(feature = "api")] pub use self::address_group::{DeleteAddressGroupOptional, DeleteAddressGroupResponse};
#[cfg(feature = "api")] pub use self::address_group::{DeleteAddressGroupStatusOptional, DeleteAddressGroupStatusResponse};
#[cfg(feature = "api")] pub use self::address_group::{ListAddressGroupForAllNamespacesOptional, ListAddressGroupForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::address_group::{ListAddressGroupOptional, ListAddressGroupResponse};
#[cfg(feature = "api")] pub use self::address_group::{PatchAddressGroupOptional, PatchAddressGroupResponse};
#[cfg(feature = "api")] pub use self::address_group::{PatchAddressGroupStatusOptional, PatchAddressGroupStatusResponse};
#[cfg(feature = "api")] pub use self::address_group::{ReadAddressGroupOptional, ReadAddressGroupResponse};
#[cfg(feature = "api")] pub use self::address_group::{ReadAddressGroupStatusOptional, ReadAddressGroupStatusResponse};
#[cfg(feature = "api")] pub use self::address_group::{ReplaceAddressGroupOptional, ReplaceAddressGroupResponse};
#[cfg(feature = "api")] pub use self::address_group::{ReplaceAddressGroupStatusOptional, ReplaceAddressGroupStatusResponse};
#[cfg(feature = "api")] pub use self::address_group::{WatchAddressGroupListForAllNamespacesOptional, WatchAddressGroupListForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::address_group::{WatchAddressGroupOptional, WatchAddressGroupResponse};
#[cfg(feature = "api")] pub use self::address_group::{WatchAddressGroupListOptional, WatchAddressGroupListResponse};
#[cfg(feature = "api")] pub use self::address_group::{WatchAddressGroupStatusOptional, WatchAddressGroupStatusResponse};

mod address_group_prefix;
pub use self::address_group_prefix::AddressGroupPrefix;

mod address_group_spec;
pub use self::address_group_spec::AddressGroupSpec;

mod address_group_status;
pub use self::address_group_status::AddressGroupStatus;

mod allowed_address_pair;
pub use self::allowed_address_pair::AllowedAddressPair;

mod allowed_address_pair_subnet;
pub use self::allowed_address_pair_subnet::AllowedAddressPairSubnet;

mod allowed_address_pairs;
pub use self::allowed_address_pairs::AllowedAddressPairs;

mod application_policy_set;
pub use self::application_policy_set::ApplicationPolicySet;
#[cfg(feature = "api")] pub use self::application_policy_set::{CreateApplicationPolicySetOptional, CreateApplicationPolicySetResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{CreateApplicationPolicySetStatusOptional, CreateApplicationPolicySetStatusResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{DeleteCollectionApplicationPolicySetOptional, DeleteCollectionApplicationPolicySetResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{DeleteApplicationPolicySetOptional, DeleteApplicationPolicySetResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{DeleteApplicationPolicySetStatusOptional, DeleteApplicationPolicySetStatusResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{ListApplicationPolicySetForAllNamespacesOptional, ListApplicationPolicySetForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{ListApplicationPolicySetOptional, ListApplicationPolicySetResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{PatchApplicationPolicySetOptional, PatchApplicationPolicySetResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{PatchApplicationPolicySetStatusOptional, PatchApplicationPolicySetStatusResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{ReadApplicationPolicySetOptional, ReadApplicationPolicySetResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{ReadApplicationPolicySetStatusOptional, ReadApplicationPolicySetStatusResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{ReplaceApplicationPolicySetOptional, ReplaceApplicationPolicySetResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{ReplaceApplicationPolicySetStatusOptional, ReplaceApplicationPolicySetStatusResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{WatchApplicationPolicySetListForAllNamespacesOptional, WatchApplicationPolicySetListForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{WatchApplicationPolicySetOptional, WatchApplicationPolicySetResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{WatchApplicationPolicySetListOptional, WatchApplicationPolicySetListResponse};
#[cfg(feature = "api")] pub use self::application_policy_set::{WatchApplicationPolicySetStatusOptional, WatchApplicationPolicySetStatusResponse};

mod application_policy_set_spec;
pub use self::application_policy_set_spec::ApplicationPolicySetSpec;

mod application_policy_set_status;
pub use self::application_policy_set_status::ApplicationPolicySetStatus;

mod authentication_data;
pub use self::authentication_data::AuthenticationData;

mod authentication_key_item;
pub use self::authentication_key_item::AuthenticationKeyItem;

mod bgp_as_a_service;
pub use self::bgp_as_a_service::BGPAsAService;
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{CreateBGPAsAServiceOptional, CreateBGPAsAServiceResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{CreateBGPAsAServiceStatusOptional, CreateBGPAsAServiceStatusResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{DeleteCollectionBGPAsAServiceOptional, DeleteCollectionBGPAsAServiceResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{DeleteBGPAsAServiceOptional, DeleteBGPAsAServiceResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{DeleteBGPAsAServiceStatusOptional, DeleteBGPAsAServiceStatusResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{ListBGPAsAServiceForAllNamespacesOptional, ListBGPAsAServiceForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{ListBGPAsAServiceOptional, ListBGPAsAServiceResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{PatchBGPAsAServiceOptional, PatchBGPAsAServiceResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{PatchBGPAsAServiceStatusOptional, PatchBGPAsAServiceStatusResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{ReadBGPAsAServiceOptional, ReadBGPAsAServiceResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{ReadBGPAsAServiceStatusOptional, ReadBGPAsAServiceStatusResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{ReplaceBGPAsAServiceOptional, ReplaceBGPAsAServiceResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{ReplaceBGPAsAServiceStatusOptional, ReplaceBGPAsAServiceStatusResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{WatchBGPAsAServiceListForAllNamespacesOptional, WatchBGPAsAServiceListForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{WatchBGPAsAServiceOptional, WatchBGPAsAServiceResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{WatchBGPAsAServiceListOptional, WatchBGPAsAServiceListResponse};
#[cfg(feature = "api")] pub use self::bgp_as_a_service::{WatchBGPAsAServiceStatusOptional, WatchBGPAsAServiceStatusResponse};

mod bgp_as_a_service_spec;
pub use self::bgp_as_a_service_spec::BGPAsAServiceSpec;

mod bgp_as_a_service_status;
pub use self::bgp_as_a_service_status::BGPAsAServiceStatus;

mod bgp_family_attributes;
pub use self::bgp_family_attributes::BGPFamilyAttributes;

mod bgp_prefix_limit;
pub use self::bgp_prefix_limit::BGPPrefixLimit;

mod bgp_router;
pub use self::bgp_router::BGPRouter;
#[cfg(feature = "api")] pub use self::bgp_router::{CreateBGPRouterOptional, CreateBGPRouterResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{CreateBGPRouterStatusOptional, CreateBGPRouterStatusResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{DeleteCollectionBGPRouterOptional, DeleteCollectionBGPRouterResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{DeleteBGPRouterOptional, DeleteBGPRouterResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{DeleteBGPRouterStatusOptional, DeleteBGPRouterStatusResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{ListBGPRouterForAllNamespacesOptional, ListBGPRouterForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{ListBGPRouterOptional, ListBGPRouterResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{PatchBGPRouterOptional, PatchBGPRouterResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{PatchBGPRouterStatusOptional, PatchBGPRouterStatusResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{ReadBGPRouterOptional, ReadBGPRouterResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{ReadBGPRouterStatusOptional, ReadBGPRouterStatusResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{ReplaceBGPRouterOptional, ReplaceBGPRouterResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{ReplaceBGPRouterStatusOptional, ReplaceBGPRouterStatusResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{WatchBGPRouterListForAllNamespacesOptional, WatchBGPRouterListForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{WatchBGPRouterOptional, WatchBGPRouterResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{WatchBGPRouterListOptional, WatchBGPRouterListResponse};
#[cfg(feature = "api")] pub use self::bgp_router::{WatchBGPRouterStatusOptional, WatchBGPRouterStatusResponse};

mod bgp_router_parameters;
pub use self::bgp_router_parameters::BGPRouterParameters;

mod bgp_router_reference;
pub use self::bgp_router_reference::BGPRouterReference;

mod bgp_router_reference_attributes;
pub use self::bgp_router_reference_attributes::BGPRouterReferenceAttributes;

mod bgp_router_spec;
pub use self::bgp_router_spec::BGPRouterSpec;

mod bgp_router_status;
pub use self::bgp_router_status::BGPRouterStatus;

mod bgp_session;
pub use self::bgp_session::BGPSession;

mod bgp_session_attributes;
pub use self::bgp_session_attributes::BGPSessionAttributes;

mod bgp_session_ip_attributes;
pub use self::bgp_session_ip_attributes::BGPSessionIPAttributes;

mod community_attributes;
pub use self::community_attributes::CommunityAttributes;

mod contrail_security_policy;
pub use self::contrail_security_policy::ContrailSecurityPolicy;
#[cfg(feature = "api")] pub use self::contrail_security_policy::{CreateContrailSecurityPolicyOptional, CreateContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{CreateContrailSecurityPolicyStatusOptional, CreateContrailSecurityPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{DeleteCollectionContrailSecurityPolicyOptional, DeleteCollectionContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{DeleteContrailSecurityPolicyOptional, DeleteContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{DeleteContrailSecurityPolicyStatusOptional, DeleteContrailSecurityPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{ListContrailSecurityPolicyForAllNamespacesOptional, ListContrailSecurityPolicyForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{ListContrailSecurityPolicyOptional, ListContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{PatchContrailSecurityPolicyOptional, PatchContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{PatchContrailSecurityPolicyStatusOptional, PatchContrailSecurityPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{ReadContrailSecurityPolicyOptional, ReadContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{ReadContrailSecurityPolicyStatusOptional, ReadContrailSecurityPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{ReplaceContrailSecurityPolicyOptional, ReplaceContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{ReplaceContrailSecurityPolicyStatusOptional, ReplaceContrailSecurityPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{WatchContrailSecurityPolicyListForAllNamespacesOptional, WatchContrailSecurityPolicyListForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{WatchContrailSecurityPolicyOptional, WatchContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{WatchContrailSecurityPolicyListOptional, WatchContrailSecurityPolicyListResponse};
#[cfg(feature = "api")] pub use self::contrail_security_policy::{WatchContrailSecurityPolicyStatusOptional, WatchContrailSecurityPolicyStatusResponse};

mod contrail_security_policy_end_point;
pub use self::contrail_security_policy_end_point::ContrailSecurityPolicyEndPoint;

mod contrail_security_policy_pod_selector;
pub use self::contrail_security_policy_pod_selector::ContrailSecurityPolicyPodSelector;

mod contrail_security_policy_rule;
pub use self::contrail_security_policy_rule::ContrailSecurityPolicyRule;

mod contrail_security_policy_selector;
pub use self::contrail_security_policy_selector::ContrailSecurityPolicySelector;

mod contrail_security_policy_spec;
pub use self::contrail_security_policy_spec::ContrailSecurityPolicySpec;

mod contrail_security_policy_status;
pub use self::contrail_security_policy_status::ContrailSecurityPolicyStatus;

mod encapsulation_priorities;
pub use self::encapsulation_priorities::EncapsulationPriorities;

mod fast_convergence_parameters_type;
pub use self::fast_convergence_parameters_type::FastConvergenceParametersType;

mod firewall_action_list_type;
pub use self::firewall_action_list_type::FirewallActionListType;

mod firewall_match_epr_list;
pub use self::firewall_match_epr_list::FirewallMatchEprList;

mod firewall_match_expr;
pub use self::firewall_match_expr::FirewallMatchExpr;

mod firewall_mirror_action_type;
pub use self::firewall_mirror_action_type::FirewallMirrorActionType;

mod firewall_policy;
pub use self::firewall_policy::FirewallPolicy;
#[cfg(feature = "api")] pub use self::firewall_policy::{CreateFirewallPolicyOptional, CreateFirewallPolicyResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{CreateFirewallPolicyStatusOptional, CreateFirewallPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{DeleteCollectionFirewallPolicyOptional, DeleteCollectionFirewallPolicyResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{DeleteFirewallPolicyOptional, DeleteFirewallPolicyResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{DeleteFirewallPolicyStatusOptional, DeleteFirewallPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{ListFirewallPolicyForAllNamespacesOptional, ListFirewallPolicyForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{ListFirewallPolicyOptional, ListFirewallPolicyResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{PatchFirewallPolicyOptional, PatchFirewallPolicyResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{PatchFirewallPolicyStatusOptional, PatchFirewallPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{ReadFirewallPolicyOptional, ReadFirewallPolicyResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{ReadFirewallPolicyStatusOptional, ReadFirewallPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{ReplaceFirewallPolicyOptional, ReplaceFirewallPolicyResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{ReplaceFirewallPolicyStatusOptional, ReplaceFirewallPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{WatchFirewallPolicyListForAllNamespacesOptional, WatchFirewallPolicyListForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{WatchFirewallPolicyOptional, WatchFirewallPolicyResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{WatchFirewallPolicyListOptional, WatchFirewallPolicyListResponse};
#[cfg(feature = "api")] pub use self::firewall_policy::{WatchFirewallPolicyStatusOptional, WatchFirewallPolicyStatusResponse};

mod firewall_policy_attribute;
pub use self::firewall_policy_attribute::FirewallPolicyAttribute;

mod firewall_policy_reference;
pub use self::firewall_policy_reference::FirewallPolicyReference;

mod firewall_policy_spec;
pub use self::firewall_policy_spec::FirewallPolicySpec;

mod firewall_policy_status;
pub use self::firewall_policy_status::FirewallPolicyStatus;

mod firewall_policy_vmi_selector;
pub use self::firewall_policy_vmi_selector::FirewallPolicyVMISelector;

mod firewall_rule;
pub use self::firewall_rule::FirewallRule;
#[cfg(feature = "api")] pub use self::firewall_rule::{CreateFirewallRuleOptional, CreateFirewallRuleResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{CreateFirewallRuleStatusOptional, CreateFirewallRuleStatusResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{DeleteCollectionFirewallRuleOptional, DeleteCollectionFirewallRuleResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{DeleteFirewallRuleOptional, DeleteFirewallRuleResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{DeleteFirewallRuleStatusOptional, DeleteFirewallRuleStatusResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{ListFirewallRuleForAllNamespacesOptional, ListFirewallRuleForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{ListFirewallRuleOptional, ListFirewallRuleResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{PatchFirewallRuleOptional, PatchFirewallRuleResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{PatchFirewallRuleStatusOptional, PatchFirewallRuleStatusResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{ReadFirewallRuleOptional, ReadFirewallRuleResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{ReadFirewallRuleStatusOptional, ReadFirewallRuleStatusResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{ReplaceFirewallRuleOptional, ReplaceFirewallRuleResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{ReplaceFirewallRuleStatusOptional, ReplaceFirewallRuleStatusResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{WatchFirewallRuleListForAllNamespacesOptional, WatchFirewallRuleListForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{WatchFirewallRuleOptional, WatchFirewallRuleResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{WatchFirewallRuleListOptional, WatchFirewallRuleListResponse};
#[cfg(feature = "api")] pub use self::firewall_rule::{WatchFirewallRuleStatusOptional, WatchFirewallRuleStatusResponse};

mod firewall_rule_endpoint_type;
pub use self::firewall_rule_endpoint_type::FirewallRuleEndpointType;

mod firewall_rule_reference;
pub use self::firewall_rule_reference::FirewallRuleReference;

mod firewall_rule_spec;
pub use self::firewall_rule_spec::FirewallRuleSpec;

mod firewall_rule_status;
pub use self::firewall_rule_status::FirewallRuleStatus;

mod firewall_service_group_type;
pub use self::firewall_service_group_type::FirewallServiceGroupType;

mod firewall_service_type;
pub use self::firewall_service_type::FirewallServiceType;

mod firewall_subnet;
pub use self::firewall_subnet::FirewallSubnet;

mod floating_ip;
pub use self::floating_ip::FloatingIP;
#[cfg(feature = "api")] pub use self::floating_ip::{CreateFloatingIPOptional, CreateFloatingIPResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{CreateFloatingIPStatusOptional, CreateFloatingIPStatusResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{DeleteCollectionFloatingIPOptional, DeleteCollectionFloatingIPResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{DeleteFloatingIPOptional, DeleteFloatingIPResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{DeleteFloatingIPStatusOptional, DeleteFloatingIPStatusResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{ListFloatingIPOptional, ListFloatingIPResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{PatchFloatingIPOptional, PatchFloatingIPResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{PatchFloatingIPStatusOptional, PatchFloatingIPStatusResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{ReadFloatingIPOptional, ReadFloatingIPResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{ReadFloatingIPStatusOptional, ReadFloatingIPStatusResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{ReplaceFloatingIPOptional, ReplaceFloatingIPResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{ReplaceFloatingIPStatusOptional, ReplaceFloatingIPStatusResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{WatchFloatingIPOptional, WatchFloatingIPResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{WatchFloatingIPListOptional, WatchFloatingIPListResponse};
#[cfg(feature = "api")] pub use self::floating_ip::{WatchFloatingIPStatusOptional, WatchFloatingIPStatusResponse};

mod floating_ip_port_mappings;
pub use self::floating_ip_port_mappings::FloatingIPPortMappings;

mod floating_ip_port_port_mapping;
pub use self::floating_ip_port_port_mapping::FloatingIPPortPortMapping;

mod floating_ip_spec;
pub use self::floating_ip_spec::FloatingIPSpec;

mod floating_ip_status;
pub use self::floating_ip_status::FloatingIPStatus;

mod global_contrail_security_policy;
pub use self::global_contrail_security_policy::GlobalContrailSecurityPolicy;
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{CreateGlobalContrailSecurityPolicyOptional, CreateGlobalContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{CreateGlobalContrailSecurityPolicyStatusOptional, CreateGlobalContrailSecurityPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{DeleteCollectionGlobalContrailSecurityPolicyOptional, DeleteCollectionGlobalContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{DeleteGlobalContrailSecurityPolicyOptional, DeleteGlobalContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{DeleteGlobalContrailSecurityPolicyStatusOptional, DeleteGlobalContrailSecurityPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{ListGlobalContrailSecurityPolicyOptional, ListGlobalContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{PatchGlobalContrailSecurityPolicyOptional, PatchGlobalContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{PatchGlobalContrailSecurityPolicyStatusOptional, PatchGlobalContrailSecurityPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{ReadGlobalContrailSecurityPolicyOptional, ReadGlobalContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{ReadGlobalContrailSecurityPolicyStatusOptional, ReadGlobalContrailSecurityPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{ReplaceGlobalContrailSecurityPolicyOptional, ReplaceGlobalContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{ReplaceGlobalContrailSecurityPolicyStatusOptional, ReplaceGlobalContrailSecurityPolicyStatusResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{WatchGlobalContrailSecurityPolicyOptional, WatchGlobalContrailSecurityPolicyResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{WatchGlobalContrailSecurityPolicyListOptional, WatchGlobalContrailSecurityPolicyListResponse};
#[cfg(feature = "api")] pub use self::global_contrail_security_policy::{WatchGlobalContrailSecurityPolicyStatusOptional, WatchGlobalContrailSecurityPolicyStatusResponse};

mod global_contrail_security_policy_pod_selector;
pub use self::global_contrail_security_policy_pod_selector::GlobalContrailSecurityPolicyPodSelector;

mod global_contrail_security_policy_spec;
pub use self::global_contrail_security_policy_spec::GlobalContrailSecurityPolicySpec;

mod global_contrail_security_policy_status;
pub use self::global_contrail_security_policy_status::GlobalContrailSecurityPolicyStatus;

mod global_system_config;
pub use self::global_system_config::GlobalSystemConfig;
#[cfg(feature = "api")] pub use self::global_system_config::{CreateGlobalSystemConfigOptional, CreateGlobalSystemConfigResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{CreateGlobalSystemConfigStatusOptional, CreateGlobalSystemConfigStatusResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{DeleteCollectionGlobalSystemConfigOptional, DeleteCollectionGlobalSystemConfigResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{DeleteGlobalSystemConfigOptional, DeleteGlobalSystemConfigResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{DeleteGlobalSystemConfigStatusOptional, DeleteGlobalSystemConfigStatusResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{ListGlobalSystemConfigOptional, ListGlobalSystemConfigResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{PatchGlobalSystemConfigOptional, PatchGlobalSystemConfigResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{PatchGlobalSystemConfigStatusOptional, PatchGlobalSystemConfigStatusResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{ReadGlobalSystemConfigOptional, ReadGlobalSystemConfigResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{ReadGlobalSystemConfigStatusOptional, ReadGlobalSystemConfigStatusResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{ReplaceGlobalSystemConfigOptional, ReplaceGlobalSystemConfigResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{ReplaceGlobalSystemConfigStatusOptional, ReplaceGlobalSystemConfigStatusResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{WatchGlobalSystemConfigOptional, WatchGlobalSystemConfigResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{WatchGlobalSystemConfigListOptional, WatchGlobalSystemConfigListResponse};
#[cfg(feature = "api")] pub use self::global_system_config::{WatchGlobalSystemConfigStatusOptional, WatchGlobalSystemConfigStatusResponse};

mod global_system_config_spec;
pub use self::global_system_config_spec::GlobalSystemConfigSpec;

mod global_system_config_status;
pub use self::global_system_config_status::GlobalSystemConfigStatus;

mod global_vrouter_config;
pub use self::global_vrouter_config::GlobalVrouterConfig;
#[cfg(feature = "api")] pub use self::global_vrouter_config::{CreateGlobalVrouterConfigOptional, CreateGlobalVrouterConfigResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{CreateGlobalVrouterConfigStatusOptional, CreateGlobalVrouterConfigStatusResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{DeleteCollectionGlobalVrouterConfigOptional, DeleteCollectionGlobalVrouterConfigResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{DeleteGlobalVrouterConfigOptional, DeleteGlobalVrouterConfigResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{DeleteGlobalVrouterConfigStatusOptional, DeleteGlobalVrouterConfigStatusResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{ListGlobalVrouterConfigOptional, ListGlobalVrouterConfigResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{PatchGlobalVrouterConfigOptional, PatchGlobalVrouterConfigResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{PatchGlobalVrouterConfigStatusOptional, PatchGlobalVrouterConfigStatusResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{ReadGlobalVrouterConfigOptional, ReadGlobalVrouterConfigResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{ReadGlobalVrouterConfigStatusOptional, ReadGlobalVrouterConfigStatusResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{ReplaceGlobalVrouterConfigOptional, ReplaceGlobalVrouterConfigResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{ReplaceGlobalVrouterConfigStatusOptional, ReplaceGlobalVrouterConfigStatusResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{WatchGlobalVrouterConfigOptional, WatchGlobalVrouterConfigResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{WatchGlobalVrouterConfigListOptional, WatchGlobalVrouterConfigListResponse};
#[cfg(feature = "api")] pub use self::global_vrouter_config::{WatchGlobalVrouterConfigStatusOptional, WatchGlobalVrouterConfigStatusResponse};

mod global_vrouter_config_spec;
pub use self::global_vrouter_config_spec::GlobalVrouterConfigSpec;

mod global_vrouter_config_status;
pub use self::global_vrouter_config_status::GlobalVrouterConfigStatus;

mod graceful_restart_parameters_type;
pub use self::graceful_restart_parameters_type::GracefulRestartParametersType;

mod ip_range;
pub use self::ip_range::IPRange;

mod import_virtual_network_router;
pub use self::import_virtual_network_router::ImportVirtualNetworkRouter;

mod instance_ip;
pub use self::instance_ip::InstanceIP;
#[cfg(feature = "api")] pub use self::instance_ip::{CreateInstanceIPOptional, CreateInstanceIPResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{CreateInstanceIPStatusOptional, CreateInstanceIPStatusResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{DeleteCollectionInstanceIPOptional, DeleteCollectionInstanceIPResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{DeleteInstanceIPOptional, DeleteInstanceIPResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{DeleteInstanceIPStatusOptional, DeleteInstanceIPStatusResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{ListInstanceIPOptional, ListInstanceIPResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{PatchInstanceIPOptional, PatchInstanceIPResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{PatchInstanceIPStatusOptional, PatchInstanceIPStatusResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{ReadInstanceIPOptional, ReadInstanceIPResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{ReadInstanceIPStatusOptional, ReadInstanceIPStatusResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{ReplaceInstanceIPOptional, ReplaceInstanceIPResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{ReplaceInstanceIPStatusOptional, ReplaceInstanceIPStatusResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{WatchInstanceIPOptional, WatchInstanceIPResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{WatchInstanceIPListOptional, WatchInstanceIPListResponse};
#[cfg(feature = "api")] pub use self::instance_ip::{WatchInstanceIPStatusOptional, WatchInstanceIPStatusResponse};

mod instance_ip_spec;
pub use self::instance_ip_spec::InstanceIPSpec;

mod instance_ip_status;
pub use self::instance_ip_status::InstanceIPStatus;

mod interface_route_table;
pub use self::interface_route_table::InterfaceRouteTable;
#[cfg(feature = "api")] pub use self::interface_route_table::{CreateInterfaceRouteTableOptional, CreateInterfaceRouteTableResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{CreateInterfaceRouteTableStatusOptional, CreateInterfaceRouteTableStatusResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{DeleteCollectionInterfaceRouteTableOptional, DeleteCollectionInterfaceRouteTableResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{DeleteInterfaceRouteTableOptional, DeleteInterfaceRouteTableResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{DeleteInterfaceRouteTableStatusOptional, DeleteInterfaceRouteTableStatusResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{ListInterfaceRouteTableForAllNamespacesOptional, ListInterfaceRouteTableForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{ListInterfaceRouteTableOptional, ListInterfaceRouteTableResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{PatchInterfaceRouteTableOptional, PatchInterfaceRouteTableResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{PatchInterfaceRouteTableStatusOptional, PatchInterfaceRouteTableStatusResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{ReadInterfaceRouteTableOptional, ReadInterfaceRouteTableResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{ReadInterfaceRouteTableStatusOptional, ReadInterfaceRouteTableStatusResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{ReplaceInterfaceRouteTableOptional, ReplaceInterfaceRouteTableResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{ReplaceInterfaceRouteTableStatusOptional, ReplaceInterfaceRouteTableStatusResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{WatchInterfaceRouteTableListForAllNamespacesOptional, WatchInterfaceRouteTableListForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{WatchInterfaceRouteTableOptional, WatchInterfaceRouteTableResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{WatchInterfaceRouteTableListOptional, WatchInterfaceRouteTableListResponse};
#[cfg(feature = "api")] pub use self::interface_route_table::{WatchInterfaceRouteTableStatusOptional, WatchInterfaceRouteTableStatusResponse};

mod interface_route_table_spec;
pub use self::interface_route_table_spec::InterfaceRouteTableSpec;

mod interface_route_table_status;
pub use self::interface_route_table_status::InterfaceRouteTableStatus;

mod interface_route_table_type;
pub use self::interface_route_table_type::InterfaceRouteTableType;

mod interface_route_type;
pub use self::interface_route_type::InterfaceRouteType;

mod ip_addresses;
pub use self::ip_addresses::IpAddresses;

mod key_value_pair;
pub use self::key_value_pair::KeyValuePair;

mod linklocal_service_entry_type;
pub use self::linklocal_service_entry_type::LinklocalServiceEntryType;

mod linklocal_services;
pub use self::linklocal_services::LinklocalServices;

mod mac_addresses;
pub use self::mac_addresses::MACAddresses;

mod mirror_destination;
pub use self::mirror_destination::MirrorDestination;
#[cfg(feature = "api")] pub use self::mirror_destination::{CreateMirrorDestinationOptional, CreateMirrorDestinationResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{CreateMirrorDestinationStatusOptional, CreateMirrorDestinationStatusResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{DeleteCollectionMirrorDestinationOptional, DeleteCollectionMirrorDestinationResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{DeleteMirrorDestinationOptional, DeleteMirrorDestinationResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{DeleteMirrorDestinationStatusOptional, DeleteMirrorDestinationStatusResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{ListMirrorDestinationOptional, ListMirrorDestinationResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{PatchMirrorDestinationOptional, PatchMirrorDestinationResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{PatchMirrorDestinationStatusOptional, PatchMirrorDestinationStatusResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{ReadMirrorDestinationOptional, ReadMirrorDestinationResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{ReadMirrorDestinationStatusOptional, ReadMirrorDestinationStatusResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{ReplaceMirrorDestinationOptional, ReplaceMirrorDestinationResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{ReplaceMirrorDestinationStatusOptional, ReplaceMirrorDestinationStatusResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{WatchMirrorDestinationOptional, WatchMirrorDestinationResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{WatchMirrorDestinationListOptional, WatchMirrorDestinationListResponse};
#[cfg(feature = "api")] pub use self::mirror_destination::{WatchMirrorDestinationStatusOptional, WatchMirrorDestinationStatusResponse};

mod mirror_destination_spec;
pub use self::mirror_destination_spec::MirrorDestinationSpec;

mod mirror_destination_status;
pub use self::mirror_destination_status::MirrorDestinationStatus;

mod nh_header_type;
pub use self::nh_header_type::NHHeaderType;

mod policy_based_forwarding_rule;
pub use self::policy_based_forwarding_rule::PolicyBasedForwardingRule;

mod port_range;
pub use self::port_range::PortRange;

mod port_translation_pool;
pub use self::port_translation_pool::PortTranslationPool;

mod port_translation_pools;
pub use self::port_translation_pools::PortTranslationPools;

mod port_type;
pub use self::port_type::PortType;

mod range;
pub use self::range::Range;

mod resource_reference;
pub use self::resource_reference::ResourceReference;

mod route_origin_override;
pub use self::route_origin_override::RouteOriginOverride;

mod route_table;
pub use self::route_table::RouteTable;
#[cfg(feature = "api")] pub use self::route_table::{CreateRouteTableOptional, CreateRouteTableResponse};
#[cfg(feature = "api")] pub use self::route_table::{CreateRouteTableStatusOptional, CreateRouteTableStatusResponse};
#[cfg(feature = "api")] pub use self::route_table::{DeleteCollectionRouteTableOptional, DeleteCollectionRouteTableResponse};
#[cfg(feature = "api")] pub use self::route_table::{DeleteRouteTableOptional, DeleteRouteTableResponse};
#[cfg(feature = "api")] pub use self::route_table::{DeleteRouteTableStatusOptional, DeleteRouteTableStatusResponse};
#[cfg(feature = "api")] pub use self::route_table::{ListRouteTableOptional, ListRouteTableResponse};
#[cfg(feature = "api")] pub use self::route_table::{ListRouteTableForAllNamespacesOptional, ListRouteTableForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::route_table::{PatchRouteTableOptional, PatchRouteTableResponse};
#[cfg(feature = "api")] pub use self::route_table::{PatchRouteTableStatusOptional, PatchRouteTableStatusResponse};
#[cfg(feature = "api")] pub use self::route_table::{ReadRouteTableOptional, ReadRouteTableResponse};
#[cfg(feature = "api")] pub use self::route_table::{ReadRouteTableStatusOptional, ReadRouteTableStatusResponse};
#[cfg(feature = "api")] pub use self::route_table::{ReplaceRouteTableOptional, ReplaceRouteTableResponse};
#[cfg(feature = "api")] pub use self::route_table::{ReplaceRouteTableStatusOptional, ReplaceRouteTableStatusResponse};
#[cfg(feature = "api")] pub use self::route_table::{WatchRouteTableOptional, WatchRouteTableResponse};
#[cfg(feature = "api")] pub use self::route_table::{WatchRouteTableListOptional, WatchRouteTableListResponse};
#[cfg(feature = "api")] pub use self::route_table::{WatchRouteTableStatusOptional, WatchRouteTableStatusResponse};
#[cfg(feature = "api")] pub use self::route_table::{WatchRouteTableListForAllNamespacesOptional, WatchRouteTableListForAllNamespacesResponse};

mod route_table_spec;
pub use self::route_table_spec::RouteTableSpec;

mod route_table_status;
pub use self::route_table_status::RouteTableStatus;

mod route_table_type;
pub use self::route_table_type::RouteTableType;

mod route_target;
pub use self::route_target::RouteTarget;
#[cfg(feature = "api")] pub use self::route_target::{CreateRouteTargetOptional, CreateRouteTargetResponse};
#[cfg(feature = "api")] pub use self::route_target::{CreateRouteTargetStatusOptional, CreateRouteTargetStatusResponse};
#[cfg(feature = "api")] pub use self::route_target::{DeleteCollectionRouteTargetOptional, DeleteCollectionRouteTargetResponse};
#[cfg(feature = "api")] pub use self::route_target::{DeleteRouteTargetOptional, DeleteRouteTargetResponse};
#[cfg(feature = "api")] pub use self::route_target::{DeleteRouteTargetStatusOptional, DeleteRouteTargetStatusResponse};
#[cfg(feature = "api")] pub use self::route_target::{ListRouteTargetOptional, ListRouteTargetResponse};
#[cfg(feature = "api")] pub use self::route_target::{PatchRouteTargetOptional, PatchRouteTargetResponse};
#[cfg(feature = "api")] pub use self::route_target::{PatchRouteTargetStatusOptional, PatchRouteTargetStatusResponse};
#[cfg(feature = "api")] pub use self::route_target::{ReadRouteTargetOptional, ReadRouteTargetResponse};
#[cfg(feature = "api")] pub use self::route_target::{ReadRouteTargetStatusOptional, ReadRouteTargetStatusResponse};
#[cfg(feature = "api")] pub use self::route_target::{ReplaceRouteTargetOptional, ReplaceRouteTargetResponse};
#[cfg(feature = "api")] pub use self::route_target::{ReplaceRouteTargetStatusOptional, ReplaceRouteTargetStatusResponse};
#[cfg(feature = "api")] pub use self::route_target::{WatchRouteTargetOptional, WatchRouteTargetResponse};
#[cfg(feature = "api")] pub use self::route_target::{WatchRouteTargetListOptional, WatchRouteTargetListResponse};
#[cfg(feature = "api")] pub use self::route_target::{WatchRouteTargetStatusOptional, WatchRouteTargetStatusResponse};

mod route_target_reference;
pub use self::route_target_reference::RouteTargetReference;

mod route_target_reference_attributes;
pub use self::route_target_reference_attributes::RouteTargetReferenceAttributes;

mod route_target_spec;
pub use self::route_target_spec::RouteTargetSpec;

mod route_target_status;
pub use self::route_target_status::RouteTargetStatus;

mod route_type;
pub use self::route_type::RouteType;

mod routing_instance;
pub use self::routing_instance::RoutingInstance;
#[cfg(feature = "api")] pub use self::routing_instance::{CreateRoutingInstanceOptional, CreateRoutingInstanceResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{CreateRoutingInstanceStatusOptional, CreateRoutingInstanceStatusResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{DeleteCollectionRoutingInstanceOptional, DeleteCollectionRoutingInstanceResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{DeleteRoutingInstanceOptional, DeleteRoutingInstanceResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{DeleteRoutingInstanceStatusOptional, DeleteRoutingInstanceStatusResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{ListRoutingInstanceOptional, ListRoutingInstanceResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{ListRoutingInstanceForAllNamespacesOptional, ListRoutingInstanceForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{PatchRoutingInstanceOptional, PatchRoutingInstanceResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{PatchRoutingInstanceStatusOptional, PatchRoutingInstanceStatusResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{ReadRoutingInstanceOptional, ReadRoutingInstanceResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{ReadRoutingInstanceStatusOptional, ReadRoutingInstanceStatusResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{ReplaceRoutingInstanceOptional, ReplaceRoutingInstanceResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{ReplaceRoutingInstanceStatusOptional, ReplaceRoutingInstanceStatusResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{WatchRoutingInstanceOptional, WatchRoutingInstanceResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{WatchRoutingInstanceListOptional, WatchRoutingInstanceListResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{WatchRoutingInstanceStatusOptional, WatchRoutingInstanceStatusResponse};
#[cfg(feature = "api")] pub use self::routing_instance::{WatchRoutingInstanceListForAllNamespacesOptional, WatchRoutingInstanceListForAllNamespacesResponse};

mod routing_instance_reference;
pub use self::routing_instance_reference::RoutingInstanceReference;

mod routing_instance_spec;
pub use self::routing_instance_spec::RoutingInstanceSpec;

mod routing_instance_status;
pub use self::routing_instance_status::RoutingInstanceStatus;

mod secondary_action_list;
pub use self::secondary_action_list::SecondaryActionList;

mod service_group;
pub use self::service_group::ServiceGroup;
#[cfg(feature = "api")] pub use self::service_group::{CreateServiceGroupOptional, CreateServiceGroupResponse};
#[cfg(feature = "api")] pub use self::service_group::{CreateServiceGroupStatusOptional, CreateServiceGroupStatusResponse};
#[cfg(feature = "api")] pub use self::service_group::{DeleteCollectionServiceGroupOptional, DeleteCollectionServiceGroupResponse};
#[cfg(feature = "api")] pub use self::service_group::{DeleteServiceGroupOptional, DeleteServiceGroupResponse};
#[cfg(feature = "api")] pub use self::service_group::{DeleteServiceGroupStatusOptional, DeleteServiceGroupStatusResponse};
#[cfg(feature = "api")] pub use self::service_group::{ListServiceGroupOptional, ListServiceGroupResponse};
#[cfg(feature = "api")] pub use self::service_group::{ListServiceGroupForAllNamespacesOptional, ListServiceGroupForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::service_group::{PatchServiceGroupOptional, PatchServiceGroupResponse};
#[cfg(feature = "api")] pub use self::service_group::{PatchServiceGroupStatusOptional, PatchServiceGroupStatusResponse};
#[cfg(feature = "api")] pub use self::service_group::{ReadServiceGroupOptional, ReadServiceGroupResponse};
#[cfg(feature = "api")] pub use self::service_group::{ReadServiceGroupStatusOptional, ReadServiceGroupStatusResponse};
#[cfg(feature = "api")] pub use self::service_group::{ReplaceServiceGroupOptional, ReplaceServiceGroupResponse};
#[cfg(feature = "api")] pub use self::service_group::{ReplaceServiceGroupStatusOptional, ReplaceServiceGroupStatusResponse};
#[cfg(feature = "api")] pub use self::service_group::{WatchServiceGroupOptional, WatchServiceGroupResponse};
#[cfg(feature = "api")] pub use self::service_group::{WatchServiceGroupListOptional, WatchServiceGroupListResponse};
#[cfg(feature = "api")] pub use self::service_group::{WatchServiceGroupStatusOptional, WatchServiceGroupStatusResponse};
#[cfg(feature = "api")] pub use self::service_group::{WatchServiceGroupListForAllNamespacesOptional, WatchServiceGroupListForAllNamespacesResponse};

mod service_group_spec;
pub use self::service_group_spec::ServiceGroupSpec;

mod service_group_status;
pub use self::service_group_status::ServiceGroupStatus;

mod service_health_check;
pub use self::service_health_check::ServiceHealthCheck;
#[cfg(feature = "api")] pub use self::service_health_check::{CreateServiceHealthCheckOptional, CreateServiceHealthCheckResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{CreateServiceHealthCheckStatusOptional, CreateServiceHealthCheckStatusResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{DeleteCollectionServiceHealthCheckOptional, DeleteCollectionServiceHealthCheckResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{DeleteServiceHealthCheckOptional, DeleteServiceHealthCheckResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{DeleteServiceHealthCheckStatusOptional, DeleteServiceHealthCheckStatusResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{ListServiceHealthCheckOptional, ListServiceHealthCheckResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{ListServiceHealthCheckForAllNamespacesOptional, ListServiceHealthCheckForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{PatchServiceHealthCheckOptional, PatchServiceHealthCheckResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{PatchServiceHealthCheckStatusOptional, PatchServiceHealthCheckStatusResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{ReadServiceHealthCheckOptional, ReadServiceHealthCheckResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{ReadServiceHealthCheckStatusOptional, ReadServiceHealthCheckStatusResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{ReplaceServiceHealthCheckOptional, ReplaceServiceHealthCheckResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{ReplaceServiceHealthCheckStatusOptional, ReplaceServiceHealthCheckStatusResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{WatchServiceHealthCheckOptional, WatchServiceHealthCheckResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{WatchServiceHealthCheckListOptional, WatchServiceHealthCheckListResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{WatchServiceHealthCheckStatusOptional, WatchServiceHealthCheckStatusResponse};
#[cfg(feature = "api")] pub use self::service_health_check::{WatchServiceHealthCheckListForAllNamespacesOptional, WatchServiceHealthCheckListForAllNamespacesResponse};

mod service_health_check_properties;
pub use self::service_health_check_properties::ServiceHealthCheckProperties;

mod service_health_check_spec;
pub use self::service_health_check_spec::ServiceHealthCheckSpec;

mod service_health_check_status;
pub use self::service_health_check_status::ServiceHealthCheckStatus;

mod static_nh_type;
pub use self::static_nh_type::StaticNHType;

mod static_route_entries_type;
pub use self::static_route_entries_type::StaticRouteEntriesType;

mod static_route_type;
pub use self::static_route_type::StaticRouteType;

mod subnet;
pub use self::subnet::Subnet;
#[cfg(feature = "api")] pub use self::subnet::{CreateSubnetOptional, CreateSubnetResponse};
#[cfg(feature = "api")] pub use self::subnet::{CreateSubnetStatusOptional, CreateSubnetStatusResponse};
#[cfg(feature = "api")] pub use self::subnet::{DeleteCollectionSubnetOptional, DeleteCollectionSubnetResponse};
#[cfg(feature = "api")] pub use self::subnet::{DeleteSubnetOptional, DeleteSubnetResponse};
#[cfg(feature = "api")] pub use self::subnet::{DeleteSubnetStatusOptional, DeleteSubnetStatusResponse};
#[cfg(feature = "api")] pub use self::subnet::{ListSubnetOptional, ListSubnetResponse};
#[cfg(feature = "api")] pub use self::subnet::{ListSubnetForAllNamespacesOptional, ListSubnetForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::subnet::{PatchSubnetOptional, PatchSubnetResponse};
#[cfg(feature = "api")] pub use self::subnet::{PatchSubnetStatusOptional, PatchSubnetStatusResponse};
#[cfg(feature = "api")] pub use self::subnet::{ReadSubnetOptional, ReadSubnetResponse};
#[cfg(feature = "api")] pub use self::subnet::{ReadSubnetStatusOptional, ReadSubnetStatusResponse};
#[cfg(feature = "api")] pub use self::subnet::{ReplaceSubnetOptional, ReplaceSubnetResponse};
#[cfg(feature = "api")] pub use self::subnet::{ReplaceSubnetStatusOptional, ReplaceSubnetStatusResponse};
#[cfg(feature = "api")] pub use self::subnet::{WatchSubnetOptional, WatchSubnetResponse};
#[cfg(feature = "api")] pub use self::subnet::{WatchSubnetListOptional, WatchSubnetListResponse};
#[cfg(feature = "api")] pub use self::subnet::{WatchSubnetStatusOptional, WatchSubnetStatusResponse};
#[cfg(feature = "api")] pub use self::subnet::{WatchSubnetListForAllNamespacesOptional, WatchSubnetListForAllNamespacesResponse};

mod subnet_reference;
pub use self::subnet_reference::SubnetReference;

mod subnet_spec;
pub use self::subnet_spec::SubnetSpec;

mod subnet_status;
pub use self::subnet_status::SubnetStatus;

mod tag;
pub use self::tag::Tag;
#[cfg(feature = "api")] pub use self::tag::{CreateTagOptional, CreateTagResponse};
#[cfg(feature = "api")] pub use self::tag::{CreateTagStatusOptional, CreateTagStatusResponse};
#[cfg(feature = "api")] pub use self::tag::{DeleteCollectionTagOptional, DeleteCollectionTagResponse};
#[cfg(feature = "api")] pub use self::tag::{DeleteTagOptional, DeleteTagResponse};
#[cfg(feature = "api")] pub use self::tag::{DeleteTagStatusOptional, DeleteTagStatusResponse};
#[cfg(feature = "api")] pub use self::tag::{ListTagOptional, ListTagResponse};
#[cfg(feature = "api")] pub use self::tag::{PatchTagOptional, PatchTagResponse};
#[cfg(feature = "api")] pub use self::tag::{PatchTagStatusOptional, PatchTagStatusResponse};
#[cfg(feature = "api")] pub use self::tag::{ReadTagOptional, ReadTagResponse};
#[cfg(feature = "api")] pub use self::tag::{ReadTagStatusOptional, ReadTagStatusResponse};
#[cfg(feature = "api")] pub use self::tag::{ReplaceTagOptional, ReplaceTagResponse};
#[cfg(feature = "api")] pub use self::tag::{ReplaceTagStatusOptional, ReplaceTagStatusResponse};
#[cfg(feature = "api")] pub use self::tag::{WatchTagOptional, WatchTagResponse};
#[cfg(feature = "api")] pub use self::tag::{WatchTagListOptional, WatchTagListResponse};
#[cfg(feature = "api")] pub use self::tag::{WatchTagStatusOptional, WatchTagStatusResponse};

mod tag_spec;
pub use self::tag_spec::TagSpec;

mod tag_status;
pub use self::tag_status::TagStatus;

mod tag_type;
pub use self::tag_type::TagType;
#[cfg(feature = "api")] pub use self::tag_type::{CreateTagTypeOptional, CreateTagTypeResponse};
#[cfg(feature = "api")] pub use self::tag_type::{CreateTagTypeStatusOptional, CreateTagTypeStatusResponse};
#[cfg(feature = "api")] pub use self::tag_type::{DeleteCollectionTagTypeOptional, DeleteCollectionTagTypeResponse};
#[cfg(feature = "api")] pub use self::tag_type::{DeleteTagTypeOptional, DeleteTagTypeResponse};
#[cfg(feature = "api")] pub use self::tag_type::{DeleteTagTypeStatusOptional, DeleteTagTypeStatusResponse};
#[cfg(feature = "api")] pub use self::tag_type::{ListTagTypeOptional, ListTagTypeResponse};
#[cfg(feature = "api")] pub use self::tag_type::{PatchTagTypeOptional, PatchTagTypeResponse};
#[cfg(feature = "api")] pub use self::tag_type::{PatchTagTypeStatusOptional, PatchTagTypeStatusResponse};
#[cfg(feature = "api")] pub use self::tag_type::{ReadTagTypeOptional, ReadTagTypeResponse};
#[cfg(feature = "api")] pub use self::tag_type::{ReadTagTypeStatusOptional, ReadTagTypeStatusResponse};
#[cfg(feature = "api")] pub use self::tag_type::{ReplaceTagTypeOptional, ReplaceTagTypeResponse};
#[cfg(feature = "api")] pub use self::tag_type::{ReplaceTagTypeStatusOptional, ReplaceTagTypeStatusResponse};
#[cfg(feature = "api")] pub use self::tag_type::{WatchTagTypeOptional, WatchTagTypeResponse};
#[cfg(feature = "api")] pub use self::tag_type::{WatchTagTypeListOptional, WatchTagTypeListResponse};
#[cfg(feature = "api")] pub use self::tag_type::{WatchTagTypeStatusOptional, WatchTagTypeStatusResponse};

mod tag_type_spec;
pub use self::tag_type_spec::TagTypeSpec;

mod tag_type_status;
pub use self::tag_type_status::TagTypeStatus;

mod virtual_machine;
pub use self::virtual_machine::VirtualMachine;
#[cfg(feature = "api")] pub use self::virtual_machine::{CreateVirtualMachineOptional, CreateVirtualMachineResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{CreateVirtualMachineStatusOptional, CreateVirtualMachineStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{DeleteCollectionVirtualMachineOptional, DeleteCollectionVirtualMachineResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{DeleteVirtualMachineOptional, DeleteVirtualMachineResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{DeleteVirtualMachineStatusOptional, DeleteVirtualMachineStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{ListVirtualMachineOptional, ListVirtualMachineResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{PatchVirtualMachineOptional, PatchVirtualMachineResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{PatchVirtualMachineStatusOptional, PatchVirtualMachineStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{ReadVirtualMachineOptional, ReadVirtualMachineResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{ReadVirtualMachineStatusOptional, ReadVirtualMachineStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{ReplaceVirtualMachineOptional, ReplaceVirtualMachineResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{ReplaceVirtualMachineStatusOptional, ReplaceVirtualMachineStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{WatchVirtualMachineOptional, WatchVirtualMachineResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{WatchVirtualMachineListOptional, WatchVirtualMachineListResponse};
#[cfg(feature = "api")] pub use self::virtual_machine::{WatchVirtualMachineStatusOptional, WatchVirtualMachineStatusResponse};

mod virtual_machine_interface;
pub use self::virtual_machine_interface::VirtualMachineInterface;
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{CreateVirtualMachineInterfaceOptional, CreateVirtualMachineInterfaceResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{CreateVirtualMachineInterfaceStatusOptional, CreateVirtualMachineInterfaceStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{DeleteCollectionVirtualMachineInterfaceOptional, DeleteCollectionVirtualMachineInterfaceResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{DeleteVirtualMachineInterfaceOptional, DeleteVirtualMachineInterfaceResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{DeleteVirtualMachineInterfaceStatusOptional, DeleteVirtualMachineInterfaceStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{ListVirtualMachineInterfaceOptional, ListVirtualMachineInterfaceResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{ListVirtualMachineInterfaceForAllNamespacesOptional, ListVirtualMachineInterfaceForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{PatchVirtualMachineInterfaceOptional, PatchVirtualMachineInterfaceResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{PatchVirtualMachineInterfaceStatusOptional, PatchVirtualMachineInterfaceStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{ReadVirtualMachineInterfaceOptional, ReadVirtualMachineInterfaceResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{ReadVirtualMachineInterfaceStatusOptional, ReadVirtualMachineInterfaceStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{ReplaceVirtualMachineInterfaceOptional, ReplaceVirtualMachineInterfaceResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{ReplaceVirtualMachineInterfaceStatusOptional, ReplaceVirtualMachineInterfaceStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{WatchVirtualMachineInterfaceOptional, WatchVirtualMachineInterfaceResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{WatchVirtualMachineInterfaceListOptional, WatchVirtualMachineInterfaceListResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{WatchVirtualMachineInterfaceStatusOptional, WatchVirtualMachineInterfaceStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_machine_interface::{WatchVirtualMachineInterfaceListForAllNamespacesOptional, WatchVirtualMachineInterfaceListForAllNamespacesResponse};

mod virtual_machine_interface_properties;
pub use self::virtual_machine_interface_properties::VirtualMachineInterfaceProperties;

mod virtual_machine_interface_spec;
pub use self::virtual_machine_interface_spec::VirtualMachineInterfaceSpec;

mod virtual_machine_interface_status;
pub use self::virtual_machine_interface_status::VirtualMachineInterfaceStatus;

mod virtual_machine_spec;
pub use self::virtual_machine_spec::VirtualMachineSpec;

mod virtual_machine_status;
pub use self::virtual_machine_status::VirtualMachineStatus;

mod virtual_network;
pub use self::virtual_network::VirtualNetwork;
#[cfg(feature = "api")] pub use self::virtual_network::{CreateVirtualNetworkOptional, CreateVirtualNetworkResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{CreateVirtualNetworkStatusOptional, CreateVirtualNetworkStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{DeleteCollectionVirtualNetworkOptional, DeleteCollectionVirtualNetworkResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{DeleteVirtualNetworkOptional, DeleteVirtualNetworkResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{DeleteVirtualNetworkStatusOptional, DeleteVirtualNetworkStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{ListVirtualNetworkOptional, ListVirtualNetworkResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{ListVirtualNetworkForAllNamespacesOptional, ListVirtualNetworkForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{PatchVirtualNetworkOptional, PatchVirtualNetworkResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{PatchVirtualNetworkStatusOptional, PatchVirtualNetworkStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{ReadVirtualNetworkOptional, ReadVirtualNetworkResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{ReadVirtualNetworkStatusOptional, ReadVirtualNetworkStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{ReplaceVirtualNetworkOptional, ReplaceVirtualNetworkResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{ReplaceVirtualNetworkStatusOptional, ReplaceVirtualNetworkStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{WatchVirtualNetworkOptional, WatchVirtualNetworkResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{WatchVirtualNetworkListOptional, WatchVirtualNetworkListResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{WatchVirtualNetworkStatusOptional, WatchVirtualNetworkStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_network::{WatchVirtualNetworkListForAllNamespacesOptional, WatchVirtualNetworkListForAllNamespacesResponse};

mod virtual_network_reference;
pub use self::virtual_network_reference::VirtualNetworkReference;

mod virtual_network_route_target_reference_list;
pub use self::virtual_network_route_target_reference_list::VirtualNetworkRouteTargetReferenceList;

mod virtual_network_router;
pub use self::virtual_network_router::VirtualNetworkRouter;
#[cfg(feature = "api")] pub use self::virtual_network_router::{CreateVirtualNetworkRouterOptional, CreateVirtualNetworkRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{CreateVirtualNetworkRouterStatusOptional, CreateVirtualNetworkRouterStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{DeleteCollectionVirtualNetworkRouterOptional, DeleteCollectionVirtualNetworkRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{DeleteVirtualNetworkRouterOptional, DeleteVirtualNetworkRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{DeleteVirtualNetworkRouterStatusOptional, DeleteVirtualNetworkRouterStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{ListVirtualNetworkRouterOptional, ListVirtualNetworkRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{ListVirtualNetworkRouterForAllNamespacesOptional, ListVirtualNetworkRouterForAllNamespacesResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{PatchVirtualNetworkRouterOptional, PatchVirtualNetworkRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{PatchVirtualNetworkRouterStatusOptional, PatchVirtualNetworkRouterStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{ReadVirtualNetworkRouterOptional, ReadVirtualNetworkRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{ReadVirtualNetworkRouterStatusOptional, ReadVirtualNetworkRouterStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{ReplaceVirtualNetworkRouterOptional, ReplaceVirtualNetworkRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{ReplaceVirtualNetworkRouterStatusOptional, ReplaceVirtualNetworkRouterStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{WatchVirtualNetworkRouterOptional, WatchVirtualNetworkRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{WatchVirtualNetworkRouterListOptional, WatchVirtualNetworkRouterListResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{WatchVirtualNetworkRouterStatusOptional, WatchVirtualNetworkRouterStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_network_router::{WatchVirtualNetworkRouterListForAllNamespacesOptional, WatchVirtualNetworkRouterListForAllNamespacesResponse};

mod virtual_network_router_entry;
pub use self::virtual_network_router_entry::VirtualNetworkRouterEntry;

mod virtual_network_router_spec;
pub use self::virtual_network_router_spec::VirtualNetworkRouterSpec;

mod virtual_network_router_status;
pub use self::virtual_network_router_status::VirtualNetworkRouterStatus;

mod virtual_network_spec;
pub use self::virtual_network_spec::VirtualNetworkSpec;

mod virtual_network_status;
pub use self::virtual_network_status::VirtualNetworkStatus;

mod virtual_network_type;
pub use self::virtual_network_type::VirtualNetworkType;

mod virtual_router;
pub use self::virtual_router::VirtualRouter;
#[cfg(feature = "api")] pub use self::virtual_router::{CreateVirtualRouterOptional, CreateVirtualRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{CreateVirtualRouterStatusOptional, CreateVirtualRouterStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{DeleteCollectionVirtualRouterOptional, DeleteCollectionVirtualRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{DeleteVirtualRouterOptional, DeleteVirtualRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{DeleteVirtualRouterStatusOptional, DeleteVirtualRouterStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{ListVirtualRouterOptional, ListVirtualRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{PatchVirtualRouterOptional, PatchVirtualRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{PatchVirtualRouterStatusOptional, PatchVirtualRouterStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{ReadVirtualRouterOptional, ReadVirtualRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{ReadVirtualRouterStatusOptional, ReadVirtualRouterStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{ReplaceVirtualRouterOptional, ReplaceVirtualRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{ReplaceVirtualRouterStatusOptional, ReplaceVirtualRouterStatusResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{WatchVirtualRouterOptional, WatchVirtualRouterResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{WatchVirtualRouterListOptional, WatchVirtualRouterListResponse};
#[cfg(feature = "api")] pub use self::virtual_router::{WatchVirtualRouterStatusOptional, WatchVirtualRouterStatusResponse};

mod virtual_router_spec;
pub use self::virtual_router_spec::VirtualRouterSpec;

mod virtual_router_status;
pub use self::virtual_router_status::VirtualRouterStatus;
