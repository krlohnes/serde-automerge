use super::Serializer;
use automerge::{transaction::Transactable, ObjId, ScalarValue};
use serde::ser;

pub struct SeqSerializer<'a, Tx: Transactable> {
    tx: &'a mut Tx,
    obj: ObjId,
    id: usize,
}

impl<'a, Tx: Transactable> SeqSerializer<'a, Tx> {
    pub fn new(tx: &'a mut Tx, obj: ObjId) -> Self {
        Self { tx, obj, id: 0 }
    }
}

impl<'a, Tx: Transactable> ser::SerializeSeq for SeqSerializer<'a, Tx> {
    type Ok = <Serializer<'a, Tx> as ser::Serializer>::Ok;
    type Error = <Serializer<'a, Tx> as ser::Serializer>::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        if self.id == self.tx.length(&self.obj) {
            self.tx.insert(&self.obj, self.id, ScalarValue::Null)?;
        }
        value.serialize(Serializer::new(self.tx, self.obj.clone(), self.id))?;
        self.id += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok((self.tx, self.obj))
    }
}

impl<'a, Tx: Transactable> ser::SerializeTuple for SeqSerializer<'a, Tx> {
    type Ok = <Self as ser::SerializeSeq>::Ok;
    type Error = <Self as ser::SerializeSeq>::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, Tx: Transactable> ser::SerializeTupleStruct for SeqSerializer<'a, Tx> {
    type Ok = <Self as ser::SerializeTuple>::Ok;
    type Error = <Self as ser::SerializeTuple>::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        ser::SerializeTuple::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeTuple::end(self)
    }
}

impl<'a, Tx: Transactable> ser::SerializeTupleVariant for SeqSerializer<'a, Tx> {
    type Ok = <Self as ser::SerializeTupleStruct>::Ok;
    type Error = <Self as ser::SerializeTupleStruct>::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        ser::SerializeTupleStruct::serialize_field(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeTupleStruct::end(self)
    }
}
