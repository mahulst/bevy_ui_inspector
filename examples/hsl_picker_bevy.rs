//! Shows how to render to a texture. Useful for mirrors, UI, or exporting images.

use std::{
    any::{Any, TypeId},
    f32::consts::PI,
};

use bevy::{color::palettes, prelude::*, window::WindowResolution};
use bevy_ui_inspector::UiInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UiInspectorPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Px(200.0),
            height: Val::Px(200.0),
            left: Val::Px(400.0),
            top: Val::Px(200.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(palettes::tailwind::AMBER_300.into()),
    ));

    commands.spawn((
        Node {
            width: Val::Px(100.0),
            height: Val::Px(400.0),
            margin: UiRect::new(Val::Px(1200.0), Val::Px(0.0), Val::Px(50.0), Val::Px(0.0)),
            ..default()
        },
        BackgroundColor(palettes::tailwind::GREEN_600.into()),
    ));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                margin: UiRect::top(Val::Px(800.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(palettes::tailwind::TEAL_500.into()),
        ))
        .with_children(|builder| {
            builder.spawn((
                Node {
                    width: Val::Px(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(palettes::tailwind::TEAL_800.into()),
            ));
        });

    commands.spawn((
        Camera3d { ..default() },
        Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        IsDefaultUiCamera,
    ));
}
