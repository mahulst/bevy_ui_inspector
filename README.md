# Bevy UI Inspector

A tool that tries to do what the html / css tools in the chrome dev tools do.

# How to use

```rust
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiInspectorPlugin::default(),
        ))
        .run()

    // To make the gizmos work correctly and not be drawn behind the UI.
    // Add a builtin bevy IsDefaultUiCamera component to your Camera.
    commands.spawn((
        Camera3d { ..default() },
        Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        IsDefaultUiCamera,
    ));
```

# Features
- A way to see the UI node hierarchy
- Fiddling with styling properties while running the app
- A picker to quickly go to a node you want to inspect

# Demo

![demo](docs/demo.gif)

# Compatibility

| bevy | bevy_ui_inspector |
|------|-------------------|
| 0.15 | 0.3               |
| 0.14 | 0.2               |

# Todos
- More styling properties
- Add background color
- Add selection / changing of text
