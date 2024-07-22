use bevy::prelude::*;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ui::ConnectionScreenPlugin)
        .run();
}
