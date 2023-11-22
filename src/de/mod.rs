use automerge::{
    Automerge, AutomergeError, ObjId, ObjType, Prop, ReadDoc as _, ScalarValue, Value,
};
use serde::{de, forward_to_deserialize_any};

mod error;
mod map;
mod seq;

pub use error::Error;
pub use map::MapDeserializer;
pub use seq::SeqDeserializer;

pub struct Deserializer<'a> {
    pub doc: &'a Automerge,
    pub value: Option<(Value<'a>, ObjId)>,
}

impl<'a> Deserializer<'a> {
    pub fn new(doc: &'a Automerge, value: Option<(Value<'a>, ObjId)>) -> Self {
        Self { doc, value }
    }
    pub fn new_found(doc: &'a Automerge, value: Value<'a>, id: ObjId) -> Self {
        Self::new(doc, Some((value, id)))
    }
    pub fn new_root(doc: &'a Automerge) -> Self {
        Self::new_found(doc, ObjType::Map.into(), ObjId::Root)
    }
    pub fn new_get<O: AsRef<ObjId>, P: Into<Prop>>(
        doc: &'a Automerge,
        key: O,
        prop: P,
    ) -> Result<Self, AutomergeError> {
        Ok(Self::new(doc, doc.get(key, prop)?))
    }
}

impl<'a> From<&'a Automerge> for Deserializer<'a> {
    fn from(doc: &'a Automerge) -> Self {
        Self::new_root(doc)
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'_> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            None => visitor.visit_none(),
            Some((Value::Object(t), id)) => match t {
                ObjType::List => visitor.visit_seq(SeqDeserializer::new(self.doc, id)),
                ObjType::Text => visitor.visit_string(self.doc.text(id)?),
                ObjType::Map | ObjType::Table => {
                    visitor.visit_map(MapDeserializer::new(self.doc, id))
                }
            },
            Some((Value::Scalar(s), _)) => match s.into_owned() {
                ScalarValue::Bytes(v) => visitor.visit_byte_buf(v),
                ScalarValue::Str(v) => visitor.visit_str(&v),
                ScalarValue::Int(v) => visitor.visit_i64(v),
                ScalarValue::Uint(v) => visitor.visit_u64(v),
                ScalarValue::F64(v) => visitor.visit_f64(v),
                ScalarValue::Counter(v) => visitor.visit_i64(v.into()),
                ScalarValue::Timestamp(v) => visitor.visit_i64(v),
                ScalarValue::Boolean(v) => visitor.visit_bool(v),
                ScalarValue::Unknown {
                    type_code: _,
                    bytes: v,
                } => visitor.visit_byte_buf(v),
                ScalarValue::Null => visitor.visit_unit(),
            },
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
