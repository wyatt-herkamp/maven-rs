macro_rules! serde_via_string_types {
    (
        $type:ty
    ) => {
        impl serde::Serialize for $type {
            fn serialize<S>(
                &self,
                serializer: S,
            ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
            where
                S: serde::Serializer,
            {
                self.to_string().serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Self::from_str(&s).map_err(serde::de::Error::custom)
            }
        }
    };
}
pub(crate) use serde_via_string_types;
