use super::{Error, KeySerializer, Serializer};
use automerge::{transaction::Transactable, ObjId};
use serde::ser;

pub struct MapSerializer<'a, Tx: Transactable> {
    tx: &'a mut Tx,
    obj: ObjId,
    next_key: Option<String>,
}

impl<'a, Tx: Transactable> MapSerializer<'a, Tx> {
    pub fn new(tx: &'a mut Tx, obj: ObjId) -> Self {
        Self {
            tx,
            obj,
            next_key: None,
        }
    }
    pub fn new_root(tx: &'a mut Tx) -> Self {
        Self::new(tx, ObjId::Root)
    }
}

impl<'a, Tx: Transactable> ser::SerializeMap for MapSerializer<'a, Tx> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.next_key = Some(key.serialize(KeySerializer)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let key = self
            .next_key
            .take()
            .expect("serialize_value called before serialize_key");
        value.serialize(Serializer::new(self.tx, self.obj.clone(), key))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, Tx: Transactable> ser::SerializeStruct for MapSerializer<'a, Tx> {
    type Ok = <Self as ser::SerializeMap>::Ok;
    type Error = <Self as ser::SerializeMap>::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeMap::end(self)
    }
}

impl<'a, Tx: Transactable> ser::SerializeStructVariant for MapSerializer<'a, Tx> {
    type Ok = <Self as ser::SerializeStruct>::Ok;
    type Error = <Self as ser::SerializeStruct>::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeStruct::end(self)
    }
}
