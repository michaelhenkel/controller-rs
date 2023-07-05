impl serde::Serialize for ApsAttribute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sequence.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.APSAttribute", len)?;
        if let Some(v) = self.sequence.as_ref() {
            struct_ser.serialize_field("sequence", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApsAttribute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sequence",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sequence,
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
                            "sequence" => Ok(GeneratedField::Sequence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApsAttribute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.APSAttribute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApsAttribute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sequence__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ = map.next_value()?;
                        }
                    }
                }
                Ok(ApsAttribute {
                    sequence: sequence__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.APSAttribute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddressFamilies {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressFamilies", len)?;
        if !self.family.is_empty() {
            struct_ser.serialize_field("family", &self.family)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddressFamilies {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Family,
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
                            "family" => Ok(GeneratedField::Family),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddressFamilies;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressFamilies")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddressFamilies, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AddressFamilies {
                    family: family__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressFamilies", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddressGroup {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroup", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddressGroup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddressGroup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroup")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddressGroup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(AddressGroup {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddressGroupList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddressGroupList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddressGroupList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddressGroupList, V::Error>
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
                    }
                }
                Ok(AddressGroupList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddressGroupPrefix {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subnet.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupPrefix", len)?;
        if !self.subnet.is_empty() {
            struct_ser.serialize_field("subnet", &self.subnet)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddressGroupPrefix {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subnet",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subnet,
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
                            "subnet" => Ok(GeneratedField::Subnet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddressGroupPrefix;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupPrefix")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddressGroupPrefix, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subnet__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Subnet => {
                            if subnet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subnet"));
                            }
                            subnet__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AddressGroupPrefix {
                    subnet: subnet__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupPrefix", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddressGroupSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.address_group_prefixes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.address_group_prefixes.as_ref() {
            struct_ser.serialize_field("addressGroupPrefixes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddressGroupSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "addressGroupPrefixes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            AddressGroupPrefixes,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "addressGroupPrefixes" => Ok(GeneratedField::AddressGroupPrefixes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddressGroupSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddressGroupSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut address_group_prefixes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::AddressGroupPrefixes => {
                            if address_group_prefixes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressGroupPrefixes"));
                            }
                            address_group_prefixes__ = map.next_value()?;
                        }
                    }
                }
                Ok(AddressGroupSpec {
                    common_spec: common_spec__,
                    address_group_prefixes: address_group_prefixes__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddressGroupStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddressGroupStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddressGroupStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddressGroupStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(AddressGroupStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AddressGroupStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AllowedAddressPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ip.is_some() {
            len += 1;
        }
        if self.mac.is_some() {
            len += 1;
        }
        if self.address_mode.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPair", len)?;
        if let Some(v) = self.ip.as_ref() {
            struct_ser.serialize_field("ip", v)?;
        }
        if let Some(v) = self.mac.as_ref() {
            struct_ser.serialize_field("mac", v)?;
        }
        if let Some(v) = self.address_mode.as_ref() {
            struct_ser.serialize_field("addressMode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowedAddressPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ip",
            "mac",
            "addressMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ip,
            Mac,
            AddressMode,
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
                            "ip" => Ok(GeneratedField::Ip),
                            "mac" => Ok(GeneratedField::Mac),
                            "addressMode" => Ok(GeneratedField::AddressMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllowedAddressPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AllowedAddressPair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ip__ = None;
                let mut mac__ = None;
                let mut address_mode__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ip => {
                            if ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            ip__ = map.next_value()?;
                        }
                        GeneratedField::Mac => {
                            if mac__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mac"));
                            }
                            mac__ = map.next_value()?;
                        }
                        GeneratedField::AddressMode => {
                            if address_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressMode"));
                            }
                            address_mode__ = map.next_value()?;
                        }
                    }
                }
                Ok(AllowedAddressPair {
                    ip: ip__,
                    mac: mac__,
                    address_mode: address_mode__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AllowedAddressPairSubnet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ip_prefix.is_some() {
            len += 1;
        }
        if self.ip_prefix_len.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPairSubnet", len)?;
        if let Some(v) = self.ip_prefix.as_ref() {
            struct_ser.serialize_field("ipPrefix", v)?;
        }
        if let Some(v) = self.ip_prefix_len.as_ref() {
            struct_ser.serialize_field("ipPrefixLen", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowedAddressPairSubnet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ipPrefix",
            "ipPrefixLen",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IpPrefix,
            IpPrefixLen,
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
                            "ipPrefix" => Ok(GeneratedField::IpPrefix),
                            "ipPrefixLen" => Ok(GeneratedField::IpPrefixLen),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllowedAddressPairSubnet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPairSubnet")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AllowedAddressPairSubnet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ip_prefix__ = None;
                let mut ip_prefix_len__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IpPrefix => {
                            if ip_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipPrefix"));
                            }
                            ip_prefix__ = map.next_value()?;
                        }
                        GeneratedField::IpPrefixLen => {
                            if ip_prefix_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipPrefixLen"));
                            }
                            ip_prefix_len__ = map.next_value()?;
                        }
                    }
                }
                Ok(AllowedAddressPairSubnet {
                    ip_prefix: ip_prefix__,
                    ip_prefix_len: ip_prefix_len__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPairSubnet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AllowedAddressPairs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allowed_address_pair.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPairs", len)?;
        if !self.allowed_address_pair.is_empty() {
            struct_ser.serialize_field("allowedAddressPair", &self.allowed_address_pair)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowedAddressPairs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowedAddressPair",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedAddressPair,
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
                            "allowedAddressPair" => Ok(GeneratedField::AllowedAddressPair),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllowedAddressPairs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPairs")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AllowedAddressPairs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_address_pair__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowedAddressPair => {
                            if allowed_address_pair__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedAddressPair"));
                            }
                            allowed_address_pair__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AllowedAddressPairs {
                    allowed_address_pair: allowed_address_pair__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPairs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApplicationPolicySet {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySet", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApplicationPolicySet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApplicationPolicySet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySet")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApplicationPolicySet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(ApplicationPolicySet {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApplicationPolicySetList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySetList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApplicationPolicySetList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApplicationPolicySetList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySetList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApplicationPolicySetList, V::Error>
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
                    }
                }
                Ok(ApplicationPolicySetList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySetList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApplicationPolicySetSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if !self.firewall_policy_references.is_empty() {
            len += 1;
        }
        if !self.tag_references.is_empty() {
            len += 1;
        }
        if self.global_vrouter_config_reference.is_some() {
            len += 1;
        }
        if self.k8smode.is_some() {
            len += 1;
        }
        if self.all_applications.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySetSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if !self.firewall_policy_references.is_empty() {
            struct_ser.serialize_field("firewallPolicyReferences", &self.firewall_policy_references)?;
        }
        if !self.tag_references.is_empty() {
            struct_ser.serialize_field("tagReferences", &self.tag_references)?;
        }
        if let Some(v) = self.global_vrouter_config_reference.as_ref() {
            struct_ser.serialize_field("globalVrouterConfigReference", v)?;
        }
        if let Some(v) = self.k8smode.as_ref() {
            struct_ser.serialize_field("k8smode", v)?;
        }
        if let Some(v) = self.all_applications.as_ref() {
            struct_ser.serialize_field("allApplications", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApplicationPolicySetSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "firewallPolicyReferences",
            "tagReferences",
            "globalVrouterConfigReference",
            "k8smode",
            "allApplications",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            FirewallPolicyReferences,
            TagReferences,
            GlobalVrouterConfigReference,
            K8smode,
            AllApplications,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "firewallPolicyReferences" => Ok(GeneratedField::FirewallPolicyReferences),
                            "tagReferences" => Ok(GeneratedField::TagReferences),
                            "globalVrouterConfigReference" => Ok(GeneratedField::GlobalVrouterConfigReference),
                            "k8smode" => Ok(GeneratedField::K8smode),
                            "allApplications" => Ok(GeneratedField::AllApplications),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApplicationPolicySetSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySetSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApplicationPolicySetSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut firewall_policy_references__ = None;
                let mut tag_references__ = None;
                let mut global_vrouter_config_reference__ = None;
                let mut k8smode__ = None;
                let mut all_applications__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::FirewallPolicyReferences => {
                            if firewall_policy_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firewallPolicyReferences"));
                            }
                            firewall_policy_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::TagReferences => {
                            if tag_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagReferences"));
                            }
                            tag_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::GlobalVrouterConfigReference => {
                            if global_vrouter_config_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("globalVrouterConfigReference"));
                            }
                            global_vrouter_config_reference__ = map.next_value()?;
                        }
                        GeneratedField::K8smode => {
                            if k8smode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("k8smode"));
                            }
                            k8smode__ = map.next_value()?;
                        }
                        GeneratedField::AllApplications => {
                            if all_applications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allApplications"));
                            }
                            all_applications__ = map.next_value()?;
                        }
                    }
                }
                Ok(ApplicationPolicySetSpec {
                    common_spec: common_spec__,
                    firewall_policy_references: firewall_policy_references__.unwrap_or_default(),
                    tag_references: tag_references__.unwrap_or_default(),
                    global_vrouter_config_reference: global_vrouter_config_reference__,
                    k8smode: k8smode__,
                    all_applications: all_applications__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySetSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApplicationPolicySetStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySetStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApplicationPolicySetStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApplicationPolicySetStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySetStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApplicationPolicySetStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(ApplicationPolicySetStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ApplicationPolicySetStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthenticationData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_type.is_some() {
            len += 1;
        }
        if !self.key_items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AuthenticationData", len)?;
        if let Some(v) = self.key_type.as_ref() {
            struct_ser.serialize_field("keyType", v)?;
        }
        if !self.key_items.is_empty() {
            struct_ser.serialize_field("keyItems", &self.key_items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticationData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keyType",
            "keyItems",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyType,
            KeyItems,
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
                            "keyType" => Ok(GeneratedField::KeyType),
                            "keyItems" => Ok(GeneratedField::KeyItems),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticationData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AuthenticationData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AuthenticationData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_type__ = None;
                let mut key_items__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::KeyType => {
                            if key_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyType"));
                            }
                            key_type__ = map.next_value()?;
                        }
                        GeneratedField::KeyItems => {
                            if key_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyItems"));
                            }
                            key_items__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AuthenticationData {
                    key_type: key_type__,
                    key_items: key_items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AuthenticationData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthenticationKeyItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_id.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AuthenticationKeyItem", len)?;
        if let Some(v) = self.key_id.as_ref() {
            struct_ser.serialize_field("keyId", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticationKeyItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keyId",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyId,
            Key,
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
                            "keyId" => Ok(GeneratedField::KeyId),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticationKeyItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AuthenticationKeyItem")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AuthenticationKeyItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_id__ = None;
                let mut key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                    }
                }
                Ok(AuthenticationKeyItem {
                    key_id: key_id__,
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.AuthenticationKeyItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpAsAService {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAService", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpAsAService {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpAsAService;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAService")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpAsAService, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(BgpAsAService {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAService", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpAsAServiceList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpAsAServiceList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpAsAServiceList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpAsAServiceList, V::Error>
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
                    }
                }
                Ok(BgpAsAServiceList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpAsAServiceSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.shared.is_some() {
            len += 1;
        }
        if self.ip_address.is_some() {
            len += 1;
        }
        if self.autonomous_system.is_some() {
            len += 1;
        }
        if self.suppress_route_advertisement.is_some() {
            len += 1;
        }
        if self.ipv4_mapped_i_pv6_next_hop.is_some() {
            len += 1;
        }
        if self.bgp_as_a_service_session_attributes.is_some() {
            len += 1;
        }
        if !self.virtual_machine_interface_references.is_empty() {
            len += 1;
        }
        if !self.virtual_machine_interfaces_selector.is_empty() {
            len += 1;
        }
        if self.service_health_check_reference.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.shared.as_ref() {
            struct_ser.serialize_field("shared", v)?;
        }
        if let Some(v) = self.ip_address.as_ref() {
            struct_ser.serialize_field("ipAddress", v)?;
        }
        if let Some(v) = self.autonomous_system.as_ref() {
            struct_ser.serialize_field("autonomousSystem", v)?;
        }
        if let Some(v) = self.suppress_route_advertisement.as_ref() {
            struct_ser.serialize_field("suppressRouteAdvertisement", v)?;
        }
        if let Some(v) = self.ipv4_mapped_i_pv6_next_hop.as_ref() {
            struct_ser.serialize_field("ipv4MappedIPv6NextHop", v)?;
        }
        if let Some(v) = self.bgp_as_a_service_session_attributes.as_ref() {
            struct_ser.serialize_field("bgpAsAServiceSessionAttributes", v)?;
        }
        if !self.virtual_machine_interface_references.is_empty() {
            struct_ser.serialize_field("virtualMachineInterfaceReferences", &self.virtual_machine_interface_references)?;
        }
        if !self.virtual_machine_interfaces_selector.is_empty() {
            struct_ser.serialize_field("virtualMachineInterfacesSelector", &self.virtual_machine_interfaces_selector)?;
        }
        if let Some(v) = self.service_health_check_reference.as_ref() {
            struct_ser.serialize_field("serviceHealthCheckReference", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpAsAServiceSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "shared",
            "ipAddress",
            "autonomousSystem",
            "suppressRouteAdvertisement",
            "ipv4MappedIPv6NextHop",
            "bgpAsAServiceSessionAttributes",
            "virtualMachineInterfaceReferences",
            "virtualMachineInterfacesSelector",
            "serviceHealthCheckReference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            Shared,
            IpAddress,
            AutonomousSystem,
            SuppressRouteAdvertisement,
            Ipv4MappedIPv6NextHop,
            BgpAsAServiceSessionAttributes,
            VirtualMachineInterfaceReferences,
            VirtualMachineInterfacesSelector,
            ServiceHealthCheckReference,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "shared" => Ok(GeneratedField::Shared),
                            "ipAddress" => Ok(GeneratedField::IpAddress),
                            "autonomousSystem" => Ok(GeneratedField::AutonomousSystem),
                            "suppressRouteAdvertisement" => Ok(GeneratedField::SuppressRouteAdvertisement),
                            "ipv4MappedIPv6NextHop" => Ok(GeneratedField::Ipv4MappedIPv6NextHop),
                            "bgpAsAServiceSessionAttributes" => Ok(GeneratedField::BgpAsAServiceSessionAttributes),
                            "virtualMachineInterfaceReferences" => Ok(GeneratedField::VirtualMachineInterfaceReferences),
                            "virtualMachineInterfacesSelector" => Ok(GeneratedField::VirtualMachineInterfacesSelector),
                            "serviceHealthCheckReference" => Ok(GeneratedField::ServiceHealthCheckReference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpAsAServiceSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpAsAServiceSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut shared__ = None;
                let mut ip_address__ = None;
                let mut autonomous_system__ = None;
                let mut suppress_route_advertisement__ = None;
                let mut ipv4_mapped_i_pv6_next_hop__ = None;
                let mut bgp_as_a_service_session_attributes__ = None;
                let mut virtual_machine_interface_references__ = None;
                let mut virtual_machine_interfaces_selector__ = None;
                let mut service_health_check_reference__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::Shared => {
                            if shared__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shared"));
                            }
                            shared__ = map.next_value()?;
                        }
                        GeneratedField::IpAddress => {
                            if ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipAddress"));
                            }
                            ip_address__ = map.next_value()?;
                        }
                        GeneratedField::AutonomousSystem => {
                            if autonomous_system__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autonomousSystem"));
                            }
                            autonomous_system__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SuppressRouteAdvertisement => {
                            if suppress_route_advertisement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suppressRouteAdvertisement"));
                            }
                            suppress_route_advertisement__ = map.next_value()?;
                        }
                        GeneratedField::Ipv4MappedIPv6NextHop => {
                            if ipv4_mapped_i_pv6_next_hop__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv4MappedIPv6NextHop"));
                            }
                            ipv4_mapped_i_pv6_next_hop__ = map.next_value()?;
                        }
                        GeneratedField::BgpAsAServiceSessionAttributes => {
                            if bgp_as_a_service_session_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bgpAsAServiceSessionAttributes"));
                            }
                            bgp_as_a_service_session_attributes__ = map.next_value()?;
                        }
                        GeneratedField::VirtualMachineInterfaceReferences => {
                            if virtual_machine_interface_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualMachineInterfaceReferences"));
                            }
                            virtual_machine_interface_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::VirtualMachineInterfacesSelector => {
                            if virtual_machine_interfaces_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualMachineInterfacesSelector"));
                            }
                            virtual_machine_interfaces_selector__ = Some(map.next_value()?);
                        }
                        GeneratedField::ServiceHealthCheckReference => {
                            if service_health_check_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceHealthCheckReference"));
                            }
                            service_health_check_reference__ = map.next_value()?;
                        }
                    }
                }
                Ok(BgpAsAServiceSpec {
                    common_spec: common_spec__,
                    shared: shared__,
                    ip_address: ip_address__,
                    autonomous_system: autonomous_system__,
                    suppress_route_advertisement: suppress_route_advertisement__,
                    ipv4_mapped_i_pv6_next_hop: ipv4_mapped_i_pv6_next_hop__,
                    bgp_as_a_service_session_attributes: bgp_as_a_service_session_attributes__,
                    virtual_machine_interface_references: virtual_machine_interface_references__.unwrap_or_default(),
                    virtual_machine_interfaces_selector: virtual_machine_interfaces_selector__.unwrap_or_default(),
                    service_health_check_reference: service_health_check_reference__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpAsAServiceStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        if !self.bgp_router_references.is_empty() {
            len += 1;
        }
        if !self.subnet_references.is_empty() {
            len += 1;
        }
        if !self.vmi_selector_references.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        if !self.bgp_router_references.is_empty() {
            struct_ser.serialize_field("bgpRouterReferences", &self.bgp_router_references)?;
        }
        if !self.subnet_references.is_empty() {
            struct_ser.serialize_field("subnetReferences", &self.subnet_references)?;
        }
        if !self.vmi_selector_references.is_empty() {
            struct_ser.serialize_field("vmiSelectorReferences", &self.vmi_selector_references)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpAsAServiceStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
            "bgpRouterReferences",
            "subnetReferences",
            "vmiSelectorReferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
            BgpRouterReferences,
            SubnetReferences,
            VmiSelectorReferences,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            "bgpRouterReferences" => Ok(GeneratedField::BgpRouterReferences),
                            "subnetReferences" => Ok(GeneratedField::SubnetReferences),
                            "vmiSelectorReferences" => Ok(GeneratedField::VmiSelectorReferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpAsAServiceStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpAsAServiceStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                let mut bgp_router_references__ = None;
                let mut subnet_references__ = None;
                let mut vmi_selector_references__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                        GeneratedField::BgpRouterReferences => {
                            if bgp_router_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bgpRouterReferences"));
                            }
                            bgp_router_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubnetReferences => {
                            if subnet_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subnetReferences"));
                            }
                            subnet_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::VmiSelectorReferences => {
                            if vmi_selector_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vmiSelectorReferences"));
                            }
                            vmi_selector_references__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BgpAsAServiceStatus {
                    common_status: common_status__,
                    bgp_router_references: bgp_router_references__.unwrap_or_default(),
                    subnet_references: subnet_references__.unwrap_or_default(),
                    vmi_selector_references: vmi_selector_references__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPAsAServiceStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpFamilyAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.address_family.is_some() {
            len += 1;
        }
        if self.loop_count.is_some() {
            len += 1;
        }
        if self.prefix_limit.is_some() {
            len += 1;
        }
        if !self.default_tunnel_encap.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPFamilyAttributes", len)?;
        if let Some(v) = self.address_family.as_ref() {
            struct_ser.serialize_field("addressFamily", v)?;
        }
        if let Some(v) = self.loop_count.as_ref() {
            struct_ser.serialize_field("loopCount", v)?;
        }
        if let Some(v) = self.prefix_limit.as_ref() {
            struct_ser.serialize_field("prefixLimit", v)?;
        }
        if !self.default_tunnel_encap.is_empty() {
            struct_ser.serialize_field("defaultTunnelEncap", &self.default_tunnel_encap)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpFamilyAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "addressFamily",
            "loopCount",
            "prefixLimit",
            "defaultTunnelEncap",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AddressFamily,
            LoopCount,
            PrefixLimit,
            DefaultTunnelEncap,
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
                            "addressFamily" => Ok(GeneratedField::AddressFamily),
                            "loopCount" => Ok(GeneratedField::LoopCount),
                            "prefixLimit" => Ok(GeneratedField::PrefixLimit),
                            "defaultTunnelEncap" => Ok(GeneratedField::DefaultTunnelEncap),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpFamilyAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPFamilyAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpFamilyAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address_family__ = None;
                let mut loop_count__ = None;
                let mut prefix_limit__ = None;
                let mut default_tunnel_encap__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AddressFamily => {
                            if address_family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressFamily"));
                            }
                            address_family__ = map.next_value()?;
                        }
                        GeneratedField::LoopCount => {
                            if loop_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loopCount"));
                            }
                            loop_count__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PrefixLimit => {
                            if prefix_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixLimit"));
                            }
                            prefix_limit__ = map.next_value()?;
                        }
                        GeneratedField::DefaultTunnelEncap => {
                            if default_tunnel_encap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultTunnelEncap"));
                            }
                            default_tunnel_encap__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BgpFamilyAttributes {
                    address_family: address_family__,
                    loop_count: loop_count__,
                    prefix_limit: prefix_limit__,
                    default_tunnel_encap: default_tunnel_encap__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPFamilyAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpPrefixLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.idle_timeout.is_some() {
            len += 1;
        }
        if self.maximum.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPPrefixLimit", len)?;
        if let Some(v) = self.idle_timeout.as_ref() {
            struct_ser.serialize_field("idleTimeout", v)?;
        }
        if let Some(v) = self.maximum.as_ref() {
            struct_ser.serialize_field("maximum", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpPrefixLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "idleTimeout",
            "maximum",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdleTimeout,
            Maximum,
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
                            "idleTimeout" => Ok(GeneratedField::IdleTimeout),
                            "maximum" => Ok(GeneratedField::Maximum),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpPrefixLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPPrefixLimit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpPrefixLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idle_timeout__ = None;
                let mut maximum__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IdleTimeout => {
                            if idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeout"));
                            }
                            idle_timeout__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Maximum => {
                            if maximum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maximum"));
                            }
                            maximum__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(BgpPrefixLimit {
                    idle_timeout: idle_timeout__,
                    maximum: maximum__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPPrefixLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpRouter {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouter", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpRouter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpRouter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpRouter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(BgpRouter {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpRouterList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpRouterList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpRouterList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpRouterList, V::Error>
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
                    }
                }
                Ok(BgpRouterList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpRouterParameters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.admin_down.is_some() {
            len += 1;
        }
        if self.vendor.is_some() {
            len += 1;
        }
        if self.cluster_id.is_some() {
            len += 1;
        }
        if self.autonomous_system.is_some() {
            len += 1;
        }
        if self.identifier.is_some() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if self.port.is_some() {
            len += 1;
        }
        if self.source_port.is_some() {
            len += 1;
        }
        if self.hold_time.is_some() {
            len += 1;
        }
        if self.address_families.is_some() {
            len += 1;
        }
        if self.auth_data.is_some() {
            len += 1;
        }
        if self.local_autonomous_system.is_some() {
            len += 1;
        }
        if self.router_type.is_some() {
            len += 1;
        }
        if self.gateway_address.is_some() {
            len += 1;
        }
        if self.ipv6_gateway_address.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterParameters", len)?;
        if let Some(v) = self.admin_down.as_ref() {
            struct_ser.serialize_field("adminDown", v)?;
        }
        if let Some(v) = self.vendor.as_ref() {
            struct_ser.serialize_field("vendor", v)?;
        }
        if let Some(v) = self.cluster_id.as_ref() {
            struct_ser.serialize_field("clusterID", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.autonomous_system.as_ref() {
            struct_ser.serialize_field("autonomousSystem", v)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            struct_ser.serialize_field("identifier", v)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.port.as_ref() {
            struct_ser.serialize_field("port", v)?;
        }
        if let Some(v) = self.source_port.as_ref() {
            struct_ser.serialize_field("sourcePort", v)?;
        }
        if let Some(v) = self.hold_time.as_ref() {
            struct_ser.serialize_field("holdTime", v)?;
        }
        if let Some(v) = self.address_families.as_ref() {
            struct_ser.serialize_field("addressFamilies", v)?;
        }
        if let Some(v) = self.auth_data.as_ref() {
            struct_ser.serialize_field("authData", v)?;
        }
        if let Some(v) = self.local_autonomous_system.as_ref() {
            struct_ser.serialize_field("localAutonomousSystem", v)?;
        }
        if let Some(v) = self.router_type.as_ref() {
            struct_ser.serialize_field("routerType", v)?;
        }
        if let Some(v) = self.gateway_address.as_ref() {
            struct_ser.serialize_field("gatewayAddress", v)?;
        }
        if let Some(v) = self.ipv6_gateway_address.as_ref() {
            struct_ser.serialize_field("ipv6GatewayAddress", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpRouterParameters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "adminDown",
            "vendor",
            "clusterID",
            "autonomousSystem",
            "identifier",
            "address",
            "port",
            "sourcePort",
            "holdTime",
            "addressFamilies",
            "authData",
            "localAutonomousSystem",
            "routerType",
            "gatewayAddress",
            "ipv6GatewayAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AdminDown,
            Vendor,
            ClusterId,
            AutonomousSystem,
            Identifier,
            Address,
            Port,
            SourcePort,
            HoldTime,
            AddressFamilies,
            AuthData,
            LocalAutonomousSystem,
            RouterType,
            GatewayAddress,
            Ipv6GatewayAddress,
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
                            "adminDown" => Ok(GeneratedField::AdminDown),
                            "vendor" => Ok(GeneratedField::Vendor),
                            "clusterID" => Ok(GeneratedField::ClusterId),
                            "autonomousSystem" => Ok(GeneratedField::AutonomousSystem),
                            "identifier" => Ok(GeneratedField::Identifier),
                            "address" => Ok(GeneratedField::Address),
                            "port" => Ok(GeneratedField::Port),
                            "sourcePort" => Ok(GeneratedField::SourcePort),
                            "holdTime" => Ok(GeneratedField::HoldTime),
                            "addressFamilies" => Ok(GeneratedField::AddressFamilies),
                            "authData" => Ok(GeneratedField::AuthData),
                            "localAutonomousSystem" => Ok(GeneratedField::LocalAutonomousSystem),
                            "routerType" => Ok(GeneratedField::RouterType),
                            "gatewayAddress" => Ok(GeneratedField::GatewayAddress),
                            "ipv6GatewayAddress" => Ok(GeneratedField::Ipv6GatewayAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpRouterParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterParameters")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpRouterParameters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin_down__ = None;
                let mut vendor__ = None;
                let mut cluster_id__ = None;
                let mut autonomous_system__ = None;
                let mut identifier__ = None;
                let mut address__ = None;
                let mut port__ = None;
                let mut source_port__ = None;
                let mut hold_time__ = None;
                let mut address_families__ = None;
                let mut auth_data__ = None;
                let mut local_autonomous_system__ = None;
                let mut router_type__ = None;
                let mut gateway_address__ = None;
                let mut ipv6_gateway_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AdminDown => {
                            if admin_down__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminDown"));
                            }
                            admin_down__ = map.next_value()?;
                        }
                        GeneratedField::Vendor => {
                            if vendor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendor"));
                            }
                            vendor__ = map.next_value()?;
                        }
                        GeneratedField::ClusterId => {
                            if cluster_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterID"));
                            }
                            cluster_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AutonomousSystem => {
                            if autonomous_system__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autonomousSystem"));
                            }
                            autonomous_system__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = map.next_value()?;
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map.next_value()?;
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SourcePort => {
                            if source_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePort"));
                            }
                            source_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::HoldTime => {
                            if hold_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("holdTime"));
                            }
                            hold_time__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AddressFamilies => {
                            if address_families__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressFamilies"));
                            }
                            address_families__ = map.next_value()?;
                        }
                        GeneratedField::AuthData => {
                            if auth_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authData"));
                            }
                            auth_data__ = map.next_value()?;
                        }
                        GeneratedField::LocalAutonomousSystem => {
                            if local_autonomous_system__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localAutonomousSystem"));
                            }
                            local_autonomous_system__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RouterType => {
                            if router_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routerType"));
                            }
                            router_type__ = map.next_value()?;
                        }
                        GeneratedField::GatewayAddress => {
                            if gateway_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gatewayAddress"));
                            }
                            gateway_address__ = map.next_value()?;
                        }
                        GeneratedField::Ipv6GatewayAddress => {
                            if ipv6_gateway_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv6GatewayAddress"));
                            }
                            ipv6_gateway_address__ = map.next_value()?;
                        }
                    }
                }
                Ok(BgpRouterParameters {
                    admin_down: admin_down__,
                    vendor: vendor__,
                    cluster_id: cluster_id__,
                    autonomous_system: autonomous_system__,
                    identifier: identifier__,
                    address: address__,
                    port: port__,
                    source_port: source_port__,
                    hold_time: hold_time__,
                    address_families: address_families__,
                    auth_data: auth_data__,
                    local_autonomous_system: local_autonomous_system__,
                    router_type: router_type__,
                    gateway_address: gateway_address__,
                    ipv6_gateway_address: ipv6_gateway_address__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterParameters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpRouterReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_reference.is_some() {
            len += 1;
        }
        if self.attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterReference", len)?;
        if let Some(v) = self.resource_reference.as_ref() {
            struct_ser.serialize_field("resourceReference", v)?;
        }
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpRouterReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resourceReference",
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceReference,
            Attributes,
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
                            "resourceReference" => Ok(GeneratedField::ResourceReference),
                            "attributes" => Ok(GeneratedField::Attributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpRouterReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpRouterReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_reference__ = None;
                let mut attributes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceReference => {
                            if resource_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceReference"));
                            }
                            resource_reference__ = map.next_value()?;
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map.next_value()?;
                        }
                    }
                }
                Ok(BgpRouterReference {
                    resource_reference: resource_reference__,
                    attributes: attributes__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpRouterReferenceAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.session.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterReferenceAttributes", len)?;
        if !self.session.is_empty() {
            struct_ser.serialize_field("session", &self.session)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpRouterReferenceAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Session,
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
                            "session" => Ok(GeneratedField::Session),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpRouterReferenceAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterReferenceAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpRouterReferenceAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Session => {
                            if session__.is_some() {
                                return Err(serde::de::Error::duplicate_field("session"));
                            }
                            session__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BgpRouterReferenceAttributes {
                    session: session__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterReferenceAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpRouterSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.parent.is_some() {
            len += 1;
        }
        if !self.bgp_router_references.is_empty() {
            len += 1;
        }
        if self.bgp_router_parameters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.parent.as_ref() {
            struct_ser.serialize_field("parent", v)?;
        }
        if !self.bgp_router_references.is_empty() {
            struct_ser.serialize_field("bgpRouterReferences", &self.bgp_router_references)?;
        }
        if let Some(v) = self.bgp_router_parameters.as_ref() {
            struct_ser.serialize_field("bgpRouterParameters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpRouterSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "parent",
            "bgpRouterReferences",
            "bgpRouterParameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            Parent,
            BgpRouterReferences,
            BgpRouterParameters,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "parent" => Ok(GeneratedField::Parent),
                            "bgpRouterReferences" => Ok(GeneratedField::BgpRouterReferences),
                            "bgpRouterParameters" => Ok(GeneratedField::BgpRouterParameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpRouterSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpRouterSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut parent__ = None;
                let mut bgp_router_references__ = None;
                let mut bgp_router_parameters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = map.next_value()?;
                        }
                        GeneratedField::BgpRouterReferences => {
                            if bgp_router_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bgpRouterReferences"));
                            }
                            bgp_router_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::BgpRouterParameters => {
                            if bgp_router_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bgpRouterParameters"));
                            }
                            bgp_router_parameters__ = map.next_value()?;
                        }
                    }
                }
                Ok(BgpRouterSpec {
                    common_spec: common_spec__,
                    parent: parent__,
                    bgp_router_references: bgp_router_references__.unwrap_or_default(),
                    bgp_router_parameters: bgp_router_parameters__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpRouterStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpRouterStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpRouterStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpRouterStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(BgpRouterStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPRouterStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpSession {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.session_attributes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPSession", len)?;
        if !self.session_attributes.is_empty() {
            struct_ser.serialize_field("sessionAttributes", &self.session_attributes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpSession {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sessionAttributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionAttributes,
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
                            "sessionAttributes" => Ok(GeneratedField::SessionAttributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpSession;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPSession")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpSession, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_attributes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SessionAttributes => {
                            if session_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionAttributes"));
                            }
                            session_attributes__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BgpSession {
                    session_attributes: session_attributes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPSession", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpSessionAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bgp_router.is_some() {
            len += 1;
        }
        if self.admin_down.is_some() {
            len += 1;
        }
        if self.passive.is_some() {
            len += 1;
        }
        if self.as_override.is_some() {
            len += 1;
        }
        if self.hold_time.is_some() {
            len += 1;
        }
        if self.loop_count.is_some() {
            len += 1;
        }
        if self.local_autonomous_system.is_some() {
            len += 1;
        }
        if self.address_families.is_some() {
            len += 1;
        }
        if self.auth_data.is_some() {
            len += 1;
        }
        if !self.family_attributes.is_empty() {
            len += 1;
        }
        if self.private_as_action.is_some() {
            len += 1;
        }
        if self.route_origin_override.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPSessionAttributes", len)?;
        if let Some(v) = self.bgp_router.as_ref() {
            struct_ser.serialize_field("bgpRouter", v)?;
        }
        if let Some(v) = self.admin_down.as_ref() {
            struct_ser.serialize_field("adminDown", v)?;
        }
        if let Some(v) = self.passive.as_ref() {
            struct_ser.serialize_field("passive", v)?;
        }
        if let Some(v) = self.as_override.as_ref() {
            struct_ser.serialize_field("asOverride", v)?;
        }
        if let Some(v) = self.hold_time.as_ref() {
            struct_ser.serialize_field("holdTime", v)?;
        }
        if let Some(v) = self.loop_count.as_ref() {
            struct_ser.serialize_field("loopCount", v)?;
        }
        if let Some(v) = self.local_autonomous_system.as_ref() {
            struct_ser.serialize_field("localAutonomousSystem", v)?;
        }
        if let Some(v) = self.address_families.as_ref() {
            struct_ser.serialize_field("addressFamilies", v)?;
        }
        if let Some(v) = self.auth_data.as_ref() {
            struct_ser.serialize_field("authData", v)?;
        }
        if !self.family_attributes.is_empty() {
            struct_ser.serialize_field("familyAttributes", &self.family_attributes)?;
        }
        if let Some(v) = self.private_as_action.as_ref() {
            struct_ser.serialize_field("privateAsAction", v)?;
        }
        if let Some(v) = self.route_origin_override.as_ref() {
            struct_ser.serialize_field("routeOriginOverride", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpSessionAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bgpRouter",
            "adminDown",
            "passive",
            "asOverride",
            "holdTime",
            "loopCount",
            "localAutonomousSystem",
            "addressFamilies",
            "authData",
            "familyAttributes",
            "privateAsAction",
            "routeOriginOverride",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BgpRouter,
            AdminDown,
            Passive,
            AsOverride,
            HoldTime,
            LoopCount,
            LocalAutonomousSystem,
            AddressFamilies,
            AuthData,
            FamilyAttributes,
            PrivateAsAction,
            RouteOriginOverride,
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
                            "bgpRouter" => Ok(GeneratedField::BgpRouter),
                            "adminDown" => Ok(GeneratedField::AdminDown),
                            "passive" => Ok(GeneratedField::Passive),
                            "asOverride" => Ok(GeneratedField::AsOverride),
                            "holdTime" => Ok(GeneratedField::HoldTime),
                            "loopCount" => Ok(GeneratedField::LoopCount),
                            "localAutonomousSystem" => Ok(GeneratedField::LocalAutonomousSystem),
                            "addressFamilies" => Ok(GeneratedField::AddressFamilies),
                            "authData" => Ok(GeneratedField::AuthData),
                            "familyAttributes" => Ok(GeneratedField::FamilyAttributes),
                            "privateAsAction" => Ok(GeneratedField::PrivateAsAction),
                            "routeOriginOverride" => Ok(GeneratedField::RouteOriginOverride),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpSessionAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPSessionAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpSessionAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bgp_router__ = None;
                let mut admin_down__ = None;
                let mut passive__ = None;
                let mut as_override__ = None;
                let mut hold_time__ = None;
                let mut loop_count__ = None;
                let mut local_autonomous_system__ = None;
                let mut address_families__ = None;
                let mut auth_data__ = None;
                let mut family_attributes__ = None;
                let mut private_as_action__ = None;
                let mut route_origin_override__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BgpRouter => {
                            if bgp_router__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bgpRouter"));
                            }
                            bgp_router__ = map.next_value()?;
                        }
                        GeneratedField::AdminDown => {
                            if admin_down__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminDown"));
                            }
                            admin_down__ = map.next_value()?;
                        }
                        GeneratedField::Passive => {
                            if passive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passive"));
                            }
                            passive__ = map.next_value()?;
                        }
                        GeneratedField::AsOverride => {
                            if as_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asOverride"));
                            }
                            as_override__ = map.next_value()?;
                        }
                        GeneratedField::HoldTime => {
                            if hold_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("holdTime"));
                            }
                            hold_time__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::LoopCount => {
                            if loop_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loopCount"));
                            }
                            loop_count__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::LocalAutonomousSystem => {
                            if local_autonomous_system__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localAutonomousSystem"));
                            }
                            local_autonomous_system__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AddressFamilies => {
                            if address_families__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressFamilies"));
                            }
                            address_families__ = map.next_value()?;
                        }
                        GeneratedField::AuthData => {
                            if auth_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authData"));
                            }
                            auth_data__ = map.next_value()?;
                        }
                        GeneratedField::FamilyAttributes => {
                            if family_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAttributes"));
                            }
                            family_attributes__ = Some(map.next_value()?);
                        }
                        GeneratedField::PrivateAsAction => {
                            if private_as_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateAsAction"));
                            }
                            private_as_action__ = map.next_value()?;
                        }
                        GeneratedField::RouteOriginOverride => {
                            if route_origin_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeOriginOverride"));
                            }
                            route_origin_override__ = map.next_value()?;
                        }
                    }
                }
                Ok(BgpSessionAttributes {
                    bgp_router: bgp_router__,
                    admin_down: admin_down__,
                    passive: passive__,
                    as_override: as_override__,
                    hold_time: hold_time__,
                    loop_count: loop_count__,
                    local_autonomous_system: local_autonomous_system__,
                    address_families: address_families__,
                    auth_data: auth_data__,
                    family_attributes: family_attributes__.unwrap_or_default(),
                    private_as_action: private_as_action__,
                    route_origin_override: route_origin_override__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPSessionAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BgpSessionIpAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bgpaas_primary_ip.is_some() {
            len += 1;
        }
        if self.bgpaas_secondary_ip.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPSessionIPAttributes", len)?;
        if let Some(v) = self.bgpaas_primary_ip.as_ref() {
            struct_ser.serialize_field("bgpaasPrimaryIP", v)?;
        }
        if let Some(v) = self.bgpaas_secondary_ip.as_ref() {
            struct_ser.serialize_field("bgpaasSecondaryIP", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BgpSessionIpAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bgpaasPrimaryIP",
            "bgpaasSecondaryIP",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BgpaasPrimaryIp,
            BgpaasSecondaryIp,
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
                            "bgpaasPrimaryIP" => Ok(GeneratedField::BgpaasPrimaryIp),
                            "bgpaasSecondaryIP" => Ok(GeneratedField::BgpaasSecondaryIp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BgpSessionIpAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPSessionIPAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BgpSessionIpAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bgpaas_primary_ip__ = None;
                let mut bgpaas_secondary_ip__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BgpaasPrimaryIp => {
                            if bgpaas_primary_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bgpaasPrimaryIP"));
                            }
                            bgpaas_primary_ip__ = map.next_value()?;
                        }
                        GeneratedField::BgpaasSecondaryIp => {
                            if bgpaas_secondary_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bgpaasSecondaryIP"));
                            }
                            bgpaas_secondary_ip__ = map.next_value()?;
                        }
                    }
                }
                Ok(BgpSessionIpAttributes {
                    bgpaas_primary_ip: bgpaas_primary_ip__,
                    bgpaas_secondary_ip: bgpaas_secondary_ip__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.BGPSessionIPAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommonSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.contrail_fq_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.CommonSpec", len)?;
        if let Some(v) = self.contrail_fq_name.as_ref() {
            struct_ser.serialize_field("contrailFqName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommonSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contrailFqName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContrailFqName,
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
                            "contrailFqName" => Ok(GeneratedField::ContrailFqName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommonSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.CommonSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommonSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut contrail_fq_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContrailFqName => {
                            if contrail_fq_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contrailFqName"));
                            }
                            contrail_fq_name__ = map.next_value()?;
                        }
                    }
                }
                Ok(CommonSpec {
                    contrail_fq_name: contrail_fq_name__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.CommonSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommonStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reconciler_state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.CommonStatus", len)?;
        if let Some(v) = self.reconciler_state.as_ref() {
            struct_ser.serialize_field("reconcilerState", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommonStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reconcilerState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReconcilerState,
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
                            "reconcilerState" => Ok(GeneratedField::ReconcilerState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommonStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.CommonStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommonStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reconciler_state__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReconcilerState => {
                            if reconciler_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reconcilerState"));
                            }
                            reconciler_state__ = map.next_value()?;
                        }
                    }
                }
                Ok(CommonStatus {
                    reconciler_state: reconciler_state__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.CommonStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommunityAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.community_attribute.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.CommunityAttributes", len)?;
        if !self.community_attribute.is_empty() {
            struct_ser.serialize_field("communityAttribute", &self.community_attribute)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommunityAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "communityAttribute",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommunityAttribute,
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
                            "communityAttribute" => Ok(GeneratedField::CommunityAttribute),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommunityAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.CommunityAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommunityAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut community_attribute__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommunityAttribute => {
                            if community_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("communityAttribute"));
                            }
                            community_attribute__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CommunityAttributes {
                    community_attribute: community_attribute__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.CommunityAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContrailFqName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fq_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailFqName", len)?;
        if !self.fq_name.is_empty() {
            struct_ser.serialize_field("fqName", &self.fq_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContrailFqName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fqName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FqName,
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
                            "fqName" => Ok(GeneratedField::FqName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContrailFqName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailFqName")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContrailFqName, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fq_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FqName => {
                            if fq_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqName"));
                            }
                            fq_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ContrailFqName {
                    fq_name: fq_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailFqName", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContrailSecurityPolicy {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicy", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContrailSecurityPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContrailSecurityPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContrailSecurityPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(ContrailSecurityPolicy {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContrailSecurityPolicyEndPoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.end_points.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyEndPoint", len)?;
        if !self.end_points.is_empty() {
            struct_ser.serialize_field("endPoints", &self.end_points)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContrailSecurityPolicyEndPoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endPoints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EndPoints,
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
                            "endPoints" => Ok(GeneratedField::EndPoints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContrailSecurityPolicyEndPoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyEndPoint")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContrailSecurityPolicyEndPoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut end_points__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EndPoints => {
                            if end_points__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endPoints"));
                            }
                            end_points__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ContrailSecurityPolicyEndPoint {
                    end_points: end_points__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyEndPoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContrailSecurityPolicyList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContrailSecurityPolicyList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContrailSecurityPolicyList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContrailSecurityPolicyList, V::Error>
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
                    }
                }
                Ok(ContrailSecurityPolicyList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContrailSecurityPolicyPodSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pod_selector.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyPodSelector", len)?;
        if let Some(v) = self.pod_selector.as_ref() {
            struct_ser.serialize_field("podSelector", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContrailSecurityPolicyPodSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "podSelector",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PodSelector,
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
                            "podSelector" => Ok(GeneratedField::PodSelector),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContrailSecurityPolicyPodSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyPodSelector")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContrailSecurityPolicyPodSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pod_selector__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PodSelector => {
                            if pod_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("podSelector"));
                            }
                            pod_selector__ = map.next_value()?;
                        }
                    }
                }
                Ok(ContrailSecurityPolicyPodSelector {
                    pod_selector: pod_selector__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyPodSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContrailSecurityPolicyRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.src_ep.is_some() {
            len += 1;
        }
        if self.dst_ep.is_some() {
            len += 1;
        }
        if !self.ports.is_empty() {
            len += 1;
        }
        if self.secondary_actions.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyRule", len)?;
        if let Some(v) = self.src_ep.as_ref() {
            struct_ser.serialize_field("srcEP", v)?;
        }
        if let Some(v) = self.dst_ep.as_ref() {
            struct_ser.serialize_field("dstEP", v)?;
        }
        if !self.ports.is_empty() {
            struct_ser.serialize_field("ports", &self.ports)?;
        }
        if let Some(v) = self.secondary_actions.as_ref() {
            struct_ser.serialize_field("secondaryActions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContrailSecurityPolicyRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "srcEP",
            "dstEP",
            "ports",
            "secondaryActions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SrcEp,
            DstEp,
            Ports,
            SecondaryActions,
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
                            "srcEP" => Ok(GeneratedField::SrcEp),
                            "dstEP" => Ok(GeneratedField::DstEp),
                            "ports" => Ok(GeneratedField::Ports),
                            "secondaryActions" => Ok(GeneratedField::SecondaryActions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContrailSecurityPolicyRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyRule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContrailSecurityPolicyRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut src_ep__ = None;
                let mut dst_ep__ = None;
                let mut ports__ = None;
                let mut secondary_actions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SrcEp => {
                            if src_ep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srcEP"));
                            }
                            src_ep__ = map.next_value()?;
                        }
                        GeneratedField::DstEp => {
                            if dst_ep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dstEP"));
                            }
                            dst_ep__ = map.next_value()?;
                        }
                        GeneratedField::Ports => {
                            if ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ports"));
                            }
                            ports__ = Some(map.next_value()?);
                        }
                        GeneratedField::SecondaryActions => {
                            if secondary_actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secondaryActions"));
                            }
                            secondary_actions__ = map.next_value()?;
                        }
                    }
                }
                Ok(ContrailSecurityPolicyRule {
                    src_ep: src_ep__,
                    dst_ep: dst_ep__,
                    ports: ports__.unwrap_or_default(),
                    secondary_actions: secondary_actions__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContrailSecurityPolicySelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pod_selector.is_some() {
            len += 1;
        }
        if self.namespace_selector.is_some() {
            len += 1;
        }
        if self.ip_block.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicySelector", len)?;
        if let Some(v) = self.pod_selector.as_ref() {
            struct_ser.serialize_field("podSelector", v)?;
        }
        if let Some(v) = self.namespace_selector.as_ref() {
            struct_ser.serialize_field("namespaceSelector", v)?;
        }
        if let Some(v) = self.ip_block.as_ref() {
            struct_ser.serialize_field("ipBlock", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContrailSecurityPolicySelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "podSelector",
            "namespaceSelector",
            "ipBlock",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PodSelector,
            NamespaceSelector,
            IpBlock,
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
                            "podSelector" => Ok(GeneratedField::PodSelector),
                            "namespaceSelector" => Ok(GeneratedField::NamespaceSelector),
                            "ipBlock" => Ok(GeneratedField::IpBlock),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContrailSecurityPolicySelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicySelector")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContrailSecurityPolicySelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pod_selector__ = None;
                let mut namespace_selector__ = None;
                let mut ip_block__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PodSelector => {
                            if pod_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("podSelector"));
                            }
                            pod_selector__ = map.next_value()?;
                        }
                        GeneratedField::NamespaceSelector => {
                            if namespace_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceSelector"));
                            }
                            namespace_selector__ = map.next_value()?;
                        }
                        GeneratedField::IpBlock => {
                            if ip_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipBlock"));
                            }
                            ip_block__ = map.next_value()?;
                        }
                    }
                }
                Ok(ContrailSecurityPolicySelector {
                    pod_selector: pod_selector__,
                    namespace_selector: namespace_selector__,
                    ip_block: ip_block__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicySelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContrailSecurityPolicySpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selectors.is_empty() {
            len += 1;
        }
        if !self.rules.is_empty() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        if self.secondary_actions.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicySpec", len)?;
        if !self.selectors.is_empty() {
            struct_ser.serialize_field("selectors", &self.selectors)?;
        }
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        if let Some(v) = self.secondary_actions.as_ref() {
            struct_ser.serialize_field("secondaryActions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContrailSecurityPolicySpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selectors",
            "rules",
            "action",
            "secondaryActions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selectors,
            Rules,
            Action,
            SecondaryActions,
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
                            "selectors" => Ok(GeneratedField::Selectors),
                            "rules" => Ok(GeneratedField::Rules),
                            "action" => Ok(GeneratedField::Action),
                            "secondaryActions" => Ok(GeneratedField::SecondaryActions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContrailSecurityPolicySpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicySpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContrailSecurityPolicySpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selectors__ = None;
                let mut rules__ = None;
                let mut action__ = None;
                let mut secondary_actions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Selectors => {
                            if selectors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selectors"));
                            }
                            selectors__ = Some(map.next_value()?);
                        }
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map.next_value()?);
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map.next_value()?;
                        }
                        GeneratedField::SecondaryActions => {
                            if secondary_actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secondaryActions"));
                            }
                            secondary_actions__ = map.next_value()?;
                        }
                    }
                }
                Ok(ContrailSecurityPolicySpec {
                    selectors: selectors__.unwrap_or_default(),
                    rules: rules__.unwrap_or_default(),
                    action: action__,
                    secondary_actions: secondary_actions__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicySpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ContrailSecurityPolicyStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContrailSecurityPolicyStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContrailSecurityPolicyStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ContrailSecurityPolicyStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(ContrailSecurityPolicyStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ContrailSecurityPolicyStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EncapsulationPriorities {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.encapsulation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.EncapsulationPriorities", len)?;
        if !self.encapsulation.is_empty() {
            struct_ser.serialize_field("encapsulation", &self.encapsulation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EncapsulationPriorities {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "encapsulation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Encapsulation,
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
                            "encapsulation" => Ok(GeneratedField::Encapsulation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EncapsulationPriorities;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.EncapsulationPriorities")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EncapsulationPriorities, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut encapsulation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Encapsulation => {
                            if encapsulation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encapsulation"));
                            }
                            encapsulation__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EncapsulationPriorities {
                    encapsulation: encapsulation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.EncapsulationPriorities", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FastConvergenceParametersType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enable.is_some() {
            len += 1;
        }
        if self.nh_reachability_check.is_some() {
            len += 1;
        }
        if self.xmpp_hold_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FastConvergenceParametersType", len)?;
        if let Some(v) = self.enable.as_ref() {
            struct_ser.serialize_field("enable", v)?;
        }
        if let Some(v) = self.nh_reachability_check.as_ref() {
            struct_ser.serialize_field("nhReachabilityCheck", v)?;
        }
        if let Some(v) = self.xmpp_hold_time.as_ref() {
            struct_ser.serialize_field("xmppHoldTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FastConvergenceParametersType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enable",
            "nhReachabilityCheck",
            "xmppHoldTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enable,
            NhReachabilityCheck,
            XmppHoldTime,
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
                            "enable" => Ok(GeneratedField::Enable),
                            "nhReachabilityCheck" => Ok(GeneratedField::NhReachabilityCheck),
                            "xmppHoldTime" => Ok(GeneratedField::XmppHoldTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FastConvergenceParametersType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FastConvergenceParametersType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FastConvergenceParametersType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enable__ = None;
                let mut nh_reachability_check__ = None;
                let mut xmpp_hold_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Enable => {
                            if enable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enable"));
                            }
                            enable__ = map.next_value()?;
                        }
                        GeneratedField::NhReachabilityCheck => {
                            if nh_reachability_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nhReachabilityCheck"));
                            }
                            nh_reachability_check__ = map.next_value()?;
                        }
                        GeneratedField::XmppHoldTime => {
                            if xmpp_hold_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("xmppHoldTime"));
                            }
                            xmpp_hold_time__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(FastConvergenceParametersType {
                    enable: enable__,
                    nh_reachability_check: nh_reachability_check__,
                    xmpp_hold_time: xmpp_hold_time__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FastConvergenceParametersType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallActionListType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.simple_action.is_some() {
            len += 1;
        }
        if self.mirror_to.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallActionListType", len)?;
        if let Some(v) = self.simple_action.as_ref() {
            struct_ser.serialize_field("simpleAction", v)?;
        }
        if let Some(v) = self.mirror_to.as_ref() {
            struct_ser.serialize_field("mirrorTo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallActionListType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "simpleAction",
            "mirrorTo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SimpleAction,
            MirrorTo,
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
                            "simpleAction" => Ok(GeneratedField::SimpleAction),
                            "mirrorTo" => Ok(GeneratedField::MirrorTo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallActionListType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallActionListType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallActionListType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut simple_action__ = None;
                let mut mirror_to__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SimpleAction => {
                            if simple_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("simpleAction"));
                            }
                            simple_action__ = map.next_value()?;
                        }
                        GeneratedField::MirrorTo => {
                            if mirror_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mirrorTo"));
                            }
                            mirror_to__ = map.next_value()?;
                        }
                    }
                }
                Ok(FirewallActionListType {
                    simple_action: simple_action__,
                    mirror_to: mirror_to__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallActionListType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallMatchEprList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.expr_list.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMatchEprList", len)?;
        if !self.expr_list.is_empty() {
            struct_ser.serialize_field("exprList", &self.expr_list)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallMatchEprList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exprList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExprList,
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
                            "exprList" => Ok(GeneratedField::ExprList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallMatchEprList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMatchEprList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallMatchEprList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expr_list__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExprList => {
                            if expr_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exprList"));
                            }
                            expr_list__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FirewallMatchEprList {
                    expr_list: expr_list__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMatchEprList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallMatchExpr {
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
        if self.oper.is_some() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        if !self.values_ids.is_empty() {
            len += 1;
        }
        if !self.except_list.is_empty() {
            len += 1;
        }
        if !self.except_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMatchExpr", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.oper.as_ref() {
            struct_ser.serialize_field("oper", v)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        if !self.values_ids.is_empty() {
            struct_ser.serialize_field("valuesIds", &self.values_ids.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.except_list.is_empty() {
            struct_ser.serialize_field("exceptList", &self.except_list)?;
        }
        if !self.except_ids.is_empty() {
            struct_ser.serialize_field("exceptIds", &self.except_ids.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallMatchExpr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "oper",
            "values",
            "valuesIds",
            "exceptList",
            "exceptIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Oper,
            Values,
            ValuesIds,
            ExceptList,
            ExceptIds,
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
                            "oper" => Ok(GeneratedField::Oper),
                            "values" => Ok(GeneratedField::Values),
                            "valuesIds" => Ok(GeneratedField::ValuesIds),
                            "exceptList" => Ok(GeneratedField::ExceptList),
                            "exceptIds" => Ok(GeneratedField::ExceptIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallMatchExpr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMatchExpr")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallMatchExpr, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut oper__ = None;
                let mut values__ = None;
                let mut values_ids__ = None;
                let mut except_list__ = None;
                let mut except_ids__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Oper => {
                            if oper__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oper"));
                            }
                            oper__ = map.next_value()?;
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValuesIds => {
                            if values_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuesIds"));
                            }
                            values_ids__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ExceptList => {
                            if except_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exceptList"));
                            }
                            except_list__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExceptIds => {
                            if except_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exceptIds"));
                            }
                            except_ids__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(FirewallMatchExpr {
                    key: key__,
                    oper: oper__,
                    values: values__.unwrap_or_default(),
                    values_ids: values_ids__.unwrap_or_default(),
                    except_list: except_list__.unwrap_or_default(),
                    except_ids: except_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMatchExpr", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallMirrorActionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.analyzer_name.is_some() {
            len += 1;
        }
        if self.encapsulation.is_some() {
            len += 1;
        }
        if self.analyzer_ip_address.is_some() {
            len += 1;
        }
        if self.analyzer_mac_address.is_some() {
            len += 1;
        }
        if self.routing_instance.is_some() {
            len += 1;
        }
        if self.udp_port.is_some() {
            len += 1;
        }
        if self.juniper_header.is_some() {
            len += 1;
        }
        if self.nh_mode.is_some() {
            len += 1;
        }
        if self.static_nh_header.is_some() {
            len += 1;
        }
        if self.nic_assisted_mirroring.is_some() {
            len += 1;
        }
        if self.nic_assisted_mirroring_vlan.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMirrorActionType", len)?;
        if let Some(v) = self.analyzer_name.as_ref() {
            struct_ser.serialize_field("analyzerName", v)?;
        }
        if let Some(v) = self.encapsulation.as_ref() {
            struct_ser.serialize_field("encapsulation", v)?;
        }
        if let Some(v) = self.analyzer_ip_address.as_ref() {
            struct_ser.serialize_field("analyzerIPAddress", v)?;
        }
        if let Some(v) = self.analyzer_mac_address.as_ref() {
            struct_ser.serialize_field("analyzerMacAddress", v)?;
        }
        if let Some(v) = self.routing_instance.as_ref() {
            struct_ser.serialize_field("routingInstance", v)?;
        }
        if let Some(v) = self.udp_port.as_ref() {
            struct_ser.serialize_field("udpPort", v)?;
        }
        if let Some(v) = self.juniper_header.as_ref() {
            struct_ser.serialize_field("juniperHeader", v)?;
        }
        if let Some(v) = self.nh_mode.as_ref() {
            struct_ser.serialize_field("nhMode", v)?;
        }
        if let Some(v) = self.static_nh_header.as_ref() {
            struct_ser.serialize_field("staticNhHeader", v)?;
        }
        if let Some(v) = self.nic_assisted_mirroring.as_ref() {
            struct_ser.serialize_field("nicAssistedMirroring", v)?;
        }
        if let Some(v) = self.nic_assisted_mirroring_vlan.as_ref() {
            struct_ser.serialize_field("nicAssistedMirroringVlan", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallMirrorActionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "analyzerName",
            "encapsulation",
            "analyzerIPAddress",
            "analyzerMacAddress",
            "routingInstance",
            "udpPort",
            "juniperHeader",
            "nhMode",
            "staticNhHeader",
            "nicAssistedMirroring",
            "nicAssistedMirroringVlan",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnalyzerName,
            Encapsulation,
            AnalyzerIpAddress,
            AnalyzerMacAddress,
            RoutingInstance,
            UdpPort,
            JuniperHeader,
            NhMode,
            StaticNhHeader,
            NicAssistedMirroring,
            NicAssistedMirroringVlan,
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
                            "analyzerName" => Ok(GeneratedField::AnalyzerName),
                            "encapsulation" => Ok(GeneratedField::Encapsulation),
                            "analyzerIPAddress" => Ok(GeneratedField::AnalyzerIpAddress),
                            "analyzerMacAddress" => Ok(GeneratedField::AnalyzerMacAddress),
                            "routingInstance" => Ok(GeneratedField::RoutingInstance),
                            "udpPort" => Ok(GeneratedField::UdpPort),
                            "juniperHeader" => Ok(GeneratedField::JuniperHeader),
                            "nhMode" => Ok(GeneratedField::NhMode),
                            "staticNhHeader" => Ok(GeneratedField::StaticNhHeader),
                            "nicAssistedMirroring" => Ok(GeneratedField::NicAssistedMirroring),
                            "nicAssistedMirroringVlan" => Ok(GeneratedField::NicAssistedMirroringVlan),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallMirrorActionType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMirrorActionType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallMirrorActionType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut analyzer_name__ = None;
                let mut encapsulation__ = None;
                let mut analyzer_ip_address__ = None;
                let mut analyzer_mac_address__ = None;
                let mut routing_instance__ = None;
                let mut udp_port__ = None;
                let mut juniper_header__ = None;
                let mut nh_mode__ = None;
                let mut static_nh_header__ = None;
                let mut nic_assisted_mirroring__ = None;
                let mut nic_assisted_mirroring_vlan__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AnalyzerName => {
                            if analyzer_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analyzerName"));
                            }
                            analyzer_name__ = map.next_value()?;
                        }
                        GeneratedField::Encapsulation => {
                            if encapsulation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encapsulation"));
                            }
                            encapsulation__ = map.next_value()?;
                        }
                        GeneratedField::AnalyzerIpAddress => {
                            if analyzer_ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analyzerIPAddress"));
                            }
                            analyzer_ip_address__ = map.next_value()?;
                        }
                        GeneratedField::AnalyzerMacAddress => {
                            if analyzer_mac_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analyzerMacAddress"));
                            }
                            analyzer_mac_address__ = map.next_value()?;
                        }
                        GeneratedField::RoutingInstance => {
                            if routing_instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routingInstance"));
                            }
                            routing_instance__ = map.next_value()?;
                        }
                        GeneratedField::UdpPort => {
                            if udp_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("udpPort"));
                            }
                            udp_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::JuniperHeader => {
                            if juniper_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("juniperHeader"));
                            }
                            juniper_header__ = map.next_value()?;
                        }
                        GeneratedField::NhMode => {
                            if nh_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nhMode"));
                            }
                            nh_mode__ = map.next_value()?;
                        }
                        GeneratedField::StaticNhHeader => {
                            if static_nh_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticNhHeader"));
                            }
                            static_nh_header__ = map.next_value()?;
                        }
                        GeneratedField::NicAssistedMirroring => {
                            if nic_assisted_mirroring__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nicAssistedMirroring"));
                            }
                            nic_assisted_mirroring__ = map.next_value()?;
                        }
                        GeneratedField::NicAssistedMirroringVlan => {
                            if nic_assisted_mirroring_vlan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nicAssistedMirroringVlan"));
                            }
                            nic_assisted_mirroring_vlan__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(FirewallMirrorActionType {
                    analyzer_name: analyzer_name__,
                    encapsulation: encapsulation__,
                    analyzer_ip_address: analyzer_ip_address__,
                    analyzer_mac_address: analyzer_mac_address__,
                    routing_instance: routing_instance__,
                    udp_port: udp_port__,
                    juniper_header: juniper_header__,
                    nh_mode: nh_mode__,
                    static_nh_header: static_nh_header__,
                    nic_assisted_mirroring: nic_assisted_mirroring__,
                    nic_assisted_mirroring_vlan: nic_assisted_mirroring_vlan__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallMirrorActionType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallPolicy {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicy", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(FirewallPolicy {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallPolicyAttribute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sequence.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyAttribute", len)?;
        if let Some(v) = self.sequence.as_ref() {
            struct_ser.serialize_field("sequence", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallPolicyAttribute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sequence",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sequence,
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
                            "sequence" => Ok(GeneratedField::Sequence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallPolicyAttribute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyAttribute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallPolicyAttribute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sequence__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ = map.next_value()?;
                        }
                    }
                }
                Ok(FirewallPolicyAttribute {
                    sequence: sequence__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyAttribute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallPolicyList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallPolicyList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallPolicyList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallPolicyList, V::Error>
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
                    }
                }
                Ok(FirewallPolicyList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallPolicyReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_reference.is_some() {
            len += 1;
        }
        if self.attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyReference", len)?;
        if let Some(v) = self.resource_reference.as_ref() {
            struct_ser.serialize_field("resourceReference", v)?;
        }
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallPolicyReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resourceReference",
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceReference,
            Attributes,
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
                            "resourceReference" => Ok(GeneratedField::ResourceReference),
                            "attributes" => Ok(GeneratedField::Attributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallPolicyReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallPolicyReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_reference__ = None;
                let mut attributes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceReference => {
                            if resource_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceReference"));
                            }
                            resource_reference__ = map.next_value()?;
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map.next_value()?;
                        }
                    }
                }
                Ok(FirewallPolicyReference {
                    resource_reference: resource_reference__,
                    attributes: attributes__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallPolicySpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if !self.firewall_rule.is_empty() {
            len += 1;
        }
        if self.vmi_selector.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicySpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if !self.firewall_rule.is_empty() {
            struct_ser.serialize_field("firewallRule", &self.firewall_rule)?;
        }
        if let Some(v) = self.vmi_selector.as_ref() {
            struct_ser.serialize_field("vmiSelector", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallPolicySpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "firewallRule",
            "vmiSelector",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            FirewallRule,
            VmiSelector,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "firewallRule" => Ok(GeneratedField::FirewallRule),
                            "vmiSelector" => Ok(GeneratedField::VmiSelector),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallPolicySpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicySpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallPolicySpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut firewall_rule__ = None;
                let mut vmi_selector__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::FirewallRule => {
                            if firewall_rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firewallRule"));
                            }
                            firewall_rule__ = Some(map.next_value()?);
                        }
                        GeneratedField::VmiSelector => {
                            if vmi_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vmiSelector"));
                            }
                            vmi_selector__ = map.next_value()?;
                        }
                    }
                }
                Ok(FirewallPolicySpec {
                    common_spec: common_spec__,
                    firewall_rule: firewall_rule__.unwrap_or_default(),
                    vmi_selector: vmi_selector__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicySpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallPolicyStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallPolicyStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallPolicyStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallPolicyStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(FirewallPolicyStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallPolicyVmiSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.match_expr_list.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyVMISelector", len)?;
        if !self.match_expr_list.is_empty() {
            struct_ser.serialize_field("matchExprList", &self.match_expr_list)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallPolicyVmiSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "matchExprList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MatchExprList,
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
                            "matchExprList" => Ok(GeneratedField::MatchExprList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallPolicyVmiSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyVMISelector")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallPolicyVmiSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut match_expr_list__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MatchExprList => {
                            if match_expr_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchExprList"));
                            }
                            match_expr_list__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FirewallPolicyVmiSelector {
                    match_expr_list: match_expr_list__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallPolicyVMISelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallRule {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRule", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(FirewallRule {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallRuleEndpointType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subnet.is_some() {
            len += 1;
        }
        if self.address_group.is_some() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if !self.tag_ids.is_empty() {
            len += 1;
        }
        if self.any.is_some() {
            len += 1;
        }
        if !self.match_expr.is_empty() {
            len += 1;
        }
        if !self.match_expr_list.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleEndpointType", len)?;
        if let Some(v) = self.subnet.as_ref() {
            struct_ser.serialize_field("subnet", v)?;
        }
        if let Some(v) = self.address_group.as_ref() {
            struct_ser.serialize_field("addressGroup", v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if !self.tag_ids.is_empty() {
            struct_ser.serialize_field("tagIds", &self.tag_ids.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.any.as_ref() {
            struct_ser.serialize_field("any", v)?;
        }
        if !self.match_expr.is_empty() {
            struct_ser.serialize_field("matchExpr", &self.match_expr)?;
        }
        if !self.match_expr_list.is_empty() {
            struct_ser.serialize_field("matchExprList", &self.match_expr_list)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallRuleEndpointType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subnet",
            "addressGroup",
            "tags",
            "tagIds",
            "any",
            "matchExpr",
            "matchExprList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subnet,
            AddressGroup,
            Tags,
            TagIds,
            Any,
            MatchExpr,
            MatchExprList,
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
                            "subnet" => Ok(GeneratedField::Subnet),
                            "addressGroup" => Ok(GeneratedField::AddressGroup),
                            "tags" => Ok(GeneratedField::Tags),
                            "tagIds" => Ok(GeneratedField::TagIds),
                            "any" => Ok(GeneratedField::Any),
                            "matchExpr" => Ok(GeneratedField::MatchExpr),
                            "matchExprList" => Ok(GeneratedField::MatchExprList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallRuleEndpointType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleEndpointType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallRuleEndpointType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subnet__ = None;
                let mut address_group__ = None;
                let mut tags__ = None;
                let mut tag_ids__ = None;
                let mut any__ = None;
                let mut match_expr__ = None;
                let mut match_expr_list__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Subnet => {
                            if subnet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subnet"));
                            }
                            subnet__ = map.next_value()?;
                        }
                        GeneratedField::AddressGroup => {
                            if address_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressGroup"));
                            }
                            address_group__ = map.next_value()?;
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map.next_value()?);
                        }
                        GeneratedField::TagIds => {
                            if tag_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagIds"));
                            }
                            tag_ids__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Any => {
                            if any__.is_some() {
                                return Err(serde::de::Error::duplicate_field("any"));
                            }
                            any__ = map.next_value()?;
                        }
                        GeneratedField::MatchExpr => {
                            if match_expr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchExpr"));
                            }
                            match_expr__ = Some(map.next_value()?);
                        }
                        GeneratedField::MatchExprList => {
                            if match_expr_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchExprList"));
                            }
                            match_expr_list__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FirewallRuleEndpointType {
                    subnet: subnet__,
                    address_group: address_group__,
                    tags: tags__.unwrap_or_default(),
                    tag_ids: tag_ids__.unwrap_or_default(),
                    any: any__,
                    match_expr: match_expr__.unwrap_or_default(),
                    match_expr_list: match_expr_list__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleEndpointType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallRuleList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallRuleList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallRuleList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallRuleList, V::Error>
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
                    }
                }
                Ok(FirewallRuleList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallRuleReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_reference.is_some() {
            len += 1;
        }
        if self.attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleReference", len)?;
        if let Some(v) = self.resource_reference.as_ref() {
            struct_ser.serialize_field("resourceReference", v)?;
        }
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallRuleReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resourceReference",
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceReference,
            Attributes,
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
                            "resourceReference" => Ok(GeneratedField::ResourceReference),
                            "attributes" => Ok(GeneratedField::Attributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallRuleReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallRuleReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_reference__ = None;
                let mut attributes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceReference => {
                            if resource_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceReference"));
                            }
                            resource_reference__ = map.next_value()?;
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map.next_value()?;
                        }
                    }
                }
                Ok(FirewallRuleReference {
                    resource_reference: resource_reference__,
                    attributes: attributes__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallRuleSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.service_group_reference.is_some() {
            len += 1;
        }
        if self.address_group_reference.is_some() {
            len += 1;
        }
        if self.action_list.is_some() {
            len += 1;
        }
        if self.service.is_some() {
            len += 1;
        }
        if self.endpoint1.is_some() {
            len += 1;
        }
        if self.endpoint2.is_some() {
            len += 1;
        }
        if !self.match_tags.is_empty() {
            len += 1;
        }
        if !self.match_tags_types.is_empty() {
            len += 1;
        }
        if self.direction.is_some() {
            len += 1;
        }
        if !self.tag_references.is_empty() {
            len += 1;
        }
        if self.k8s_mode.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.service_group_reference.as_ref() {
            struct_ser.serialize_field("serviceGroupReference", v)?;
        }
        if let Some(v) = self.address_group_reference.as_ref() {
            struct_ser.serialize_field("addressGroupReference", v)?;
        }
        if let Some(v) = self.action_list.as_ref() {
            struct_ser.serialize_field("actionList", v)?;
        }
        if let Some(v) = self.service.as_ref() {
            struct_ser.serialize_field("service", v)?;
        }
        if let Some(v) = self.endpoint1.as_ref() {
            struct_ser.serialize_field("Endpoint1", v)?;
        }
        if let Some(v) = self.endpoint2.as_ref() {
            struct_ser.serialize_field("Endpoint2", v)?;
        }
        if !self.match_tags.is_empty() {
            struct_ser.serialize_field("matchTags", &self.match_tags)?;
        }
        if !self.match_tags_types.is_empty() {
            struct_ser.serialize_field("matchTagsTypes", &self.match_tags_types.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.direction.as_ref() {
            struct_ser.serialize_field("direction", v)?;
        }
        if !self.tag_references.is_empty() {
            struct_ser.serialize_field("tagReferences", &self.tag_references)?;
        }
        if let Some(v) = self.k8s_mode.as_ref() {
            struct_ser.serialize_field("k8sMode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallRuleSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "serviceGroupReference",
            "addressGroupReference",
            "actionList",
            "service",
            "Endpoint1",
            "Endpoint2",
            "matchTags",
            "matchTagsTypes",
            "direction",
            "tagReferences",
            "k8sMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            ServiceGroupReference,
            AddressGroupReference,
            ActionList,
            Service,
            Endpoint1,
            Endpoint2,
            MatchTags,
            MatchTagsTypes,
            Direction,
            TagReferences,
            K8sMode,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "serviceGroupReference" => Ok(GeneratedField::ServiceGroupReference),
                            "addressGroupReference" => Ok(GeneratedField::AddressGroupReference),
                            "actionList" => Ok(GeneratedField::ActionList),
                            "service" => Ok(GeneratedField::Service),
                            "Endpoint1" => Ok(GeneratedField::Endpoint1),
                            "Endpoint2" => Ok(GeneratedField::Endpoint2),
                            "matchTags" => Ok(GeneratedField::MatchTags),
                            "matchTagsTypes" => Ok(GeneratedField::MatchTagsTypes),
                            "direction" => Ok(GeneratedField::Direction),
                            "tagReferences" => Ok(GeneratedField::TagReferences),
                            "k8sMode" => Ok(GeneratedField::K8sMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallRuleSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallRuleSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut service_group_reference__ = None;
                let mut address_group_reference__ = None;
                let mut action_list__ = None;
                let mut service__ = None;
                let mut endpoint1__ = None;
                let mut endpoint2__ = None;
                let mut match_tags__ = None;
                let mut match_tags_types__ = None;
                let mut direction__ = None;
                let mut tag_references__ = None;
                let mut k8s_mode__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::ServiceGroupReference => {
                            if service_group_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceGroupReference"));
                            }
                            service_group_reference__ = map.next_value()?;
                        }
                        GeneratedField::AddressGroupReference => {
                            if address_group_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressGroupReference"));
                            }
                            address_group_reference__ = map.next_value()?;
                        }
                        GeneratedField::ActionList => {
                            if action_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionList"));
                            }
                            action_list__ = map.next_value()?;
                        }
                        GeneratedField::Service => {
                            if service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service__ = map.next_value()?;
                        }
                        GeneratedField::Endpoint1 => {
                            if endpoint1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Endpoint1"));
                            }
                            endpoint1__ = map.next_value()?;
                        }
                        GeneratedField::Endpoint2 => {
                            if endpoint2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Endpoint2"));
                            }
                            endpoint2__ = map.next_value()?;
                        }
                        GeneratedField::MatchTags => {
                            if match_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchTags"));
                            }
                            match_tags__ = Some(map.next_value()?);
                        }
                        GeneratedField::MatchTagsTypes => {
                            if match_tags_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchTagsTypes"));
                            }
                            match_tags_types__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = map.next_value()?;
                        }
                        GeneratedField::TagReferences => {
                            if tag_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagReferences"));
                            }
                            tag_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::K8sMode => {
                            if k8s_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("k8sMode"));
                            }
                            k8s_mode__ = map.next_value()?;
                        }
                    }
                }
                Ok(FirewallRuleSpec {
                    common_spec: common_spec__,
                    service_group_reference: service_group_reference__,
                    address_group_reference: address_group_reference__,
                    action_list: action_list__,
                    service: service__,
                    endpoint1: endpoint1__,
                    endpoint2: endpoint2__,
                    match_tags: match_tags__.unwrap_or_default(),
                    match_tags_types: match_tags_types__.unwrap_or_default(),
                    direction: direction__,
                    tag_references: tag_references__.unwrap_or_default(),
                    k8s_mode: k8s_mode__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallRuleStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallRuleStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallRuleStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallRuleStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(FirewallRuleStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallRuleStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallServiceGroupType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.firewall_service.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallServiceGroupType", len)?;
        if !self.firewall_service.is_empty() {
            struct_ser.serialize_field("FirewallService", &self.firewall_service)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallServiceGroupType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FirewallService",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FirewallService,
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
                            "FirewallService" => Ok(GeneratedField::FirewallService),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallServiceGroupType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallServiceGroupType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallServiceGroupType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut firewall_service__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FirewallService => {
                            if firewall_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("FirewallService"));
                            }
                            firewall_service__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FirewallServiceGroupType {
                    firewall_service: firewall_service__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallServiceGroupType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallServiceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.protocol.is_some() {
            len += 1;
        }
        if self.protocol_id.is_some() {
            len += 1;
        }
        if self.source_ports.is_some() {
            len += 1;
        }
        if self.destination_ports.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallServiceType", len)?;
        if let Some(v) = self.protocol.as_ref() {
            struct_ser.serialize_field("protocol", v)?;
        }
        if let Some(v) = self.protocol_id.as_ref() {
            struct_ser.serialize_field("protocolId", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.source_ports.as_ref() {
            struct_ser.serialize_field("sourcePorts", v)?;
        }
        if let Some(v) = self.destination_ports.as_ref() {
            struct_ser.serialize_field("destinationPorts", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallServiceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protocol",
            "protocolId",
            "sourcePorts",
            "destinationPorts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Protocol,
            ProtocolId,
            SourcePorts,
            DestinationPorts,
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
                            "protocol" => Ok(GeneratedField::Protocol),
                            "protocolId" => Ok(GeneratedField::ProtocolId),
                            "sourcePorts" => Ok(GeneratedField::SourcePorts),
                            "destinationPorts" => Ok(GeneratedField::DestinationPorts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallServiceType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallServiceType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallServiceType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protocol__ = None;
                let mut protocol_id__ = None;
                let mut source_ports__ = None;
                let mut destination_ports__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = map.next_value()?;
                        }
                        GeneratedField::ProtocolId => {
                            if protocol_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolId"));
                            }
                            protocol_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SourcePorts => {
                            if source_ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePorts"));
                            }
                            source_ports__ = map.next_value()?;
                        }
                        GeneratedField::DestinationPorts => {
                            if destination_ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationPorts"));
                            }
                            destination_ports__ = map.next_value()?;
                        }
                    }
                }
                Ok(FirewallServiceType {
                    protocol: protocol__,
                    protocol_id: protocol_id__,
                    source_ports: source_ports__,
                    destination_ports: destination_ports__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallServiceType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirewallSubnet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ip_prefix.is_some() {
            len += 1;
        }
        if self.ip_prefix_len.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallSubnet", len)?;
        if let Some(v) = self.ip_prefix.as_ref() {
            struct_ser.serialize_field("ipPrefix", v)?;
        }
        if let Some(v) = self.ip_prefix_len.as_ref() {
            struct_ser.serialize_field("ipPrefixLen", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirewallSubnet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ipPrefix",
            "ipPrefixLen",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IpPrefix,
            IpPrefixLen,
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
                            "ipPrefix" => Ok(GeneratedField::IpPrefix),
                            "ipPrefixLen" => Ok(GeneratedField::IpPrefixLen),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirewallSubnet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallSubnet")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FirewallSubnet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ip_prefix__ = None;
                let mut ip_prefix_len__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IpPrefix => {
                            if ip_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipPrefix"));
                            }
                            ip_prefix__ = map.next_value()?;
                        }
                        GeneratedField::IpPrefixLen => {
                            if ip_prefix_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipPrefixLen"));
                            }
                            ip_prefix_len__ = map.next_value()?;
                        }
                    }
                }
                Ok(FirewallSubnet {
                    ip_prefix: ip_prefix__,
                    ip_prefix_len: ip_prefix_len__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FirewallSubnet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FloatingIp {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIP", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FloatingIp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIP")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FloatingIp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(FloatingIp {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FloatingIpList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIpList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FloatingIpList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FloatingIpList, V::Error>
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
                    }
                }
                Ok(FloatingIpList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FloatingIpPortMappings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.port_mappings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPPortMappings", len)?;
        if !self.port_mappings.is_empty() {
            struct_ser.serialize_field("portMappings", &self.port_mappings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIpPortMappings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "portMappings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PortMappings,
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
                            "portMappings" => Ok(GeneratedField::PortMappings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FloatingIpPortMappings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPPortMappings")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FloatingIpPortMappings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut port_mappings__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PortMappings => {
                            if port_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portMappings"));
                            }
                            port_mappings__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FloatingIpPortMappings {
                    port_mappings: port_mappings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPPortMappings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FloatingIpPortPortMapping {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.src_port.is_some() {
            len += 1;
        }
        if self.dst_port.is_some() {
            len += 1;
        }
        if self.protocol.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPPortPortMapping", len)?;
        if let Some(v) = self.src_port.as_ref() {
            struct_ser.serialize_field("srcPort", v)?;
        }
        if let Some(v) = self.dst_port.as_ref() {
            struct_ser.serialize_field("dstPort", v)?;
        }
        if let Some(v) = self.protocol.as_ref() {
            struct_ser.serialize_field("protocol", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIpPortPortMapping {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "srcPort",
            "dstPort",
            "protocol",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SrcPort,
            DstPort,
            Protocol,
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
                            "srcPort" => Ok(GeneratedField::SrcPort),
                            "dstPort" => Ok(GeneratedField::DstPort),
                            "protocol" => Ok(GeneratedField::Protocol),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FloatingIpPortPortMapping;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPPortPortMapping")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FloatingIpPortPortMapping, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut src_port__ = None;
                let mut dst_port__ = None;
                let mut protocol__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SrcPort => {
                            if src_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srcPort"));
                            }
                            src_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::DstPort => {
                            if dst_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dstPort"));
                            }
                            dst_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = map.next_value()?;
                        }
                    }
                }
                Ok(FloatingIpPortPortMapping {
                    src_port: src_port__,
                    dst_port: dst_port__,
                    protocol: protocol__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPPortPortMapping", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FloatingIpSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.floating_ip_address.is_some() {
            len += 1;
        }
        if self.floating_ip_port_mappings.is_some() {
            len += 1;
        }
        if self.floating_ip_traffic_direction.is_some() {
            len += 1;
        }
        if self.parent.is_some() {
            len += 1;
        }
        if !self.virtual_machine_interface_references.is_empty() {
            len += 1;
        }
        if self.floating_ip_port_mappings_enable.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.floating_ip_address.as_ref() {
            struct_ser.serialize_field("floatingIPAddress", v)?;
        }
        if let Some(v) = self.floating_ip_port_mappings.as_ref() {
            struct_ser.serialize_field("floatingIPPortMappings", v)?;
        }
        if let Some(v) = self.floating_ip_traffic_direction.as_ref() {
            struct_ser.serialize_field("floatingIPTrafficDirection", v)?;
        }
        if let Some(v) = self.parent.as_ref() {
            struct_ser.serialize_field("parent", v)?;
        }
        if !self.virtual_machine_interface_references.is_empty() {
            struct_ser.serialize_field("virtualMachineInterfaceReferences", &self.virtual_machine_interface_references)?;
        }
        if let Some(v) = self.floating_ip_port_mappings_enable.as_ref() {
            struct_ser.serialize_field("floatingIPPortMappingsEnable", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIpSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "floatingIPAddress",
            "floatingIPPortMappings",
            "floatingIPTrafficDirection",
            "parent",
            "virtualMachineInterfaceReferences",
            "floatingIPPortMappingsEnable",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            FloatingIpAddress,
            FloatingIpPortMappings,
            FloatingIpTrafficDirection,
            Parent,
            VirtualMachineInterfaceReferences,
            FloatingIpPortMappingsEnable,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "floatingIPAddress" => Ok(GeneratedField::FloatingIpAddress),
                            "floatingIPPortMappings" => Ok(GeneratedField::FloatingIpPortMappings),
                            "floatingIPTrafficDirection" => Ok(GeneratedField::FloatingIpTrafficDirection),
                            "parent" => Ok(GeneratedField::Parent),
                            "virtualMachineInterfaceReferences" => Ok(GeneratedField::VirtualMachineInterfaceReferences),
                            "floatingIPPortMappingsEnable" => Ok(GeneratedField::FloatingIpPortMappingsEnable),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FloatingIpSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FloatingIpSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut floating_ip_address__ = None;
                let mut floating_ip_port_mappings__ = None;
                let mut floating_ip_traffic_direction__ = None;
                let mut parent__ = None;
                let mut virtual_machine_interface_references__ = None;
                let mut floating_ip_port_mappings_enable__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::FloatingIpAddress => {
                            if floating_ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floatingIPAddress"));
                            }
                            floating_ip_address__ = map.next_value()?;
                        }
                        GeneratedField::FloatingIpPortMappings => {
                            if floating_ip_port_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floatingIPPortMappings"));
                            }
                            floating_ip_port_mappings__ = map.next_value()?;
                        }
                        GeneratedField::FloatingIpTrafficDirection => {
                            if floating_ip_traffic_direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floatingIPTrafficDirection"));
                            }
                            floating_ip_traffic_direction__ = map.next_value()?;
                        }
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = map.next_value()?;
                        }
                        GeneratedField::VirtualMachineInterfaceReferences => {
                            if virtual_machine_interface_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualMachineInterfaceReferences"));
                            }
                            virtual_machine_interface_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::FloatingIpPortMappingsEnable => {
                            if floating_ip_port_mappings_enable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floatingIPPortMappingsEnable"));
                            }
                            floating_ip_port_mappings_enable__ = map.next_value()?;
                        }
                    }
                }
                Ok(FloatingIpSpec {
                    common_spec: common_spec__,
                    floating_ip_address: floating_ip_address__,
                    floating_ip_port_mappings: floating_ip_port_mappings__,
                    floating_ip_traffic_direction: floating_ip_traffic_direction__,
                    parent: parent__,
                    virtual_machine_interface_references: virtual_machine_interface_references__.unwrap_or_default(),
                    floating_ip_port_mappings_enable: floating_ip_port_mappings_enable__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FloatingIpStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        if self.subnet_reference.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        if let Some(v) = self.subnet_reference.as_ref() {
            struct_ser.serialize_field("subnetReference", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIpStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
            "subnetReference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
            SubnetReference,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            "subnetReference" => Ok(GeneratedField::SubnetReference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FloatingIpStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FloatingIpStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                let mut subnet_reference__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                        GeneratedField::SubnetReference => {
                            if subnet_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subnetReference"));
                            }
                            subnet_reference__ = map.next_value()?;
                        }
                    }
                }
                Ok(FloatingIpStatus {
                    common_status: common_status__,
                    subnet_reference: subnet_reference__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalContrailSecurityPolicy {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicy", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalContrailSecurityPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalContrailSecurityPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalContrailSecurityPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(GlobalContrailSecurityPolicy {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalContrailSecurityPolicyList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicyList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalContrailSecurityPolicyList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalContrailSecurityPolicyList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicyList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalContrailSecurityPolicyList, V::Error>
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
                    }
                }
                Ok(GlobalContrailSecurityPolicyList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicyList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalContrailSecurityPolicyPodSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pod_selector.is_some() {
            len += 1;
        }
        if self.namespace_selector.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicyPodSelector", len)?;
        if let Some(v) = self.pod_selector.as_ref() {
            struct_ser.serialize_field("podSelector", v)?;
        }
        if let Some(v) = self.namespace_selector.as_ref() {
            struct_ser.serialize_field("namespaceSelector", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalContrailSecurityPolicyPodSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "podSelector",
            "namespaceSelector",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PodSelector,
            NamespaceSelector,
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
                            "podSelector" => Ok(GeneratedField::PodSelector),
                            "namespaceSelector" => Ok(GeneratedField::NamespaceSelector),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalContrailSecurityPolicyPodSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicyPodSelector")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalContrailSecurityPolicyPodSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pod_selector__ = None;
                let mut namespace_selector__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PodSelector => {
                            if pod_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("podSelector"));
                            }
                            pod_selector__ = map.next_value()?;
                        }
                        GeneratedField::NamespaceSelector => {
                            if namespace_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceSelector"));
                            }
                            namespace_selector__ = map.next_value()?;
                        }
                    }
                }
                Ok(GlobalContrailSecurityPolicyPodSelector {
                    pod_selector: pod_selector__,
                    namespace_selector: namespace_selector__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicyPodSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalContrailSecurityPolicySpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.selectors.is_empty() {
            len += 1;
        }
        if !self.rules.is_empty() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        if self.secondary_actions.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicySpec", len)?;
        if !self.selectors.is_empty() {
            struct_ser.serialize_field("selectors", &self.selectors)?;
        }
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        if let Some(v) = self.secondary_actions.as_ref() {
            struct_ser.serialize_field("secondaryActions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalContrailSecurityPolicySpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "selectors",
            "rules",
            "action",
            "secondaryActions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Selectors,
            Rules,
            Action,
            SecondaryActions,
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
                            "selectors" => Ok(GeneratedField::Selectors),
                            "rules" => Ok(GeneratedField::Rules),
                            "action" => Ok(GeneratedField::Action),
                            "secondaryActions" => Ok(GeneratedField::SecondaryActions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalContrailSecurityPolicySpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicySpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalContrailSecurityPolicySpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut selectors__ = None;
                let mut rules__ = None;
                let mut action__ = None;
                let mut secondary_actions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Selectors => {
                            if selectors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selectors"));
                            }
                            selectors__ = Some(map.next_value()?);
                        }
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map.next_value()?);
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map.next_value()?;
                        }
                        GeneratedField::SecondaryActions => {
                            if secondary_actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secondaryActions"));
                            }
                            secondary_actions__ = map.next_value()?;
                        }
                    }
                }
                Ok(GlobalContrailSecurityPolicySpec {
                    selectors: selectors__.unwrap_or_default(),
                    rules: rules__.unwrap_or_default(),
                    action: action__,
                    secondary_actions: secondary_actions__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicySpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalContrailSecurityPolicyStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicyStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalContrailSecurityPolicyStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalContrailSecurityPolicyStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicyStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalContrailSecurityPolicyStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(GlobalContrailSecurityPolicyStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalContrailSecurityPolicyStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalSystemConfig {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfig", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalSystemConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalSystemConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalSystemConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(GlobalSystemConfig {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalSystemConfigList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfigList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalSystemConfigList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalSystemConfigList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfigList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalSystemConfigList, V::Error>
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
                    }
                }
                Ok(GlobalSystemConfigList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfigList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalSystemConfigSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.enable4bytes_as.is_some() {
            len += 1;
        }
        if self.autonomous_system.is_some() {
            len += 1;
        }
        if !self.bgp_router_references.is_empty() {
            len += 1;
        }
        if self.ibgp_auto_mesh.is_some() {
            len += 1;
        }
        if self.default_enable_snat.is_some() {
            len += 1;
        }
        if self.graceful_restart_parameters.is_some() {
            len += 1;
        }
        if self.fast_convergence_parameters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfigSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.enable4bytes_as.as_ref() {
            struct_ser.serialize_field("enable4bytesAS", v)?;
        }
        if let Some(v) = self.autonomous_system.as_ref() {
            struct_ser.serialize_field("autonomousSystem", v)?;
        }
        if !self.bgp_router_references.is_empty() {
            struct_ser.serialize_field("bgpRouterReferences", &self.bgp_router_references)?;
        }
        if let Some(v) = self.ibgp_auto_mesh.as_ref() {
            struct_ser.serialize_field("ibgpAutoMesh", v)?;
        }
        if let Some(v) = self.default_enable_snat.as_ref() {
            struct_ser.serialize_field("defaultEnableSNAT", v)?;
        }
        if let Some(v) = self.graceful_restart_parameters.as_ref() {
            struct_ser.serialize_field("gracefulRestartParameters", v)?;
        }
        if let Some(v) = self.fast_convergence_parameters.as_ref() {
            struct_ser.serialize_field("fastConvergenceParameters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalSystemConfigSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "enable4bytesAS",
            "autonomousSystem",
            "bgpRouterReferences",
            "ibgpAutoMesh",
            "defaultEnableSNAT",
            "gracefulRestartParameters",
            "fastConvergenceParameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            Enable4bytesAs,
            AutonomousSystem,
            BgpRouterReferences,
            IbgpAutoMesh,
            DefaultEnableSnat,
            GracefulRestartParameters,
            FastConvergenceParameters,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "enable4bytesAS" => Ok(GeneratedField::Enable4bytesAs),
                            "autonomousSystem" => Ok(GeneratedField::AutonomousSystem),
                            "bgpRouterReferences" => Ok(GeneratedField::BgpRouterReferences),
                            "ibgpAutoMesh" => Ok(GeneratedField::IbgpAutoMesh),
                            "defaultEnableSNAT" => Ok(GeneratedField::DefaultEnableSnat),
                            "gracefulRestartParameters" => Ok(GeneratedField::GracefulRestartParameters),
                            "fastConvergenceParameters" => Ok(GeneratedField::FastConvergenceParameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalSystemConfigSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfigSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalSystemConfigSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut enable4bytes_as__ = None;
                let mut autonomous_system__ = None;
                let mut bgp_router_references__ = None;
                let mut ibgp_auto_mesh__ = None;
                let mut default_enable_snat__ = None;
                let mut graceful_restart_parameters__ = None;
                let mut fast_convergence_parameters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::Enable4bytesAs => {
                            if enable4bytes_as__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enable4bytesAS"));
                            }
                            enable4bytes_as__ = map.next_value()?;
                        }
                        GeneratedField::AutonomousSystem => {
                            if autonomous_system__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autonomousSystem"));
                            }
                            autonomous_system__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BgpRouterReferences => {
                            if bgp_router_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bgpRouterReferences"));
                            }
                            bgp_router_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::IbgpAutoMesh => {
                            if ibgp_auto_mesh__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ibgpAutoMesh"));
                            }
                            ibgp_auto_mesh__ = map.next_value()?;
                        }
                        GeneratedField::DefaultEnableSnat => {
                            if default_enable_snat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultEnableSNAT"));
                            }
                            default_enable_snat__ = map.next_value()?;
                        }
                        GeneratedField::GracefulRestartParameters => {
                            if graceful_restart_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gracefulRestartParameters"));
                            }
                            graceful_restart_parameters__ = map.next_value()?;
                        }
                        GeneratedField::FastConvergenceParameters => {
                            if fast_convergence_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fastConvergenceParameters"));
                            }
                            fast_convergence_parameters__ = map.next_value()?;
                        }
                    }
                }
                Ok(GlobalSystemConfigSpec {
                    common_spec: common_spec__,
                    enable4bytes_as: enable4bytes_as__,
                    autonomous_system: autonomous_system__,
                    bgp_router_references: bgp_router_references__.unwrap_or_default(),
                    ibgp_auto_mesh: ibgp_auto_mesh__,
                    default_enable_snat: default_enable_snat__,
                    graceful_restart_parameters: graceful_restart_parameters__,
                    fast_convergence_parameters: fast_convergence_parameters__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfigSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalSystemConfigStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfigStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalSystemConfigStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalSystemConfigStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfigStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalSystemConfigStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(GlobalSystemConfigStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalSystemConfigStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalVrouterConfig {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfig", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalVrouterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalVrouterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalVrouterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(GlobalVrouterConfig {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalVrouterConfigList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfigList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalVrouterConfigList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalVrouterConfigList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfigList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalVrouterConfigList, V::Error>
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
                    }
                }
                Ok(GlobalVrouterConfigList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfigList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalVrouterConfigSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.parent.is_some() {
            len += 1;
        }
        if self.encapsulation_priorities.is_some() {
            len += 1;
        }
        if self.linklocal_services.is_some() {
            len += 1;
        }
        if self.port_translation_pools.is_some() {
            len += 1;
        }
        if self.flow_export_rate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfigSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.parent.as_ref() {
            struct_ser.serialize_field("parent", v)?;
        }
        if let Some(v) = self.encapsulation_priorities.as_ref() {
            struct_ser.serialize_field("encapsulationPriorities", v)?;
        }
        if let Some(v) = self.linklocal_services.as_ref() {
            struct_ser.serialize_field("linklocalServices", v)?;
        }
        if let Some(v) = self.port_translation_pools.as_ref() {
            struct_ser.serialize_field("portTranslationPools", v)?;
        }
        if let Some(v) = self.flow_export_rate.as_ref() {
            struct_ser.serialize_field("flowExportRate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalVrouterConfigSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "parent",
            "encapsulationPriorities",
            "linklocalServices",
            "portTranslationPools",
            "flowExportRate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            Parent,
            EncapsulationPriorities,
            LinklocalServices,
            PortTranslationPools,
            FlowExportRate,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "parent" => Ok(GeneratedField::Parent),
                            "encapsulationPriorities" => Ok(GeneratedField::EncapsulationPriorities),
                            "linklocalServices" => Ok(GeneratedField::LinklocalServices),
                            "portTranslationPools" => Ok(GeneratedField::PortTranslationPools),
                            "flowExportRate" => Ok(GeneratedField::FlowExportRate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalVrouterConfigSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfigSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalVrouterConfigSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut parent__ = None;
                let mut encapsulation_priorities__ = None;
                let mut linklocal_services__ = None;
                let mut port_translation_pools__ = None;
                let mut flow_export_rate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = map.next_value()?;
                        }
                        GeneratedField::EncapsulationPriorities => {
                            if encapsulation_priorities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encapsulationPriorities"));
                            }
                            encapsulation_priorities__ = map.next_value()?;
                        }
                        GeneratedField::LinklocalServices => {
                            if linklocal_services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linklocalServices"));
                            }
                            linklocal_services__ = map.next_value()?;
                        }
                        GeneratedField::PortTranslationPools => {
                            if port_translation_pools__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portTranslationPools"));
                            }
                            port_translation_pools__ = map.next_value()?;
                        }
                        GeneratedField::FlowExportRate => {
                            if flow_export_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flowExportRate"));
                            }
                            flow_export_rate__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(GlobalVrouterConfigSpec {
                    common_spec: common_spec__,
                    parent: parent__,
                    encapsulation_priorities: encapsulation_priorities__,
                    linklocal_services: linklocal_services__,
                    port_translation_pools: port_translation_pools__,
                    flow_export_rate: flow_export_rate__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfigSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GlobalVrouterConfigStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfigStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalVrouterConfigStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GlobalVrouterConfigStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfigStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GlobalVrouterConfigStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(GlobalVrouterConfigStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GlobalVrouterConfigStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GracefulRestartParametersType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bgp_helper_enable.is_some() {
            len += 1;
        }
        if self.enable.is_some() {
            len += 1;
        }
        if self.end_of_rib_timeout.is_some() {
            len += 1;
        }
        if self.long_lived_restart_time.is_some() {
            len += 1;
        }
        if self.restart_time.is_some() {
            len += 1;
        }
        if self.xmpp_helper_enable.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GracefulRestartParametersType", len)?;
        if let Some(v) = self.bgp_helper_enable.as_ref() {
            struct_ser.serialize_field("bgpHelperEnable", v)?;
        }
        if let Some(v) = self.enable.as_ref() {
            struct_ser.serialize_field("enable", v)?;
        }
        if let Some(v) = self.end_of_rib_timeout.as_ref() {
            struct_ser.serialize_field("endOfRibTimeout", v)?;
        }
        if let Some(v) = self.long_lived_restart_time.as_ref() {
            struct_ser.serialize_field("longLivedRestartTime", v)?;
        }
        if let Some(v) = self.restart_time.as_ref() {
            struct_ser.serialize_field("restartTime", v)?;
        }
        if let Some(v) = self.xmpp_helper_enable.as_ref() {
            struct_ser.serialize_field("xmppHelperEnable", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GracefulRestartParametersType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bgpHelperEnable",
            "enable",
            "endOfRibTimeout",
            "longLivedRestartTime",
            "restartTime",
            "xmppHelperEnable",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BgpHelperEnable,
            Enable,
            EndOfRibTimeout,
            LongLivedRestartTime,
            RestartTime,
            XmppHelperEnable,
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
                            "bgpHelperEnable" => Ok(GeneratedField::BgpHelperEnable),
                            "enable" => Ok(GeneratedField::Enable),
                            "endOfRibTimeout" => Ok(GeneratedField::EndOfRibTimeout),
                            "longLivedRestartTime" => Ok(GeneratedField::LongLivedRestartTime),
                            "restartTime" => Ok(GeneratedField::RestartTime),
                            "xmppHelperEnable" => Ok(GeneratedField::XmppHelperEnable),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GracefulRestartParametersType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GracefulRestartParametersType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GracefulRestartParametersType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bgp_helper_enable__ = None;
                let mut enable__ = None;
                let mut end_of_rib_timeout__ = None;
                let mut long_lived_restart_time__ = None;
                let mut restart_time__ = None;
                let mut xmpp_helper_enable__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BgpHelperEnable => {
                            if bgp_helper_enable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bgpHelperEnable"));
                            }
                            bgp_helper_enable__ = map.next_value()?;
                        }
                        GeneratedField::Enable => {
                            if enable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enable"));
                            }
                            enable__ = map.next_value()?;
                        }
                        GeneratedField::EndOfRibTimeout => {
                            if end_of_rib_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endOfRibTimeout"));
                            }
                            end_of_rib_timeout__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::LongLivedRestartTime => {
                            if long_lived_restart_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("longLivedRestartTime"));
                            }
                            long_lived_restart_time__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RestartTime => {
                            if restart_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restartTime"));
                            }
                            restart_time__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::XmppHelperEnable => {
                            if xmpp_helper_enable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("xmppHelperEnable"));
                            }
                            xmpp_helper_enable__ = map.next_value()?;
                        }
                    }
                }
                Ok(GracefulRestartParametersType {
                    bgp_helper_enable: bgp_helper_enable__,
                    enable: enable__,
                    end_of_rib_timeout: end_of_rib_timeout__,
                    long_lived_restart_time: long_lived_restart_time__,
                    restart_time: restart_time__,
                    xmpp_helper_enable: xmpp_helper_enable__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.GracefulRestartParametersType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IpRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.from.is_some() {
            len += 1;
        }
        if self.to.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.IPRange", len)?;
        if let Some(v) = self.from.as_ref() {
            struct_ser.serialize_field("from", v)?;
        }
        if let Some(v) = self.to.as_ref() {
            struct_ser.serialize_field("to", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IpRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "to",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            To,
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
                            "from" => Ok(GeneratedField::From),
                            "to" => Ok(GeneratedField::To),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IpRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.IPRange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IpRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut to__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = map.next_value()?;
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = map.next_value()?;
                        }
                    }
                }
                Ok(IpRange {
                    from: from__,
                    to: to__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.IPRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportVirtualNetworkRouter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.virtual_network_routers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ImportVirtualNetworkRouter", len)?;
        if !self.virtual_network_routers.is_empty() {
            struct_ser.serialize_field("virtualNetworkRouters", &self.virtual_network_routers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportVirtualNetworkRouter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "virtualNetworkRouters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VirtualNetworkRouters,
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
                            "virtualNetworkRouters" => Ok(GeneratedField::VirtualNetworkRouters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportVirtualNetworkRouter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ImportVirtualNetworkRouter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ImportVirtualNetworkRouter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut virtual_network_routers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VirtualNetworkRouters => {
                            if virtual_network_routers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualNetworkRouters"));
                            }
                            virtual_network_routers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ImportVirtualNetworkRouter {
                    virtual_network_routers: virtual_network_routers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ImportVirtualNetworkRouter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InstanceIp {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIP", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InstanceIp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstanceIp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIP")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InstanceIp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(InstanceIp {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InstanceIpList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIPList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InstanceIpList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstanceIpList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIPList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InstanceIpList, V::Error>
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
                    }
                }
                Ok(InstanceIpList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIPList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InstanceIpSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.instance_ip_address.is_some() {
            len += 1;
        }
        if self.instance_ip_family.is_some() {
            len += 1;
        }
        if self.cidr.is_some() {
            len += 1;
        }
        if self.virtual_network_reference.is_some() {
            len += 1;
        }
        if !self.virtual_machine_interface_references.is_empty() {
            len += 1;
        }
        if !self.ip_range_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIPSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.instance_ip_address.as_ref() {
            struct_ser.serialize_field("instanceIPAddress", v)?;
        }
        if let Some(v) = self.instance_ip_family.as_ref() {
            struct_ser.serialize_field("instanceIPFamily", v)?;
        }
        if let Some(v) = self.cidr.as_ref() {
            struct_ser.serialize_field("cidr", v)?;
        }
        if let Some(v) = self.virtual_network_reference.as_ref() {
            struct_ser.serialize_field("virtualNetworkReference", v)?;
        }
        if !self.virtual_machine_interface_references.is_empty() {
            struct_ser.serialize_field("virtualMachineInterfaceReferences", &self.virtual_machine_interface_references)?;
        }
        if !self.ip_range_keys.is_empty() {
            struct_ser.serialize_field("ipRangeKeys", &self.ip_range_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InstanceIpSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "instanceIPAddress",
            "instanceIPFamily",
            "cidr",
            "virtualNetworkReference",
            "virtualMachineInterfaceReferences",
            "ipRangeKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            InstanceIpAddress,
            InstanceIpFamily,
            Cidr,
            VirtualNetworkReference,
            VirtualMachineInterfaceReferences,
            IpRangeKeys,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "instanceIPAddress" => Ok(GeneratedField::InstanceIpAddress),
                            "instanceIPFamily" => Ok(GeneratedField::InstanceIpFamily),
                            "cidr" => Ok(GeneratedField::Cidr),
                            "virtualNetworkReference" => Ok(GeneratedField::VirtualNetworkReference),
                            "virtualMachineInterfaceReferences" => Ok(GeneratedField::VirtualMachineInterfaceReferences),
                            "ipRangeKeys" => Ok(GeneratedField::IpRangeKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstanceIpSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIPSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InstanceIpSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut instance_ip_address__ = None;
                let mut instance_ip_family__ = None;
                let mut cidr__ = None;
                let mut virtual_network_reference__ = None;
                let mut virtual_machine_interface_references__ = None;
                let mut ip_range_keys__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::InstanceIpAddress => {
                            if instance_ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceIPAddress"));
                            }
                            instance_ip_address__ = map.next_value()?;
                        }
                        GeneratedField::InstanceIpFamily => {
                            if instance_ip_family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceIPFamily"));
                            }
                            instance_ip_family__ = map.next_value()?;
                        }
                        GeneratedField::Cidr => {
                            if cidr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cidr"));
                            }
                            cidr__ = map.next_value()?;
                        }
                        GeneratedField::VirtualNetworkReference => {
                            if virtual_network_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualNetworkReference"));
                            }
                            virtual_network_reference__ = map.next_value()?;
                        }
                        GeneratedField::VirtualMachineInterfaceReferences => {
                            if virtual_machine_interface_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualMachineInterfaceReferences"));
                            }
                            virtual_machine_interface_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::IpRangeKeys => {
                            if ip_range_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipRangeKeys"));
                            }
                            ip_range_keys__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(InstanceIpSpec {
                    common_spec: common_spec__,
                    instance_ip_address: instance_ip_address__,
                    instance_ip_family: instance_ip_family__,
                    cidr: cidr__,
                    virtual_network_reference: virtual_network_reference__,
                    virtual_machine_interface_references: virtual_machine_interface_references__.unwrap_or_default(),
                    ip_range_keys: ip_range_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIPSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InstanceIpStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        if self.subnet_reference.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIPStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        if let Some(v) = self.subnet_reference.as_ref() {
            struct_ser.serialize_field("subnetReference", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InstanceIpStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
            "subnetReference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
            SubnetReference,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            "subnetReference" => Ok(GeneratedField::SubnetReference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstanceIpStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIPStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InstanceIpStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                let mut subnet_reference__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                        GeneratedField::SubnetReference => {
                            if subnet_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subnetReference"));
                            }
                            subnet_reference__ = map.next_value()?;
                        }
                    }
                }
                Ok(InstanceIpStatus {
                    common_status: common_status__,
                    subnet_reference: subnet_reference__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InstanceIPStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InterfaceRouteTable {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTable", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InterfaceRouteTable {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InterfaceRouteTable;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTable")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InterfaceRouteTable, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(InterfaceRouteTable {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTable", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InterfaceRouteTableList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTableList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InterfaceRouteTableList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InterfaceRouteTableList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTableList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InterfaceRouteTableList, V::Error>
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
                    }
                }
                Ok(InterfaceRouteTableList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTableList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InterfaceRouteTableSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.interface_route_table_routes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTableSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.interface_route_table_routes.as_ref() {
            struct_ser.serialize_field("interfaceRouteTableRoutes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InterfaceRouteTableSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "interfaceRouteTableRoutes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            InterfaceRouteTableRoutes,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "interfaceRouteTableRoutes" => Ok(GeneratedField::InterfaceRouteTableRoutes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InterfaceRouteTableSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTableSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InterfaceRouteTableSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut interface_route_table_routes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::InterfaceRouteTableRoutes => {
                            if interface_route_table_routes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interfaceRouteTableRoutes"));
                            }
                            interface_route_table_routes__ = map.next_value()?;
                        }
                    }
                }
                Ok(InterfaceRouteTableSpec {
                    common_spec: common_spec__,
                    interface_route_table_routes: interface_route_table_routes__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTableSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InterfaceRouteTableStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTableStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InterfaceRouteTableStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InterfaceRouteTableStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTableStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InterfaceRouteTableStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(InterfaceRouteTableStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTableStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InterfaceRouteTableType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.route.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTableType", len)?;
        if !self.route.is_empty() {
            struct_ser.serialize_field("route", &self.route)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InterfaceRouteTableType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "route",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Route,
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
                            "route" => Ok(GeneratedField::Route),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InterfaceRouteTableType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTableType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InterfaceRouteTableType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut route__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Route => {
                            if route__.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            route__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(InterfaceRouteTableType {
                    route: route__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteTableType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InterfaceRouteType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.community_attributes.is_some() {
            len += 1;
        }
        if self.prefix.is_some() {
            len += 1;
        }
        if self.next_hop_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteType", len)?;
        if let Some(v) = self.community_attributes.as_ref() {
            struct_ser.serialize_field("communityAttributes", v)?;
        }
        if let Some(v) = self.prefix.as_ref() {
            struct_ser.serialize_field("prefix", v)?;
        }
        if let Some(v) = self.next_hop_type.as_ref() {
            struct_ser.serialize_field("nextHopType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InterfaceRouteType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "communityAttributes",
            "prefix",
            "nextHopType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommunityAttributes,
            Prefix,
            NextHopType,
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
                            "communityAttributes" => Ok(GeneratedField::CommunityAttributes),
                            "prefix" => Ok(GeneratedField::Prefix),
                            "nextHopType" => Ok(GeneratedField::NextHopType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InterfaceRouteType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InterfaceRouteType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut community_attributes__ = None;
                let mut prefix__ = None;
                let mut next_hop_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommunityAttributes => {
                            if community_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("communityAttributes"));
                            }
                            community_attributes__ = map.next_value()?;
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = map.next_value()?;
                        }
                        GeneratedField::NextHopType => {
                            if next_hop_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextHopType"));
                            }
                            next_hop_type__ = map.next_value()?;
                        }
                    }
                }
                Ok(InterfaceRouteType {
                    community_attributes: community_attributes__,
                    prefix: prefix__,
                    next_hop_type: next_hop_type__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.InterfaceRouteType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IpAddresses {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.target_ip_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.IpAddresses", len)?;
        if !self.target_ip_address.is_empty() {
            struct_ser.serialize_field("targetIpAddress", &self.target_ip_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IpAddresses {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "targetIpAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TargetIpAddress,
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
                            "targetIpAddress" => Ok(GeneratedField::TargetIpAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IpAddresses;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.IpAddresses")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IpAddresses, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target_ip_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TargetIpAddress => {
                            if target_ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetIpAddress"));
                            }
                            target_ip_address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(IpAddresses {
                    target_ip_address: target_ip_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.IpAddresses", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyValuePair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.virtual_network_virtual_network_router_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.KeyValuePair", len)?;
        if let Some(v) = self.virtual_network_virtual_network_router_name.as_ref() {
            struct_ser.serialize_field("virtualNetworkVirtualNetworkRouterName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeyValuePair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "virtualNetworkVirtualNetworkRouterName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VirtualNetworkVirtualNetworkRouterName,
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
                            "virtualNetworkVirtualNetworkRouterName" => Ok(GeneratedField::VirtualNetworkVirtualNetworkRouterName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyValuePair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.KeyValuePair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<KeyValuePair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut virtual_network_virtual_network_router_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VirtualNetworkVirtualNetworkRouterName => {
                            if virtual_network_virtual_network_router_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualNetworkVirtualNetworkRouterName"));
                            }
                            virtual_network_virtual_network_router_name__ = map.next_value()?;
                        }
                    }
                }
                Ok(KeyValuePair {
                    virtual_network_virtual_network_router_name: virtual_network_virtual_network_router_name__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.KeyValuePair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LinklocalServiceEntryType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ip_fabric_dns_service_name.is_some() {
            len += 1;
        }
        if !self.ip_fabric_service_ip.is_empty() {
            len += 1;
        }
        if self.ip_fabric_service_port.is_some() {
            len += 1;
        }
        if self.linklocal_service_ip.is_some() {
            len += 1;
        }
        if self.linklocal_service_name.is_some() {
            len += 1;
        }
        if self.linklocal_service_port.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.LinklocalServiceEntryType", len)?;
        if let Some(v) = self.ip_fabric_dns_service_name.as_ref() {
            struct_ser.serialize_field("ipFabricDNSServiceName", v)?;
        }
        if !self.ip_fabric_service_ip.is_empty() {
            struct_ser.serialize_field("ipFabricServiceIP", &self.ip_fabric_service_ip)?;
        }
        if let Some(v) = self.ip_fabric_service_port.as_ref() {
            struct_ser.serialize_field("ipFabricServicePort", v)?;
        }
        if let Some(v) = self.linklocal_service_ip.as_ref() {
            struct_ser.serialize_field("linklocalServiceIP", v)?;
        }
        if let Some(v) = self.linklocal_service_name.as_ref() {
            struct_ser.serialize_field("linklocalServiceName", v)?;
        }
        if let Some(v) = self.linklocal_service_port.as_ref() {
            struct_ser.serialize_field("linklocalServicePort", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LinklocalServiceEntryType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ipFabricDNSServiceName",
            "ipFabricServiceIP",
            "ipFabricServicePort",
            "linklocalServiceIP",
            "linklocalServiceName",
            "linklocalServicePort",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IpFabricDnsServiceName,
            IpFabricServiceIp,
            IpFabricServicePort,
            LinklocalServiceIp,
            LinklocalServiceName,
            LinklocalServicePort,
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
                            "ipFabricDNSServiceName" => Ok(GeneratedField::IpFabricDnsServiceName),
                            "ipFabricServiceIP" => Ok(GeneratedField::IpFabricServiceIp),
                            "ipFabricServicePort" => Ok(GeneratedField::IpFabricServicePort),
                            "linklocalServiceIP" => Ok(GeneratedField::LinklocalServiceIp),
                            "linklocalServiceName" => Ok(GeneratedField::LinklocalServiceName),
                            "linklocalServicePort" => Ok(GeneratedField::LinklocalServicePort),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LinklocalServiceEntryType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.LinklocalServiceEntryType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LinklocalServiceEntryType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ip_fabric_dns_service_name__ = None;
                let mut ip_fabric_service_ip__ = None;
                let mut ip_fabric_service_port__ = None;
                let mut linklocal_service_ip__ = None;
                let mut linklocal_service_name__ = None;
                let mut linklocal_service_port__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IpFabricDnsServiceName => {
                            if ip_fabric_dns_service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipFabricDNSServiceName"));
                            }
                            ip_fabric_dns_service_name__ = map.next_value()?;
                        }
                        GeneratedField::IpFabricServiceIp => {
                            if ip_fabric_service_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipFabricServiceIP"));
                            }
                            ip_fabric_service_ip__ = Some(map.next_value()?);
                        }
                        GeneratedField::IpFabricServicePort => {
                            if ip_fabric_service_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipFabricServicePort"));
                            }
                            ip_fabric_service_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::LinklocalServiceIp => {
                            if linklocal_service_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linklocalServiceIP"));
                            }
                            linklocal_service_ip__ = map.next_value()?;
                        }
                        GeneratedField::LinklocalServiceName => {
                            if linklocal_service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linklocalServiceName"));
                            }
                            linklocal_service_name__ = map.next_value()?;
                        }
                        GeneratedField::LinklocalServicePort => {
                            if linklocal_service_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linklocalServicePort"));
                            }
                            linklocal_service_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(LinklocalServiceEntryType {
                    ip_fabric_dns_service_name: ip_fabric_dns_service_name__,
                    ip_fabric_service_ip: ip_fabric_service_ip__.unwrap_or_default(),
                    ip_fabric_service_port: ip_fabric_service_port__,
                    linklocal_service_ip: linklocal_service_ip__,
                    linklocal_service_name: linklocal_service_name__,
                    linklocal_service_port: linklocal_service_port__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.LinklocalServiceEntryType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LinklocalServices {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.linklocal_service_entry.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.LinklocalServices", len)?;
        if !self.linklocal_service_entry.is_empty() {
            struct_ser.serialize_field("linklocalServiceEntry", &self.linklocal_service_entry)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LinklocalServices {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "linklocalServiceEntry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LinklocalServiceEntry,
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
                            "linklocalServiceEntry" => Ok(GeneratedField::LinklocalServiceEntry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LinklocalServices;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.LinklocalServices")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LinklocalServices, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut linklocal_service_entry__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LinklocalServiceEntry => {
                            if linklocal_service_entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linklocalServiceEntry"));
                            }
                            linklocal_service_entry__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LinklocalServices {
                    linklocal_service_entry: linklocal_service_entry__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.LinklocalServices", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MacAddresses {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.mac_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MACAddresses", len)?;
        if !self.mac_address.is_empty() {
            struct_ser.serialize_field("macAddress", &self.mac_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MacAddresses {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "macAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MacAddress,
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
                            "macAddress" => Ok(GeneratedField::MacAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MacAddresses;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MACAddresses")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MacAddresses, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mac_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MacAddress => {
                            if mac_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("macAddress"));
                            }
                            mac_address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MacAddresses {
                    mac_address: mac_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MACAddresses", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MirrorDestination {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestination", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MirrorDestination {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MirrorDestination;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestination")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MirrorDestination, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(MirrorDestination {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestination", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MirrorDestinationList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MirrorDestinationList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MirrorDestinationList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MirrorDestinationList, V::Error>
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
                    }
                }
                Ok(MirrorDestinationList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MirrorDestinationSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.traffic_destination.is_some() {
            len += 1;
        }
        if self.udp_port.is_some() {
            len += 1;
        }
        if self.juniper_header.is_some() {
            len += 1;
        }
        if self.next_hop_mode.is_some() {
            len += 1;
        }
        if self.static_next_hop_header.is_some() {
            len += 1;
        }
        if self.nic_assisted_mirroring.is_some() {
            len += 1;
        }
        if self.nic_assisted_vlan_id.is_some() {
            len += 1;
        }
        if self.external_analyzer.is_some() {
            len += 1;
        }
        if self.analyzer_ip.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.traffic_destination.as_ref() {
            struct_ser.serialize_field("trafficDestination", v)?;
        }
        if let Some(v) = self.udp_port.as_ref() {
            struct_ser.serialize_field("udpPort", v)?;
        }
        if let Some(v) = self.juniper_header.as_ref() {
            struct_ser.serialize_field("juniperHeader", v)?;
        }
        if let Some(v) = self.next_hop_mode.as_ref() {
            struct_ser.serialize_field("nextHopMode", v)?;
        }
        if let Some(v) = self.static_next_hop_header.as_ref() {
            struct_ser.serialize_field("staticNextHopHeader", v)?;
        }
        if let Some(v) = self.nic_assisted_mirroring.as_ref() {
            struct_ser.serialize_field("nicAssistedMirroring", v)?;
        }
        if let Some(v) = self.nic_assisted_vlan_id.as_ref() {
            struct_ser.serialize_field("nicAssistedVlanID", v)?;
        }
        if let Some(v) = self.external_analyzer.as_ref() {
            struct_ser.serialize_field("externalAnalyzer", v)?;
        }
        if let Some(v) = self.analyzer_ip.as_ref() {
            struct_ser.serialize_field("analyzerIP", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MirrorDestinationSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "trafficDestination",
            "udpPort",
            "juniperHeader",
            "nextHopMode",
            "staticNextHopHeader",
            "nicAssistedMirroring",
            "nicAssistedVlanID",
            "externalAnalyzer",
            "analyzerIP",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            TrafficDestination,
            UdpPort,
            JuniperHeader,
            NextHopMode,
            StaticNextHopHeader,
            NicAssistedMirroring,
            NicAssistedVlanId,
            ExternalAnalyzer,
            AnalyzerIp,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "trafficDestination" => Ok(GeneratedField::TrafficDestination),
                            "udpPort" => Ok(GeneratedField::UdpPort),
                            "juniperHeader" => Ok(GeneratedField::JuniperHeader),
                            "nextHopMode" => Ok(GeneratedField::NextHopMode),
                            "staticNextHopHeader" => Ok(GeneratedField::StaticNextHopHeader),
                            "nicAssistedMirroring" => Ok(GeneratedField::NicAssistedMirroring),
                            "nicAssistedVlanID" => Ok(GeneratedField::NicAssistedVlanId),
                            "externalAnalyzer" => Ok(GeneratedField::ExternalAnalyzer),
                            "analyzerIP" => Ok(GeneratedField::AnalyzerIp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MirrorDestinationSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MirrorDestinationSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut traffic_destination__ = None;
                let mut udp_port__ = None;
                let mut juniper_header__ = None;
                let mut next_hop_mode__ = None;
                let mut static_next_hop_header__ = None;
                let mut nic_assisted_mirroring__ = None;
                let mut nic_assisted_vlan_id__ = None;
                let mut external_analyzer__ = None;
                let mut analyzer_ip__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::TrafficDestination => {
                            if traffic_destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trafficDestination"));
                            }
                            traffic_destination__ = map.next_value()?;
                        }
                        GeneratedField::UdpPort => {
                            if udp_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("udpPort"));
                            }
                            udp_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::JuniperHeader => {
                            if juniper_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("juniperHeader"));
                            }
                            juniper_header__ = map.next_value()?;
                        }
                        GeneratedField::NextHopMode => {
                            if next_hop_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextHopMode"));
                            }
                            next_hop_mode__ = map.next_value()?;
                        }
                        GeneratedField::StaticNextHopHeader => {
                            if static_next_hop_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticNextHopHeader"));
                            }
                            static_next_hop_header__ = map.next_value()?;
                        }
                        GeneratedField::NicAssistedMirroring => {
                            if nic_assisted_mirroring__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nicAssistedMirroring"));
                            }
                            nic_assisted_mirroring__ = map.next_value()?;
                        }
                        GeneratedField::NicAssistedVlanId => {
                            if nic_assisted_vlan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nicAssistedVlanID"));
                            }
                            nic_assisted_vlan_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ExternalAnalyzer => {
                            if external_analyzer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalAnalyzer"));
                            }
                            external_analyzer__ = map.next_value()?;
                        }
                        GeneratedField::AnalyzerIp => {
                            if analyzer_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analyzerIP"));
                            }
                            analyzer_ip__ = map.next_value()?;
                        }
                    }
                }
                Ok(MirrorDestinationSpec {
                    common_spec: common_spec__,
                    traffic_destination: traffic_destination__,
                    udp_port: udp_port__,
                    juniper_header: juniper_header__,
                    next_hop_mode: next_hop_mode__,
                    static_next_hop_header: static_next_hop_header__,
                    nic_assisted_mirroring: nic_assisted_mirroring__,
                    nic_assisted_vlan_id: nic_assisted_vlan_id__,
                    external_analyzer: external_analyzer__,
                    analyzer_ip: analyzer_ip__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MirrorDestinationStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        if self.virtual_machine_interface_reference.is_some() {
            len += 1;
        }
        if self.instance_ip_reference.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        if let Some(v) = self.virtual_machine_interface_reference.as_ref() {
            struct_ser.serialize_field("virtualMachineInterfaceReference", v)?;
        }
        if let Some(v) = self.instance_ip_reference.as_ref() {
            struct_ser.serialize_field("instanceIPReference", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MirrorDestinationStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
            "virtualMachineInterfaceReference",
            "instanceIPReference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
            VirtualMachineInterfaceReference,
            InstanceIpReference,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            "virtualMachineInterfaceReference" => Ok(GeneratedField::VirtualMachineInterfaceReference),
                            "instanceIPReference" => Ok(GeneratedField::InstanceIpReference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MirrorDestinationStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MirrorDestinationStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                let mut virtual_machine_interface_reference__ = None;
                let mut instance_ip_reference__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                        GeneratedField::VirtualMachineInterfaceReference => {
                            if virtual_machine_interface_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualMachineInterfaceReference"));
                            }
                            virtual_machine_interface_reference__ = map.next_value()?;
                        }
                        GeneratedField::InstanceIpReference => {
                            if instance_ip_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceIPReference"));
                            }
                            instance_ip_reference__ = map.next_value()?;
                        }
                    }
                }
                Ok(MirrorDestinationStatus {
                    common_status: common_status__,
                    virtual_machine_interface_reference: virtual_machine_interface_reference__,
                    instance_ip_reference: instance_ip_reference__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.MirrorDestinationStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NhHeaderType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.v_tep_destination_ip.is_some() {
            len += 1;
        }
        if self.v_tep_destination_mac.is_some() {
            len += 1;
        }
        if self.vxlan_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.NHHeaderType", len)?;
        if let Some(v) = self.v_tep_destination_ip.as_ref() {
            struct_ser.serialize_field("vTEPDestinationIP", v)?;
        }
        if let Some(v) = self.v_tep_destination_mac.as_ref() {
            struct_ser.serialize_field("vTEPDestinationMac", v)?;
        }
        if let Some(v) = self.vxlan_id.as_ref() {
            struct_ser.serialize_field("vxlanID", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NhHeaderType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vTEPDestinationIP",
            "vTEPDestinationMac",
            "vxlanID",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VTepDestinationIp,
            VTepDestinationMac,
            VxlanId,
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
                            "vTEPDestinationIP" => Ok(GeneratedField::VTepDestinationIp),
                            "vTEPDestinationMac" => Ok(GeneratedField::VTepDestinationMac),
                            "vxlanID" => Ok(GeneratedField::VxlanId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NhHeaderType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.NHHeaderType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NhHeaderType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut v_tep_destination_ip__ = None;
                let mut v_tep_destination_mac__ = None;
                let mut vxlan_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VTepDestinationIp => {
                            if v_tep_destination_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vTEPDestinationIP"));
                            }
                            v_tep_destination_ip__ = map.next_value()?;
                        }
                        GeneratedField::VTepDestinationMac => {
                            if v_tep_destination_mac__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vTEPDestinationMac"));
                            }
                            v_tep_destination_mac__ = map.next_value()?;
                        }
                        GeneratedField::VxlanId => {
                            if vxlan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vxlanID"));
                            }
                            vxlan_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(NhHeaderType {
                    v_tep_destination_ip: v_tep_destination_ip__,
                    v_tep_destination_mac: v_tep_destination_mac__,
                    vxlan_id: vxlan_id__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.NHHeaderType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PolicyBasedForwardingRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.direction.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PolicyBasedForwardingRule", len)?;
        if let Some(v) = self.direction.as_ref() {
            struct_ser.serialize_field("direction", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PolicyBasedForwardingRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "direction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Direction,
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
                            "direction" => Ok(GeneratedField::Direction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PolicyBasedForwardingRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PolicyBasedForwardingRule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PolicyBasedForwardingRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut direction__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = map.next_value()?;
                        }
                    }
                }
                Ok(PolicyBasedForwardingRule {
                    direction: direction__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PolicyBasedForwardingRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PortRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_port.is_some() {
            len += 1;
        }
        if self.end_port.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PortRange", len)?;
        if let Some(v) = self.start_port.as_ref() {
            struct_ser.serialize_field("startPort", v)?;
        }
        if let Some(v) = self.end_port.as_ref() {
            struct_ser.serialize_field("endPort", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PortRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "startPort",
            "endPort",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartPort,
            EndPort,
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
                            "startPort" => Ok(GeneratedField::StartPort),
                            "endPort" => Ok(GeneratedField::EndPort),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PortRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PortRange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PortRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_port__ = None;
                let mut end_port__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StartPort => {
                            if start_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startPort"));
                            }
                            start_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::EndPort => {
                            if end_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endPort"));
                            }
                            end_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(PortRange {
                    start_port: start_port__,
                    end_port: end_port__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PortRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PortTranslationPool {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.protocol.is_some() {
            len += 1;
        }
        if self.port_range.is_some() {
            len += 1;
        }
        if self.port_count.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PortTranslationPool", len)?;
        if let Some(v) = self.protocol.as_ref() {
            struct_ser.serialize_field("protocol", v)?;
        }
        if let Some(v) = self.port_range.as_ref() {
            struct_ser.serialize_field("portRange", v)?;
        }
        if let Some(v) = self.port_count.as_ref() {
            struct_ser.serialize_field("portCount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PortTranslationPool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protocol",
            "portRange",
            "portCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Protocol,
            PortRange,
            PortCount,
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
                            "protocol" => Ok(GeneratedField::Protocol),
                            "portRange" => Ok(GeneratedField::PortRange),
                            "portCount" => Ok(GeneratedField::PortCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PortTranslationPool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PortTranslationPool")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PortTranslationPool, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protocol__ = None;
                let mut port_range__ = None;
                let mut port_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = map.next_value()?;
                        }
                        GeneratedField::PortRange => {
                            if port_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portRange"));
                            }
                            port_range__ = map.next_value()?;
                        }
                        GeneratedField::PortCount => {
                            if port_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portCount"));
                            }
                            port_count__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(PortTranslationPool {
                    protocol: protocol__,
                    port_range: port_range__,
                    port_count: port_count__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PortTranslationPool", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PortTranslationPools {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pools.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PortTranslationPools", len)?;
        if !self.pools.is_empty() {
            struct_ser.serialize_field("pools", &self.pools)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PortTranslationPools {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pools",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pools,
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
                            "pools" => Ok(GeneratedField::Pools),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PortTranslationPools;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PortTranslationPools")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PortTranslationPools, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pools__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pools => {
                            if pools__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pools"));
                            }
                            pools__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PortTranslationPools {
                    pools: pools__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PortTranslationPools", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PortType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_port.is_some() {
            len += 1;
        }
        if self.end_port.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PortType", len)?;
        if let Some(v) = self.start_port.as_ref() {
            struct_ser.serialize_field("startPort", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.end_port.as_ref() {
            struct_ser.serialize_field("endPort", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PortType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "startPort",
            "endPort",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartPort,
            EndPort,
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
                            "startPort" => Ok(GeneratedField::StartPort),
                            "endPort" => Ok(GeneratedField::EndPort),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PortType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PortType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PortType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_port__ = None;
                let mut end_port__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StartPort => {
                            if start_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startPort"));
                            }
                            start_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::EndPort => {
                            if end_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endPort"));
                            }
                            end_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(PortType {
                    start_port: start_port__,
                    end_port: end_port__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.PortType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Range {
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
        if !self.ip_ranges.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.Range", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if !self.ip_ranges.is_empty() {
            struct_ser.serialize_field("ipRanges", &self.ip_ranges)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Range {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "ipRanges",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            IpRanges,
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
                            "ipRanges" => Ok(GeneratedField::IpRanges),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Range;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.Range")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Range, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut ip_ranges__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::IpRanges => {
                            if ip_ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipRanges"));
                            }
                            ip_ranges__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Range {
                    key: key__,
                    ip_ranges: ip_ranges__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.Range", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReconcilerState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state.is_some() {
            len += 1;
        }
        if self.observation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ReconcilerState", len)?;
        if let Some(v) = self.state.as_ref() {
            struct_ser.serialize_field("state", v)?;
        }
        if let Some(v) = self.observation.as_ref() {
            struct_ser.serialize_field("observation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReconcilerState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
            "observation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
            Observation,
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
                            "state" => Ok(GeneratedField::State),
                            "observation" => Ok(GeneratedField::Observation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReconcilerState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ReconcilerState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ReconcilerState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                let mut observation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = map.next_value()?;
                        }
                        GeneratedField::Observation => {
                            if observation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("observation"));
                            }
                            observation__ = map.next_value()?;
                        }
                    }
                }
                Ok(ReconcilerState {
                    state: state__,
                    observation: observation__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ReconcilerState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.object_reference.is_some() {
            len += 1;
        }
        if self.contrail_fq_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ResourceReference", len)?;
        if let Some(v) = self.object_reference.as_ref() {
            struct_ser.serialize_field("objectReference", v)?;
        }
        if let Some(v) = self.contrail_fq_name.as_ref() {
            struct_ser.serialize_field("contrailFqName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objectReference",
            "contrailFqName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectReference,
            ContrailFqName,
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
                            "objectReference" => Ok(GeneratedField::ObjectReference),
                            "contrailFqName" => Ok(GeneratedField::ContrailFqName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ResourceReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResourceReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object_reference__ = None;
                let mut contrail_fq_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ObjectReference => {
                            if object_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectReference"));
                            }
                            object_reference__ = map.next_value()?;
                        }
                        GeneratedField::ContrailFqName => {
                            if contrail_fq_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contrailFqName"));
                            }
                            contrail_fq_name__ = map.next_value()?;
                        }
                    }
                }
                Ok(ResourceReference {
                    object_reference: object_reference__,
                    contrail_fq_name: contrail_fq_name__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ResourceReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteOriginOverride {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.origin_override.is_some() {
            len += 1;
        }
        if self.origin.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteOriginOverride", len)?;
        if let Some(v) = self.origin_override.as_ref() {
            struct_ser.serialize_field("originOverride", v)?;
        }
        if let Some(v) = self.origin.as_ref() {
            struct_ser.serialize_field("origin", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteOriginOverride {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "originOverride",
            "origin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OriginOverride,
            Origin,
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
                            "originOverride" => Ok(GeneratedField::OriginOverride),
                            "origin" => Ok(GeneratedField::Origin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteOriginOverride;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteOriginOverride")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteOriginOverride, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut origin_override__ = None;
                let mut origin__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OriginOverride => {
                            if origin_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originOverride"));
                            }
                            origin_override__ = map.next_value()?;
                        }
                        GeneratedField::Origin => {
                            if origin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            origin__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteOriginOverride {
                    origin_override: origin_override__,
                    origin: origin__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteOriginOverride", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteTable {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTable", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteTable {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteTable;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTable")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteTable, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteTable {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTable", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteTableList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTableList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteTableList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteTableList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTableList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteTableList, V::Error>
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
                    }
                }
                Ok(RouteTableList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTableList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteTableSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.routes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTableSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.routes.as_ref() {
            struct_ser.serialize_field("routes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteTableSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "routes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            Routes,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "routes" => Ok(GeneratedField::Routes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteTableSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTableSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteTableSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut routes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::Routes => {
                            if routes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routes"));
                            }
                            routes__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteTableSpec {
                    common_spec: common_spec__,
                    routes: routes__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTableSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteTableStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTableStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteTableStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteTableStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTableStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteTableStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteTableStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTableStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteTableType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.route.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTableType", len)?;
        if !self.route.is_empty() {
            struct_ser.serialize_field("route", &self.route)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteTableType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "route",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Route,
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
                            "route" => Ok(GeneratedField::Route),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteTableType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTableType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteTableType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut route__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Route => {
                            if route__.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            route__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RouteTableType {
                    route: route__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTableType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteTarget {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTarget", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteTarget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTarget")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteTarget, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteTarget {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTarget", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteTargetList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteTargetList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteTargetList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteTargetList, V::Error>
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
                    }
                }
                Ok(RouteTargetList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteTargetReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_reference.is_some() {
            len += 1;
        }
        if self.attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetReference", len)?;
        if let Some(v) = self.resource_reference.as_ref() {
            struct_ser.serialize_field("resourceReference", v)?;
        }
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteTargetReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resourceReference",
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceReference,
            Attributes,
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
                            "resourceReference" => Ok(GeneratedField::ResourceReference),
                            "attributes" => Ok(GeneratedField::Attributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteTargetReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteTargetReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_reference__ = None;
                let mut attributes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceReference => {
                            if resource_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceReference"));
                            }
                            resource_reference__ = map.next_value()?;
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteTargetReference {
                    resource_reference: resource_reference__,
                    attributes: attributes__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteTargetReferenceAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.import_export.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetReferenceAttributes", len)?;
        if let Some(v) = self.import_export.as_ref() {
            struct_ser.serialize_field("importExport", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteTargetReferenceAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "importExport",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ImportExport,
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
                            "importExport" => Ok(GeneratedField::ImportExport),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteTargetReferenceAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetReferenceAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteTargetReferenceAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut import_export__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ImportExport => {
                            if import_export__.is_some() {
                                return Err(serde::de::Error::duplicate_field("importExport"));
                            }
                            import_export__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteTargetReferenceAttributes {
                    import_export: import_export__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetReferenceAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteTargetSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteTargetSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteTargetSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteTargetSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteTargetSpec {
                    common_spec: common_spec__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteTargetStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteTargetStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteTargetStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteTargetStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteTargetStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteTargetStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.community_attributes.is_some() {
            len += 1;
        }
        if self.prefix.is_some() {
            len += 1;
        }
        if self.next_hop.is_some() {
            len += 1;
        }
        if self.next_hop_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteType", len)?;
        if let Some(v) = self.community_attributes.as_ref() {
            struct_ser.serialize_field("communityAttributes", v)?;
        }
        if let Some(v) = self.prefix.as_ref() {
            struct_ser.serialize_field("prefix", v)?;
        }
        if let Some(v) = self.next_hop.as_ref() {
            struct_ser.serialize_field("nextHop", v)?;
        }
        if let Some(v) = self.next_hop_type.as_ref() {
            struct_ser.serialize_field("nextHopType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "communityAttributes",
            "prefix",
            "nextHop",
            "nextHopType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommunityAttributes,
            Prefix,
            NextHop,
            NextHopType,
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
                            "communityAttributes" => Ok(GeneratedField::CommunityAttributes),
                            "prefix" => Ok(GeneratedField::Prefix),
                            "nextHop" => Ok(GeneratedField::NextHop),
                            "nextHopType" => Ok(GeneratedField::NextHopType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut community_attributes__ = None;
                let mut prefix__ = None;
                let mut next_hop__ = None;
                let mut next_hop_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommunityAttributes => {
                            if community_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("communityAttributes"));
                            }
                            community_attributes__ = map.next_value()?;
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = map.next_value()?;
                        }
                        GeneratedField::NextHop => {
                            if next_hop__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextHop"));
                            }
                            next_hop__ = map.next_value()?;
                        }
                        GeneratedField::NextHopType => {
                            if next_hop_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextHopType"));
                            }
                            next_hop_type__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteType {
                    community_attributes: community_attributes__,
                    prefix: prefix__,
                    next_hop: next_hop__,
                    next_hop_type: next_hop_type__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RouteType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoutingInstance {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstance", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoutingInstance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoutingInstance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstance")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RoutingInstance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(RoutingInstance {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoutingInstanceList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoutingInstanceList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoutingInstanceList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RoutingInstanceList, V::Error>
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
                    }
                }
                Ok(RoutingInstanceList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoutingInstanceReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_reference.is_some() {
            len += 1;
        }
        if self.attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceReference", len)?;
        if let Some(v) = self.resource_reference.as_ref() {
            struct_ser.serialize_field("resourceReference", v)?;
        }
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoutingInstanceReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resourceReference",
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceReference,
            Attributes,
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
                            "resourceReference" => Ok(GeneratedField::ResourceReference),
                            "attributes" => Ok(GeneratedField::Attributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoutingInstanceReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RoutingInstanceReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_reference__ = None;
                let mut attributes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceReference => {
                            if resource_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceReference"));
                            }
                            resource_reference__ = map.next_value()?;
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map.next_value()?;
                        }
                    }
                }
                Ok(RoutingInstanceReference {
                    resource_reference: resource_reference__,
                    attributes: attributes__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoutingInstanceSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.parent.is_some() {
            len += 1;
        }
        if !self.route_target_references.is_empty() {
            len += 1;
        }
        if self.static_route_entries.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.parent.as_ref() {
            struct_ser.serialize_field("parent", v)?;
        }
        if !self.route_target_references.is_empty() {
            struct_ser.serialize_field("routeTargetReferences", &self.route_target_references)?;
        }
        if let Some(v) = self.static_route_entries.as_ref() {
            struct_ser.serialize_field("staticRouteEntries", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoutingInstanceSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "parent",
            "routeTargetReferences",
            "staticRouteEntries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            Parent,
            RouteTargetReferences,
            StaticRouteEntries,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "parent" => Ok(GeneratedField::Parent),
                            "routeTargetReferences" => Ok(GeneratedField::RouteTargetReferences),
                            "staticRouteEntries" => Ok(GeneratedField::StaticRouteEntries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoutingInstanceSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RoutingInstanceSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut parent__ = None;
                let mut route_target_references__ = None;
                let mut static_route_entries__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = map.next_value()?;
                        }
                        GeneratedField::RouteTargetReferences => {
                            if route_target_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeTargetReferences"));
                            }
                            route_target_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::StaticRouteEntries => {
                            if static_route_entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticRouteEntries"));
                            }
                            static_route_entries__ = map.next_value()?;
                        }
                    }
                }
                Ok(RoutingInstanceSpec {
                    common_spec: common_spec__,
                    parent: parent__,
                    route_target_references: route_target_references__.unwrap_or_default(),
                    static_route_entries: static_route_entries__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoutingInstanceStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        if self.is_default.is_some() {
            len += 1;
        }
        if self.routing_instance_fabric_snat.is_some() {
            len += 1;
        }
        if self.default_route_target_reference.is_some() {
            len += 1;
        }
        if !self.virtual_network_router_route_target_references.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        if let Some(v) = self.is_default.as_ref() {
            struct_ser.serialize_field("isDefault", v)?;
        }
        if let Some(v) = self.routing_instance_fabric_snat.as_ref() {
            struct_ser.serialize_field("routingInstanceFabricSNAT", v)?;
        }
        if let Some(v) = self.default_route_target_reference.as_ref() {
            struct_ser.serialize_field("defaultRouteTargetReference", v)?;
        }
        if !self.virtual_network_router_route_target_references.is_empty() {
            struct_ser.serialize_field("virtualNetworkRouterRouteTargetReferences", &self.virtual_network_router_route_target_references)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoutingInstanceStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
            "isDefault",
            "routingInstanceFabricSNAT",
            "defaultRouteTargetReference",
            "virtualNetworkRouterRouteTargetReferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
            IsDefault,
            RoutingInstanceFabricSnat,
            DefaultRouteTargetReference,
            VirtualNetworkRouterRouteTargetReferences,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            "isDefault" => Ok(GeneratedField::IsDefault),
                            "routingInstanceFabricSNAT" => Ok(GeneratedField::RoutingInstanceFabricSnat),
                            "defaultRouteTargetReference" => Ok(GeneratedField::DefaultRouteTargetReference),
                            "virtualNetworkRouterRouteTargetReferences" => Ok(GeneratedField::VirtualNetworkRouterRouteTargetReferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoutingInstanceStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RoutingInstanceStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                let mut is_default__ = None;
                let mut routing_instance_fabric_snat__ = None;
                let mut default_route_target_reference__ = None;
                let mut virtual_network_router_route_target_references__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = map.next_value()?;
                        }
                        GeneratedField::RoutingInstanceFabricSnat => {
                            if routing_instance_fabric_snat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routingInstanceFabricSNAT"));
                            }
                            routing_instance_fabric_snat__ = map.next_value()?;
                        }
                        GeneratedField::DefaultRouteTargetReference => {
                            if default_route_target_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultRouteTargetReference"));
                            }
                            default_route_target_reference__ = map.next_value()?;
                        }
                        GeneratedField::VirtualNetworkRouterRouteTargetReferences => {
                            if virtual_network_router_route_target_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualNetworkRouterRouteTargetReferences"));
                            }
                            virtual_network_router_route_target_references__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(RoutingInstanceStatus {
                    common_status: common_status__,
                    is_default: is_default__,
                    routing_instance_fabric_snat: routing_instance_fabric_snat__,
                    default_route_target_reference: default_route_target_reference__,
                    virtual_network_router_route_target_references: virtual_network_router_route_target_references__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.RoutingInstanceStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SecondaryActionList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mirror_to.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SecondaryActionList", len)?;
        if let Some(v) = self.mirror_to.as_ref() {
            struct_ser.serialize_field("mirrorTo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SecondaryActionList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mirrorTo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MirrorTo,
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
                            "mirrorTo" => Ok(GeneratedField::MirrorTo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SecondaryActionList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SecondaryActionList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SecondaryActionList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mirror_to__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MirrorTo => {
                            if mirror_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mirrorTo"));
                            }
                            mirror_to__ = map.next_value()?;
                        }
                    }
                }
                Ok(SecondaryActionList {
                    mirror_to: mirror_to__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SecondaryActionList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceGroup {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroup", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceGroup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceGroup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroup")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceGroup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(ServiceGroup {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceGroupList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroupList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceGroupList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceGroupList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroupList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceGroupList, V::Error>
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
                    }
                }
                Ok(ServiceGroupList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroupList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceGroupSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.service_group_firewall_service_list.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroupSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.service_group_firewall_service_list.as_ref() {
            struct_ser.serialize_field("serviceGroupFirewallServiceList", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceGroupSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "serviceGroupFirewallServiceList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            ServiceGroupFirewallServiceList,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "serviceGroupFirewallServiceList" => Ok(GeneratedField::ServiceGroupFirewallServiceList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceGroupSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroupSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceGroupSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut service_group_firewall_service_list__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::ServiceGroupFirewallServiceList => {
                            if service_group_firewall_service_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceGroupFirewallServiceList"));
                            }
                            service_group_firewall_service_list__ = map.next_value()?;
                        }
                    }
                }
                Ok(ServiceGroupSpec {
                    common_spec: common_spec__,
                    service_group_firewall_service_list: service_group_firewall_service_list__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroupSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceGroupStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroupStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceGroupStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceGroupStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroupStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceGroupStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(ServiceGroupStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceGroupStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceHealthCheck {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheck", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceHealthCheck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceHealthCheck;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheck")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceHealthCheck, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(ServiceHealthCheck {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheck", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceHealthCheckList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceHealthCheckList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceHealthCheckList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceHealthCheckList, V::Error>
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
                    }
                }
                Ok(ServiceHealthCheckList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceHealthCheckProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.delay.is_some() {
            len += 1;
        }
        if self.delay_usecs.is_some() {
            len += 1;
        }
        if self.enabled.is_some() {
            len += 1;
        }
        if self.expected_codes.is_some() {
            len += 1;
        }
        if self.health_check_type.is_some() {
            len += 1;
        }
        if self.http_method.is_some() {
            len += 1;
        }
        if self.max_retries.is_some() {
            len += 1;
        }
        if self.monitor_type.is_some() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        if self.timeout_usecs.is_some() {
            len += 1;
        }
        if self.url_path.is_some() {
            len += 1;
        }
        if self.target_ip_all.is_some() {
            len += 1;
        }
        if self.target_ip_list.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckProperties", len)?;
        if let Some(v) = self.delay.as_ref() {
            struct_ser.serialize_field("delay", v)?;
        }
        if let Some(v) = self.delay_usecs.as_ref() {
            struct_ser.serialize_field("delayUsecs", v)?;
        }
        if let Some(v) = self.enabled.as_ref() {
            struct_ser.serialize_field("enabled", v)?;
        }
        if let Some(v) = self.expected_codes.as_ref() {
            struct_ser.serialize_field("expectedCodes", v)?;
        }
        if let Some(v) = self.health_check_type.as_ref() {
            struct_ser.serialize_field("healthCheckType", v)?;
        }
        if let Some(v) = self.http_method.as_ref() {
            struct_ser.serialize_field("httpMethod", v)?;
        }
        if let Some(v) = self.max_retries.as_ref() {
            struct_ser.serialize_field("maxRetries", v)?;
        }
        if let Some(v) = self.monitor_type.as_ref() {
            struct_ser.serialize_field("monitorType", v)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if let Some(v) = self.timeout_usecs.as_ref() {
            struct_ser.serialize_field("timeoutUsecs", v)?;
        }
        if let Some(v) = self.url_path.as_ref() {
            struct_ser.serialize_field("urlPath", v)?;
        }
        if let Some(v) = self.target_ip_all.as_ref() {
            struct_ser.serialize_field("targetIpAll", v)?;
        }
        if let Some(v) = self.target_ip_list.as_ref() {
            struct_ser.serialize_field("targetIpList", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceHealthCheckProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delay",
            "delayUsecs",
            "enabled",
            "expectedCodes",
            "healthCheckType",
            "httpMethod",
            "maxRetries",
            "monitorType",
            "timeout",
            "timeoutUsecs",
            "urlPath",
            "targetIpAll",
            "targetIpList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Delay,
            DelayUsecs,
            Enabled,
            ExpectedCodes,
            HealthCheckType,
            HttpMethod,
            MaxRetries,
            MonitorType,
            Timeout,
            TimeoutUsecs,
            UrlPath,
            TargetIpAll,
            TargetIpList,
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
                            "delay" => Ok(GeneratedField::Delay),
                            "delayUsecs" => Ok(GeneratedField::DelayUsecs),
                            "enabled" => Ok(GeneratedField::Enabled),
                            "expectedCodes" => Ok(GeneratedField::ExpectedCodes),
                            "healthCheckType" => Ok(GeneratedField::HealthCheckType),
                            "httpMethod" => Ok(GeneratedField::HttpMethod),
                            "maxRetries" => Ok(GeneratedField::MaxRetries),
                            "monitorType" => Ok(GeneratedField::MonitorType),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "timeoutUsecs" => Ok(GeneratedField::TimeoutUsecs),
                            "urlPath" => Ok(GeneratedField::UrlPath),
                            "targetIpAll" => Ok(GeneratedField::TargetIpAll),
                            "targetIpList" => Ok(GeneratedField::TargetIpList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceHealthCheckProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceHealthCheckProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut delay__ = None;
                let mut delay_usecs__ = None;
                let mut enabled__ = None;
                let mut expected_codes__ = None;
                let mut health_check_type__ = None;
                let mut http_method__ = None;
                let mut max_retries__ = None;
                let mut monitor_type__ = None;
                let mut timeout__ = None;
                let mut timeout_usecs__ = None;
                let mut url_path__ = None;
                let mut target_ip_all__ = None;
                let mut target_ip_list__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Delay => {
                            if delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delay"));
                            }
                            delay__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::DelayUsecs => {
                            if delay_usecs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delayUsecs"));
                            }
                            delay_usecs__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = map.next_value()?;
                        }
                        GeneratedField::ExpectedCodes => {
                            if expected_codes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expectedCodes"));
                            }
                            expected_codes__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::HealthCheckType => {
                            if health_check_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthCheckType"));
                            }
                            health_check_type__ = map.next_value()?;
                        }
                        GeneratedField::HttpMethod => {
                            if http_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpMethod"));
                            }
                            http_method__ = map.next_value()?;
                        }
                        GeneratedField::MaxRetries => {
                            if max_retries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRetries"));
                            }
                            max_retries__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MonitorType => {
                            if monitor_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monitorType"));
                            }
                            monitor_type__ = map.next_value()?;
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TimeoutUsecs => {
                            if timeout_usecs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutUsecs"));
                            }
                            timeout_usecs__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::UrlPath => {
                            if url_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urlPath"));
                            }
                            url_path__ = map.next_value()?;
                        }
                        GeneratedField::TargetIpAll => {
                            if target_ip_all__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetIpAll"));
                            }
                            target_ip_all__ = map.next_value()?;
                        }
                        GeneratedField::TargetIpList => {
                            if target_ip_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetIpList"));
                            }
                            target_ip_list__ = map.next_value()?;
                        }
                    }
                }
                Ok(ServiceHealthCheckProperties {
                    delay: delay__,
                    delay_usecs: delay_usecs__,
                    enabled: enabled__,
                    expected_codes: expected_codes__,
                    health_check_type: health_check_type__,
                    http_method: http_method__,
                    max_retries: max_retries__,
                    monitor_type: monitor_type__,
                    timeout: timeout__,
                    timeout_usecs: timeout_usecs__,
                    url_path: url_path__,
                    target_ip_all: target_ip_all__,
                    target_ip_list: target_ip_list__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceHealthCheckSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.service_health_check_properties.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.service_health_check_properties.as_ref() {
            struct_ser.serialize_field("serviceHealthCheckProperties", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceHealthCheckSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "serviceHealthCheckProperties",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            ServiceHealthCheckProperties,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "serviceHealthCheckProperties" => Ok(GeneratedField::ServiceHealthCheckProperties),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceHealthCheckSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceHealthCheckSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut service_health_check_properties__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::ServiceHealthCheckProperties => {
                            if service_health_check_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceHealthCheckProperties"));
                            }
                            service_health_check_properties__ = map.next_value()?;
                        }
                    }
                }
                Ok(ServiceHealthCheckSpec {
                    common_spec: common_spec__,
                    service_health_check_properties: service_health_check_properties__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceHealthCheckStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceHealthCheckStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceHealthCheckStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceHealthCheckStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(ServiceHealthCheckStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.ServiceHealthCheckStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StaticNhType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.vtep_dst_ip_address.is_some() {
            len += 1;
        }
        if self.vtep_dst_mac_address.is_some() {
            len += 1;
        }
        if self.vni.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.StaticNHType", len)?;
        if let Some(v) = self.vtep_dst_ip_address.as_ref() {
            struct_ser.serialize_field("vtepDstIPAddress", v)?;
        }
        if let Some(v) = self.vtep_dst_mac_address.as_ref() {
            struct_ser.serialize_field("vtepDstMacAddress", v)?;
        }
        if let Some(v) = self.vni.as_ref() {
            struct_ser.serialize_field("vni", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StaticNhType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vtepDstIPAddress",
            "vtepDstMacAddress",
            "vni",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VtepDstIpAddress,
            VtepDstMacAddress,
            Vni,
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
                            "vtepDstIPAddress" => Ok(GeneratedField::VtepDstIpAddress),
                            "vtepDstMacAddress" => Ok(GeneratedField::VtepDstMacAddress),
                            "vni" => Ok(GeneratedField::Vni),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StaticNhType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.StaticNHType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StaticNhType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vtep_dst_ip_address__ = None;
                let mut vtep_dst_mac_address__ = None;
                let mut vni__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VtepDstIpAddress => {
                            if vtep_dst_ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vtepDstIPAddress"));
                            }
                            vtep_dst_ip_address__ = map.next_value()?;
                        }
                        GeneratedField::VtepDstMacAddress => {
                            if vtep_dst_mac_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vtepDstMacAddress"));
                            }
                            vtep_dst_mac_address__ = map.next_value()?;
                        }
                        GeneratedField::Vni => {
                            if vni__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vni"));
                            }
                            vni__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(StaticNhType {
                    vtep_dst_ip_address: vtep_dst_ip_address__,
                    vtep_dst_mac_address: vtep_dst_mac_address__,
                    vni: vni__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.StaticNHType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StaticRouteEntriesType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.route.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.StaticRouteEntriesType", len)?;
        if !self.route.is_empty() {
            struct_ser.serialize_field("route", &self.route)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StaticRouteEntriesType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "route",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Route,
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
                            "route" => Ok(GeneratedField::Route),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StaticRouteEntriesType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.StaticRouteEntriesType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StaticRouteEntriesType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut route__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Route => {
                            if route__.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            route__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StaticRouteEntriesType {
                    route: route__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.StaticRouteEntriesType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StaticRouteType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.community.is_empty() {
            len += 1;
        }
        if self.next_hop.is_some() {
            len += 1;
        }
        if self.prefix.is_some() {
            len += 1;
        }
        if !self.route_target.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.StaticRouteType", len)?;
        if !self.community.is_empty() {
            struct_ser.serialize_field("community", &self.community)?;
        }
        if let Some(v) = self.next_hop.as_ref() {
            struct_ser.serialize_field("nextHop", v)?;
        }
        if let Some(v) = self.prefix.as_ref() {
            struct_ser.serialize_field("prefix", v)?;
        }
        if !self.route_target.is_empty() {
            struct_ser.serialize_field("routeTarget", &self.route_target)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StaticRouteType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "community",
            "nextHop",
            "prefix",
            "routeTarget",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Community,
            NextHop,
            Prefix,
            RouteTarget,
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
                            "community" => Ok(GeneratedField::Community),
                            "nextHop" => Ok(GeneratedField::NextHop),
                            "prefix" => Ok(GeneratedField::Prefix),
                            "routeTarget" => Ok(GeneratedField::RouteTarget),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StaticRouteType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.StaticRouteType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StaticRouteType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut community__ = None;
                let mut next_hop__ = None;
                let mut prefix__ = None;
                let mut route_target__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Community => {
                            if community__.is_some() {
                                return Err(serde::de::Error::duplicate_field("community"));
                            }
                            community__ = Some(map.next_value()?);
                        }
                        GeneratedField::NextHop => {
                            if next_hop__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextHop"));
                            }
                            next_hop__ = map.next_value()?;
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = map.next_value()?;
                        }
                        GeneratedField::RouteTarget => {
                            if route_target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeTarget"));
                            }
                            route_target__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StaticRouteType {
                    community: community__.unwrap_or_default(),
                    next_hop: next_hop__,
                    prefix: prefix__,
                    route_target: route_target__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.StaticRouteType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Subnet {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.Subnet", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Subnet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Subnet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.Subnet")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Subnet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(Subnet {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.Subnet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubnetList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SubnetList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubnetList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubnetList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SubnetList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubnetList, V::Error>
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
                    }
                }
                Ok(SubnetList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SubnetList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubnetReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_reference.is_some() {
            len += 1;
        }
        if self.attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SubnetReference", len)?;
        if let Some(v) = self.resource_reference.as_ref() {
            struct_ser.serialize_field("resourceReference", v)?;
        }
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubnetReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resourceReference",
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceReference,
            Attributes,
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
                            "resourceReference" => Ok(GeneratedField::ResourceReference),
                            "attributes" => Ok(GeneratedField::Attributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubnetReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SubnetReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubnetReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_reference__ = None;
                let mut attributes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceReference => {
                            if resource_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceReference"));
                            }
                            resource_reference__ = map.next_value()?;
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map.next_value()?;
                        }
                    }
                }
                Ok(SubnetReference {
                    resource_reference: resource_reference__,
                    attributes: attributes__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SubnetReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubnetSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.cidr.is_some() {
            len += 1;
        }
        if self.default_gateway.is_some() {
            len += 1;
        }
        if !self.dns_nameservers.is_empty() {
            len += 1;
        }
        if !self.ranges.is_empty() {
            len += 1;
        }
        if self.disable_bg_paa_sip_auto_allocation.is_some() {
            len += 1;
        }
        if self.bgpaas_primary_ip.is_some() {
            len += 1;
        }
        if self.bgpaas_secondary_ip.is_some() {
            len += 1;
        }
        if self.enable_dhcp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SubnetSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.cidr.as_ref() {
            struct_ser.serialize_field("cidr", v)?;
        }
        if let Some(v) = self.default_gateway.as_ref() {
            struct_ser.serialize_field("defaultGateway", v)?;
        }
        if !self.dns_nameservers.is_empty() {
            struct_ser.serialize_field("dnsNameservers", &self.dns_nameservers)?;
        }
        if !self.ranges.is_empty() {
            struct_ser.serialize_field("ranges", &self.ranges)?;
        }
        if let Some(v) = self.disable_bg_paa_sip_auto_allocation.as_ref() {
            struct_ser.serialize_field("disableBGPaaSIPAutoAllocation", v)?;
        }
        if let Some(v) = self.bgpaas_primary_ip.as_ref() {
            struct_ser.serialize_field("bgpaasPrimaryIP", v)?;
        }
        if let Some(v) = self.bgpaas_secondary_ip.as_ref() {
            struct_ser.serialize_field("bgpaasSecondaryIP", v)?;
        }
        if let Some(v) = self.enable_dhcp.as_ref() {
            struct_ser.serialize_field("enableDhcp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubnetSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "cidr",
            "defaultGateway",
            "dnsNameservers",
            "ranges",
            "disableBGPaaSIPAutoAllocation",
            "bgpaasPrimaryIP",
            "bgpaasSecondaryIP",
            "enableDhcp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            Cidr,
            DefaultGateway,
            DnsNameservers,
            Ranges,
            DisableBgPaaSipAutoAllocation,
            BgpaasPrimaryIp,
            BgpaasSecondaryIp,
            EnableDhcp,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "cidr" => Ok(GeneratedField::Cidr),
                            "defaultGateway" => Ok(GeneratedField::DefaultGateway),
                            "dnsNameservers" => Ok(GeneratedField::DnsNameservers),
                            "ranges" => Ok(GeneratedField::Ranges),
                            "disableBGPaaSIPAutoAllocation" => Ok(GeneratedField::DisableBgPaaSipAutoAllocation),
                            "bgpaasPrimaryIP" => Ok(GeneratedField::BgpaasPrimaryIp),
                            "bgpaasSecondaryIP" => Ok(GeneratedField::BgpaasSecondaryIp),
                            "enableDhcp" => Ok(GeneratedField::EnableDhcp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubnetSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SubnetSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubnetSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut cidr__ = None;
                let mut default_gateway__ = None;
                let mut dns_nameservers__ = None;
                let mut ranges__ = None;
                let mut disable_bg_paa_sip_auto_allocation__ = None;
                let mut bgpaas_primary_ip__ = None;
                let mut bgpaas_secondary_ip__ = None;
                let mut enable_dhcp__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::Cidr => {
                            if cidr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cidr"));
                            }
                            cidr__ = map.next_value()?;
                        }
                        GeneratedField::DefaultGateway => {
                            if default_gateway__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultGateway"));
                            }
                            default_gateway__ = map.next_value()?;
                        }
                        GeneratedField::DnsNameservers => {
                            if dns_nameservers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsNameservers"));
                            }
                            dns_nameservers__ = Some(map.next_value()?);
                        }
                        GeneratedField::Ranges => {
                            if ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ranges"));
                            }
                            ranges__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisableBgPaaSipAutoAllocation => {
                            if disable_bg_paa_sip_auto_allocation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableBGPaaSIPAutoAllocation"));
                            }
                            disable_bg_paa_sip_auto_allocation__ = map.next_value()?;
                        }
                        GeneratedField::BgpaasPrimaryIp => {
                            if bgpaas_primary_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bgpaasPrimaryIP"));
                            }
                            bgpaas_primary_ip__ = map.next_value()?;
                        }
                        GeneratedField::BgpaasSecondaryIp => {
                            if bgpaas_secondary_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bgpaasSecondaryIP"));
                            }
                            bgpaas_secondary_ip__ = map.next_value()?;
                        }
                        GeneratedField::EnableDhcp => {
                            if enable_dhcp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableDhcp"));
                            }
                            enable_dhcp__ = map.next_value()?;
                        }
                    }
                }
                Ok(SubnetSpec {
                    common_spec: common_spec__,
                    cidr: cidr__,
                    default_gateway: default_gateway__,
                    dns_nameservers: dns_nameservers__.unwrap_or_default(),
                    ranges: ranges__.unwrap_or_default(),
                    disable_bg_paa_sip_auto_allocation: disable_bg_paa_sip_auto_allocation__,
                    bgpaas_primary_ip: bgpaas_primary_ip__,
                    bgpaas_secondary_ip: bgpaas_secondary_ip__,
                    enable_dhcp: enable_dhcp__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SubnetSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubnetStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        if self.ip_count.is_some() {
            len += 1;
        }
        if self.allocation_usage.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SubnetStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        if let Some(v) = self.ip_count.as_ref() {
            struct_ser.serialize_field("ipCount", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.allocation_usage.as_ref() {
            struct_ser.serialize_field("allocationUsage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubnetStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
            "ipCount",
            "allocationUsage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
            IpCount,
            AllocationUsage,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            "ipCount" => Ok(GeneratedField::IpCount),
                            "allocationUsage" => Ok(GeneratedField::AllocationUsage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubnetStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SubnetStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubnetStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                let mut ip_count__ = None;
                let mut allocation_usage__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                        GeneratedField::IpCount => {
                            if ip_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipCount"));
                            }
                            ip_count__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AllocationUsage => {
                            if allocation_usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allocationUsage"));
                            }
                            allocation_usage__ = map.next_value()?;
                        }
                    }
                }
                Ok(SubnetStatus {
                    common_status: common_status__,
                    ip_count: ip_count__,
                    allocation_usage: allocation_usage__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.SubnetStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Tag {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.Tag", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.Tag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Tag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(Tag {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.Tag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TagList, V::Error>
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
                    }
                }
                Ok(TagList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.tag_type_name.is_some() {
            len += 1;
        }
        if self.tag_value.is_some() {
            len += 1;
        }
        if self.tag_type_reference.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.tag_type_name.as_ref() {
            struct_ser.serialize_field("tagTypeName", v)?;
        }
        if let Some(v) = self.tag_value.as_ref() {
            struct_ser.serialize_field("tagValue", v)?;
        }
        if let Some(v) = self.tag_type_reference.as_ref() {
            struct_ser.serialize_field("tagTypeReference", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "tagTypeName",
            "tagValue",
            "tagTypeReference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            TagTypeName,
            TagValue,
            TagTypeReference,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "tagTypeName" => Ok(GeneratedField::TagTypeName),
                            "tagValue" => Ok(GeneratedField::TagValue),
                            "tagTypeReference" => Ok(GeneratedField::TagTypeReference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TagSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut tag_type_name__ = None;
                let mut tag_value__ = None;
                let mut tag_type_reference__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::TagTypeName => {
                            if tag_type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagTypeName"));
                            }
                            tag_type_name__ = map.next_value()?;
                        }
                        GeneratedField::TagValue => {
                            if tag_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagValue"));
                            }
                            tag_value__ = map.next_value()?;
                        }
                        GeneratedField::TagTypeReference => {
                            if tag_type_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagTypeReference"));
                            }
                            tag_type_reference__ = map.next_value()?;
                        }
                    }
                }
                Ok(TagSpec {
                    common_spec: common_spec__,
                    tag_type_name: tag_type_name__,
                    tag_value: tag_value__,
                    tag_type_reference: tag_type_reference__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        if self.tag_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        if let Some(v) = self.tag_id.as_ref() {
            struct_ser.serialize_field("tagId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
            "tagId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
            TagId,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            "tagId" => Ok(GeneratedField::TagId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TagStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                let mut tag_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                        GeneratedField::TagId => {
                            if tag_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagId"));
                            }
                            tag_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(TagStatus {
                    common_status: common_status__,
                    tag_id: tag_id__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagType {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagType", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TagType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(TagType {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagTypeList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagTypeList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagTypeList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagTypeList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagTypeList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TagTypeList, V::Error>
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
                    }
                }
                Ok(TagTypeList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagTypeList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagTypeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagTypeSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagTypeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagTypeSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagTypeSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TagTypeSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                    }
                }
                Ok(TagTypeSpec {
                    common_spec: common_spec__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagTypeSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagTypeStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        if self.tag_type_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagTypeStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        if let Some(v) = self.tag_type_id.as_ref() {
            struct_ser.serialize_field("tagTypeId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagTypeStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
            "tagTypeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
            TagTypeId,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            "tagTypeId" => Ok(GeneratedField::TagTypeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagTypeStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagTypeStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TagTypeStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                let mut tag_type_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                        GeneratedField::TagTypeId => {
                            if tag_type_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagTypeId"));
                            }
                            tag_type_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(TagTypeStatus {
                    common_status: common_status__,
                    tag_type_id: tag_type_id__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.TagTypeStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualMachine {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachine", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualMachine {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualMachine;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachine")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualMachine, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(VirtualMachine {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachine", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualMachineInterface {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterface", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualMachineInterface {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualMachineInterface;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterface")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualMachineInterface, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(VirtualMachineInterface {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterface", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualMachineInterfaceList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualMachineInterfaceList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualMachineInterfaceList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualMachineInterfaceList, V::Error>
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
                    }
                }
                Ok(VirtualMachineInterfaceList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualMachineInterfaceProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sub_interface_vlan_tag.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceProperties", len)?;
        if let Some(v) = self.sub_interface_vlan_tag.as_ref() {
            struct_ser.serialize_field("subInterfaceVlanTag", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualMachineInterfaceProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subInterfaceVlanTag",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubInterfaceVlanTag,
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
                            "subInterfaceVlanTag" => Ok(GeneratedField::SubInterfaceVlanTag),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualMachineInterfaceProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualMachineInterfaceProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sub_interface_vlan_tag__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubInterfaceVlanTag => {
                            if sub_interface_vlan_tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subInterfaceVlanTag"));
                            }
                            sub_interface_vlan_tag__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(VirtualMachineInterfaceProperties {
                    sub_interface_vlan_tag: sub_interface_vlan_tag__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualMachineInterfaceSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.parent.is_some() {
            len += 1;
        }
        if self.virtual_machine_interface_mac_addresses.is_some() {
            len += 1;
        }
        if self.virtual_network_reference.is_some() {
            len += 1;
        }
        if !self.virtual_machine_references.is_empty() {
            len += 1;
        }
        if self.virtual_machine_interface_disable_policy.is_some() {
            len += 1;
        }
        if self.allowed_address_pairs.is_some() {
            len += 1;
        }
        if self.port_security_enabled.is_some() {
            len += 1;
        }
        if !self.virtual_machine_interface_references.is_empty() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        if !self.tag_references.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.parent.as_ref() {
            struct_ser.serialize_field("parent", v)?;
        }
        if let Some(v) = self.virtual_machine_interface_mac_addresses.as_ref() {
            struct_ser.serialize_field("virtualMachineInterfaceMacAddresses", v)?;
        }
        if let Some(v) = self.virtual_network_reference.as_ref() {
            struct_ser.serialize_field("virtualNetworkReference", v)?;
        }
        if !self.virtual_machine_references.is_empty() {
            struct_ser.serialize_field("virtualMachineReferences", &self.virtual_machine_references)?;
        }
        if let Some(v) = self.virtual_machine_interface_disable_policy.as_ref() {
            struct_ser.serialize_field("virtualMachineInterfaceDisablePolicy", v)?;
        }
        if let Some(v) = self.allowed_address_pairs.as_ref() {
            struct_ser.serialize_field("allowedAddressPairs", v)?;
        }
        if let Some(v) = self.port_security_enabled.as_ref() {
            struct_ser.serialize_field("portSecurityEnabled", v)?;
        }
        if !self.virtual_machine_interface_references.is_empty() {
            struct_ser.serialize_field("virtualMachineInterfaceReferences", &self.virtual_machine_interface_references)?;
        }
        if let Some(v) = self.properties.as_ref() {
            struct_ser.serialize_field("properties", v)?;
        }
        if !self.tag_references.is_empty() {
            struct_ser.serialize_field("tagReferences", &self.tag_references)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualMachineInterfaceSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "parent",
            "virtualMachineInterfaceMacAddresses",
            "virtualNetworkReference",
            "virtualMachineReferences",
            "virtualMachineInterfaceDisablePolicy",
            "allowedAddressPairs",
            "portSecurityEnabled",
            "virtualMachineInterfaceReferences",
            "properties",
            "tagReferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            Parent,
            VirtualMachineInterfaceMacAddresses,
            VirtualNetworkReference,
            VirtualMachineReferences,
            VirtualMachineInterfaceDisablePolicy,
            AllowedAddressPairs,
            PortSecurityEnabled,
            VirtualMachineInterfaceReferences,
            Properties,
            TagReferences,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "parent" => Ok(GeneratedField::Parent),
                            "virtualMachineInterfaceMacAddresses" => Ok(GeneratedField::VirtualMachineInterfaceMacAddresses),
                            "virtualNetworkReference" => Ok(GeneratedField::VirtualNetworkReference),
                            "virtualMachineReferences" => Ok(GeneratedField::VirtualMachineReferences),
                            "virtualMachineInterfaceDisablePolicy" => Ok(GeneratedField::VirtualMachineInterfaceDisablePolicy),
                            "allowedAddressPairs" => Ok(GeneratedField::AllowedAddressPairs),
                            "portSecurityEnabled" => Ok(GeneratedField::PortSecurityEnabled),
                            "virtualMachineInterfaceReferences" => Ok(GeneratedField::VirtualMachineInterfaceReferences),
                            "properties" => Ok(GeneratedField::Properties),
                            "tagReferences" => Ok(GeneratedField::TagReferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualMachineInterfaceSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualMachineInterfaceSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut parent__ = None;
                let mut virtual_machine_interface_mac_addresses__ = None;
                let mut virtual_network_reference__ = None;
                let mut virtual_machine_references__ = None;
                let mut virtual_machine_interface_disable_policy__ = None;
                let mut allowed_address_pairs__ = None;
                let mut port_security_enabled__ = None;
                let mut virtual_machine_interface_references__ = None;
                let mut properties__ = None;
                let mut tag_references__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = map.next_value()?;
                        }
                        GeneratedField::VirtualMachineInterfaceMacAddresses => {
                            if virtual_machine_interface_mac_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualMachineInterfaceMacAddresses"));
                            }
                            virtual_machine_interface_mac_addresses__ = map.next_value()?;
                        }
                        GeneratedField::VirtualNetworkReference => {
                            if virtual_network_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualNetworkReference"));
                            }
                            virtual_network_reference__ = map.next_value()?;
                        }
                        GeneratedField::VirtualMachineReferences => {
                            if virtual_machine_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualMachineReferences"));
                            }
                            virtual_machine_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::VirtualMachineInterfaceDisablePolicy => {
                            if virtual_machine_interface_disable_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualMachineInterfaceDisablePolicy"));
                            }
                            virtual_machine_interface_disable_policy__ = map.next_value()?;
                        }
                        GeneratedField::AllowedAddressPairs => {
                            if allowed_address_pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedAddressPairs"));
                            }
                            allowed_address_pairs__ = map.next_value()?;
                        }
                        GeneratedField::PortSecurityEnabled => {
                            if port_security_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portSecurityEnabled"));
                            }
                            port_security_enabled__ = map.next_value()?;
                        }
                        GeneratedField::VirtualMachineInterfaceReferences => {
                            if virtual_machine_interface_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualMachineInterfaceReferences"));
                            }
                            virtual_machine_interface_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::Properties => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties__ = map.next_value()?;
                        }
                        GeneratedField::TagReferences => {
                            if tag_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagReferences"));
                            }
                            tag_references__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VirtualMachineInterfaceSpec {
                    common_spec: common_spec__,
                    parent: parent__,
                    virtual_machine_interface_mac_addresses: virtual_machine_interface_mac_addresses__,
                    virtual_network_reference: virtual_network_reference__,
                    virtual_machine_references: virtual_machine_references__.unwrap_or_default(),
                    virtual_machine_interface_disable_policy: virtual_machine_interface_disable_policy__,
                    allowed_address_pairs: allowed_address_pairs__,
                    port_security_enabled: port_security_enabled__,
                    virtual_machine_interface_references: virtual_machine_interface_references__.unwrap_or_default(),
                    properties: properties__,
                    tag_references: tag_references__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualMachineInterfaceStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        if !self.routing_instance_references.is_empty() {
            len += 1;
        }
        if self.bgp_router_reference.is_some() {
            len += 1;
        }
        if !self.service_health_check.is_empty() {
            len += 1;
        }
        if !self.interface_route_table_references.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        if !self.routing_instance_references.is_empty() {
            struct_ser.serialize_field("routingInstanceReferences", &self.routing_instance_references)?;
        }
        if let Some(v) = self.bgp_router_reference.as_ref() {
            struct_ser.serialize_field("bgpRouterReference", v)?;
        }
        if !self.service_health_check.is_empty() {
            struct_ser.serialize_field("serviceHealthCheck", &self.service_health_check)?;
        }
        if !self.interface_route_table_references.is_empty() {
            struct_ser.serialize_field("interfaceRouteTableReferences", &self.interface_route_table_references)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualMachineInterfaceStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
            "routingInstanceReferences",
            "bgpRouterReference",
            "serviceHealthCheck",
            "interfaceRouteTableReferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
            RoutingInstanceReferences,
            BgpRouterReference,
            ServiceHealthCheck,
            InterfaceRouteTableReferences,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            "routingInstanceReferences" => Ok(GeneratedField::RoutingInstanceReferences),
                            "bgpRouterReference" => Ok(GeneratedField::BgpRouterReference),
                            "serviceHealthCheck" => Ok(GeneratedField::ServiceHealthCheck),
                            "interfaceRouteTableReferences" => Ok(GeneratedField::InterfaceRouteTableReferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualMachineInterfaceStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualMachineInterfaceStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                let mut routing_instance_references__ = None;
                let mut bgp_router_reference__ = None;
                let mut service_health_check__ = None;
                let mut interface_route_table_references__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                        GeneratedField::RoutingInstanceReferences => {
                            if routing_instance_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routingInstanceReferences"));
                            }
                            routing_instance_references__ = Some(map.next_value()?);
                        }
                        GeneratedField::BgpRouterReference => {
                            if bgp_router_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bgpRouterReference"));
                            }
                            bgp_router_reference__ = map.next_value()?;
                        }
                        GeneratedField::ServiceHealthCheck => {
                            if service_health_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceHealthCheck"));
                            }
                            service_health_check__ = Some(map.next_value()?);
                        }
                        GeneratedField::InterfaceRouteTableReferences => {
                            if interface_route_table_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interfaceRouteTableReferences"));
                            }
                            interface_route_table_references__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VirtualMachineInterfaceStatus {
                    common_status: common_status__,
                    routing_instance_references: routing_instance_references__.unwrap_or_default(),
                    bgp_router_reference: bgp_router_reference__,
                    service_health_check: service_health_check__.unwrap_or_default(),
                    interface_route_table_references: interface_route_table_references__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineInterfaceStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualMachineList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualMachineList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualMachineList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualMachineList, V::Error>
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
                    }
                }
                Ok(VirtualMachineList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualMachineSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.server_type.is_some() {
            len += 1;
        }
        if self.server_name.is_some() {
            len += 1;
        }
        if self.server_namespace.is_some() {
            len += 1;
        }
        if self.server_cluster_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.server_type.as_ref() {
            struct_ser.serialize_field("serverType", v)?;
        }
        if let Some(v) = self.server_name.as_ref() {
            struct_ser.serialize_field("serverName", v)?;
        }
        if let Some(v) = self.server_namespace.as_ref() {
            struct_ser.serialize_field("serverNamespace", v)?;
        }
        if let Some(v) = self.server_cluster_name.as_ref() {
            struct_ser.serialize_field("serverClusterName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualMachineSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "serverType",
            "serverName",
            "serverNamespace",
            "serverClusterName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            ServerType,
            ServerName,
            ServerNamespace,
            ServerClusterName,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "serverType" => Ok(GeneratedField::ServerType),
                            "serverName" => Ok(GeneratedField::ServerName),
                            "serverNamespace" => Ok(GeneratedField::ServerNamespace),
                            "serverClusterName" => Ok(GeneratedField::ServerClusterName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualMachineSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualMachineSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut server_type__ = None;
                let mut server_name__ = None;
                let mut server_namespace__ = None;
                let mut server_cluster_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::ServerType => {
                            if server_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverType"));
                            }
                            server_type__ = map.next_value()?;
                        }
                        GeneratedField::ServerName => {
                            if server_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverName"));
                            }
                            server_name__ = map.next_value()?;
                        }
                        GeneratedField::ServerNamespace => {
                            if server_namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverNamespace"));
                            }
                            server_namespace__ = map.next_value()?;
                        }
                        GeneratedField::ServerClusterName => {
                            if server_cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverClusterName"));
                            }
                            server_cluster_name__ = map.next_value()?;
                        }
                    }
                }
                Ok(VirtualMachineSpec {
                    common_spec: common_spec__,
                    server_type: server_type__,
                    server_name: server_name__,
                    server_namespace: server_namespace__,
                    server_cluster_name: server_cluster_name__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualMachineStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualMachineStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualMachineStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualMachineStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(VirtualMachineStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualMachineStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualNetwork {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetwork", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualNetwork {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualNetwork;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetwork")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualNetwork, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(VirtualNetwork {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetwork", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualNetworkList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualNetworkList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualNetworkList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualNetworkList, V::Error>
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
                    }
                }
                Ok(VirtualNetworkList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualNetworkReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_reference.is_some() {
            len += 1;
        }
        if self.attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkReference", len)?;
        if let Some(v) = self.resource_reference.as_ref() {
            struct_ser.serialize_field("resourceReference", v)?;
        }
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualNetworkReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resourceReference",
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceReference,
            Attributes,
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
                            "resourceReference" => Ok(GeneratedField::ResourceReference),
                            "attributes" => Ok(GeneratedField::Attributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualNetworkReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualNetworkReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_reference__ = None;
                let mut attributes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceReference => {
                            if resource_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceReference"));
                            }
                            resource_reference__ = map.next_value()?;
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map.next_value()?;
                        }
                    }
                }
                Ok(VirtualNetworkReference {
                    resource_reference: resource_reference__,
                    attributes: attributes__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualNetworkRouteTargetReferenceList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.route_target_references.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouteTargetReferenceList", len)?;
        if !self.route_target_references.is_empty() {
            struct_ser.serialize_field("routeTargetReferences", &self.route_target_references)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualNetworkRouteTargetReferenceList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "routeTargetReferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RouteTargetReferences,
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
                            "routeTargetReferences" => Ok(GeneratedField::RouteTargetReferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualNetworkRouteTargetReferenceList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouteTargetReferenceList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualNetworkRouteTargetReferenceList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut route_target_references__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RouteTargetReferences => {
                            if route_target_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeTargetReferences"));
                            }
                            route_target_references__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VirtualNetworkRouteTargetReferenceList {
                    route_target_references: route_target_references__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouteTargetReferenceList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualNetworkRouter {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouter", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualNetworkRouter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualNetworkRouter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualNetworkRouter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(VirtualNetworkRouter {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualNetworkRouterEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.virtual_network_router_selector.is_some() {
            len += 1;
        }
        if self.namespace_selector.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterEntry", len)?;
        if let Some(v) = self.virtual_network_router_selector.as_ref() {
            struct_ser.serialize_field("virtualNetworkRouterSelector", v)?;
        }
        if let Some(v) = self.namespace_selector.as_ref() {
            struct_ser.serialize_field("namespaceSelector", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualNetworkRouterEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "virtualNetworkRouterSelector",
            "namespaceSelector",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VirtualNetworkRouterSelector,
            NamespaceSelector,
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
                            "virtualNetworkRouterSelector" => Ok(GeneratedField::VirtualNetworkRouterSelector),
                            "namespaceSelector" => Ok(GeneratedField::NamespaceSelector),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualNetworkRouterEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterEntry")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualNetworkRouterEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut virtual_network_router_selector__ = None;
                let mut namespace_selector__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VirtualNetworkRouterSelector => {
                            if virtual_network_router_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualNetworkRouterSelector"));
                            }
                            virtual_network_router_selector__ = map.next_value()?;
                        }
                        GeneratedField::NamespaceSelector => {
                            if namespace_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceSelector"));
                            }
                            namespace_selector__ = map.next_value()?;
                        }
                    }
                }
                Ok(VirtualNetworkRouterEntry {
                    virtual_network_router_selector: virtual_network_router_selector__,
                    namespace_selector: namespace_selector__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualNetworkRouterList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualNetworkRouterList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualNetworkRouterList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualNetworkRouterList, V::Error>
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
                    }
                }
                Ok(VirtualNetworkRouterList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualNetworkRouterSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        if self.virtual_network_selector.is_some() {
            len += 1;
        }
        if self.import.is_some() {
            len += 1;
        }
        if self.route_target.is_some() {
            len += 1;
        }
        if self.routing_type.is_some() {
            len += 1;
        }
        if self.l3vxlan_network_identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        if let Some(v) = self.virtual_network_selector.as_ref() {
            struct_ser.serialize_field("virtualNetworkSelector", v)?;
        }
        if let Some(v) = self.import.as_ref() {
            struct_ser.serialize_field("import", v)?;
        }
        if let Some(v) = self.route_target.as_ref() {
            struct_ser.serialize_field("routeTarget", v)?;
        }
        if let Some(v) = self.routing_type.as_ref() {
            struct_ser.serialize_field("routingType", v)?;
        }
        if let Some(v) = self.l3vxlan_network_identifier.as_ref() {
            struct_ser.serialize_field("l3vxlanNetworkIdentifier", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualNetworkRouterSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "type",
            "virtualNetworkSelector",
            "import",
            "routeTarget",
            "routingType",
            "l3vxlanNetworkIdentifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            Type,
            VirtualNetworkSelector,
            Import,
            RouteTarget,
            RoutingType,
            L3vxlanNetworkIdentifier,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "type" => Ok(GeneratedField::Type),
                            "virtualNetworkSelector" => Ok(GeneratedField::VirtualNetworkSelector),
                            "import" => Ok(GeneratedField::Import),
                            "routeTarget" => Ok(GeneratedField::RouteTarget),
                            "routingType" => Ok(GeneratedField::RoutingType),
                            "l3vxlanNetworkIdentifier" => Ok(GeneratedField::L3vxlanNetworkIdentifier),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualNetworkRouterSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualNetworkRouterSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut r#type__ = None;
                let mut virtual_network_selector__ = None;
                let mut import__ = None;
                let mut route_target__ = None;
                let mut routing_type__ = None;
                let mut l3vxlan_network_identifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map.next_value()?;
                        }
                        GeneratedField::VirtualNetworkSelector => {
                            if virtual_network_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualNetworkSelector"));
                            }
                            virtual_network_selector__ = map.next_value()?;
                        }
                        GeneratedField::Import => {
                            if import__.is_some() {
                                return Err(serde::de::Error::duplicate_field("import"));
                            }
                            import__ = map.next_value()?;
                        }
                        GeneratedField::RouteTarget => {
                            if route_target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeTarget"));
                            }
                            route_target__ = map.next_value()?;
                        }
                        GeneratedField::RoutingType => {
                            if routing_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routingType"));
                            }
                            routing_type__ = map.next_value()?;
                        }
                        GeneratedField::L3vxlanNetworkIdentifier => {
                            if l3vxlan_network_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("l3vxlanNetworkIdentifier"));
                            }
                            l3vxlan_network_identifier__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(VirtualNetworkRouterSpec {
                    common_spec: common_spec__,
                    r#type: r#type__,
                    virtual_network_selector: virtual_network_selector__,
                    import: import__,
                    route_target: route_target__,
                    routing_type: routing_type__,
                    l3vxlan_network_identifier: l3vxlan_network_identifier__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualNetworkRouterStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        if self.evpn_routing_virtual_network_reference.is_some() {
            len += 1;
        }
        if self.l3vxlan_network_identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        if let Some(v) = self.evpn_routing_virtual_network_reference.as_ref() {
            struct_ser.serialize_field("evpnRoutingVirtualNetworkReference", v)?;
        }
        if let Some(v) = self.l3vxlan_network_identifier.as_ref() {
            struct_ser.serialize_field("l3vxlanNetworkIdentifier", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualNetworkRouterStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
            "evpnRoutingVirtualNetworkReference",
            "l3vxlanNetworkIdentifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
            EvpnRoutingVirtualNetworkReference,
            L3vxlanNetworkIdentifier,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            "evpnRoutingVirtualNetworkReference" => Ok(GeneratedField::EvpnRoutingVirtualNetworkReference),
                            "l3vxlanNetworkIdentifier" => Ok(GeneratedField::L3vxlanNetworkIdentifier),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualNetworkRouterStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualNetworkRouterStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                let mut evpn_routing_virtual_network_reference__ = None;
                let mut l3vxlan_network_identifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                        GeneratedField::EvpnRoutingVirtualNetworkReference => {
                            if evpn_routing_virtual_network_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("evpnRoutingVirtualNetworkReference"));
                            }
                            evpn_routing_virtual_network_reference__ = map.next_value()?;
                        }
                        GeneratedField::L3vxlanNetworkIdentifier => {
                            if l3vxlan_network_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("l3vxlanNetworkIdentifier"));
                            }
                            l3vxlan_network_identifier__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(VirtualNetworkRouterStatus {
                    common_status: common_status__,
                    evpn_routing_virtual_network_reference: evpn_routing_virtual_network_reference__,
                    l3vxlan_network_identifier: l3vxlan_network_identifier__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualNetworkSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.fabric_snat.is_some() {
            len += 1;
        }
        if self.v4_subnet_reference.is_some() {
            len += 1;
        }
        if self.v6_subnet_reference.is_some() {
            len += 1;
        }
        if !self.route_target_list.is_empty() {
            len += 1;
        }
        if !self.import_route_target_list.is_empty() {
            len += 1;
        }
        if !self.export_route_target_list.is_empty() {
            len += 1;
        }
        if self.virtual_network_properties.is_some() {
            len += 1;
        }
        if self.provider_network_reference.is_some() {
            len += 1;
        }
        if self.is_provider_network.is_some() {
            len += 1;
        }
        if self.fabric_forwarding.is_some() {
            len += 1;
        }
        if self.vlan_id.is_some() {
            len += 1;
        }
        if self.virtual_network_network_id.is_some() {
            len += 1;
        }
        if self.pod_network.is_some() {
            len += 1;
        }
        if !self.route_table_references.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.fabric_snat.as_ref() {
            struct_ser.serialize_field("fabricSNAT", v)?;
        }
        if let Some(v) = self.v4_subnet_reference.as_ref() {
            struct_ser.serialize_field("v4SubnetReference", v)?;
        }
        if let Some(v) = self.v6_subnet_reference.as_ref() {
            struct_ser.serialize_field("v6SubnetReference", v)?;
        }
        if !self.route_target_list.is_empty() {
            struct_ser.serialize_field("routeTargetList", &self.route_target_list)?;
        }
        if !self.import_route_target_list.is_empty() {
            struct_ser.serialize_field("importRouteTargetList", &self.import_route_target_list)?;
        }
        if !self.export_route_target_list.is_empty() {
            struct_ser.serialize_field("exportRouteTargetList", &self.export_route_target_list)?;
        }
        if let Some(v) = self.virtual_network_properties.as_ref() {
            struct_ser.serialize_field("virtualNetworkProperties", v)?;
        }
        if let Some(v) = self.provider_network_reference.as_ref() {
            struct_ser.serialize_field("providerNetworkReference", v)?;
        }
        if let Some(v) = self.is_provider_network.as_ref() {
            struct_ser.serialize_field("isProviderNetwork", v)?;
        }
        if let Some(v) = self.fabric_forwarding.as_ref() {
            struct_ser.serialize_field("fabricForwarding", v)?;
        }
        if let Some(v) = self.vlan_id.as_ref() {
            struct_ser.serialize_field("vlanID", v)?;
        }
        if let Some(v) = self.virtual_network_network_id.as_ref() {
            struct_ser.serialize_field("virtualNetworkNetworkId", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.pod_network.as_ref() {
            struct_ser.serialize_field("podNetwork", v)?;
        }
        if !self.route_table_references.is_empty() {
            struct_ser.serialize_field("routeTableReferences", &self.route_table_references)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualNetworkSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "fabricSNAT",
            "v4SubnetReference",
            "v6SubnetReference",
            "routeTargetList",
            "importRouteTargetList",
            "exportRouteTargetList",
            "virtualNetworkProperties",
            "providerNetworkReference",
            "isProviderNetwork",
            "fabricForwarding",
            "vlanID",
            "virtualNetworkNetworkId",
            "podNetwork",
            "routeTableReferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            FabricSnat,
            V4SubnetReference,
            V6SubnetReference,
            RouteTargetList,
            ImportRouteTargetList,
            ExportRouteTargetList,
            VirtualNetworkProperties,
            ProviderNetworkReference,
            IsProviderNetwork,
            FabricForwarding,
            VlanId,
            VirtualNetworkNetworkId,
            PodNetwork,
            RouteTableReferences,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "fabricSNAT" => Ok(GeneratedField::FabricSnat),
                            "v4SubnetReference" => Ok(GeneratedField::V4SubnetReference),
                            "v6SubnetReference" => Ok(GeneratedField::V6SubnetReference),
                            "routeTargetList" => Ok(GeneratedField::RouteTargetList),
                            "importRouteTargetList" => Ok(GeneratedField::ImportRouteTargetList),
                            "exportRouteTargetList" => Ok(GeneratedField::ExportRouteTargetList),
                            "virtualNetworkProperties" => Ok(GeneratedField::VirtualNetworkProperties),
                            "providerNetworkReference" => Ok(GeneratedField::ProviderNetworkReference),
                            "isProviderNetwork" => Ok(GeneratedField::IsProviderNetwork),
                            "fabricForwarding" => Ok(GeneratedField::FabricForwarding),
                            "vlanID" => Ok(GeneratedField::VlanId),
                            "virtualNetworkNetworkId" => Ok(GeneratedField::VirtualNetworkNetworkId),
                            "podNetwork" => Ok(GeneratedField::PodNetwork),
                            "routeTableReferences" => Ok(GeneratedField::RouteTableReferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualNetworkSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualNetworkSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut fabric_snat__ = None;
                let mut v4_subnet_reference__ = None;
                let mut v6_subnet_reference__ = None;
                let mut route_target_list__ = None;
                let mut import_route_target_list__ = None;
                let mut export_route_target_list__ = None;
                let mut virtual_network_properties__ = None;
                let mut provider_network_reference__ = None;
                let mut is_provider_network__ = None;
                let mut fabric_forwarding__ = None;
                let mut vlan_id__ = None;
                let mut virtual_network_network_id__ = None;
                let mut pod_network__ = None;
                let mut route_table_references__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::FabricSnat => {
                            if fabric_snat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fabricSNAT"));
                            }
                            fabric_snat__ = map.next_value()?;
                        }
                        GeneratedField::V4SubnetReference => {
                            if v4_subnet_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("v4SubnetReference"));
                            }
                            v4_subnet_reference__ = map.next_value()?;
                        }
                        GeneratedField::V6SubnetReference => {
                            if v6_subnet_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("v6SubnetReference"));
                            }
                            v6_subnet_reference__ = map.next_value()?;
                        }
                        GeneratedField::RouteTargetList => {
                            if route_target_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeTargetList"));
                            }
                            route_target_list__ = Some(map.next_value()?);
                        }
                        GeneratedField::ImportRouteTargetList => {
                            if import_route_target_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("importRouteTargetList"));
                            }
                            import_route_target_list__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExportRouteTargetList => {
                            if export_route_target_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exportRouteTargetList"));
                            }
                            export_route_target_list__ = Some(map.next_value()?);
                        }
                        GeneratedField::VirtualNetworkProperties => {
                            if virtual_network_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualNetworkProperties"));
                            }
                            virtual_network_properties__ = map.next_value()?;
                        }
                        GeneratedField::ProviderNetworkReference => {
                            if provider_network_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerNetworkReference"));
                            }
                            provider_network_reference__ = map.next_value()?;
                        }
                        GeneratedField::IsProviderNetwork => {
                            if is_provider_network__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isProviderNetwork"));
                            }
                            is_provider_network__ = map.next_value()?;
                        }
                        GeneratedField::FabricForwarding => {
                            if fabric_forwarding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fabricForwarding"));
                            }
                            fabric_forwarding__ = map.next_value()?;
                        }
                        GeneratedField::VlanId => {
                            if vlan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vlanID"));
                            }
                            vlan_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::VirtualNetworkNetworkId => {
                            if virtual_network_network_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualNetworkNetworkId"));
                            }
                            virtual_network_network_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PodNetwork => {
                            if pod_network__.is_some() {
                                return Err(serde::de::Error::duplicate_field("podNetwork"));
                            }
                            pod_network__ = map.next_value()?;
                        }
                        GeneratedField::RouteTableReferences => {
                            if route_table_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeTableReferences"));
                            }
                            route_table_references__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VirtualNetworkSpec {
                    common_spec: common_spec__,
                    fabric_snat: fabric_snat__,
                    v4_subnet_reference: v4_subnet_reference__,
                    v6_subnet_reference: v6_subnet_reference__,
                    route_target_list: route_target_list__.unwrap_or_default(),
                    import_route_target_list: import_route_target_list__.unwrap_or_default(),
                    export_route_target_list: export_route_target_list__.unwrap_or_default(),
                    virtual_network_properties: virtual_network_properties__,
                    provider_network_reference: provider_network_reference__,
                    is_provider_network: is_provider_network__,
                    fabric_forwarding: fabric_forwarding__,
                    vlan_id: vlan_id__,
                    virtual_network_network_id: virtual_network_network_id__,
                    pod_network: pod_network__,
                    route_table_references: route_table_references__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualNetworkStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        if self.virtual_network_network_id.is_some() {
            len += 1;
        }
        if !self.evpn_routing_virtual_network_references.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        if let Some(v) = self.virtual_network_network_id.as_ref() {
            struct_ser.serialize_field("virtualNetworkNetworkId", ToString::to_string(&v).as_str())?;
        }
        if !self.evpn_routing_virtual_network_references.is_empty() {
            struct_ser.serialize_field("evpnRoutingVirtualNetworkReferences", &self.evpn_routing_virtual_network_references)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualNetworkStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
            "virtualNetworkNetworkId",
            "evpnRoutingVirtualNetworkReferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
            VirtualNetworkNetworkId,
            EvpnRoutingVirtualNetworkReferences,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            "virtualNetworkNetworkId" => Ok(GeneratedField::VirtualNetworkNetworkId),
                            "evpnRoutingVirtualNetworkReferences" => Ok(GeneratedField::EvpnRoutingVirtualNetworkReferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualNetworkStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualNetworkStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                let mut virtual_network_network_id__ = None;
                let mut evpn_routing_virtual_network_references__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                        GeneratedField::VirtualNetworkNetworkId => {
                            if virtual_network_network_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualNetworkNetworkId"));
                            }
                            virtual_network_network_id__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::EvpnRoutingVirtualNetworkReferences => {
                            if evpn_routing_virtual_network_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("evpnRoutingVirtualNetworkReferences"));
                            }
                            evpn_routing_virtual_network_references__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VirtualNetworkStatus {
                    common_status: common_status__,
                    virtual_network_network_id: virtual_network_network_id__,
                    evpn_routing_virtual_network_references: evpn_routing_virtual_network_references__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualNetworkType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rpf.is_some() {
            len += 1;
        }
        if self.forwarding_mode.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkType", len)?;
        if let Some(v) = self.rpf.as_ref() {
            struct_ser.serialize_field("rpf", v)?;
        }
        if let Some(v) = self.forwarding_mode.as_ref() {
            struct_ser.serialize_field("forwardingMode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualNetworkType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rpf",
            "forwardingMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rpf,
            ForwardingMode,
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
                            "rpf" => Ok(GeneratedField::Rpf),
                            "forwardingMode" => Ok(GeneratedField::ForwardingMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualNetworkType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualNetworkType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rpf__ = None;
                let mut forwarding_mode__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rpf => {
                            if rpf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rpf"));
                            }
                            rpf__ = map.next_value()?;
                        }
                        GeneratedField::ForwardingMode => {
                            if forwarding_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardingMode"));
                            }
                            forwarding_mode__ = map.next_value()?;
                        }
                    }
                }
                Ok(VirtualNetworkType {
                    rpf: rpf__,
                    forwarding_mode: forwarding_mode__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualRouter {
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
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouter", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualRouter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualRouter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualRouter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(VirtualRouter {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualRouterList {
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
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouterList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualRouterList {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualRouterList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouterList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualRouterList, V::Error>
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
                    }
                }
                Ok(VirtualRouterList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouterList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualRouterSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_spec.is_some() {
            len += 1;
        }
        if self.parent.is_some() {
            len += 1;
        }
        if self.virtual_router_dpdk_enabled.is_some() {
            len += 1;
        }
        if self.virtual_router_ip_address.is_some() {
            len += 1;
        }
        if self.virtual_router_type.is_some() {
            len += 1;
        }
        if !self.virtual_machine_references.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouterSpec", len)?;
        if let Some(v) = self.common_spec.as_ref() {
            struct_ser.serialize_field("commonSpec", v)?;
        }
        if let Some(v) = self.parent.as_ref() {
            struct_ser.serialize_field("parent", v)?;
        }
        if let Some(v) = self.virtual_router_dpdk_enabled.as_ref() {
            struct_ser.serialize_field("virtualRouterDpdkEnabled", v)?;
        }
        if let Some(v) = self.virtual_router_ip_address.as_ref() {
            struct_ser.serialize_field("virtualRouterIPAddress", v)?;
        }
        if let Some(v) = self.virtual_router_type.as_ref() {
            struct_ser.serialize_field("virtualRouterType", v)?;
        }
        if !self.virtual_machine_references.is_empty() {
            struct_ser.serialize_field("virtualMachineReferences", &self.virtual_machine_references)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualRouterSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonSpec",
            "parent",
            "virtualRouterDpdkEnabled",
            "virtualRouterIPAddress",
            "virtualRouterType",
            "virtualMachineReferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonSpec,
            Parent,
            VirtualRouterDpdkEnabled,
            VirtualRouterIpAddress,
            VirtualRouterType,
            VirtualMachineReferences,
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
                            "commonSpec" => Ok(GeneratedField::CommonSpec),
                            "parent" => Ok(GeneratedField::Parent),
                            "virtualRouterDpdkEnabled" => Ok(GeneratedField::VirtualRouterDpdkEnabled),
                            "virtualRouterIPAddress" => Ok(GeneratedField::VirtualRouterIpAddress),
                            "virtualRouterType" => Ok(GeneratedField::VirtualRouterType),
                            "virtualMachineReferences" => Ok(GeneratedField::VirtualMachineReferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualRouterSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouterSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualRouterSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_spec__ = None;
                let mut parent__ = None;
                let mut virtual_router_dpdk_enabled__ = None;
                let mut virtual_router_ip_address__ = None;
                let mut virtual_router_type__ = None;
                let mut virtual_machine_references__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonSpec => {
                            if common_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonSpec"));
                            }
                            common_spec__ = map.next_value()?;
                        }
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = map.next_value()?;
                        }
                        GeneratedField::VirtualRouterDpdkEnabled => {
                            if virtual_router_dpdk_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualRouterDpdkEnabled"));
                            }
                            virtual_router_dpdk_enabled__ = map.next_value()?;
                        }
                        GeneratedField::VirtualRouterIpAddress => {
                            if virtual_router_ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualRouterIPAddress"));
                            }
                            virtual_router_ip_address__ = map.next_value()?;
                        }
                        GeneratedField::VirtualRouterType => {
                            if virtual_router_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualRouterType"));
                            }
                            virtual_router_type__ = map.next_value()?;
                        }
                        GeneratedField::VirtualMachineReferences => {
                            if virtual_machine_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualMachineReferences"));
                            }
                            virtual_machine_references__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VirtualRouterSpec {
                    common_spec: common_spec__,
                    parent: parent__,
                    virtual_router_dpdk_enabled: virtual_router_dpdk_enabled__,
                    virtual_router_ip_address: virtual_router_ip_address__,
                    virtual_router_type: virtual_router_type__,
                    virtual_machine_references: virtual_machine_references__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouterSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualRouterStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouterStatus", len)?;
        if let Some(v) = self.common_status.as_ref() {
            struct_ser.serialize_field("commonStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualRouterStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commonStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonStatus,
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
                            "commonStatus" => Ok(GeneratedField::CommonStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualRouterStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouterStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualRouterStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonStatus => {
                            if common_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonStatus"));
                            }
                            common_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(VirtualRouterStatus {
                    common_status: common_status__,
                })
            }
        }
        deserializer.deserialize_struct("ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4.VirtualRouterStatus", FIELDS, GeneratedVisitor)
    }
}
