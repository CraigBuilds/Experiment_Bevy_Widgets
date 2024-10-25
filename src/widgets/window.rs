use bevy::prelude::*;

/// A Marker component for a window. Spawning this will spawn a Node that fills the screen, and children
/// of this Node will be centered in the screen.
#[derive(Component, Default)]
#[require(
    Node(|| {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(5.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    })
)]
pub struct WWindow;