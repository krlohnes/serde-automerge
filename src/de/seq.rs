use super::{Deserializer as ValueDeserializer, Error};
use automerge::{
    iter::{ListRange, ListRangeItem},
    Automerge, ObjId, ReadDoc as _,
};
use serde::de::{self};
use std::ops::RangeFull;

pub struct SeqDeserializer<'a> {
    doc: &'a Automerge,
    values: ListRange<'a, RangeFull>,
}

impl<'a> SeqDeserializer<'a> {
    pub fn new(doc: &'a Automerge, id: ObjId) -> Self {
        Self {
            doc,
            values: doc.list_range(id, ..),
        }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for SeqDeserializer<'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(ListRangeItem { value, id, .. }) = self.values.next() {
            seed.deserialize(ValueDeserializer::new_found(self.doc, value, id))
                .map(Some)
        } else {
            Ok(None)
        }
    }
}
