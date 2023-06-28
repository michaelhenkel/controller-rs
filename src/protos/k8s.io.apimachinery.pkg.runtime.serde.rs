impl serde::Serialize for RawExtension {
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
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.runtime.RawExtension", len)?;
        if let Some(v) = self.raw.as_ref() {
            struct_ser.serialize_field("raw", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RawExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "raw",
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
                            "raw" => Ok(GeneratedField::Raw),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RawExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.runtime.RawExtension")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RawExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut raw__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Raw => {
                            if raw__.is_some() {
                                return Err(serde::de::Error::duplicate_field("raw"));
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
                Ok(RawExtension {
                    raw: raw__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.runtime.RawExtension", FIELDS, GeneratedVisitor)
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
        if self.api_version.is_some() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.runtime.TypeMeta", len)?;
        if let Some(v) = self.api_version.as_ref() {
            struct_ser.serialize_field("apiVersion", v)?;
        }
        if let Some(v) = self.kind.as_ref() {
            struct_ser.serialize_field("kind", v)?;
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
            "apiVersion",
            "kind",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ApiVersion,
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
                            "apiVersion" => Ok(GeneratedField::ApiVersion),
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
            type Value = TypeMeta;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.runtime.TypeMeta")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TypeMeta, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut api_version__ = None;
                let mut kind__ = None;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TypeMeta {
                    api_version: api_version__,
                    kind: kind__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.runtime.TypeMeta", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Unknown {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.type_meta.is_some() {
            len += 1;
        }
        if self.raw.is_some() {
            len += 1;
        }
        if self.content_encoding.is_some() {
            len += 1;
        }
        if self.content_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.runtime.Unknown", len)?;
        if let Some(v) = self.type_meta.as_ref() {
            struct_ser.serialize_field("typeMeta", v)?;
        }
        if let Some(v) = self.raw.as_ref() {
            struct_ser.serialize_field("raw", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.content_encoding.as_ref() {
            struct_ser.serialize_field("contentEncoding", v)?;
        }
        if let Some(v) = self.content_type.as_ref() {
            struct_ser.serialize_field("contentType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Unknown {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "typeMeta",
            "raw",
            "contentEncoding",
            "contentType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypeMeta,
            Raw,
            ContentEncoding,
            ContentType,
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
                            "typeMeta" => Ok(GeneratedField::TypeMeta),
                            "raw" => Ok(GeneratedField::Raw),
                            "contentEncoding" => Ok(GeneratedField::ContentEncoding),
                            "contentType" => Ok(GeneratedField::ContentType),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Unknown;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.runtime.Unknown")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Unknown, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut type_meta__ = None;
                let mut raw__ = None;
                let mut content_encoding__ = None;
                let mut content_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TypeMeta => {
                            if type_meta__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeMeta"));
                            }
                            type_meta__ = map.next_value()?;
                        }
                        GeneratedField::Raw => {
                            if raw__.is_some() {
                                return Err(serde::de::Error::duplicate_field("raw"));
                            }
                            raw__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ContentEncoding => {
                            if content_encoding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentEncoding"));
                            }
                            content_encoding__ = map.next_value()?;
                        }
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Unknown {
                    type_meta: type_meta__,
                    raw: raw__,
                    content_encoding: content_encoding__,
                    content_type: content_type__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.runtime.Unknown", FIELDS, GeneratedVisitor)
    }
}
