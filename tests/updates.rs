use automerge::{transaction::Transactable, AutoCommit, ReadDoc};
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
fn test_updates_and_merging_on_manual_commit() {
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
    let mut tx = doc_send.transaction();
    let (player_id, camera_id, numbers_id) =
        send(&mut tx, &player_send, &camera_send, &numbers_send);
    let (commit, _patch_log) = tx.commit();
    let base_commit = commit.unwrap();
    // Send the whole document
    let data = doc_send.save();

    // Receiving
    let mut doc_receive = Automerge::load(&data).unwrap();
    let (mut player_receive, mut camera_receive, mut numbers_receive) = receive(&doc_receive);
    assert_eq!(player_send, player_receive);
    assert_eq!(camera_send, camera_receive);
    assert_eq!(numbers_send, numbers_receive);

    // Updating
    let mut tx = doc_send.transaction();
    update1(
        &mut tx,
        &mut player_send,
        &mut camera_send,
        &mut numbers_send,
        &player_id,
        &camera_id,
        &numbers_id,
    );
    let (_change_hash_after_update1, _patch_log) =
        tx.commit_with(CommitOptions::default().with_message("Updating position"));

    assert_ne!(player_send, player_receive);

    let data = doc_send.save_after(&[base_commit]);

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

    // Receiving
    doc_receive.load_incremental(&data).unwrap();
    let (player_receive2, camera_receive2, numbers_receive2) = receive(&doc_receive);

    // Assert that local and remote changes were merged successfully
    assert_eq!(player_send, player_receive2);

    assert_eq!(camera_receive, camera_receive2);
    assert_eq!(numbers_receive, numbers_receive2);
}

#[test]
fn test_updates_and_merging_on_autocommit() {
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
    let mut doc_send = AutoCommit::new();
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
    doc_send.commit_with(CommitOptions::default().with_message("Updating position"));

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

    // Receiving
    doc_receive.load_incremental(&data).unwrap();
    let (player_receive2, camera_receive2, numbers_receive2) = receive(&doc_receive);

    // Assert that local and remote changes were merged successfully
    assert_eq!(player_send, player_receive2);

    assert_eq!(camera_receive, camera_receive2);
    assert_eq!(numbers_receive, numbers_receive2);
}

fn send(
    transaction: &mut impl Transactable,
    player: &Player,
    camera: &Camera,
    numbers: &[i32],
) -> (ObjId, ObjId, ObjId) {
    println!("Send:");
    println!("{:?}", player);
    println!("{:?}", camera);
    println!("{:?}", numbers);

    let (_, id_player) = player
        .serialize(Serializer::new_root(transaction, PLAYER))
        .unwrap();
    let (_, id_camera) = camera
        .serialize(Serializer::new_root(transaction, CAMERA))
        .unwrap();
    let (_, id_numbers) = numbers
        .serialize(Serializer::new_root(transaction, NUMBERS))
        .unwrap();
    (id_player, id_camera, id_numbers)
}

fn update1(
    tx: &mut impl Transactable,
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
    // Since Serializer uses a &mut we can also use the transact functionallity
    let (_doc, _ex_id) = player
        .position
        .serialize(Serializer::new(tx, id_player.clone(), POSITION))
        .expect("Serialize failed");

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
