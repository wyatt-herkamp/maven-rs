macro_rules! define_time_format {
    (   $(#[$docs:meta])*
        $time:ident($format:literal)
    ) => {
        $(#[$docs])*
        pub mod $time {
            use chrono::NaiveDateTime;
            use serde::{Deserialize, Deserializer, Serializer};

            static FORMAT: &str = $format;

            pub fn serialize<S>(
                date: &Option<NaiveDateTime>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                if let Some(date) = date {
                    let s = format!("{}", date.format(FORMAT));
                    serializer.serialize_str(&s)
                } else {
                    serializer.serialize_none()
                }
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                NaiveDateTime::parse_from_str(&s, FORMAT)
                    .map_err(|err| {
                        serde::de::Error::custom(format!("Failed to parse date: {} {} ", s, err))
                    })
                    .map(Some)
            }
        }
    };
}
define_time_format!(
    /// Standard time format
    ///
    /// Format: `%Y%m%d%H%M%S`
    standard_time("%Y%m%d%H%M%S")
);
define_time_format!(
    /// Snapshot time format
    ///
    /// Format: `%Y%m%d.%H%M%S`
    snapshot_time("%Y%m%d.%H%M%S")
);
