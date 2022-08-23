use super::{Error, Serializer};
use automerge::{transaction::Transactable, ObjId};
use serde::ser::{self, Impossible};

pub struct SeqSerializer<'a, Tx: Transactable> {
    tx: &'a Tx,
    obj: ObjId,
    id: usize,
}

impl<'a, Tx: Transactable> SeqSerializer<'a, Tx> {
    pub fn new(tx: &'a Tx, obj: ObjId) -> Self {
        Self { tx, obj, id: 0 }
    }
}

impl<'a, Tx: Transactable> ser::SerializeSeq for SeqSerializer<'a, Tx> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(Serializer::new(self.tx, self.obj, self.id))?;
        self.id += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
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

pub struct MapSerializer<'a, Tx: Transactable> {
    tx: &'a Tx,
    obj: ObjId,
    next_key: Option<String>,
}

impl<'a, Tx: Transactable> MapSerializer<'a, Tx> {
    pub fn new(tx: &'a Tx, obj: ObjId) -> Self {
        Self {
            tx,
            obj,
            next_key: None,
        }
    }
}

impl<'a, Tx: Transactable> ser::SerializeMap for MapSerializer<'a, Tx> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        // TODO: Serialize the key and set that to self
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let key = self
            .next_key
            .take()
            .expect("serialize_value called before serialize_key");
        value.serialize(Serializer::new(self.tx, self.obj, key))?;
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

struct KeySerializer;

macro_rules! serialize_to_string {
    ($method:ident, $type:ty) => {
        fn $method(self, value: $type) -> Result<String, Error> {
            Ok(value.to_string())
        }
    };
}

macro_rules! serialize_to_string_error {
    ($method:ident$(, $type:ty)*) => {
        fn $method(self$(, _: $type)*) -> Result<Self::Ok, Self::Error> {
            Err(Error::KeysMustBeAString)
        }
    };
}
macro_rules! serialize_to_string_error_imp {
    ($method:ident$(, $type:ty)*) => {
        fn $method(self$(, _: $type)*) -> Result<Impossible<String, Error>, Self::Error> {
            Err(Error::KeysMustBeAString)
        }
    };
}
macro_rules! serialize_to_string_error_T {
    ($method:ident$(, $type:ty)*) => {
        fn $method<T: ?Sized + ser::Serialize>(self$(, _: $type)*, _: &T) -> Result<String, Error> {
            Err(Error::KeysMustBeAString)
        }
    };
}

impl serde::Serializer for KeySerializer {
    type Ok = String;
    type Error = Error;

    type SerializeSeq = Impossible<String, Error>;
    type SerializeTuple = Impossible<String, Error>;
    type SerializeTupleStruct = Impossible<String, Error>;
    type SerializeTupleVariant = Impossible<String, Error>;
    type SerializeMap = Impossible<String, Error>;
    type SerializeStruct = Impossible<String, Error>;
    type SerializeStructVariant = Impossible<String, Error>;

    // TODO: serde-json throws an error, instead of changing them into a string, on the following types:
    // bool, f32, f64, bytes

    serialize_to_string!(serialize_bool, bool);

    serialize_to_string!(serialize_f32, f32);
    serialize_to_string!(serialize_f64, f64);

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(String::from_utf8_lossy(v).to_string())
    }

    serialize_to_string!(serialize_i8, i8);
    serialize_to_string!(serialize_i16, i16);
    serialize_to_string!(serialize_i32, i32);
    serialize_to_string!(serialize_i64, i64);

    serialize_to_string!(serialize_u8, u8);
    serialize_to_string!(serialize_u16, u16);
    serialize_to_string!(serialize_u32, u32);
    serialize_to_string!(serialize_u64, u64);

    serialize_to_string!(serialize_char, char);
    serialize_to_string!(serialize_str, &str);

    serialize_to_string_error!(serialize_none);
    serialize_to_string_error!(serialize_unit);
    serialize_to_string_error!(serialize_unit_struct, &'static str);
    serialize_to_string_error!(serialize_unit_variant, &'static str, u32, &'static str);

    serialize_to_string_error_T!(serialize_some);
    serialize_to_string_error_T!(serialize_newtype_struct, &'static str);
    serialize_to_string_error_T!(serialize_newtype_variant, &'static str, u32, &'static str);

    serialize_to_string_error_imp!(serialize_seq, Option<usize>);
    serialize_to_string_error_imp!(serialize_tuple, usize);
    serialize_to_string_error_imp!(serialize_tuple_struct, &'static str, usize);
    serialize_to_string_error_imp!(serialize_map, Option<usize>);
    serialize_to_string_error_imp!(serialize_struct, &'static str, usize);

    serialize_to_string_error_imp!(
        serialize_tuple_variant,
        &'static str,
        u32,
        &'static str,
        usize
    );
    serialize_to_string_error_imp!(
        serialize_struct_variant,
        &'static str,
        u32,
        &'static str,
        usize
    );
}
