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
const POSITION: &str = "position";

const PLAYER: &str = "player";
const CAMERA: &str = "camera";
const NUMBERS: &str = "numbers";

#[test]
fn test_updates_and_merging() {
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
        send(&mut doc_send, &player_send, &camera_send, &numbers_send);
    let data = doc_send.save();

    // Receiving
    let mut doc_receive = Automerge::load(&data).unwrap();
    let (mut player_receive, mut camera_receive, mut numbers_receive) = receive(&doc_receive);
    assert_eq!(player_send, player_receive);
    assert_eq!(camera_send, camera_receive);
    assert_eq!(numbers_send, numbers_receive);

    // Updating
    update1(
        &mut doc_send,
        &mut player_send,
        &mut camera_send,
        &mut numbers_send,
        &player_id,
        &camera_id,
        &numbers_id,
    );

    assert_ne!(player_send, player_receive);

    let data = doc_send.save_incremental();

    // Update receiving side in the meantime too
    let (player_id, camera_id, numbers_id) = (
        doc_receive.get(ObjId::Root, PLAYER).unwrap().unwrap().1,
        doc_receive.get(ObjId::Root, CAMERA).unwrap().unwrap().1,
        doc_receive.get(ObjId::Root, NUMBERS).unwrap().unwrap().1,
    );
    update2(
        &mut doc_receive,
        &mut player_receive,
        &mut camera_receive,
        &mut numbers_receive,
        &player_id,
        &camera_id,
        &numbers_id,
    );
    println!();

    // Receiving
    doc_receive.load_incremental(&data).unwrap();
    let (player_receive2, camera_receive2, numbers_receive2) = receive(&doc_receive);

    // Assert that local and remote changes were merged successfully
    // XXX: Inline the functions, the code below implies knowing thwat update1() and update2() touched...
    assert_eq!(player_send, player_receive2);

    assert_eq!(camera_receive, camera_receive2);
    assert_eq!(numbers_receive, numbers_receive2);
}

fn send(
    doc: &mut Automerge,
    player: &Player,
    camera: &Camera,
    numbers: &[i32],
) -> (ObjId, ObjId, ObjId) {
    println!("Send:");
    println!("{:?}", player);
    println!("{:?}", camera);
    println!("{:?}", numbers);

    let mut transaction = doc.transaction();
    let (_, id_player) = player
        .serialize(Serializer::new_root(&mut transaction, PLAYER))
        .unwrap();
    let (_, id_camera) = camera
        .serialize(Serializer::new_root(&mut transaction, CAMERA))
        .unwrap();
    let (_, id_numbers) = numbers
        .serialize(Serializer::new_root(&mut transaction, NUMBERS))
        .unwrap();
    transaction.commit();
    (id_player, id_camera, id_numbers)
}

fn update1(
    doc: &mut Automerge,
    player: &mut Player,
    camera: &mut Camera,
    numbers: &mut [i32],
    id_player: &ObjId,
    _id_camera: &ObjId,
    _id_numbers: &ObjId,
) {
    // We update the players position
    player.position.x += 5;
    player.position.y += 5;
    player.position.z += 5;

    // We update _only_ the player with the previously obtained player_id
    // Since Serializer uses an &mut we can also use the transact functionallity
    let _pos_id = doc
        .transact_with::<_, _, _, _>(
            |_| CommitOptions::default().with_message("Updating position"),
            |tx| {
                player
                    .position
                    .serialize(Serializer::new(tx, id_player.clone(), POSITION))
                    .map(|v| v.1)
            },
        )
        .unwrap()
        .result;

    println!("Update 1:");
    println!("{:?}", player);
    println!("{:?}", camera);
    println!("{:?}", numbers);
}

fn update2(
    doc: &mut Automerge,
    player: &mut Player,
    camera: &mut Camera,
    numbers: &mut [i32],
    _id_player: &ObjId,
    _id_camera: &ObjId,
    id_numbers: &ObjId,
) {
    camera.x -= 2;
    camera.y -= 2;
    camera.z -= 2;

    use serde_automerge::AutomergeSetExtension;
    doc.set_value(ObjId::Root, CAMERA, &camera).unwrap();

    numbers[2] = 34;
    doc.set_value(id_numbers.clone(), 2, 34).unwrap();

    println!("Update 2:");
    println!("{:?}", player);
    println!("{:?}", camera);
    println!("{:?}", numbers);
}

fn receive(doc: &Automerge) -> (Player, Camera, Vec<i32>) {
    let player =
        Player::deserialize(Deserializer::new_get(doc, ObjId::Root, PLAYER).unwrap()).unwrap();
    let camera =
        Camera::deserialize(Deserializer::new_get(doc, ObjId::Root, CAMERA).unwrap()).unwrap();
    let numbers =
        Vec::<i32>::deserialize(Deserializer::new_get(doc, ObjId::Root, NUMBERS).unwrap()).unwrap();

    println!("Receive:");
    println!("{:?}", player);
    println!("{:?}", camera);
    println!("{:?}", numbers);

    (player, camera, numbers)
}
