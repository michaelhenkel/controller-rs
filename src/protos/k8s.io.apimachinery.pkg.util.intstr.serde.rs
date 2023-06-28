impl serde::Serialize for IntOrString {
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
        if self.int_val.is_some() {
            len += 1;
        }
        if self.str_val.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("k8s.io.apimachinery.pkg.util.intstr.IntOrString", len)?;
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.int_val.as_ref() {
            struct_ser.serialize_field("intVal", v)?;
        }
        if let Some(v) = self.str_val.as_ref() {
            struct_ser.serialize_field("strVal", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntOrString {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "intVal",
            "strVal",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            IntVal,
            StrVal,
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
                            "intVal" => Ok(GeneratedField::IntVal),
                            "strVal" => Ok(GeneratedField::StrVal),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntOrString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct k8s.io.apimachinery.pkg.util.intstr.IntOrString")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IntOrString, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut int_val__ = None;
                let mut str_val__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IntVal => {
                            if int_val__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intVal"));
                            }
                            int_val__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::StrVal => {
                            if str_val__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strVal"));
                            }
                            str_val__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IntOrString {
                    r#type: r#type__,
                    int_val: int_val__,
                    str_val: str_val__,
                })
            }
        }
        deserializer.deserialize_struct("k8s.io.apimachinery.pkg.util.intstr.IntOrString", FIELDS, GeneratedVisitor)
    }
}
