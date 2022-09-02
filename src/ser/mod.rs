use automerge::{transaction::Transactable, AutomergeError, ObjId, ObjType, Prop, ScalarValue};
use serde::{
    ser::{self},
    serde_if_integer128,
};

// TODO: Add inline definitions where possible

mod error;
mod key;
mod map;
mod seq;
pub use error::*;
pub use key::*;
pub use map::MapSerializer;
pub use map::*;
pub use seq::*;

pub struct Serializer<'a, Tx: Transactable> {
    tx: &'a mut Tx,
    obj: ObjId,
    prop: Prop,
}

impl<'a, Tx: Transactable> Serializer<'a, Tx> {
    pub fn new<P: Into<Prop>>(tx: &'a mut Tx, obj: ObjId, prop: P) -> Self {
        Self {
            tx,
            obj,
            prop: prop.into(),
        }
    }
    pub fn new_root<P: Into<Prop>>(tx: &'a mut Tx, prop: P) -> Self {
        Self::new(tx, ObjId::Root, prop)
    }
    fn put<V: Into<ScalarValue>>(self, value: V) -> Result<(), AutomergeError> {
        self.tx.put(&self.obj, self.prop, value)
    }
    fn put_object(self, value: ObjType) -> Result<(&'a mut Tx, ObjId), AutomergeError> {
        let obj = self.tx.put_object(&self.obj, self.prop, value)?;
        Ok((self.tx, obj))
    }
    fn put_variant(self, variant: &'static str) -> Result<Self, AutomergeError> {
        let (tx, obj) = self.put_object(ObjType::Map)?;
        Ok(Self::new(tx, obj, variant))
    }
}

macro_rules! serialize_put {
    ($method:ident, $type:ty$( as $as:ty)?) => {
        fn $method(self, v: $type) -> Result<(), Self::Error> {
            Ok(self.put(v$(as $as)?)?)
        }
    };
}

impl<'a, Tx: Transactable> ser::Serializer for Serializer<'a, Tx> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, Tx>;
    type SerializeTuple = SeqSerializer<'a, Tx>;
    type SerializeTupleStruct = SeqSerializer<'a, Tx>;
    type SerializeTupleVariant = SeqSerializer<'a, Tx>;
    type SerializeMap = MapSerializer<'a, Tx>;
    type SerializeStruct = MapSerializer<'a, Tx>;
    type SerializeStructVariant = MapSerializer<'a, Tx>;

    serialize_put!(serialize_bool, bool);

    serialize_put!(serialize_i8, i8 as i64);
    serialize_put!(serialize_i16, i16 as i64);
    serialize_put!(serialize_i32, i32 as i64);
    serialize_put!(serialize_i64, i64);

    serialize_put!(serialize_u8, u8 as u64);
    serialize_put!(serialize_u16, u16 as u64);
    serialize_put!(serialize_u32, u32 as u64);
    serialize_put!(serialize_u64, u64);

    serde_if_integer128! {
        serialize_put!(serialize_i128, i128 as i64);
        serialize_put!(serialize_u128, u128 as u64);
    }

    serialize_put!(serialize_f32, f32 as f64);
    serialize_put!(serialize_f64, f64);

    serialize_put!(serialize_char, char);
    serialize_put!(serialize_str, &str);

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(self.put(v.to_owned())?)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.put(())?)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        self.put_variant(variant)?
            .serialize_newtype_struct(name, value)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let (tx, obj) = self.put_object(ObjType::List)?;
        Ok(SeqSerializer::new(tx, obj))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.put_variant(variant)?.serialize_tuple_struct(name, len)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let (tx, obj) = self.put_object(ObjType::Map)?;
        Ok(MapSerializer::new(tx, obj))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.put_variant(variant)?.serialize_struct(name, len)
    }
}
