# Bevy Widgets

Until BSN is available, creating UI Nodes with children in Bevy is quite verbose. For example, the 0.15 Bevy button-with-text example spans about [36 lines of code](https://github.com/bevyengine/bevy/blob/c6a66a7e96a0a24faab0cade1801910c40aa7ee7/examples/ui/button.rs#L57-L92).

This small experiment uses `required components` and `WithChild` from `i-can't-believe-its-not-bsn` to cut down the code needed to create UI Nodes with children. It’s not groundbreaking, but it may be a helpful reference for developers aiming to build a proper widget set and a more ergonomic UI experience with Bevy UI. It provides an example showing the use of `required components` and `WithChild` as they were intended to be used, and defines 3 widgets (marker components) that have slightly different default values for their required components than the standard bevy widgets. 

Widgets provided in this crate are prefixed with Mi to distinguish them from the standard Bevy widgets.

*`i-can't-beleive-its-not-bsn` is currently aligned with Bevy 0.14. Until it is updated, I'm using my own simplified implementation of `WithChild` in this crate.

# Widgets

- MiWindow: A Node that fills the window and centers its children
- MiButton: A Button with a Text child
- MiColumn: A Node that stacks its children vertically

# Example use

### Basic usage

```rust
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
```

This defines a component `MyUI` that will spawn the required components onto the entity it is spawned on. The two required components are `MiWindow` and `WithChild<MiButton>`. The `MiWindow` component will spawm a Node that fills the window and centers its children. The `WithChild<MiButton>` is a spceial component that uses hooks to spawn the `MiButton` component on a seperate entity and attach it as a child to the entity with the `WithChild<MiButton>` component. i.e, there will be an entity with a window-sized Node component, with a child entity that has a `MiButton` component.

### Customizing the Button

```rust
#[derive(Component)]
#[require(
    MiWindow,
    WithChild<(MiButton, WithChild<Text>)>(|| {WithChild::new(MiButton::with_text("Hello World"))})
)]
struct MyUI;

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
    cmds.spawn(MyUI);
}
```

This example shows how to customize the `MiButton` by using a closure to customise the `WithChild<MiButton>` component. Instead of using the default `MiButton`, this uses `MiButton::with_text("Hello World")` to spawn a `(MiButton, WithChild<Text>)` bundle, i.e, it overides the default `WithChild<Text>` component of the `MiButton` with a new `Text` component that has the text `"Hello World"`.

### Column

```rust
#[derive(Component)]
#[require(
    MiWindow,
    WithChild<(MiColumn, WithChildren<MiButton>)>(||{
        WithChild::new(
            MiColumn::with_children(vec![MiButton; 3])
        )
    })
)]
struct MyUI;

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
    cmds.spawn(MyUI);
}
```

This example shows how to use the `MiColumn` widget to stack 3 `MiButton` widgets vertically.
The `MiWindow` has a child entity with a `MiColumn` component, and the entity with the `MiColumn` component also has a `WithChildren<MiButton>` component that spawns 3 `MiButton` components as children of the enity with the `MiColumn` component.

### Roadmap

- [ ] Better looking defaults. Maybe the button should be an image? I'd like a widget set that looks good out of the box, even if it has an opinionated style. 
- [ ] Add more widgets. Use this as reference: https://archive.slint.dev/0.2.0/docs/rust/slint/docs/widgets/
- [ ] `OnClicked`, `OnHovered` etc. components that wrap callbacks. Add systems that delegate `Changed<Interaction>` events to these callbacks. 
- [ ] macro_rules! to define components with required components and `WithChild` and `WithChildren` components. At this point, it will be something a tiny bit like bsn.
- [ ] Add more customization options

This is an experiment, I may never finish it.
