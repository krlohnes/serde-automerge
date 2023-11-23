use super::{Deserializer as ValueDeserializer, Error};
use automerge::{
    iter::{ListRange, ListRangeItem},
    ObjId, ReadDoc,
};
use serde::de::{self};
use std::ops::RangeFull;

pub struct SeqDeserializer<'a, Rx: ReadDoc> {
    doc: &'a Rx,
    values: ListRange<'a, RangeFull>,
}

impl<'a, Rx: ReadDoc> SeqDeserializer<'a, Rx> {
    pub fn new(doc: &'a Rx, id: ObjId) -> Self {
        Self {
            doc,
            values: doc.list_range(id, ..),
        }
    }
}

impl<'de, 'a, Rx: ReadDoc> de::SeqAccess<'de> for SeqDeserializer<'a, Rx> {
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
