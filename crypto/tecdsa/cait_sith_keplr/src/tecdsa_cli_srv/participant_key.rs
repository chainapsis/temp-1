use serde::ser::SerializeMap;
use serde::{Deserializer, Serializer};
use std::collections::HashMap;
use std::str::FromStr;

pub fn serialize<S, K, V>(map: &HashMap<K, V>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    K: std::fmt::Display,
    V: serde::Serialize,
{
    let mut state = ser.serialize_map(Some(map.len()))?;
    for (k, v) in map {
        state.serialize_entry(&k.to_string(), v)?;
    }
    state.end()
}

pub fn deserialize<'de, D, K, V>(de: D) -> Result<HashMap<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: FromStr + Eq + std::hash::Hash,
    K::Err: std::fmt::Display,
    V: serde::Deserialize<'de>,
{
    use serde::de::{Error, MapAccess, Visitor};
    use std::fmt;
    use std::marker::PhantomData;

    struct MapVisitor<K, V> {
        marker: PhantomData<HashMap<K, V>>,
    }

    impl<'de, K, V> Visitor<'de> for MapVisitor<K, V>
    where
        K: FromStr + Eq + std::hash::Hash,
        K::Err: std::fmt::Display,
        V: serde::Deserialize<'de>,
    {
        type Value = HashMap<K, V>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of string keys to values")
        }

        fn visit_map<M>(self, mut access: M) -> Result<HashMap<K, V>, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = HashMap::new();
            while let Some((k_str, value)) = access.next_entry::<String, V>()? {
                let parsed_key = k_str.parse::<K>().map_err(Error::custom)?;
                map.insert(parsed_key, value);
            }
            Ok(map)
        }
    }

    let visitor = MapVisitor {
        marker: PhantomData,
    };
    de.deserialize_map(visitor)
}
