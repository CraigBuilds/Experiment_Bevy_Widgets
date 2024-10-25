# Bevy Widgets

Until we get BSN, creating UI Nodes with children is very verbose. The 0.15 Bevy example to spawn a Button with text is about [36 lines of code](https://github.com/bevyengine/bevy/blob/c6a66a7e96a0a24faab0cade1801910c40aa7ee7/examples/ui/button.rs#L57-L92)

This is an experimental crate that uses required components and `WithChild` from `i-can't-beleive-its-not-bsn` to reduce the amount of code needed to create UI Nodes with children. 

Widgets provided in this crate are prefixed with Mi to distinguish them from the standard Bevy widgets.

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