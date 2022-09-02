use automerge::{Automerge, ObjId};
use serde::{Deserialize, Serialize};

use serde_automerge::{de::Deserializer, ser::Serializer};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

fn main() -> anyhow::Result<()> {
    // We start with a new position
    let pos_send = Position {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    println!("Send position:\n{:?}", pos_send);

    // Create a doc on the sending party and serialize the position into it
    let mut doc_send = Automerge::new();
    {
        let mut transaction = doc_send.transaction();
        pos_send.serialize(Serializer::new_root(&mut transaction, "pos"))?;
        transaction.commit();
    }

    // This is the content which is send
    let all_binary_data = doc_send.save();

    // Create a doc on the receiving party and receive the stored state
    let doc_receive_all = Automerge::load(&all_binary_data)?;

    let pos_receive =
        Position::deserialize(Deserializer::new_get(&doc_receive_all, ObjId::Root, "pos")?)?;
    println!("Received position:\n{:?}", pos_receive);

    // TODO
    // let mut doc_receive_partial = Automerge::new();

    Ok(())
}
