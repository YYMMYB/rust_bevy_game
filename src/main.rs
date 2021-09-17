use bevy::prelude::*;

mod test;

mod debug_plugin;
mod input_action;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(input_action::MainPlugin)
        .add_plugin(debug_plugin::MainPlugin)
        .run();
}
