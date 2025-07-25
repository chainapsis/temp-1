use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::protocol::Participant;

// Serialize HashMap<Participant, V> to object with string keys
pub fn serialize<S, V>(map: &HashMap<Participant, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    V: Serialize,
{
    let mut map_serializer = serializer.serialize_map(Some(map.len()))?;

    for (key, value) in map {
        // Convert Participant to u32, then to string
        let key_u32: u32 = (*key).into();
        let key_str = key_u32.to_string();
        map_serializer.serialize_entry(&key_str, value)?;
    }

    map_serializer.end()
}

// Deserialize object with string keys to HashMap<Participant, V>
pub fn deserialize<'de, D, V>(deserializer: D) -> Result<HashMap<Participant, V>, D::Error>
where
    D: Deserializer<'de>,
    V: Deserialize<'de>,
{
    struct ParticipantMapVisitor<V> {
        marker: PhantomData<V>,
    }

    impl<'de, V> Visitor<'de> for ParticipantMapVisitor<V>
    where
        V: Deserialize<'de>,
    {
        type Value = HashMap<Participant, V>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with string keys")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));

            while let Some((key_str, value)) = access.next_entry::<String, V>()? {
                // Convert string to u32, then to Participant
                let participant_id = u32::from_str(&key_str).map_err(serde::de::Error::custom)?;

                map.insert(Participant::from(participant_id), value);
            }

            Ok(map)
        }
    }

    deserializer.deserialize_map(ParticipantMapVisitor {
        marker: PhantomData,
    })
}
