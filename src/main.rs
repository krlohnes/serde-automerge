use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_automerge::{
    de::Deserializer, ser::Serializer, transaction::CommitOptions, Automerge, ObjId,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct Float3 {
    x: i32,
    y: i32,
    z: i32,
}

type Camera = Float3;
type Position = Float3;
type Direction = Float3;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct Player {
    position: Position,
    direction: Direction,
}
// This must be equal to the member variable
const POSITION: &'static str = "position";

const PLAYER: &'static str = "player";
const CAMERA: &'static str = "camera";
const NUMBERS: &'static str = "numbers";

fn main() -> anyhow::Result<()> {
    // Initial dummy values
    let mut player_send = Player {
        position: Position { x: 1, y: 2, z: 3 },
        direction: Direction {
            x: 10,
            y: 11,
            z: 12,
        },
    };
    let mut camera_send = Camera {
        x: 20,
        y: 21,
        z: 22,
    };
    let mut numbers_send = vec![31, 32, 33];

    // Sending
    let mut doc_send = Automerge::new();
    let (player_id, camera_id, numbers_id) =
        send(&mut doc_send, &player_send, &camera_send, &numbers_send)?;
    let data = doc_send.save();

    // Receiving
    let mut doc_receive = Automerge::load(&data)?;
    let received = receive(&doc_receive)?;
    correct(&player_send, &camera_send, &numbers_send, &received);

    // Updating
    update(
        &mut doc_send,
        &mut player_send,
        &mut camera_send,
        &mut numbers_send,
        player_id,
        camera_id,
        numbers_id,
    )?;
    let data = doc_send.save_incremental();

    // Receiving
    doc_receive.load_incremental(&data)?;
    let received = receive(&doc_receive)?;
    correct(&player_send, &camera_send, &numbers_send, &received);

    Ok(())
}

fn send(
    doc: &mut Automerge,
    player: &Player,
    camera: &Camera,
    numbers: &[i32],
) -> Result<(ObjId, ObjId, ObjId)> {
    println!("Send:");
    println!("{:?}", camera);
    println!("{:?}", player);
    println!("{:?}", numbers);

    let mut transaction = doc.transaction();
    let (_, id_player) = player.serialize(Serializer::new_root(&mut transaction, PLAYER))?;
    let (_, id_camera) = camera.serialize(Serializer::new_root(&mut transaction, CAMERA))?;
    let (_, id_numbers) = numbers.serialize(Serializer::new_root(&mut transaction, NUMBERS))?;
    transaction.commit();
    Ok((id_player, id_camera, id_numbers))
}

fn update(
    doc: &mut Automerge,
    player: &mut Player,
    camera: &mut Camera,
    numbers: &mut [i32],
    id_player: ObjId,
    _id_camera: ObjId,
    id_numbers: ObjId,
) -> Result<()> {
    // We update the players position
    player.position.x += 5;
    player.position.y += 5;
    player.position.z += 5;

    // We update _only_ the player with the previously obtained player_id
    // Since Serializer uses an &mut we can also use the transact functionallity
    let _pos_id = doc
        .transact_with::<_, _, _, _, ()>(
            |_| CommitOptions::default().with_message("Updating position"),
            |tx| {
                player
                    .position
                    .serialize(Serializer::new(tx, id_player, POSITION))
                    .map(|v| v.1)
            },
        )
        .map(|s| s.result)
        .map_err(|f| f.error)?;

    // We update the camera position
    camera.x -= 2;
    camera.y -= 2;
    camera.z -= 2;

    use serde_automerge::AutomergeSetExtension;
    doc.set_value(ObjId::Root, CAMERA, &camera)?;

    numbers[2] = 34;
    doc.set_value(id_numbers, 2, 34)?;

    println!("Update:");
    println!("{:?}", camera);
    println!("{:?}", player);
    println!("{:?}", numbers);
    Ok(())
}

fn receive(doc: &Automerge) -> Result<(Player, Camera, Vec<i32>)> {
    let player = Player::deserialize(Deserializer::new_get(&doc, ObjId::Root, PLAYER)?)?;
    let camera = Camera::deserialize(Deserializer::new_get(&doc, ObjId::Root, CAMERA)?)?;
    let numbers = Vec::<i32>::deserialize(Deserializer::new_get(&doc, ObjId::Root, NUMBERS)?)?;

    println!("Receive:");
    println!("{:?}", camera);
    println!("{:?}", player);
    println!("{:?}", numbers);

    Ok((player, camera, numbers))
}

fn correct(
    player: &Player,
    camera: &Camera,
    numbers: &Vec<i32>,
    received: &(Player, Camera, Vec<i32>),
) {
    println!(
        "Equal: {}",
        player == &received.0 && camera == &received.1 && numbers == &received.2
    );
    println!();
}
