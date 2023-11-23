use super::{Deserializer as ValueDeserializer, Error};
use automerge::{
    iter::{MapRange, MapRangeItem},
    ObjId, ReadDoc, Value,
};
use serde::de::{self, IntoDeserializer};
use std::ops::RangeFull;

pub struct MapDeserializer<'a, Rx: ReadDoc> {
    doc: &'a Rx,
    values: MapRange<'a, RangeFull>,
    current: Option<(Value<'a>, ObjId)>,
}

impl<'a, Rx: ReadDoc> MapDeserializer<'a, Rx> {
    pub fn new(doc: &'a Rx, id: ObjId) -> Self {
        Self {
            doc,
            values: doc.map_range(id, ..),
            current: None,
        }
    }
    pub fn new_root(doc: &'a Rx) -> Self {
        Self::new(doc, ObjId::Root)
    }
}

impl<'a, Rx: ReadDoc> From<&'a Rx> for MapDeserializer<'a, Rx> {
    fn from(doc: &'a Rx) -> Self {
        Self::new_root(doc)
    }
}

impl<'de, 'a, Rx: ReadDoc> de::MapAccess<'de> for MapDeserializer<'a, Rx> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some(MapRangeItem {
            key,
            value,
            id,
            conflict: _,
        }) = self.values.next()
        {
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
        let (value, id) = self
            .current
            .take()
            .expect("next_value_seed called before next_key_seed");
        seed.deserialize(ValueDeserializer::new_found(self.doc, value, id))
    }
}
