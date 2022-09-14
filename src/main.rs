use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_automerge::{
    de::Deserializer, ser::Serializer, transaction::CommitOptions, Automerge,
    AutomergeSetExtension, ObjId,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Float3 {
    x: f32,
    y: f32,
    z: f32,
}

type Camera = Float3;
type Position = Float3;
type Direction = Float3;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Player {
    position: Position,
    direction: Direction,
}
// This must be equal to the member variable
const POSITION: &'static str = "position";

const PLAYER: &'static str = "player";
const CAMERA: &'static str = "camera";

fn main() -> anyhow::Result<()> {
    let data = sending_client()?;
    receiving_client(data)?;
    Ok(())
}

fn sending_client() -> Result<Vec<u8>> {
    // Initial dummy values
    let mut player_send = Player {
        position: Position {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        direction: Direction {
            x: 10.0,
            y: 11.0,
            z: 12.0,
        },
    };
    let camera_send = Camera {
        x: 20.0,
        y: 21.0,
        z: 22.0,
    };
    println!("Sending:");
    println!("{:?}", camera_send);
    println!("{:?}", player_send);
    println!();

    // Create a doc on the sending party and serialize the data into it
    let mut doc_send = Automerge::new();

    let player_id = {
        let mut transaction = doc_send.transaction();
        camera_send.serialize(Serializer::new_root(&mut transaction, CAMERA))?;
        let (_, id) = player_send.serialize(Serializer::new_root(&mut transaction, PLAYER))?;
        transaction.commit();
        id
    };

    // We update the players position
    player_send.position.x += 5.0;
    player_send.position.y += 5.0;
    player_send.position.z += 5.0;

    // We update _only_ the player with the previously obtained player_id
    // Since Serializer uses an &mut we can also use the transact functionallity
    let _pos_id = doc_send
        .transact_with::<_, _, _, _, ()>(
            |_| CommitOptions::default().with_message("Updating position"),
            |tx| {
                player_send
                    .position
                    .serialize(Serializer::new(tx, player_id, POSITION))
                    .map(|v| v.1)
            },
        )
        .map(|s| s.result)
        .map_err(|f| f.error)?;

    // We update the players position
    player_send.position.x -= 2.0;
    player_send.position.y -= 2.0;
    player_send.position.z -= 2.0;

    doc_send.set_value(ObjId::Root, PLAYER, player_send);

    // This is the content which we send and receive
    Ok(doc_send.save())
}

fn receiving_client(received_data: Vec<u8>) -> Result<()> {
    // Create a doc on the receiving party and load the stored state
    let doc_receive_all = Automerge::load(&received_data)?;

    let player_receive = Player::deserialize(Deserializer::new_get(
        &doc_receive_all,
        ObjId::Root,
        PLAYER,
    )?)?;
    let camera_receive = Camera::deserialize(Deserializer::new_get(
        &doc_receive_all,
        ObjId::Root,
        CAMERA,
    )?)?;

    println!("Received:");
    println!("{:?}", camera_receive);
    println!("{:?}", player_receive);

    // TODO: sending/receiving partial updates

    Ok(())
}
