use std::{iter::Peekable, ops::RangeFull};

use crate::error::Error;
use automerge::{
    transaction::Transactable, Automerge, MapRange, ObjId, ObjType, Prop, ScalarValue, Value,
};
use serde::de::{self};

fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Object(t) => match t {
            ObjType::Map => "map",
            ObjType::Table => "table",
            ObjType::List => "list",
            ObjType::Text => "text",
        },
        Value::Scalar(s) => match s.as_ref() {
            ScalarValue::Bytes(_) => "byte buffer",
            ScalarValue::Str(_) => "string",
            ScalarValue::Int(_) => "integer number",
            ScalarValue::Uint(_) => "unsigned integer number",
            ScalarValue::F64(_) => "floating point number",
            ScalarValue::Counter(_) => "counter",
            ScalarValue::Timestamp(_) => "timestamp",
            ScalarValue::Boolean(_) => "boolean",
            ScalarValue::Null => "null",
        },
    }
}

pub struct Deserializer<'a, 'i> {
    doc: &'a Automerge,
    key: (&'i ObjId, Prop),
}

impl<'a, 'i> Deserializer<'a, 'i> {
    pub fn into_value(self) -> Result<Option<ValueWrap<'a>>, Error> {
        Ok(self
            .doc
            .get(&self.key.0, self.key.1)
            .map_err(|e| Error::Custom(e.to_string()))?
            .map(|(v, k)| ValueWrap::new(self.doc, v, k)))
    }
}

struct ValueWrap<'a> {
    doc: &'a Automerge,
    value: Value<'a>,
    id: ObjId,
}

impl<'a> ValueWrap<'a> {
    pub fn new(doc: &'a Automerge, value: Value<'a>, id: ObjId) -> Self {
        Self { doc, value, id }
    }
    pub fn type_name(&self) -> &'static str {
        value_type_name(&self.value)
    }
    pub fn into_byte_buff(self) -> Result<Vec<u8>, Error> {
        self.value
            .into_bytes()
            .map_err(|v| Error::ExpectedBoolean(value_type_name(&v)))
    }
    pub fn to_bytes(&self) -> Result<&[u8], Error> {
        self.value
            .to_bytes()
            .ok_or_else(|| Error::ExpectedBoolean(self.type_name()))
    }
    pub fn into_string(self) -> Result<String, Error> {
        self.value
            .into_string()
            .map_err(|v| Error::ExpectedString(value_type_name(&v)))
    }
    pub fn to_str(&self) -> Result<&str, Error> {
        self.value
            .to_str()
            .ok_or_else(|| Error::ExpectedString(self.type_name()))
    }
    pub fn to_i64(&self) -> Result<i64, Error> {
        self.value
            .to_i64()
            .ok_or_else(|| Error::ExpectedInteger(self.type_name()))
    }
    pub fn to_u64(&self) -> Result<u64, Error> {
        self.value
            .to_u64()
            .ok_or_else(|| Error::ExpectedInteger(self.type_name()))
    }
    pub fn to_f64(&self) -> Result<f64, Error> {
        self.value
            .to_f64()
            .ok_or_else(|| Error::ExpectedFloat(self.type_name()))
    }
    pub fn to_bool(&self) -> Result<bool, Error> {
        self.value
            .to_bool()
            .ok_or_else(|| Error::ExpectedBoolean(self.type_name()))
    }
    pub fn is_null(&self) -> bool {
        self.value.is_null()
    }
    pub fn to_null(&self) -> Result<(), Error> {
        self.is_null()
            .then_some(())
            .ok_or_else(|| Error::ExpectedNull(self.type_name()))
    }
}

struct MapValueWrap<'a> {
    doc: &'a Automerge,
    values: Peekable<MapRange<'a, RangeFull>>,
}

impl<'a> MapValueWrap<'a> {
    pub fn new(v: ValueWrap<'a>) -> Self {
        Self {
            doc: v.doc,
            values: v.doc.map_range(v.id, ..).peekable(),
        }
    }
}

impl<'de, 'a> de::MapAccess<'de> for MapValueWrap<'a> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some((key, _, _)) = self.values.peek() {
            unimplemented!()
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let (_, value, id) = self.values.next().unwrap();
        seed.deserialize(ValueWrap::new(self.doc, value, id))
    }
}

impl<'de> de::Deserializer<'de> for ValueWrap<'_> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match &self.value {
            Value::Object(t) => match t {
                ObjType::Map => self.deserialize_map(visitor),
                ObjType::Table => unimplemented!(),
                ObjType::List => self.deserialize_seq(visitor),
                ObjType::Text => unimplemented!(),
            },
            Value::Scalar(s) => match s.as_ref() {
                ScalarValue::Bytes(_) => self.deserialize_byte_buf(visitor),
                ScalarValue::Str(_) => self.deserialize_str(visitor),
                ScalarValue::Int(_) => self.deserialize_i64(visitor),
                ScalarValue::Uint(_) => self.deserialize_u64(visitor),
                ScalarValue::F64(_) => self.deserialize_f64(visitor),
                ScalarValue::Counter(_) => self.deserialize_i64(visitor),
                ScalarValue::Timestamp(_) => self.deserialize_i64(visitor),
                ScalarValue::Boolean(_) => self.deserialize_bool(visitor),
                ScalarValue::Null => self.deserialize_unit(visitor),
            },
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bool(self.to_bool()?)
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.to_i64()?)
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.to_u64()?)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.to_f64()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_str(self.to_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.into_string()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bytes(self.to_bytes()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.into_byte_buff()?)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.is_null() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.to_null()?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.value
            .to_objtype()
            .map(|t| t == ObjType::Map)
            .unwrap_or(false)
            .then_some(())
            .ok_or_else(|| Error::ExpectedMap(self.type_name()))?;
        visitor.visit_map(MapValueWrap::new(self))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }
}
