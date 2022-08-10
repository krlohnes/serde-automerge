use anyhow::Result;
use automerge::{AutoCommit, ROOT};

use serde_automerge::*;

fn main() {
    let mut doc = AutoCommit::new();
    // let id = doc
    //     .put_object(ROOT, "some object", automerge::ObjType::Map)
    //     .unwrap();
    // doc.put(id, "hoi", "jasper").unwrap();

    // let id = doc
    //     .put_object(ROOT, "some object", automerge::ObjType::Map)
    //     .unwrap();
    // doc.put(id, "hoi", "jaspe23r").unwrap();

    // let id = doc
    //     .put_object(ROOT, "some object1", automerge::ObjType::Map)
    //     .unwrap();
    // doc.put(id, "hoi", "jaspe23r").unwrap();
    // let data = doc.save();

    // export_json(std::io::Cursor::new(data), std::io::stdout(), true);

    let p = Position {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let mut d = JasperDoc {
        stuff: std::marker::PhantomData,
        doc: &mut doc,
        key: None,
        parent: ROOT,
    };

    put_automerge_point(&mut d, "stuff", p);

    // let result = p.serialize(&mut d);
    let data = d.doc.save();

    export_json(std::io::Cursor::new(data), std::io::stdout(), true);
}

// store key in JasperDoc
// serialize_f32 can use key to store the value
