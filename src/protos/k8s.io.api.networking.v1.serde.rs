impl serde::Serialize for HttpIngressPath {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.path.is_some() {
            len += 1;
        }
        if self.path_type.is_some() {
            len += 1;
        }
        if self.backend.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.HTTPIngressPath", len)?;
        if let Some(v) = self.path.as_ref() {
            struct_ser.serialize_field("path", v)?;
        }
        if let Some(v) = self.path_type.as_ref() {
            struct_ser.serialize_field("pathType", v)?;
        }
        if let Some(v) = self.backend.as_ref() {
            struct_ser.serialize_field("backend", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpIngressPath {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "pathType",
            "backend",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            PathType,
            Backend,
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
                            "path" => Ok(GeneratedField::Path),
                            "pathType" => Ok(GeneratedField::PathType),
                            "backend" => Ok(GeneratedField::Backend),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpIngressPath;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.HTTPIngressPath")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpIngressPath, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut path_type__ = None;
                let mut backend__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = map.next_value()?;
                        }
                        GeneratedField::PathType => {
                            if path_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathType"));
                            }
                            path_type__ = map.next_value()?;
                        }
                        GeneratedField::Backend => {
                            if backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backend"));
                            }
                            backend__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HttpIngressPath {
                    path: path__,
                    path_type: path_type__,
                    backend: backend__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.HTTPIngressPath", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpIngressRuleValue {
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
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.HTTPIngressRuleValue", len)?;
        if !self.paths.is_empty() {
            struct_ser.serialize_field("paths", &self.paths)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpIngressRuleValue {
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
            type Value = HttpIngressRuleValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.HTTPIngressRuleValue")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpIngressRuleValue, V::Error>
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
                Ok(HttpIngressRuleValue {
                    paths: paths__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.HTTPIngressRuleValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IpBlock {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cidr.is_some() {
            len += 1;
        }
        if !self.except.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IPBlock", len)?;
        if let Some(v) = self.cidr.as_ref() {
            struct_ser.serialize_field("cidr", v)?;
        }
        if !self.except.is_empty() {
            struct_ser.serialize_field("except", &self.except)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IpBlock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cidr",
            "except",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cidr,
            Except,
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
                            "cidr" => Ok(GeneratedField::Cidr),
                            "except" => Ok(GeneratedField::Except),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IpBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IPBlock")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IpBlock, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cidr__ = None;
                let mut except__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Cidr => {
                            if cidr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cidr"));
                            }
                            cidr__ = map.next_value()?;
                        }
                        GeneratedField::Except => {
                            if except__.is_some() {
                                return Err(serde::de::Error::duplicate_field("except"));
                            }
                            except__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IpBlock {
                    cidr: cidr__,
                    except: except__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IPBlock", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Ingress {
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
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.Ingress", len)?;
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
impl<'de> serde::Deserialize<'de> for Ingress {
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
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Ingress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.Ingress")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Ingress, V::Error>
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Ingress {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.Ingress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressBackend {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.service.is_some() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressBackend", len)?;
        if let Some(v) = self.service.as_ref() {
            struct_ser.serialize_field("service", v)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressBackend {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service",
            "resource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Service,
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
                            "service" => Ok(GeneratedField::Service),
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
            type Value = IngressBackend;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressBackend")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressBackend, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service__ = None;
                let mut resource__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Service => {
                            if service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service__ = map.next_value()?;
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
                Ok(IngressBackend {
                    service: service__,
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressBackend", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressClass {
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
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressClass", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressClass {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
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
                            "spec" => Ok(GeneratedField::Spec),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngressClass;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressClass")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressClass, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IngressClass {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressClass", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressClassList {
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
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressClassList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressClassList {
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
            type Value = IngressClassList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressClassList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressClassList, V::Error>
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
                Ok(IngressClassList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressClassList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressClassParametersReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.a_pi_group.is_some() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        if self.scope.is_some() {
            len += 1;
        }
        if self.namespace.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressClassParametersReference", len)?;
        if let Some(v) = self.a_pi_group.as_ref() {
            struct_ser.serialize_field("aPIGroup", v)?;
        }
        if let Some(v) = self.kind.as_ref() {
            struct_ser.serialize_field("kind", v)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.scope.as_ref() {
            struct_ser.serialize_field("scope", v)?;
        }
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressClassParametersReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "aPIGroup",
            "kind",
            "name",
            "scope",
            "namespace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            APiGroup,
            Kind,
            Name,
            Scope,
            Namespace,
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
                            "aPIGroup" => Ok(GeneratedField::APiGroup),
                            "kind" => Ok(GeneratedField::Kind),
                            "name" => Ok(GeneratedField::Name),
                            "scope" => Ok(GeneratedField::Scope),
                            "namespace" => Ok(GeneratedField::Namespace),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngressClassParametersReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressClassParametersReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressClassParametersReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut a_pi_group__ = None;
                let mut kind__ = None;
                let mut name__ = None;
                let mut scope__ = None;
                let mut namespace__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::APiGroup => {
                            if a_pi_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aPIGroup"));
                            }
                            a_pi_group__ = map.next_value()?;
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
                        GeneratedField::Scope => {
                            if scope__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            scope__ = map.next_value()?;
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IngressClassParametersReference {
                    a_pi_group: a_pi_group__,
                    kind: kind__,
                    name: name__,
                    scope: scope__,
                    namespace: namespace__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressClassParametersReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressClassSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.controller.is_some() {
            len += 1;
        }
        if self.parameters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressClassSpec", len)?;
        if let Some(v) = self.controller.as_ref() {
            struct_ser.serialize_field("controller", v)?;
        }
        if let Some(v) = self.parameters.as_ref() {
            struct_ser.serialize_field("parameters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressClassSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "controller",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Controller,
            Parameters,
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
                            "controller" => Ok(GeneratedField::Controller),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngressClassSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressClassSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressClassSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut controller__ = None;
                let mut parameters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Controller => {
                            if controller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("controller"));
                            }
                            controller__ = map.next_value()?;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IngressClassSpec {
                    controller: controller__,
                    parameters: parameters__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressClassSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressList {
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
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressList {
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
            type Value = IngressList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressList, V::Error>
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
                Ok(IngressList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressLoadBalancerIngress {
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
        if self.hostname.is_some() {
            len += 1;
        }
        if !self.ports.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressLoadBalancerIngress", len)?;
        if let Some(v) = self.ip.as_ref() {
            struct_ser.serialize_field("ip", v)?;
        }
        if let Some(v) = self.hostname.as_ref() {
            struct_ser.serialize_field("hostname", v)?;
        }
        if !self.ports.is_empty() {
            struct_ser.serialize_field("ports", &self.ports)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressLoadBalancerIngress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ip",
            "hostname",
            "ports",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ip,
            Hostname,
            Ports,
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
                            "ip" => Ok(GeneratedField::Ip),
                            "hostname" => Ok(GeneratedField::Hostname),
                            "ports" => Ok(GeneratedField::Ports),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngressLoadBalancerIngress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressLoadBalancerIngress")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressLoadBalancerIngress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ip__ = None;
                let mut hostname__ = None;
                let mut ports__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ip => {
                            if ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            ip__ = map.next_value()?;
                        }
                        GeneratedField::Hostname => {
                            if hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            hostname__ = map.next_value()?;
                        }
                        GeneratedField::Ports => {
                            if ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ports"));
                            }
                            ports__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IngressLoadBalancerIngress {
                    ip: ip__,
                    hostname: hostname__,
                    ports: ports__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressLoadBalancerIngress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressLoadBalancerStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ingress.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressLoadBalancerStatus", len)?;
        if !self.ingress.is_empty() {
            struct_ser.serialize_field("ingress", &self.ingress)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressLoadBalancerStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ingress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ingress,
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
                            "ingress" => Ok(GeneratedField::Ingress),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngressLoadBalancerStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressLoadBalancerStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressLoadBalancerStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ingress__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ingress => {
                            if ingress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingress"));
                            }
                            ingress__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IngressLoadBalancerStatus {
                    ingress: ingress__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressLoadBalancerStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressPortStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.port.is_some() {
            len += 1;
        }
        if self.protocol.is_some() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressPortStatus", len)?;
        if let Some(v) = self.port.as_ref() {
            struct_ser.serialize_field("port", v)?;
        }
        if let Some(v) = self.protocol.as_ref() {
            struct_ser.serialize_field("protocol", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressPortStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "port",
            "protocol",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Port,
            Protocol,
            Error,
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
                            "port" => Ok(GeneratedField::Port),
                            "protocol" => Ok(GeneratedField::Protocol),
                            "error" => Ok(GeneratedField::Error),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngressPortStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressPortStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressPortStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut port__ = None;
                let mut protocol__ = None;
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IngressPortStatus {
                    port: port__,
                    protocol: protocol__,
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressPortStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.host.is_some() {
            len += 1;
        }
        if self.ingress_rule_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressRule", len)?;
        if let Some(v) = self.host.as_ref() {
            struct_ser.serialize_field("host", v)?;
        }
        if let Some(v) = self.ingress_rule_value.as_ref() {
            struct_ser.serialize_field("ingressRuleValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host",
            "ingressRuleValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Host,
            IngressRuleValue,
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
                            "host" => Ok(GeneratedField::Host),
                            "ingressRuleValue" => Ok(GeneratedField::IngressRuleValue),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngressRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressRule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host__ = None;
                let mut ingress_rule_value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = map.next_value()?;
                        }
                        GeneratedField::IngressRuleValue => {
                            if ingress_rule_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingressRuleValue"));
                            }
                            ingress_rule_value__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IngressRule {
                    host: host__,
                    ingress_rule_value: ingress_rule_value__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressRuleValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressRuleValue", len)?;
        if let Some(v) = self.http.as_ref() {
            struct_ser.serialize_field("http", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressRuleValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Http,
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
                            "http" => Ok(GeneratedField::Http),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngressRuleValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressRuleValue")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressRuleValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Http => {
                            if http__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            http__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IngressRuleValue {
                    http: http__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressRuleValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressServiceBackend {
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
        if self.port.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressServiceBackend", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.port.as_ref() {
            struct_ser.serialize_field("port", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressServiceBackend {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "port",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Port,
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
                            "port" => Ok(GeneratedField::Port),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngressServiceBackend;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressServiceBackend")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressServiceBackend, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut port__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IngressServiceBackend {
                    name: name__,
                    port: port__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressServiceBackend", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ingress_class_name.is_some() {
            len += 1;
        }
        if self.default_backend.is_some() {
            len += 1;
        }
        if !self.tls.is_empty() {
            len += 1;
        }
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressSpec", len)?;
        if let Some(v) = self.ingress_class_name.as_ref() {
            struct_ser.serialize_field("ingressClassName", v)?;
        }
        if let Some(v) = self.default_backend.as_ref() {
            struct_ser.serialize_field("defaultBackend", v)?;
        }
        if !self.tls.is_empty() {
            struct_ser.serialize_field("tls", &self.tls)?;
        }
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ingressClassName",
            "defaultBackend",
            "tls",
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IngressClassName,
            DefaultBackend,
            Tls,
            Rules,
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
                            "ingressClassName" => Ok(GeneratedField::IngressClassName),
                            "defaultBackend" => Ok(GeneratedField::DefaultBackend),
                            "tls" => Ok(GeneratedField::Tls),
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngressSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressSpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ingress_class_name__ = None;
                let mut default_backend__ = None;
                let mut tls__ = None;
                let mut rules__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IngressClassName => {
                            if ingress_class_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingressClassName"));
                            }
                            ingress_class_name__ = map.next_value()?;
                        }
                        GeneratedField::DefaultBackend => {
                            if default_backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultBackend"));
                            }
                            default_backend__ = map.next_value()?;
                        }
                        GeneratedField::Tls => {
                            if tls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tls"));
                            }
                            tls__ = Some(map.next_value()?);
                        }
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IngressSpec {
                    ingress_class_name: ingress_class_name__,
                    default_backend: default_backend__,
                    tls: tls__.unwrap_or_default(),
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.load_balancer.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressStatus", len)?;
        if let Some(v) = self.load_balancer.as_ref() {
            struct_ser.serialize_field("loadBalancer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "loadBalancer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoadBalancer,
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
                            "loadBalancer" => Ok(GeneratedField::LoadBalancer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngressStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut load_balancer__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LoadBalancer => {
                            if load_balancer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadBalancer"));
                            }
                            load_balancer__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IngressStatus {
                    load_balancer: load_balancer__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngressTls {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hosts.is_empty() {
            len += 1;
        }
        if self.secret_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.IngressTLS", len)?;
        if !self.hosts.is_empty() {
            struct_ser.serialize_field("hosts", &self.hosts)?;
        }
        if let Some(v) = self.secret_name.as_ref() {
            struct_ser.serialize_field("secretName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngressTls {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hosts",
            "secretName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hosts,
            SecretName,
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
                            "hosts" => Ok(GeneratedField::Hosts),
                            "secretName" => Ok(GeneratedField::SecretName),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngressTls;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.IngressTLS")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IngressTls, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hosts__ = None;
                let mut secret_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Hosts => {
                            if hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hosts"));
                            }
                            hosts__ = Some(map.next_value()?);
                        }
                        GeneratedField::SecretName => {
                            if secret_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secretName"));
                            }
                            secret_name__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IngressTls {
                    hosts: hosts__.unwrap_or_default(),
                    secret_name: secret_name__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.IngressTLS", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NetworkPolicy {
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
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.NetworkPolicy", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NetworkPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
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
                            "spec" => Ok(GeneratedField::Spec),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NetworkPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.NetworkPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NetworkPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(NetworkPolicy {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.NetworkPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NetworkPolicyEgressRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ports.is_empty() {
            len += 1;
        }
        if !self.to.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.NetworkPolicyEgressRule", len)?;
        if !self.ports.is_empty() {
            struct_ser.serialize_field("ports", &self.ports)?;
        }
        if !self.to.is_empty() {
            struct_ser.serialize_field("to", &self.to)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NetworkPolicyEgressRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ports",
            "to",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ports,
            To,
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
                            "ports" => Ok(GeneratedField::Ports),
                            "to" => Ok(GeneratedField::To),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NetworkPolicyEgressRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.NetworkPolicyEgressRule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NetworkPolicyEgressRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ports__ = None;
                let mut to__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ports => {
                            if ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ports"));
                            }
                            ports__ = Some(map.next_value()?);
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(NetworkPolicyEgressRule {
                    ports: ports__.unwrap_or_default(),
                    to: to__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.NetworkPolicyEgressRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NetworkPolicyIngressRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ports.is_empty() {
            len += 1;
        }
        if !self.from.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.NetworkPolicyIngressRule", len)?;
        if !self.ports.is_empty() {
            struct_ser.serialize_field("ports", &self.ports)?;
        }
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NetworkPolicyIngressRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ports",
            "from",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ports,
            From,
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
                            "ports" => Ok(GeneratedField::Ports),
                            "from" => Ok(GeneratedField::From),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NetworkPolicyIngressRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.NetworkPolicyIngressRule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NetworkPolicyIngressRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ports__ = None;
                let mut from__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ports => {
                            if ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ports"));
                            }
                            ports__ = Some(map.next_value()?);
                        }
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(NetworkPolicyIngressRule {
                    ports: ports__.unwrap_or_default(),
                    from: from__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.NetworkPolicyIngressRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NetworkPolicyList {
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
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.NetworkPolicyList", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NetworkPolicyList {
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
            type Value = NetworkPolicyList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.NetworkPolicyList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NetworkPolicyList, V::Error>
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
                Ok(NetworkPolicyList {
                    metadata: metadata__,
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.NetworkPolicyList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NetworkPolicyPeer {
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
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.NetworkPolicyPeer", len)?;
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
impl<'de> serde::Deserialize<'de> for NetworkPolicyPeer {
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
                            "podSelector" => Ok(GeneratedField::PodSelector),
                            "namespaceSelector" => Ok(GeneratedField::NamespaceSelector),
                            "ipBlock" => Ok(GeneratedField::IpBlock),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NetworkPolicyPeer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.NetworkPolicyPeer")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NetworkPolicyPeer, V::Error>
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(NetworkPolicyPeer {
                    pod_selector: pod_selector__,
                    namespace_selector: namespace_selector__,
                    ip_block: ip_block__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.NetworkPolicyPeer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NetworkPolicyPort {
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
        if self.port.is_some() {
            len += 1;
        }
        if self.end_port.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.NetworkPolicyPort", len)?;
        if let Some(v) = self.protocol.as_ref() {
            struct_ser.serialize_field("protocol", v)?;
        }
        if let Some(v) = self.port.as_ref() {
            struct_ser.serialize_field("port", v)?;
        }
        if let Some(v) = self.end_port.as_ref() {
            struct_ser.serialize_field("endPort", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NetworkPolicyPort {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protocol",
            "port",
            "endPort",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Protocol,
            Port,
            EndPort,
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
                            "protocol" => Ok(GeneratedField::Protocol),
                            "port" => Ok(GeneratedField::Port),
                            "endPort" => Ok(GeneratedField::EndPort),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NetworkPolicyPort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.NetworkPolicyPort")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NetworkPolicyPort, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protocol__ = None;
                let mut port__ = None;
                let mut end_port__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = map.next_value()?;
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = map.next_value()?;
                        }
                        GeneratedField::EndPort => {
                            if end_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endPort"));
                            }
                            end_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(NetworkPolicyPort {
                    protocol: protocol__,
                    port: port__,
                    end_port: end_port__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.NetworkPolicyPort", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NetworkPolicySpec {
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
        if !self.ingress.is_empty() {
            len += 1;
        }
        if !self.egress.is_empty() {
            len += 1;
        }
        if !self.policy_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.NetworkPolicySpec", len)?;
        if let Some(v) = self.pod_selector.as_ref() {
            struct_ser.serialize_field("podSelector", v)?;
        }
        if !self.ingress.is_empty() {
            struct_ser.serialize_field("ingress", &self.ingress)?;
        }
        if !self.egress.is_empty() {
            struct_ser.serialize_field("egress", &self.egress)?;
        }
        if !self.policy_types.is_empty() {
            struct_ser.serialize_field("policyTypes", &self.policy_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NetworkPolicySpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "podSelector",
            "ingress",
            "egress",
            "policyTypes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PodSelector,
            Ingress,
            Egress,
            PolicyTypes,
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
                            "podSelector" => Ok(GeneratedField::PodSelector),
                            "ingress" => Ok(GeneratedField::Ingress),
                            "egress" => Ok(GeneratedField::Egress),
                            "policyTypes" => Ok(GeneratedField::PolicyTypes),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NetworkPolicySpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.NetworkPolicySpec")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NetworkPolicySpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pod_selector__ = None;
                let mut ingress__ = None;
                let mut egress__ = None;
                let mut policy_types__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PodSelector => {
                            if pod_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("podSelector"));
                            }
                            pod_selector__ = map.next_value()?;
                        }
                        GeneratedField::Ingress => {
                            if ingress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingress"));
                            }
                            ingress__ = Some(map.next_value()?);
                        }
                        GeneratedField::Egress => {
                            if egress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("egress"));
                            }
                            egress__ = Some(map.next_value()?);
                        }
                        GeneratedField::PolicyTypes => {
                            if policy_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyTypes"));
                            }
                            policy_types__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(NetworkPolicySpec {
                    pod_selector: pod_selector__,
                    ingress: ingress__.unwrap_or_default(),
                    egress: egress__.unwrap_or_default(),
                    policy_types: policy_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.NetworkPolicySpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceBackendPort {
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
        if self.number.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.api.networking.v1.ServiceBackendPort", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.number.as_ref() {
            struct_ser.serialize_field("number", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceBackendPort {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "number",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Number,
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
                            "number" => Ok(GeneratedField::Number),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceBackendPort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.api.networking.v1.ServiceBackendPort")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceBackendPort, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut number__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ServiceBackendPort {
                    name: name__,
                    number: number__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.api.networking.v1.ServiceBackendPort", FIELDS, GeneratedVisitor)
    }
}
