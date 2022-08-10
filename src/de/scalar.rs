use crate::error::Error;
use automerge::ScalarValue;
use serde::de;

pub struct Deserializer(ScalarValue);

impl Deserializer {
    pub fn type_name(&self) -> &'static str {
        Self::value_type_name(&self.0)
    }
    pub fn value_type_name(v: &ScalarValue) -> &'static str {
        match v {
            ScalarValue::Bytes(_) => "byte buffer",
            ScalarValue::Str(_) => "string",
            ScalarValue::Int(_) => "integer number",
            ScalarValue::Uint(_) => "unsigned integer number",
            ScalarValue::F64(_) => "floating point number",
            ScalarValue::Counter(_) => "counter",
            ScalarValue::Timestamp(_) => "timestamp",
            ScalarValue::Boolean(_) => "boolean",
            ScalarValue::Null => "null",
        }
    }
    pub fn into_byte_buff(self) -> Result<Vec<u8>, Error> {
        self.0
            .into_bytes()
            .map_err(|v| Error::ExpectedBoolean(Self::value_type_name(&v)))
    }
    pub fn to_bytes(&self) -> Result<&[u8], Error> {
        self.0
            .to_bytes()
            .ok_or_else(|| Error::ExpectedBoolean(self.type_name()))
    }
    pub fn into_string(self) -> Result<String, Error> {
        self.0
            .into_string()
            .map_err(|v| Error::ExpectedString(Self::value_type_name(&v)))
    }
    pub fn to_str(&self) -> Result<&str, Error> {
        self.0
            .to_str()
            .ok_or_else(|| Error::ExpectedString(self.type_name()))
    }
    pub fn to_i64(&self) -> Result<i64, Error> {
        self.0
            .to_i64()
            .ok_or_else(|| Error::ExpectedInteger(self.type_name()))
    }
    pub fn to_u64(&self) -> Result<u64, Error> {
        self.0
            .to_u64()
            .ok_or_else(|| Error::ExpectedInteger(self.type_name()))
    }
    pub fn to_f64(&self) -> Result<f64, Error> {
        self.0
            .to_f64()
            .ok_or_else(|| Error::ExpectedFloat(self.type_name()))
    }
    pub fn to_bool(&self) -> Result<bool, Error> {
        self.0
            .to_bool()
            .ok_or_else(|| Error::ExpectedBoolean(self.type_name()))
    }
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
    pub fn to_null(&self) -> Result<(), Error> {
        self.is_null()
            .then_some(())
            .ok_or_else(|| Error::ExpectedNull(self.type_name()))
    }
}

impl From<ScalarValue> for Deserializer {
    fn from(v: ScalarValue) -> Self {
        Self(v)
    }
}

impl<'de> de::Deserializer<'de> for Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.0 {
            ScalarValue::Bytes(_) => self.deserialize_byte_buf(visitor),
            ScalarValue::Str(_) => self.deserialize_str(visitor),
            ScalarValue::Int(_) => self.deserialize_i64(visitor),
            ScalarValue::Uint(_) => self.deserialize_u64(visitor),
            ScalarValue::F64(_) => self.deserialize_f64(visitor),
            ScalarValue::Counter(_) => self.deserialize_i64(visitor),
            ScalarValue::Timestamp(_) => self.deserialize_i64(visitor),
            ScalarValue::Boolean(_) => self.deserialize_bool(visitor),
            ScalarValue::Null => self.deserialize_unit(visitor),
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

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
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
