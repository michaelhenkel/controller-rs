pub mod k8s {
    pub mod io {
        pub mod api {
            pub mod core {
                pub mod v1 {
                    include!("k8s.io.api.core.v1.rs");
                    include!("k8s.io.api.core.v1.serde.rs");
                }
            }
            pub mod networking {
                pub mod v1 {
                    include!("k8s.io.api.networking.v1.rs");
                    include!("k8s.io.api.networking.v1.serde.rs");
                }
            }
        }
        pub mod apimachinery {
            pub mod pkg {
                pub mod api {
                    pub mod resource {
                        include!("k8s.io.apimachinery.pkg.api.resource.rs");
                        include!("k8s.io.apimachinery.pkg.api.resource.serde.rs");
                    }
                }
                pub mod apis {
                    pub mod meta {
                        pub mod v1 {
                            include!("k8s.io.apimachinery.pkg.apis.meta.v1.rs");
                            include!("k8s.io.apimachinery.pkg.apis.meta.v1.serde.rs");
                        }
                    }
                }
                pub mod runtime {
                    include!("k8s.io.apimachinery.pkg.runtime.rs");
                    include!("k8s.io.apimachinery.pkg.runtime.serde.rs");
                }
                pub mod util {
                    pub mod intstr {
                        include!("k8s.io.apimachinery.pkg.util.intstr.rs");
                        include!("k8s.io.apimachinery.pkg.util.intstr.serde.rs");
                    }
                }
            }
        }
    }
}
pub mod ssd_git {
    pub mod juniper {
        pub mod net {
            pub mod contrail {
                pub mod cn2 {
                    pub mod contrail {
                        pub mod pkg {
                            pub mod apis {
                                pub mod core {
                                    pub mod v4 {
                                        include!("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.rs");
                                        include!("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.serde.rs");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}