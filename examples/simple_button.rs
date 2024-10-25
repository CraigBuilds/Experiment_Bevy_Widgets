use bevy::prelude::*;
use bevy_widge::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
#[require(
    MiWindow,
    WithChild<MiButton>
)]
struct MyUI;

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
    cmds.spawn(MyUI);
}