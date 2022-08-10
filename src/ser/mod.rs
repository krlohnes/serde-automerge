use crate::{Error, JasperDoc};
use automerge::{transaction::Transactable, ObjId};
use serde::{ser, Serialize, Serializer};

pub struct SerializeSeq;
pub struct SerializeTable<'a, 'b, Tx: Transactable> {
    ser: &'b mut JasperDoc<'a, Tx>,
    parent: ObjId,
}

impl<'a, 'b, Tx: Transactable> ser::SerializeMap for SerializeTable<'a, 'b, Tx> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, input: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn end(self) -> Result<(), Error> {
        Err(Error::UnsupportedType)
    }
}

impl<'a, 'b, Tx: Transactable> ser::SerializeStruct for SerializeTable<'a, 'b, Tx> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        value.serialize(&mut JasperDoc {
            stuff: std::marker::PhantomData,
            doc: self.ser.doc,
            key: Some(key),
            parent: self.parent.clone(),
        });
        Ok(())
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

impl ser::SerializeSeq for SerializeSeq {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn end(self) -> Result<(), Error> {
        Err(Error::UnsupportedType)
    }
}

impl ser::SerializeTuple for SerializeSeq {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleVariant for SerializeSeq {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SerializeSeq {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, 'b, Tx: Transactable> Serializer for &'b mut JasperDoc<'a, Tx> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeSeq;
    type SerializeTuple = SerializeSeq;
    type SerializeTupleStruct = SerializeSeq;
    type SerializeTupleVariant = SerializeSeq;
    type SerializeMap = SerializeTable<'a, 'b, Tx>;
    type SerializeStruct = SerializeTable<'a, 'b, Tx>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.doc.put(
            &self.parent,
            self.key.expect("Must have `name` to store value to"),
            v as f64,
        );
        Ok(())
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::UnsupportedType)
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::UnsupportedType)
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::UnsupportedType)
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::UnsupportedType)
    }
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let parent = self.parent.clone();
        Ok(SerializeTable { ser: self, parent })
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::UnsupportedType)
    }
}
