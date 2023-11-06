use super::{Deserializer as ValueDeserializer, Error};
use automerge::{
    iter::{MapRange, MapRangeItem},
    Automerge, ObjId, ReadDoc as _, Value,
};
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
    pub fn new_root(doc: &'a Automerge) -> Self {
        Self::new(doc, ObjId::Root)
    }
}

impl<'a> From<&'a Automerge> for MapDeserializer<'a> {
    fn from(doc: &'a Automerge) -> Self {
        Self::new_root(doc)
    }
}

impl<'de, 'a> de::MapAccess<'de> for MapDeserializer<'a> {
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
