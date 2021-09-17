use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};


#[derive(Debug, Default)]
pub struct Move{pub x:f32, pub y:f32}
#[derive(Debug, Default)]
pub struct Jump;
#[derive(Debug, Default)]
pub struct Turn{pub x:f32, pub y:f32}
#[derive(Debug, Default)]
pub struct Zoom{pub z:f32}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<Move>()
            .add_event::<Jump>()
            .add_event::<Turn>()
            .add_event::<Zoom>()
            .add_system_set(SystemSet::new()
            .label("input")
            .before("logic")
            .with_system(send_move)
            .with_system(send_jump)
            .with_system(send_turn)
            .with_system(send_zoom)
        );
    }
}

// fn input(
//     mut keyboard : Res<Input<KeyCode>>,
//     mut mouse_button : Res<Input<MouseButton>>,

//     mut mouse_delta : EventReader<MouseMotion>,
//     mut mouse_position : EventReader<CursorMoved>,
//     mut mouse_wheel : EventReader<MouseWheel>,
// ) {
// }

fn send_move(
    mut last_W_S : Local<Option<KeyCode>>,
    mut last_D_A : Local<Option<KeyCode>>,
    mut keyboard : Res<Input<KeyCode>>,
    mut ew : EventWriter<Move>,
){
    let mut y = if keyboard.just_pressed(KeyCode::W) {*last_W_S=Some(KeyCode::W);1.0}
    else if keyboard.just_pressed(KeyCode::S) {*last_W_S=Some(KeyCode::S);-1.0}
    else {0.0};

    let mut x = if keyboard.just_pressed(KeyCode::D) {*last_D_A=Some(KeyCode::D);1.0}
    else if keyboard.just_pressed(KeyCode::A) {*last_D_A=Some(KeyCode::A);-1.0}
    else {0.0};

    if keyboard.just_released(KeyCode::W) {*last_W_S=Some(KeyCode::S);}
    else if keyboard.just_released(KeyCode::S) {*last_W_S=Some(KeyCode::W);}

    if keyboard.just_released(KeyCode::D) {*last_D_A=Some(KeyCode::A);}
    else if keyboard.just_released(KeyCode::A) {*last_D_A=Some(KeyCode::D);}

    y = match *last_W_S {
        None | Some(KeyCode::W) if keyboard.pressed(KeyCode::W) => 1.0,
        None | Some(KeyCode::S) if keyboard.pressed(KeyCode::S) => -1.0,
        _ => 0.0
    };

    x = match *last_D_A {
        None | Some(KeyCode::D) if keyboard.pressed(KeyCode::D) => 1.0,
        None | Some(KeyCode::A) if keyboard.pressed(KeyCode::A) => -1.0,
        _ => 0.0
    };

    if x==0.0 && y==0.0 {return;}

    ew.send(Move{x:x,y:y});
}

fn send_jump(
    mut keyboard : Res<Input<KeyCode>>,
    mut ew : EventWriter<Jump>,
){
    if keyboard.just_pressed(KeyCode::Space){
        ew.send(Jump);
    }
}

fn send_turn( // XXX: 改成函数式写法
    mut mouse_delta : EventReader<MouseMotion>,
    mut ew : EventWriter<Turn>,
){
    let mut x = 0.0;
    let mut y = 0.0;
    for mouse in mouse_delta.iter() {
        x += mouse.delta.x;
        y += mouse.delta.y;
    }
    if x==0.0 && y==0.0 {return;}
    ew.send(Turn{x:x, y:y});
}

fn send_zoom(
    mut mouse_wheel : EventReader<MouseWheel>,
    mut ew : EventWriter<Zoom>,
){
    let z:f32 = mouse_wheel.iter().map(|w|w.y).sum();
    ew.send(Zoom{z:z});
}