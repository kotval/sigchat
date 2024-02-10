// TODO: refactor this. Inappropriate use of modules

// Signal sometimes adds padding, sometimes it does not.
// This requires a custom decoding engine.
// This engine is as general as possible.
pub const BASE64_RELAXED: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
    &base64::alphabet::STANDARD,
    base64::engine::GeneralPurposeConfig::new()
        .with_encode_padding(true)
        .with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent),
);

pub mod serde_base64 {
    use base64::prelude::*;
    use serde::{Deserialize, Deserializer, Serializer};

    use super::BASE64_RELAXED;

    pub fn serialize<T, S>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8]>,
        S: Serializer,
    {
        serializer.serialize_str(&BASE64_RELAXED.encode(bytes.as_ref()))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        String::deserialize(deserializer)
            .and_then(|string| BASE64_RELAXED.decode(string).map_err(|err| Error::custom(err.to_string())))
    }
}

pub mod serde_optional_base64 {
    use base64::prelude::*;
    use serde::{Deserialize, Deserializer, Serializer};

    use super::serde_base64;
    use super::BASE64_RELAXED;

    pub fn serialize<T, S>(bytes: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8]>,
        S: Serializer,
    {
        match bytes {
            Some(bytes) => serde_base64::serialize(bytes, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        match Option::<String>::deserialize(deserializer)? {
            Some(s) => BASE64_RELAXED.decode(s).map_err(|err| Error::custom(err.to_string())).map(Some),
            None => Ok(None),
        }
    }
}
