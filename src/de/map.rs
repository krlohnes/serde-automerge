use super::{Deserializer as ValueDeserializer, Error};
use automerge::{Automerge, MapRange, ObjId, Value};
use serde::de::{self, IntoDeserializer};
use std::ops::RangeFull;

pub struct MapDeserializer<'a> {
    doc: &'a Automerge,
    values: MapRange<'a, RangeFull>,
    current: Option<(Value<'a>, ObjId)>,
}

impl<'a> MapDeserializer<'a> {
    pub fn new(doc: &'a Automerge, id: ObjId) -> Self {
        Self {
            doc,
            values: doc.map_range(id, ..),
            current: None,
        }
    }
}

impl<'de, 'a> de::MapAccess<'de> for MapDeserializer<'a> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some((key, value, id)) = self.values.next() {
            self.current = Some((value, id));
            seed.deserialize(key.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let (value, id) = self.current.take().unwrap();
        seed.deserialize(ValueDeserializer::new_found(self.doc, value, id))
    }
}
