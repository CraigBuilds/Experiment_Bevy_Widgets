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
    WWindow,
    WithChild<(WButton, WithChild<Text>)>(|| {WithChild::new(WButton::with_text("Hello World"))})
)]
struct MyUI;

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
    cmds.spawn(MyUI);
}