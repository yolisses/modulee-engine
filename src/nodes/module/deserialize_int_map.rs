use nohash_hasher::BuildNoHashHasher;
use serde::de::{self, Visitor};
use serde::Deserializer;
use std::collections::HashMap;
use std::fmt;

type IntMap<K, V> = HashMap<K, V, BuildNoHashHasher<K>>;

pub(crate) fn deserialize_int_map<'de, D>(deserializer: D) -> Result<IntMap<usize, usize>, D::Error>
where
    D: Deserializer<'de>,
{
    // Define a visitor to handle the conversion from string keys to usize
    struct IntMapVisitor;

    impl<'de> Visitor<'de> for IntMapVisitor {
        type Value = IntMap<usize, usize>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with string keys representing usize values")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let mut map = HashMap::with_hasher(BuildNoHashHasher::default());

            // Iterate over the key-value pairs in the JSON object
            while let Some((key, value)) = access.next_entry::<String, usize>()? {
                // Parse the string key into a usize
                let parsed_key = key.parse::<usize>().map_err(de::Error::custom)?;
                map.insert(parsed_key, value);
            }

            Ok(map)
        }
    }

    // Use the visitor to deserialize the map
    deserializer.deserialize_map(IntMapVisitor)
}
