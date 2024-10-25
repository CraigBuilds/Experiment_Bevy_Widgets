use bevy::prelude::*;
use crate::with_child::WithChild;

const NORMAL_BUTTON: Color = Color::srgb(0.2, 0.2, 0.2);

#[derive(Component, Default, Clone)]
#[require(
    Button,
    Node(||{
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }),
    BorderColor(|| {BorderColor(Color::BLACK)}),
    BorderRadius(|| {BorderRadius::MAX}),
    BackgroundColor(|| {BackgroundColor(NORMAL_BUTTON)}),
    WithChild<Text>(|| {WithChild::new(Text::new("Button"))})
)]
pub struct WButton;

impl WButton {
    pub fn with_text(text: &str) -> (Self, WithChild<Text>) {
        (Self, WithChild::new(Text::new(text)))
    }
}