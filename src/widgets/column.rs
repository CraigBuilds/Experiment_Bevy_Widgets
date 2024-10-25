use bevy::prelude::*;
use crate::with_child::WithChildren;

#[derive(Component, Default, Clone)]
#[require(
    Button,
    Node(||{
        Node {
            width: Val::Px(150.0),
            height: Val::Px(300.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        }
    }),
    BorderColor(|| {BorderColor(Color::WHITE)}),
)]
pub struct MiColumn;

impl MiColumn {
    pub fn with_children<B: Bundle + Clone>(children: Vec<B>) -> (Self, WithChildren<B>) {
        (Self, WithChildren::new(children))
    }
}