impl serde::Serialize for ApiGroup {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if !self.versions.is_empty() {
            len += 1;
        }
        if self.preferred_version.is_some() {
            len += 1;
        }
        if !self.server_address_by_client_cid_rs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.APIGroup", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if !self.versions.is_empty() {
            struct_ser.serialize_field("versions", &self.versions)?;
        }
        if let Some(v) = self.preferred_version.as_ref() {
            struct_ser.serialize_field("preferredVersion", v)?;
        }
        if !self.server_address_by_client_cid_rs.is_empty() {
            struct_ser.serialize_field("serverAddressByClientCIDRs", &self.server_address_by_client_cid_rs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiGroup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "versions",
            "preferredVersion",
            "serverAddressByClientCIDRs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Versions,
            PreferredVersion,
            ServerAddressByClientCidRs,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "versions" => Ok(GeneratedField::Versions),
                            "preferredVersion" => Ok(GeneratedField::PreferredVersion),
                            "serverAddressByClientCIDRs" => Ok(GeneratedField::ServerAddressByClientCidRs),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiGroup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.APIGroup")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApiGroup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut versions__ = None;
                let mut preferred_version__ = None;
                let mut server_address_by_client_cid_rs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Versions => {
                            if versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versions"));
                            }
                            versions__ = Some(map.next_value()?);
                        }
                        GeneratedField::PreferredVersion => {
                            if preferred_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredVersion"));
                            }
                            preferred_version__ = map.next_value()?;
                        }
                        GeneratedField::ServerAddressByClientCidRs => {
                            if server_address_by_client_cid_rs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverAddressByClientCIDRs"));
                            }
                            server_address_by_client_cid_rs__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ApiGroup {
                    name: name__,
                    versions: versions__.unwrap_or_default(),
                    preferred_version: preferred_version__,
                    server_address_by_client_cid_rs: server_address_by_client_cid_rs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.APIGroup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApiGroupList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.groups.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.APIGroupList", len)?;
        if !self.groups.is_empty() {
            struct_ser.serialize_field("groups", &self.groups)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiGroupList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "groups",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Groups,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "groups" => Ok(GeneratedField::Groups),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiGroupList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.APIGroupList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApiGroupList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut groups__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Groups => {
                            if groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groups"));
                            }
                            groups__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ApiGroupList {
                    groups: groups__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.APIGroupList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApiResource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.singular_name.is_some() {
            len += 1;
        }
        if self.namespaced.is_some() {
            len += 1;
        }
        if self.group.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        if self.verbs.is_some() {
            len += 1;
        }
        if !self.short_names.is_empty() {
            len += 1;
        }
        if !self.categories.is_empty() {
            len += 1;
        }
        if self.storage_version_hash.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.APIResource", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.singular_name.as_ref() {
            struct_ser.serialize_field("singularName", v)?;
        }
        if let Some(v) = self.namespaced.as_ref() {
            struct_ser.serialize_field("namespaced", v)?;
        }
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.kind.as_ref() {
            struct_ser.serialize_field("kind", v)?;
        }
        if let Some(v) = self.verbs.as_ref() {
            struct_ser.serialize_field("verbs", v)?;
        }
        if !self.short_names.is_empty() {
            struct_ser.serialize_field("shortNames", &self.short_names)?;
        }
        if !self.categories.is_empty() {
            struct_ser.serialize_field("categories", &self.categories)?;
        }
        if let Some(v) = self.storage_version_hash.as_ref() {
            struct_ser.serialize_field("storageVersionHash", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiResource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "singularName",
            "namespaced",
            "group",
            "version",
            "kind",
            "verbs",
            "shortNames",
            "categories",
            "storageVersionHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            SingularName,
            Namespaced,
            Group,
            Version,
            Kind,
            Verbs,
            ShortNames,
            Categories,
            StorageVersionHash,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "singularName" => Ok(GeneratedField::SingularName),
                            "namespaced" => Ok(GeneratedField::Namespaced),
                            "group" => Ok(GeneratedField::Group),
                            "version" => Ok(GeneratedField::Version),
                            "kind" => Ok(GeneratedField::Kind),
                            "verbs" => Ok(GeneratedField::Verbs),
                            "shortNames" => Ok(GeneratedField::ShortNames),
                            "categories" => Ok(GeneratedField::Categories),
                            "storageVersionHash" => Ok(GeneratedField::StorageVersionHash),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiResource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.APIResource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApiResource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut singular_name__ = None;
                let mut namespaced__ = None;
                let mut group__ = None;
                let mut version__ = None;
                let mut kind__ = None;
                let mut verbs__ = None;
                let mut short_names__ = None;
                let mut categories__ = None;
                let mut storage_version_hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::SingularName => {
                            if singular_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("singularName"));
                            }
                            singular_name__ = map.next_value()?;
                        }
                        GeneratedField::Namespaced => {
                            if namespaced__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaced"));
                            }
                            namespaced__ = map.next_value()?;
                        }
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = map.next_value()?;
                        }
                        GeneratedField::Verbs => {
                            if verbs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verbs"));
                            }
                            verbs__ = map.next_value()?;
                        }
                        GeneratedField::ShortNames => {
                            if short_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shortNames"));
                            }
                            short_names__ = Some(map.next_value()?);
                        }
                        GeneratedField::Categories => {
                            if categories__.is_some() {
                                return Err(serde::de::Error::duplicate_field("categories"));
                            }
                            categories__ = Some(map.next_value()?);
                        }
                        GeneratedField::StorageVersionHash => {
                            if storage_version_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageVersionHash"));
                            }
                            storage_version_hash__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ApiResource {
                    name: name__,
                    singular_name: singular_name__,
                    namespaced: namespaced__,
                    group: group__,
                    version: version__,
                    kind: kind__,
                    verbs: verbs__,
                    short_names: short_names__.unwrap_or_default(),
                    categories: categories__.unwrap_or_default(),
                    storage_version_hash: storage_version_hash__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.APIResource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApiResourceList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group_version.is_some() {
            len += 1;
        }
        if !self.resources.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.APIResourceList", len)?;
        if let Some(v) = self.group_version.as_ref() {
            struct_ser.serialize_field("groupVersion", v)?;
        }
        if !self.resources.is_empty() {
            struct_ser.serialize_field("resources", &self.resources)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiResourceList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "groupVersion",
            "resources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GroupVersion,
            Resources,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "groupVersion" => Ok(GeneratedField::GroupVersion),
                            "resources" => Ok(GeneratedField::Resources),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiResourceList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.APIResourceList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApiResourceList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group_version__ = None;
                let mut resources__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GroupVersion => {
                            if group_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupVersion"));
                            }
                            group_version__ = map.next_value()?;
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ApiResourceList {
                    group_version: group_version__,
                    resources: resources__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.APIResourceList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApiVersions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.versions.is_empty() {
            len += 1;
        }
        if !self.server_address_by_client_cid_rs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.APIVersions", len)?;
        if !self.versions.is_empty() {
            struct_ser.serialize_field("versions", &self.versions)?;
        }
        if !self.server_address_by_client_cid_rs.is_empty() {
            struct_ser.serialize_field("serverAddressByClientCIDRs", &self.server_address_by_client_cid_rs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiVersions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "versions",
            "serverAddressByClientCIDRs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Versions,
            ServerAddressByClientCidRs,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "versions" => Ok(GeneratedField::Versions),
                            "serverAddressByClientCIDRs" => Ok(GeneratedField::ServerAddressByClientCidRs),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiVersions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.APIVersions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApiVersions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut versions__ = None;
                let mut server_address_by_client_cid_rs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Versions => {
                            if versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versions"));
                            }
                            versions__ = Some(map.next_value()?);
                        }
                        GeneratedField::ServerAddressByClientCidRs => {
                            if server_address_by_client_cid_rs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverAddressByClientCIDRs"));
                            }
                            server_address_by_client_cid_rs__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ApiVersions {
                    versions: versions__.unwrap_or_default(),
                    server_address_by_client_cid_rs: server_address_by_client_cid_rs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.APIVersions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApplyOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dry_run.is_empty() {
            len += 1;
        }
        if self.force.is_some() {
            len += 1;
        }
        if self.field_manager.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.ApplyOptions", len)?;
        if !self.dry_run.is_empty() {
            struct_ser.serialize_field("dryRun", &self.dry_run)?;
        }
        if let Some(v) = self.force.as_ref() {
            struct_ser.serialize_field("force", v)?;
        }
        if let Some(v) = self.field_manager.as_ref() {
            struct_ser.serialize_field("fieldManager", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApplyOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dryRun",
            "force",
            "fieldManager",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DryRun,
            Force,
            FieldManager,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dryRun" => Ok(GeneratedField::DryRun),
                            "force" => Ok(GeneratedField::Force),
                            "fieldManager" => Ok(GeneratedField::FieldManager),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApplyOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.ApplyOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApplyOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dry_run__ = None;
                let mut force__ = None;
                let mut field_manager__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map.next_value()?);
                        }
                        GeneratedField::Force => {
                            if force__.is_some() {
                                return Err(serde::de::Error::duplicate_field("force"));
                            }
                            force__ = map.next_value()?;
                        }
                        GeneratedField::FieldManager => {
                            if field_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldManager"));
                            }
                            field_manager__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ApplyOptions {
                    dry_run: dry_run__.unwrap_or_default(),
                    force: force__,
                    field_manager: field_manager__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.ApplyOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Condition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.observed_generation.is_some() {
            len += 1;
        }
        if self.last_transition_time.is_some() {
            len += 1;
        }
        if self.reason.is_some() {
            len += 1;
        }
        if self.message.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Condition", len)?;
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.observed_generation.as_ref() {
            struct_ser.serialize_field("observedGeneration", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.last_transition_time.as_ref() {
            struct_ser.serialize_field("lastTransitionTime", v)?;
        }
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        if let Some(v) = self.message.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Condition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "status",
            "observedGeneration",
            "lastTransitionTime",
            "reason",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Status,
            ObservedGeneration,
            LastTransitionTime,
            Reason,
            Message,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "status" => Ok(GeneratedField::Status),
                            "observedGeneration" => Ok(GeneratedField::ObservedGeneration),
                            "lastTransitionTime" => Ok(GeneratedField::LastTransitionTime),
                            "reason" => Ok(GeneratedField::Reason),
                            "message" => Ok(GeneratedField::Message),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Condition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.Condition")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Condition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut status__ = None;
                let mut observed_generation__ = None;
                let mut last_transition_time__ = None;
                let mut reason__ = None;
                let mut message__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                        GeneratedField::ObservedGeneration => {
                            if observed_generation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("observedGeneration"));
                            }
                            observed_generation__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::LastTransitionTime => {
                            if last_transition_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastTransitionTime"));
                            }
                            last_transition_time__ = map.next_value()?;
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map.next_value()?;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Condition {
                    r#type: r#type__,
                    status: status__,
                    observed_generation: observed_generation__,
                    last_transition_time: last_transition_time__,
                    reason: reason__,
                    message: message__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Condition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dry_run.is_empty() {
            len += 1;
        }
        if self.field_manager.is_some() {
            len += 1;
        }
        if self.field_validation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.CreateOptions", len)?;
        if !self.dry_run.is_empty() {
            struct_ser.serialize_field("dryRun", &self.dry_run)?;
        }
        if let Some(v) = self.field_manager.as_ref() {
            struct_ser.serialize_field("fieldManager", v)?;
        }
        if let Some(v) = self.field_validation.as_ref() {
            struct_ser.serialize_field("fieldValidation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dryRun",
            "fieldManager",
            "fieldValidation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DryRun,
            FieldManager,
            FieldValidation,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dryRun" => Ok(GeneratedField::DryRun),
                            "fieldManager" => Ok(GeneratedField::FieldManager),
                            "fieldValidation" => Ok(GeneratedField::FieldValidation),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.CreateOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dry_run__ = None;
                let mut field_manager__ = None;
                let mut field_validation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map.next_value()?);
                        }
                        GeneratedField::FieldManager => {
                            if field_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldManager"));
                            }
                            field_manager__ = map.next_value()?;
                        }
                        GeneratedField::FieldValidation => {
                            if field_validation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldValidation"));
                            }
                            field_validation__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateOptions {
                    dry_run: dry_run__.unwrap_or_default(),
                    field_manager: field_manager__,
                    field_validation: field_validation__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.CreateOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.grace_period_seconds.is_some() {
            len += 1;
        }
        if self.preconditions.is_some() {
            len += 1;
        }
        if self.orphan_dependents.is_some() {
            len += 1;
        }
        if self.propagation_policy.is_some() {
            len += 1;
        }
        if !self.dry_run.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.DeleteOptions", len)?;
        if let Some(v) = self.grace_period_seconds.as_ref() {
            struct_ser.serialize_field("gracePeriodSeconds", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.preconditions.as_ref() {
            struct_ser.serialize_field("preconditions", v)?;
        }
        if let Some(v) = self.orphan_dependents.as_ref() {
            struct_ser.serialize_field("orphanDependents", v)?;
        }
        if let Some(v) = self.propagation_policy.as_ref() {
            struct_ser.serialize_field("propagationPolicy", v)?;
        }
        if !self.dry_run.is_empty() {
            struct_ser.serialize_field("dryRun", &self.dry_run)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gracePeriodSeconds",
            "preconditions",
            "orphanDependents",
            "propagationPolicy",
            "dryRun",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GracePeriodSeconds,
            Preconditions,
            OrphanDependents,
            PropagationPolicy,
            DryRun,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "gracePeriodSeconds" => Ok(GeneratedField::GracePeriodSeconds),
                            "preconditions" => Ok(GeneratedField::Preconditions),
                            "orphanDependents" => Ok(GeneratedField::OrphanDependents),
                            "propagationPolicy" => Ok(GeneratedField::PropagationPolicy),
                            "dryRun" => Ok(GeneratedField::DryRun),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.DeleteOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grace_period_seconds__ = None;
                let mut preconditions__ = None;
                let mut orphan_dependents__ = None;
                let mut propagation_policy__ = None;
                let mut dry_run__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GracePeriodSeconds => {
                            if grace_period_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gracePeriodSeconds"));
                            }
                            grace_period_seconds__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Preconditions => {
                            if preconditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preconditions"));
                            }
                            preconditions__ = map.next_value()?;
                        }
                        GeneratedField::OrphanDependents => {
                            if orphan_dependents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orphanDependents"));
                            }
                            orphan_dependents__ = map.next_value()?;
                        }
                        GeneratedField::PropagationPolicy => {
                            if propagation_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("propagationPolicy"));
                            }
                            propagation_policy__ = map.next_value()?;
                        }
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteOptions {
                    grace_period_seconds: grace_period_seconds__,
                    preconditions: preconditions__,
                    orphan_dependents: orphan_dependents__,
                    propagation_policy: propagation_policy__,
                    dry_run: dry_run__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.DeleteOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Duration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.duration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Duration", len)?;
        if let Some(v) = self.duration.as_ref() {
            struct_ser.serialize_field("duration", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Duration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "duration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Duration,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "duration" => Ok(GeneratedField::Duration),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.Duration")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Duration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut duration__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Duration {
                    duration: duration__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Duration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FieldsV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.raw.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.FieldsV1", len)?;
        if let Some(v) = self.raw.as_ref() {
            struct_ser.serialize_field("Raw", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldsV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Raw",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Raw,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "Raw" => Ok(GeneratedField::Raw),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldsV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.FieldsV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FieldsV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut raw__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Raw => {
                            if raw__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Raw"));
                            }
                            raw__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(FieldsV1 {
                    raw: raw__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.FieldsV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GetOptions", len)?;
        if let Some(v) = self.resource_version.as_ref() {
            struct_ser.serialize_field("resourceVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resourceVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceVersion,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resourceVersion" => Ok(GeneratedField::ResourceVersion),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.GetOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceVersion => {
                            if resource_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceVersion"));
                            }
                            resource_version__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetOptions {
                    resource_version: resource_version__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GetOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GroupKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group.is_some() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GroupKind", len)?;
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        if let Some(v) = self.kind.as_ref() {
            struct_ser.serialize_field("kind", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GroupKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group",
            "kind",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Group,
            Kind,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "group" => Ok(GeneratedField::Group),
                            "kind" => Ok(GeneratedField::Kind),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GroupKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.GroupKind")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GroupKind, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group__ = None;
                let mut kind__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map.next_value()?;
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GroupKind {
                    group: group__,
                    kind: kind__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GroupKind", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GroupResource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group.is_some() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GroupResource", len)?;
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GroupResource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group",
            "resource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Group,
            Resource,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "group" => Ok(GeneratedField::Group),
                            "resource" => Ok(GeneratedField::Resource),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GroupResource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.GroupResource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GroupResource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group__ = None;
                let mut resource__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map.next_value()?;
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GroupResource {
                    group: group__,
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GroupResource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GroupVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GroupVersion", len)?;
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GroupVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group",
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Group,
            Version,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "group" => Ok(GeneratedField::Group),
                            "version" => Ok(GeneratedField::Version),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GroupVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.GroupVersion")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GroupVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group__ = None;
                let mut version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GroupVersion {
                    group: group__,
                    version: version__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GroupVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GroupVersionForDiscovery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group_version.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GroupVersionForDiscovery", len)?;
        if let Some(v) = self.group_version.as_ref() {
            struct_ser.serialize_field("groupVersion", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GroupVersionForDiscovery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "groupVersion",
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GroupVersion,
            Version,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "groupVersion" => Ok(GeneratedField::GroupVersion),
                            "version" => Ok(GeneratedField::Version),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GroupVersionForDiscovery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.GroupVersionForDiscovery")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GroupVersionForDiscovery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group_version__ = None;
                let mut version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GroupVersion => {
                            if group_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupVersion"));
                            }
                            group_version__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GroupVersionForDiscovery {
                    group_version: group_version__,
                    version: version__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GroupVersionForDiscovery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GroupVersionKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GroupVersionKind", len)?;
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.kind.as_ref() {
            struct_ser.serialize_field("kind", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GroupVersionKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group",
            "version",
            "kind",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Group,
            Version,
            Kind,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "group" => Ok(GeneratedField::Group),
                            "version" => Ok(GeneratedField::Version),
                            "kind" => Ok(GeneratedField::Kind),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GroupVersionKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.GroupVersionKind")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GroupVersionKind, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group__ = None;
                let mut version__ = None;
                let mut kind__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GroupVersionKind {
                    group: group__,
                    version: version__,
                    kind: kind__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GroupVersionKind", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GroupVersionResource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GroupVersionResource", len)?;
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GroupVersionResource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group",
            "version",
            "resource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Group,
            Version,
            Resource,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "group" => Ok(GeneratedField::Group),
                            "version" => Ok(GeneratedField::Version),
                            "resource" => Ok(GeneratedField::Resource),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GroupVersionResource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.GroupVersionResource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GroupVersionResource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group__ = None;
                let mut version__ = None;
                let mut resource__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GroupVersionResource {
                    group: group__,
                    version: version__,
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.GroupVersionResource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LabelSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.match_labels.is_empty() {
            len += 1;
        }
        if !self.match_expressions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelector", len)?;
        if !self.match_labels.is_empty() {
            struct_ser.serialize_field("matchLabels", &self.match_labels)?;
        }
        if !self.match_expressions.is_empty() {
            struct_ser.serialize_field("matchExpressions", &self.match_expressions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LabelSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "matchLabels",
            "matchExpressions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MatchLabels,
            MatchExpressions,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "matchLabels" => Ok(GeneratedField::MatchLabels),
                            "matchExpressions" => Ok(GeneratedField::MatchExpressions),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LabelSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelector")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LabelSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut match_labels__ = None;
                let mut match_expressions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MatchLabels => {
                            if match_labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchLabels"));
                            }
                            match_labels__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::MatchExpressions => {
                            if match_expressions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchExpressions"));
                            }
                            match_expressions__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(LabelSelector {
                    match_labels: match_labels__.unwrap_or_default(),
                    match_expressions: match_expressions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LabelSelectorRequirement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if self.operator.is_some() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelectorRequirement", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.operator.as_ref() {
            struct_ser.serialize_field("operator", v)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LabelSelectorRequirement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "operator",
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Operator,
            Values,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "operator" => Ok(GeneratedField::Operator),
                            "values" => Ok(GeneratedField::Values),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LabelSelectorRequirement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelectorRequirement")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LabelSelectorRequirement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut operator__ = None;
                let mut values__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Operator => {
                            if operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            operator__ = map.next_value()?;
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(LabelSelectorRequirement {
                    key: key__,
                    operator: operator__,
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelectorRequirement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for List {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if !self.items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.List", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for List {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "items",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Items,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "items" => Ok(GeneratedField::Items),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = List;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.List")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<List, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut items__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(List {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.List", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMeta {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.self_link.is_some() {
            len += 1;
        }
        if self.resource_version.is_some() {
            len += 1;
        }
        if self.r#continue.is_some() {
            len += 1;
        }
        if self.remaining_item_count.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.ListMeta", len)?;
        if let Some(v) = self.self_link.as_ref() {
            struct_ser.serialize_field("selfLink", v)?;
        }
        if let Some(v) = self.resource_version.as_ref() {
            struct_ser.serialize_field("resourceVersion", v)?;
        }
        if let Some(v) = self.r#continue.as_ref() {
            struct_ser.serialize_field("continue", v)?;
        }
        if let Some(v) = self.remaining_item_count.as_ref() {
            struct_ser.serialize_field("remainingItemCount", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMeta {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selfLink",
            "resourceVersion",
            "continue",
            "remainingItemCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SelfLink,
            ResourceVersion,
            Continue,
            RemainingItemCount,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "selfLink" => Ok(GeneratedField::SelfLink),
                            "resourceVersion" => Ok(GeneratedField::ResourceVersion),
                            "continue" => Ok(GeneratedField::Continue),
                            "remainingItemCount" => Ok(GeneratedField::RemainingItemCount),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListMeta;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.ListMeta")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ListMeta, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut self_link__ = None;
                let mut resource_version__ = None;
                let mut r#continue__ = None;
                let mut remaining_item_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SelfLink => {
                            if self_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selfLink"));
                            }
                            self_link__ = map.next_value()?;
                        }
                        GeneratedField::ResourceVersion => {
                            if resource_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceVersion"));
                            }
                            resource_version__ = map.next_value()?;
                        }
                        GeneratedField::Continue => {
                            if r#continue__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continue"));
                            }
                            r#continue__ = map.next_value()?;
                        }
                        GeneratedField::RemainingItemCount => {
                            if remaining_item_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remainingItemCount"));
                            }
                            remaining_item_count__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListMeta {
                    self_link: self_link__,
                    resource_version: resource_version__,
                    r#continue: r#continue__,
                    remaining_item_count: remaining_item_count__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.ListMeta", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.label_selector.is_some() {
            len += 1;
        }
        if self.field_selector.is_some() {
            len += 1;
        }
        if self.watch.is_some() {
            len += 1;
        }
        if self.allow_watch_bookmarks.is_some() {
            len += 1;
        }
        if self.resource_version.is_some() {
            len += 1;
        }
        if self.resource_version_match.is_some() {
            len += 1;
        }
        if self.timeout_seconds.is_some() {
            len += 1;
        }
        if self.limit.is_some() {
            len += 1;
        }
        if self.r#continue.is_some() {
            len += 1;
        }
        if self.send_initial_events.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.ListOptions", len)?;
        if let Some(v) = self.label_selector.as_ref() {
            struct_ser.serialize_field("labelSelector", v)?;
        }
        if let Some(v) = self.field_selector.as_ref() {
            struct_ser.serialize_field("fieldSelector", v)?;
        }
        if let Some(v) = self.watch.as_ref() {
            struct_ser.serialize_field("watch", v)?;
        }
        if let Some(v) = self.allow_watch_bookmarks.as_ref() {
            struct_ser.serialize_field("allowWatchBookmarks", v)?;
        }
        if let Some(v) = self.resource_version.as_ref() {
            struct_ser.serialize_field("resourceVersion", v)?;
        }
        if let Some(v) = self.resource_version_match.as_ref() {
            struct_ser.serialize_field("resourceVersionMatch", v)?;
        }
        if let Some(v) = self.timeout_seconds.as_ref() {
            struct_ser.serialize_field("timeoutSeconds", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.r#continue.as_ref() {
            struct_ser.serialize_field("continue", v)?;
        }
        if let Some(v) = self.send_initial_events.as_ref() {
            struct_ser.serialize_field("sendInitialEvents", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "labelSelector",
            "fieldSelector",
            "watch",
            "allowWatchBookmarks",
            "resourceVersion",
            "resourceVersionMatch",
            "timeoutSeconds",
            "limit",
            "continue",
            "sendInitialEvents",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LabelSelector,
            FieldSelector,
            Watch,
            AllowWatchBookmarks,
            ResourceVersion,
            ResourceVersionMatch,
            TimeoutSeconds,
            Limit,
            Continue,
            SendInitialEvents,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "labelSelector" => Ok(GeneratedField::LabelSelector),
                            "fieldSelector" => Ok(GeneratedField::FieldSelector),
                            "watch" => Ok(GeneratedField::Watch),
                            "allowWatchBookmarks" => Ok(GeneratedField::AllowWatchBookmarks),
                            "resourceVersion" => Ok(GeneratedField::ResourceVersion),
                            "resourceVersionMatch" => Ok(GeneratedField::ResourceVersionMatch),
                            "timeoutSeconds" => Ok(GeneratedField::TimeoutSeconds),
                            "limit" => Ok(GeneratedField::Limit),
                            "continue" => Ok(GeneratedField::Continue),
                            "sendInitialEvents" => Ok(GeneratedField::SendInitialEvents),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.ListOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ListOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut label_selector__ = None;
                let mut field_selector__ = None;
                let mut watch__ = None;
                let mut allow_watch_bookmarks__ = None;
                let mut resource_version__ = None;
                let mut resource_version_match__ = None;
                let mut timeout_seconds__ = None;
                let mut limit__ = None;
                let mut r#continue__ = None;
                let mut send_initial_events__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LabelSelector => {
                            if label_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labelSelector"));
                            }
                            label_selector__ = map.next_value()?;
                        }
                        GeneratedField::FieldSelector => {
                            if field_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldSelector"));
                            }
                            field_selector__ = map.next_value()?;
                        }
                        GeneratedField::Watch => {
                            if watch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watch"));
                            }
                            watch__ = map.next_value()?;
                        }
                        GeneratedField::AllowWatchBookmarks => {
                            if allow_watch_bookmarks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowWatchBookmarks"));
                            }
                            allow_watch_bookmarks__ = map.next_value()?;
                        }
                        GeneratedField::ResourceVersion => {
                            if resource_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceVersion"));
                            }
                            resource_version__ = map.next_value()?;
                        }
                        GeneratedField::ResourceVersionMatch => {
                            if resource_version_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceVersionMatch"));
                            }
                            resource_version_match__ = map.next_value()?;
                        }
                        GeneratedField::TimeoutSeconds => {
                            if timeout_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutSeconds"));
                            }
                            timeout_seconds__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Continue => {
                            if r#continue__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continue"));
                            }
                            r#continue__ = map.next_value()?;
                        }
                        GeneratedField::SendInitialEvents => {
                            if send_initial_events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendInitialEvents"));
                            }
                            send_initial_events__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListOptions {
                    label_selector: label_selector__,
                    field_selector: field_selector__,
                    watch: watch__,
                    allow_watch_bookmarks: allow_watch_bookmarks__,
                    resource_version: resource_version__,
                    resource_version_match: resource_version_match__,
                    timeout_seconds: timeout_seconds__,
                    limit: limit__,
                    r#continue: r#continue__,
                    send_initial_events: send_initial_events__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.ListOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ManagedFieldsEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.manager.is_some() {
            len += 1;
        }
        if self.operation.is_some() {
            len += 1;
        }
        if self.api_version.is_some() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if self.fields_type.is_some() {
            len += 1;
        }
        if self.fields_v1.is_some() {
            len += 1;
        }
        if self.subresource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.ManagedFieldsEntry", len)?;
        if let Some(v) = self.manager.as_ref() {
            struct_ser.serialize_field("manager", v)?;
        }
        if let Some(v) = self.operation.as_ref() {
            struct_ser.serialize_field("operation", v)?;
        }
        if let Some(v) = self.api_version.as_ref() {
            struct_ser.serialize_field("apiVersion", v)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if let Some(v) = self.fields_type.as_ref() {
            struct_ser.serialize_field("fieldsType", v)?;
        }
        if let Some(v) = self.fields_v1.as_ref() {
            struct_ser.serialize_field("fieldsV1", v)?;
        }
        if let Some(v) = self.subresource.as_ref() {
            struct_ser.serialize_field("subresource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManagedFieldsEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "manager",
            "operation",
            "apiVersion",
            "time",
            "fieldsType",
            "fieldsV1",
            "subresource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Manager,
            Operation,
            ApiVersion,
            Time,
            FieldsType,
            FieldsV1,
            Subresource,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "manager" => Ok(GeneratedField::Manager),
                            "operation" => Ok(GeneratedField::Operation),
                            "apiVersion" => Ok(GeneratedField::ApiVersion),
                            "time" => Ok(GeneratedField::Time),
                            "fieldsType" => Ok(GeneratedField::FieldsType),
                            "fieldsV1" => Ok(GeneratedField::FieldsV1),
                            "subresource" => Ok(GeneratedField::Subresource),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManagedFieldsEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.ManagedFieldsEntry")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ManagedFieldsEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut manager__ = None;
                let mut operation__ = None;
                let mut api_version__ = None;
                let mut time__ = None;
                let mut fields_type__ = None;
                let mut fields_v1__ = None;
                let mut subresource__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Manager => {
                            if manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("manager"));
                            }
                            manager__ = map.next_value()?;
                        }
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = map.next_value()?;
                        }
                        GeneratedField::ApiVersion => {
                            if api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiVersion"));
                            }
                            api_version__ = map.next_value()?;
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map.next_value()?;
                        }
                        GeneratedField::FieldsType => {
                            if fields_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldsType"));
                            }
                            fields_type__ = map.next_value()?;
                        }
                        GeneratedField::FieldsV1 => {
                            if fields_v1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldsV1"));
                            }
                            fields_v1__ = map.next_value()?;
                        }
                        GeneratedField::Subresource => {
                            if subresource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subresource"));
                            }
                            subresource__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManagedFieldsEntry {
                    manager: manager__,
                    operation: operation__,
                    api_version: api_version__,
                    time: time__,
                    fields_type: fields_type__,
                    fields_v1: fields_v1__,
                    subresource: subresource__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.ManagedFieldsEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MicroTime {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.seconds.is_some() {
            len += 1;
        }
        if self.nanos.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.MicroTime", len)?;
        if let Some(v) = self.seconds.as_ref() {
            struct_ser.serialize_field("seconds", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.nanos.as_ref() {
            struct_ser.serialize_field("nanos", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MicroTime {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seconds",
            "nanos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seconds,
            Nanos,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "seconds" => Ok(GeneratedField::Seconds),
                            "nanos" => Ok(GeneratedField::Nanos),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MicroTime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.MicroTime")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MicroTime, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut seconds__ = None;
                let mut nanos__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Seconds => {
                            if seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seconds"));
                            }
                            seconds__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Nanos => {
                            if nanos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MicroTime {
                    seconds: seconds__,
                    nanos: nanos__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.MicroTime", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ObjectMeta {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.generate_name.is_some() {
            len += 1;
        }
        if self.namespace.is_some() {
            len += 1;
        }
        if self.self_link.is_some() {
            len += 1;
        }
        if self.uid.is_some() {
            len += 1;
        }
        if self.resource_version.is_some() {
            len += 1;
        }
        if self.generation.is_some() {
            len += 1;
        }
        if self.creation_timestamp.is_some() {
            len += 1;
        }
        if self.deletion_timestamp.is_some() {
            len += 1;
        }
        if self.deletion_grace_period_seconds.is_some() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.owner_references.is_empty() {
            len += 1;
        }
        if !self.finalizers.is_empty() {
            len += 1;
        }
        if !self.managed_fields.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMeta", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.generate_name.as_ref() {
            struct_ser.serialize_field("generateName", v)?;
        }
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        if let Some(v) = self.self_link.as_ref() {
            struct_ser.serialize_field("selfLink", v)?;
        }
        if let Some(v) = self.uid.as_ref() {
            struct_ser.serialize_field("uid", v)?;
        }
        if let Some(v) = self.resource_version.as_ref() {
            struct_ser.serialize_field("resourceVersion", v)?;
        }
        if let Some(v) = self.generation.as_ref() {
            struct_ser.serialize_field("generation", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.creation_timestamp.as_ref() {
            struct_ser.serialize_field("creationTimestamp", v)?;
        }
        if let Some(v) = self.deletion_timestamp.as_ref() {
            struct_ser.serialize_field("deletionTimestamp", v)?;
        }
        if let Some(v) = self.deletion_grace_period_seconds.as_ref() {
            struct_ser.serialize_field("deletionGracePeriodSeconds", ToString::to_string(&v).as_str())?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.owner_references.is_empty() {
            struct_ser.serialize_field("ownerReferences", &self.owner_references)?;
        }
        if !self.finalizers.is_empty() {
            struct_ser.serialize_field("finalizers", &self.finalizers)?;
        }
        if !self.managed_fields.is_empty() {
            struct_ser.serialize_field("managedFields", &self.managed_fields)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ObjectMeta {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "generateName",
            "namespace",
            "selfLink",
            "uid",
            "resourceVersion",
            "generation",
            "creationTimestamp",
            "deletionTimestamp",
            "deletionGracePeriodSeconds",
            "labels",
            "annotations",
            "ownerReferences",
            "finalizers",
            "managedFields",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            GenerateName,
            Namespace,
            SelfLink,
            Uid,
            ResourceVersion,
            Generation,
            CreationTimestamp,
            DeletionTimestamp,
            DeletionGracePeriodSeconds,
            Labels,
            Annotations,
            OwnerReferences,
            Finalizers,
            ManagedFields,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "generateName" => Ok(GeneratedField::GenerateName),
                            "namespace" => Ok(GeneratedField::Namespace),
                            "selfLink" => Ok(GeneratedField::SelfLink),
                            "uid" => Ok(GeneratedField::Uid),
                            "resourceVersion" => Ok(GeneratedField::ResourceVersion),
                            "generation" => Ok(GeneratedField::Generation),
                            "creationTimestamp" => Ok(GeneratedField::CreationTimestamp),
                            "deletionTimestamp" => Ok(GeneratedField::DeletionTimestamp),
                            "deletionGracePeriodSeconds" => Ok(GeneratedField::DeletionGracePeriodSeconds),
                            "labels" => Ok(GeneratedField::Labels),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "ownerReferences" => Ok(GeneratedField::OwnerReferences),
                            "finalizers" => Ok(GeneratedField::Finalizers),
                            "managedFields" => Ok(GeneratedField::ManagedFields),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ObjectMeta;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMeta")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ObjectMeta, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut generate_name__ = None;
                let mut namespace__ = None;
                let mut self_link__ = None;
                let mut uid__ = None;
                let mut resource_version__ = None;
                let mut generation__ = None;
                let mut creation_timestamp__ = None;
                let mut deletion_timestamp__ = None;
                let mut deletion_grace_period_seconds__ = None;
                let mut labels__ = None;
                let mut annotations__ = None;
                let mut owner_references__ = None;
                let mut finalizers__ = None;
                let mut managed_fields__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::GenerateName => {
                            if generate_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("generateName"));
                            }
                            generate_name__ = map.next_value()?;
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = map.next_value()?;
                        }
                        GeneratedField::SelfLink => {
                            if self_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selfLink"));
                            }
                            self_link__ = map.next_value()?;
                        }
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = map.next_value()?;
                        }
                        GeneratedField::ResourceVersion => {
                            if resource_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceVersion"));
                            }
                            resource_version__ = map.next_value()?;
                        }
                        GeneratedField::Generation => {
                            if generation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("generation"));
                            }
                            generation__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CreationTimestamp => {
                            if creation_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationTimestamp"));
                            }
                            creation_timestamp__ = map.next_value()?;
                        }
                        GeneratedField::DeletionTimestamp => {
                            if deletion_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletionTimestamp"));
                            }
                            deletion_timestamp__ = map.next_value()?;
                        }
                        GeneratedField::DeletionGracePeriodSeconds => {
                            if deletion_grace_period_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletionGracePeriodSeconds"));
                            }
                            deletion_grace_period_seconds__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::OwnerReferences => {
                            if owner_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ownerReferences"));
                            }
                            owner_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::Finalizers => {
                            if finalizers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finalizers"));
                            }
                            finalizers__ = Some(map.next_value()?);
                        }
                        GeneratedField::ManagedFields => {
                            if managed_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("managedFields"));
                            }
                            managed_fields__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ObjectMeta {
                    name: name__,
                    generate_name: generate_name__,
                    namespace: namespace__,
                    self_link: self_link__,
                    uid: uid__,
                    resource_version: resource_version__,
                    generation: generation__,
                    creation_timestamp: creation_timestamp__,
                    deletion_timestamp: deletion_timestamp__,
                    deletion_grace_period_seconds: deletion_grace_period_seconds__,
                    labels: labels__.unwrap_or_default(),
                    annotations: annotations__.unwrap_or_default(),
                    owner_references: owner_references__.unwrap_or_default(),
                    finalizers: finalizers__.unwrap_or_default(),
                    managed_fields: managed_fields__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMeta", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OwnerReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.api_version.is_some() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        if self.uid.is_some() {
            len += 1;
        }
        if self.controller.is_some() {
            len += 1;
        }
        if self.block_owner_deletion.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.OwnerReference", len)?;
        if let Some(v) = self.api_version.as_ref() {
            struct_ser.serialize_field("apiVersion", v)?;
        }
        if let Some(v) = self.kind.as_ref() {
            struct_ser.serialize_field("kind", v)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.uid.as_ref() {
            struct_ser.serialize_field("uid", v)?;
        }
        if let Some(v) = self.controller.as_ref() {
            struct_ser.serialize_field("controller", v)?;
        }
        if let Some(v) = self.block_owner_deletion.as_ref() {
            struct_ser.serialize_field("blockOwnerDeletion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OwnerReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "apiVersion",
            "kind",
            "name",
            "uid",
            "controller",
            "blockOwnerDeletion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ApiVersion,
            Kind,
            Name,
            Uid,
            Controller,
            BlockOwnerDeletion,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "apiVersion" => Ok(GeneratedField::ApiVersion),
                            "kind" => Ok(GeneratedField::Kind),
                            "name" => Ok(GeneratedField::Name),
                            "uid" => Ok(GeneratedField::Uid),
                            "controller" => Ok(GeneratedField::Controller),
                            "blockOwnerDeletion" => Ok(GeneratedField::BlockOwnerDeletion),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OwnerReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.OwnerReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OwnerReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut api_version__ = None;
                let mut kind__ = None;
                let mut name__ = None;
                let mut uid__ = None;
                let mut controller__ = None;
                let mut block_owner_deletion__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ApiVersion => {
                            if api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiVersion"));
                            }
                            api_version__ = map.next_value()?;
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = map.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = map.next_value()?;
                        }
                        GeneratedField::Controller => {
                            if controller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("controller"));
                            }
                            controller__ = map.next_value()?;
                        }
                        GeneratedField::BlockOwnerDeletion => {
                            if block_owner_deletion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockOwnerDeletion"));
                            }
                            block_owner_deletion__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(OwnerReference {
                    api_version: api_version__,
                    kind: kind__,
                    name: name__,
                    uid: uid__,
                    controller: controller__,
                    block_owner_deletion: block_owner_deletion__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.OwnerReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PartialObjectMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.PartialObjectMetadata", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PartialObjectMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PartialObjectMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.PartialObjectMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PartialObjectMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PartialObjectMetadata {
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.PartialObjectMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PartialObjectMetadataList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if !self.items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.PartialObjectMetadataList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PartialObjectMetadataList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "items",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Items,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "items" => Ok(GeneratedField::Items),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PartialObjectMetadataList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.PartialObjectMetadataList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PartialObjectMetadataList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut items__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PartialObjectMetadataList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.PartialObjectMetadataList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Patch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Patch", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Patch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Patch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.Patch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Patch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Patch {
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Patch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PatchOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dry_run.is_empty() {
            len += 1;
        }
        if self.force.is_some() {
            len += 1;
        }
        if self.field_manager.is_some() {
            len += 1;
        }
        if self.field_validation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.PatchOptions", len)?;
        if !self.dry_run.is_empty() {
            struct_ser.serialize_field("dryRun", &self.dry_run)?;
        }
        if let Some(v) = self.force.as_ref() {
            struct_ser.serialize_field("force", v)?;
        }
        if let Some(v) = self.field_manager.as_ref() {
            struct_ser.serialize_field("fieldManager", v)?;
        }
        if let Some(v) = self.field_validation.as_ref() {
            struct_ser.serialize_field("fieldValidation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PatchOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dryRun",
            "force",
            "fieldManager",
            "fieldValidation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DryRun,
            Force,
            FieldManager,
            FieldValidation,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dryRun" => Ok(GeneratedField::DryRun),
                            "force" => Ok(GeneratedField::Force),
                            "fieldManager" => Ok(GeneratedField::FieldManager),
                            "fieldValidation" => Ok(GeneratedField::FieldValidation),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PatchOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.PatchOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PatchOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dry_run__ = None;
                let mut force__ = None;
                let mut field_manager__ = None;
                let mut field_validation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map.next_value()?);
                        }
                        GeneratedField::Force => {
                            if force__.is_some() {
                                return Err(serde::de::Error::duplicate_field("force"));
                            }
                            force__ = map.next_value()?;
                        }
                        GeneratedField::FieldManager => {
                            if field_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldManager"));
                            }
                            field_manager__ = map.next_value()?;
                        }
                        GeneratedField::FieldValidation => {
                            if field_validation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldValidation"));
                            }
                            field_validation__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PatchOptions {
                    dry_run: dry_run__.unwrap_or_default(),
                    force: force__,
                    field_manager: field_manager__,
                    field_validation: field_validation__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.PatchOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Preconditions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.uid.is_some() {
            len += 1;
        }
        if self.resource_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Preconditions", len)?;
        if let Some(v) = self.uid.as_ref() {
            struct_ser.serialize_field("uid", v)?;
        }
        if let Some(v) = self.resource_version.as_ref() {
            struct_ser.serialize_field("resourceVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Preconditions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uid",
            "resourceVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uid,
            ResourceVersion,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uid" => Ok(GeneratedField::Uid),
                            "resourceVersion" => Ok(GeneratedField::ResourceVersion),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Preconditions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.Preconditions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Preconditions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uid__ = None;
                let mut resource_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = map.next_value()?;
                        }
                        GeneratedField::ResourceVersion => {
                            if resource_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceVersion"));
                            }
                            resource_version__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Preconditions {
                    uid: uid__,
                    resource_version: resource_version__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Preconditions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RootPaths {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.paths.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.RootPaths", len)?;
        if !self.paths.is_empty() {
            struct_ser.serialize_field("paths", &self.paths)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RootPaths {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "paths",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Paths,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "paths" => Ok(GeneratedField::Paths),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RootPaths;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.RootPaths")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RootPaths, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut paths__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Paths => {
                            if paths__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paths"));
                            }
                            paths__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RootPaths {
                    paths: paths__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.RootPaths", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerAddressByClientCidr {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.client_cidr.is_some() {
            len += 1;
        }
        if self.server_address.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.ServerAddressByClientCIDR", len)?;
        if let Some(v) = self.client_cidr.as_ref() {
            struct_ser.serialize_field("clientCIDR", v)?;
        }
        if let Some(v) = self.server_address.as_ref() {
            struct_ser.serialize_field("serverAddress", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerAddressByClientCidr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "clientCIDR",
            "serverAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientCidr,
            ServerAddress,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "clientCIDR" => Ok(GeneratedField::ClientCidr),
                            "serverAddress" => Ok(GeneratedField::ServerAddress),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerAddressByClientCidr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.ServerAddressByClientCIDR")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServerAddressByClientCidr, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_cidr__ = None;
                let mut server_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClientCidr => {
                            if client_cidr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientCIDR"));
                            }
                            client_cidr__ = map.next_value()?;
                        }
                        GeneratedField::ServerAddress => {
                            if server_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverAddress"));
                            }
                            server_address__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ServerAddressByClientCidr {
                    client_cidr: client_cidr__,
                    server_address: server_address__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.ServerAddressByClientCIDR", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.message.is_some() {
            len += 1;
        }
        if self.reason.is_some() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if self.code.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Status", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.message.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "status",
            "message",
            "reason",
            "details",
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Status,
            Message,
            Reason,
            Details,
            Code,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "status" => Ok(GeneratedField::Status),
                            "message" => Ok(GeneratedField::Message),
                            "reason" => Ok(GeneratedField::Reason),
                            "details" => Ok(GeneratedField::Details),
                            "code" => Ok(GeneratedField::Code),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Status;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.Status")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Status, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut status__ = None;
                let mut message__ = None;
                let mut reason__ = None;
                let mut details__ = None;
                let mut code__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = map.next_value()?;
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map.next_value()?;
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map.next_value()?;
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Status {
                    metadata: metadata__,
                    status: status__,
                    message: message__,
                    reason: reason__,
                    details: details__,
                    code: code__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Status", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatusCause {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason.is_some() {
            len += 1;
        }
        if self.message.is_some() {
            len += 1;
        }
        if self.field.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.StatusCause", len)?;
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        if let Some(v) = self.message.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        if let Some(v) = self.field.as_ref() {
            struct_ser.serialize_field("field", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatusCause {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
            "message",
            "field",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
            Message,
            Field,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reason" => Ok(GeneratedField::Reason),
                            "message" => Ok(GeneratedField::Message),
                            "field" => Ok(GeneratedField::Field),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatusCause;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.StatusCause")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StatusCause, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                let mut message__ = None;
                let mut field__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map.next_value()?;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = map.next_value()?;
                        }
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(StatusCause {
                    reason: reason__,
                    message: message__,
                    field: field__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.StatusCause", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatusDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.group.is_some() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        if self.uid.is_some() {
            len += 1;
        }
        if !self.causes.is_empty() {
            len += 1;
        }
        if self.retry_after_seconds.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.StatusDetails", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        if let Some(v) = self.kind.as_ref() {
            struct_ser.serialize_field("kind", v)?;
        }
        if let Some(v) = self.uid.as_ref() {
            struct_ser.serialize_field("uid", v)?;
        }
        if !self.causes.is_empty() {
            struct_ser.serialize_field("causes", &self.causes)?;
        }
        if let Some(v) = self.retry_after_seconds.as_ref() {
            struct_ser.serialize_field("retryAfterSeconds", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatusDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "group",
            "kind",
            "uid",
            "causes",
            "retryAfterSeconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Group,
            Kind,
            Uid,
            Causes,
            RetryAfterSeconds,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "group" => Ok(GeneratedField::Group),
                            "kind" => Ok(GeneratedField::Kind),
                            "uid" => Ok(GeneratedField::Uid),
                            "causes" => Ok(GeneratedField::Causes),
                            "retryAfterSeconds" => Ok(GeneratedField::RetryAfterSeconds),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatusDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.StatusDetails")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StatusDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut group__ = None;
                let mut kind__ = None;
                let mut uid__ = None;
                let mut causes__ = None;
                let mut retry_after_seconds__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map.next_value()?;
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = map.next_value()?;
                        }
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = map.next_value()?;
                        }
                        GeneratedField::Causes => {
                            if causes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("causes"));
                            }
                            causes__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetryAfterSeconds => {
                            if retry_after_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryAfterSeconds"));
                            }
                            retry_after_seconds__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(StatusDetails {
                    name: name__,
                    group: group__,
                    kind: kind__,
                    uid: uid__,
                    causes: causes__.unwrap_or_default(),
                    retry_after_seconds: retry_after_seconds__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.StatusDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TableOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.include_object.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.TableOptions", len)?;
        if let Some(v) = self.include_object.as_ref() {
            struct_ser.serialize_field("includeObject", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TableOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "includeObject",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IncludeObject,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "includeObject" => Ok(GeneratedField::IncludeObject),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TableOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.TableOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TableOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut include_object__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IncludeObject => {
                            if include_object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeObject"));
                            }
                            include_object__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TableOptions {
                    include_object: include_object__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.TableOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Time {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.seconds.is_some() {
            len += 1;
        }
        if self.nanos.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Time", len)?;
        if let Some(v) = self.seconds.as_ref() {
            struct_ser.serialize_field("seconds", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.nanos.as_ref() {
            struct_ser.serialize_field("nanos", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Time {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seconds",
            "nanos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seconds,
            Nanos,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "seconds" => Ok(GeneratedField::Seconds),
                            "nanos" => Ok(GeneratedField::Nanos),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Time;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.Time")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Time, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut seconds__ = None;
                let mut nanos__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Seconds => {
                            if seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seconds"));
                            }
                            seconds__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Nanos => {
                            if nanos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Time {
                    seconds: seconds__,
                    nanos: nanos__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Time", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Timestamp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.seconds.is_some() {
            len += 1;
        }
        if self.nanos.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Timestamp", len)?;
        if let Some(v) = self.seconds.as_ref() {
            struct_ser.serialize_field("seconds", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.nanos.as_ref() {
            struct_ser.serialize_field("nanos", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Timestamp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seconds",
            "nanos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seconds,
            Nanos,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "seconds" => Ok(GeneratedField::Seconds),
                            "nanos" => Ok(GeneratedField::Nanos),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Timestamp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.Timestamp")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Timestamp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut seconds__ = None;
                let mut nanos__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Seconds => {
                            if seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seconds"));
                            }
                            seconds__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Nanos => {
                            if nanos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Timestamp {
                    seconds: seconds__,
                    nanos: nanos__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Timestamp", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TypeMeta {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kind.is_some() {
            len += 1;
        }
        if self.api_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.TypeMeta", len)?;
        if let Some(v) = self.kind.as_ref() {
            struct_ser.serialize_field("kind", v)?;
        }
        if let Some(v) = self.api_version.as_ref() {
            struct_ser.serialize_field("apiVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypeMeta {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "apiVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            ApiVersion,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kind" => Ok(GeneratedField::Kind),
                            "apiVersion" => Ok(GeneratedField::ApiVersion),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TypeMeta;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.TypeMeta")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TypeMeta, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut api_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = map.next_value()?;
                        }
                        GeneratedField::ApiVersion => {
                            if api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiVersion"));
                            }
                            api_version__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TypeMeta {
                    kind: kind__,
                    api_version: api_version__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.TypeMeta", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dry_run.is_empty() {
            len += 1;
        }
        if self.field_manager.is_some() {
            len += 1;
        }
        if self.field_validation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.UpdateOptions", len)?;
        if !self.dry_run.is_empty() {
            struct_ser.serialize_field("dryRun", &self.dry_run)?;
        }
        if let Some(v) = self.field_manager.as_ref() {
            struct_ser.serialize_field("fieldManager", v)?;
        }
        if let Some(v) = self.field_validation.as_ref() {
            struct_ser.serialize_field("fieldValidation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dryRun",
            "fieldManager",
            "fieldValidation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DryRun,
            FieldManager,
            FieldValidation,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dryRun" => Ok(GeneratedField::DryRun),
                            "fieldManager" => Ok(GeneratedField::FieldManager),
                            "fieldValidation" => Ok(GeneratedField::FieldValidation),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.UpdateOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dry_run__ = None;
                let mut field_manager__ = None;
                let mut field_validation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map.next_value()?);
                        }
                        GeneratedField::FieldManager => {
                            if field_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldManager"));
                            }
                            field_manager__ = map.next_value()?;
                        }
                        GeneratedField::FieldValidation => {
                            if field_validation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldValidation"));
                            }
                            field_validation__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateOptions {
                    dry_run: dry_run__.unwrap_or_default(),
                    field_manager: field_manager__,
                    field_validation: field_validation__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.UpdateOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Verbs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Verbs", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Verbs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "items" => Ok(GeneratedField::Items),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Verbs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.Verbs")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Verbs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Verbs {
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.Verbs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        if self.object.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.WatchEvent", len)?;
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        if let Some(v) = self.object.as_ref() {
            struct_ser.serialize_field("object", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "object",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Object,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "object" => Ok(GeneratedField::Object),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WatchEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.apis.meta.v1.WatchEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WatchEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut object__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map.next_value()?;
                        }
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(WatchEvent {
                    r#type: r#type__,
                    object: object__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.apis.meta.v1.WatchEvent", FIELDS, GeneratedVisitor)
    }
}
