use std::{net::TcpStream, io::{Write, Read}};

use bevy::{prelude::*, ecs::bundle};
use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize)]
// struct Test_packet {
//     x: f64,
//     y: f64,
// }

// const SERVER_ADDR:&str = "192.168.222.252:6968";

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerName(String);

#[derive(Component)]
struct PlayerHealth(f32);

#[derive(Component)]
struct Block;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor{
            title: "Multiplayer Game".to_string(),
            width: 600.0,
            height: 600.0,
            scale_factor_override: Some(1.0),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_stage("game_setup_actors", SystemStage::single(setup_player))
        .add_system(input_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
}

fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Sprite, &mut Transform)>
) {
    for (_player, mut sprite, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 1.0;
            sprite.flip_x = true;
        }

        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 1.0;
            sprite.flip_x = false;
        }
    }
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("C:/Users/schon/sync/projects/RUST/multiplayer_platformer/sprites/Player.png"),
        transform: Transform {
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Player);
}

// fn tcp_test() {
//     let mut stream = TcpStream::connect(SERVER_ADDR).unwrap();

//     let mut packet = Test_packet{x: 1.0, y: 1.0};

//     stream.write(bincode::serialize(&packet).unwrap().as_slice()).unwrap();

//     let mut buffer = [0; 1024];

//     let _n = stream.read(&mut buffer[..]).unwrap();

//     packet = bincode::deserialize(&buffer).unwrap();

//     println!("x: {}, y: {}", packet.x, packet.y);
// }