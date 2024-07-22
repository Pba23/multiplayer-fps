use bevy::prelude::*;
mod ui;
mod client;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ui::ConnectionScreenPlugin)
        .run();
}
