use serde::{de::Error as SerdeError, Deserialize, Deserializer};

pub trait CustomDeserializable: Sized {
    fn from_str(input: &str) -> Result<Self, String>;
}

pub fn custom_deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: CustomDeserializable,
{
    let input: String = Deserialize::deserialize(deserializer)?;
    T::from_str(&input).map_err(SerdeError::custom)
}
