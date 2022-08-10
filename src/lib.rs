use anyhow::Result;
use automerge as am;
use automerge::transaction::Transactable;
use automerge::AutoCommit;
use automerge::ObjId;
use automerge::ROOT;

pub(crate) fn map_to_json(doc: &am::Automerge, obj: &am::ObjId) -> serde_json::Value {
    let keys = doc.keys(obj);
    let mut map = serde_json::Map::new();
    for k in keys {
        let val = doc.get(obj, &k);
        match val {
            Ok(Some((am::Value::Object(o), exid)))
                if o == am::ObjType::Map || o == am::ObjType::Table =>
            {
                map.insert(k.to_owned(), map_to_json(doc, &exid));
            }
            Ok(Some((am::Value::Object(_), exid))) => {
                map.insert(k.to_owned(), list_to_json(doc, &exid));
            }
            Ok(Some((am::Value::Scalar(v), _))) => {
                map.insert(k.to_owned(), scalar_to_json(&v));
            }
            _ => (),
        };
    }
    serde_json::Value::Object(map)
}

fn list_to_json(doc: &am::Automerge, obj: &am::ObjId) -> serde_json::Value {
    let len = doc.length(obj);
    let mut array = Vec::new();
    for i in 0..len {
        let val = doc.get(obj, i as usize);
        match val {
            Ok(Some((am::Value::Object(o), exid)))
                if o == am::ObjType::Map || o == am::ObjType::Table =>
            {
                array.push(map_to_json(doc, &exid));
            }
            Ok(Some((am::Value::Object(_), exid))) => {
                array.push(list_to_json(doc, &exid));
            }
            Ok(Some((am::Value::Scalar(v), _))) => {
                array.push(scalar_to_json(&v));
            }
            _ => (),
        };
    }
    serde_json::Value::Array(array)
}

fn scalar_to_json(val: &am::ScalarValue) -> serde_json::Value {
    match val {
        am::ScalarValue::Str(s) => serde_json::Value::String(s.to_string()),
        am::ScalarValue::Bytes(b) => serde_json::Value::Array(
            b.iter()
                .map(|byte| serde_json::Value::Number((*byte).into()))
                .collect(),
        ),
        am::ScalarValue::Int(n) => serde_json::Value::Number((*n).into()),
        am::ScalarValue::Uint(n) => serde_json::Value::Number((*n).into()),
        am::ScalarValue::F64(n) => serde_json::Number::from_f64(*n)
            .unwrap_or_else(|| 0_i64.into())
            .into(),
        am::ScalarValue::Counter(c) => serde_json::Value::Number(i64::from(c).into()),
        am::ScalarValue::Timestamp(n) => serde_json::Value::Number((*n).into()),
        am::ScalarValue::Boolean(b) => serde_json::Value::Bool(*b),
        am::ScalarValue::Null => serde_json::Value::Null,
    }
}

fn get_state_json(input_data: Vec<u8>) -> Result<serde_json::Value> {
    let doc = am::Automerge::load(&input_data).unwrap(); // FIXME
    Ok(map_to_json(&doc, &am::ObjId::Root))
}

pub fn export_json(
    mut changes_reader: impl std::io::Read,
    mut writer: impl std::io::Write,
    is_tty: bool,
) -> Result<()> {
    let mut input_data = vec![];
    changes_reader.read_to_end(&mut input_data)?;

    let state_json = get_state_json(input_data)?;
    if is_tty {
        colored_json::write_colored_json(&state_json, &mut writer).unwrap();
        writeln!(writer).unwrap();
    } else {
        writeln!(
            writer,
            "{}",
            serde_json::to_string_pretty(&state_json).unwrap()
        )?;
    }
    Ok(())
}

use serde::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    UnsupportedType,
    Custom(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnsupportedType => "unsupported Rust type".fmt(f),
            Error::Custom(s) => write!(f, "{}", s),
        }
    }
}
impl std::error::Error for Error {}
impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}
impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}

#[derive(Debug)]
pub struct JasperDoc<'a, Tx: Transactable> {
    pub doc: &'a mut Tx,
    pub key: Option<&'static str>,
    pub parent: ObjId,
    pub stuff: std::marker::PhantomData<&'a ()>,
}

mod ser;
use ser::*;

mod de;
use de::*;

#[derive(Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn put_automerge_point<'a, Tx: Transactable, S: Serialize>(
    doc: &mut JasperDoc<'a, Tx>,
    name: &'static str,
    v: S,
) {
    doc.parent = doc
        .doc
        .put_object(&doc.parent, name, automerge::ObjType::Map)
        .unwrap();

    let result = v.serialize(doc);
}
