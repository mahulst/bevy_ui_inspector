# Bevy UI Inspector

A tool that tries to do what the html / css tools in the chrome dev tools do.

# How to use

```rust
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiInpector,
        ))
        .run()

```

# Features
- A way to see the UI node hierarchy
- Fiddling with styling properties while running the app
- A picker to quickly go to a node you want to inspect

# Demo

![demo](docs/demo.gif)

# Todos
- More styling properties
- Add background color
- Add selection / changing of text
- Different way of highlighting UI nodes
