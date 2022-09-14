macro_rules! define_time_format {
    ($time:ident,$format:literal) => {
        pub mod $time {
            use chrono::{DateTime, TimeZone, Utc};
            use serde::{Deserialize, Deserializer, Serializer};

            const FORMAT: &str = $format;


    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
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

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom).map(Some)
    }
}
    };
}
define_time_format!(standard_time, "%Y%m%d%H%M%S");
define_time_format!(snapshot_time, "%Y%m%d.%H%M%S");
