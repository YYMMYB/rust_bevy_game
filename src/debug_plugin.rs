use std::{f32::consts::PI, ops::Mul};

use bevy::prelude::*;

use crate::input_action::*;

macro_rules! default_struct {
    ($p:vis $name:ident { $($n:ident : $t:ty = $v:expr),* $(,)?}) => {
        #[derive(Debug)]
        $p struct $name {
            $($p $n : $t,)*
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    $($n : $v,)*
                }
            }
        }
    }
}

default_struct!(pub PlayerControlConfig{
    move_speed : f32 = 5.0,
    turn_speed : f32 = 10.0,
    zoom_speed : f32 = 30.0,
});

default_struct!(pub MainCamTrans {
    distance : f32 = 10.0,
    angle : f32 = 60.0,
    target : Vec3 = -Vec3::Z,
});

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        let cam_trans = MainCamTrans::default();
        app
            .add_startup_system(setup_static_scene)
            .insert_resource(PlayerControlConfig::default())
            .add_system(player_and_main_cam_move.config(|params|{
                params.0 = Some(cam_trans);
            }).label("logic"))
            ;
    }
}

struct MainCam;
struct Player;

fn setup_static_scene(
    mut commands : Commands,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>
){
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 1.0, -1.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    let cam = commands.spawn()
        .insert(MainCam)
        .insert_bundle(PerspectiveCameraBundle::default()).id();

    let test_cube = commands.spawn()
        .insert_bundle(PbrBundle{
            mesh:meshes.add(shape::Cube::default().into()),
            transform:Transform::from_xyz(0.0, 0.0, 4.0),
            ..Default::default()})
        .id();
    // player
    let player = commands.spawn()
        .insert(Player)
        .insert_bundle(PbrBundle{
            mesh : meshes.add(Mesh::from(shape::Cube::default())),
            material : materials.add(Color::ORANGE_RED.into()),
            ..Default::default()
        }).id()
        ;

    commands.entity(player).push_children(&[cam, test_cube]);
}

fn player_and_main_cam_move(
    mut cam_trans : Local<MainCamTrans>,

    mut qs: QuerySet<(
        QueryState<&mut Transform, With<Player>>,
        QueryState<&mut Transform, With<MainCam>>,
    )>,
    time : Res<Time>,
    config : Res<PlayerControlConfig>,
    mut er_move : EventReader<Move>,
    mut er_jump : EventReader<Jump>,
    mut er_turn : EventReader<Turn>,
    mut er_zoom : EventReader<Zoom>,
){
    // turn
    let mut v_turn = Vec3::ZERO;
    for e in er_turn.iter() {
        v_turn += - Vec3::new(e.y, e.x, 0.0);
    };
    v_turn *= config.turn_speed*time.delta_seconds();

    // move
    let mut v_move = Vec3::ZERO;
    for e in er_move.iter() {
        v_move += Vec3::new(e.x, 0.0, - e.y);
    };
    v_move *= config.move_speed*time.delta_seconds();

    let mut in_distance = 0.0;
    for e in er_zoom.iter() {
        in_distance += - e.z;
    };
    in_distance *= config.zoom_speed*time.delta_seconds();

    cam_trans.angle += v_turn.x;
    cam_trans.angle = cam_trans.angle.clamp(-89.0, 89.0);
    cam_trans.distance += in_distance;
    cam_trans.distance = cam_trans.distance.max(3.0);

    let mut q_player = qs.q0();
    let mut player = q_player.single_mut();
    player.rotate(Quat::from_axis_angle(Vec3::Y, v_turn.y  * PI / 180.0));
    let rot_p = player.rotation;
    player.translation += rot_p.mul(v_move);

    let mut q_cam = qs.q1();
    let mut cam = q_cam.single_mut();
    cam.translation = Quat::from_axis_angle(Vec3::X, cam_trans.angle  * PI / 180.0).mul_vec3(Vec3::Z * cam_trans.distance);
    cam.look_at(Vec3::Z*-2.0, Vec3::Y);
}