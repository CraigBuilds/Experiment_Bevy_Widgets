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
    WithChild<(WColumn, WithChildren<WButton>)>(||{
        WithChild::new(
            WColumn::with_children(vec![WButton; 3])
        )
    })
)]
struct MyUI;

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
    cmds.spawn(MyUI);
}