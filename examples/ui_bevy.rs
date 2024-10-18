use bevy::{color::palettes::tailwind::*, prelude::*, window::WindowResolution};
use bevy_ui_inspector::dropdown::{
    self, create_dropdown, Dropdown, DropdownBox, DropdownItem, DropdownPlugin, DropdownSelected,
};
use bevy_ui_inspector::icons::{setup_icons, Icons};
use bevy_ui_inspector::theme::Theme;
use bevy_ui_inspector::{UiInspectorPlugin, ValTypes};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Icons::default())
        .insert_resource(Theme::default())
        .insert_resource(ActiveStyleInspection::default())
        .add_plugins(UiInspectorPlugin)
        .add_plugins(DropdownPlugin)
        .add_systems(Startup, setup_icons)
        .add_systems(Startup, spawn_layout.after(setup_icons))
        .add_systems(
            Update,
            (
                update_style_property,
                val_input_width_fixer,
                update_text_input,
                text_input_focus,
                handle_keyboard_input,
                background_click_system_input_focus,
            ),
        )
        .run();
}

fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    icons: Res<Icons>,
    mut theme: ResMut<Theme>,
    mut active_style_inspection: ResMut<ActiveStyleInspection>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    theme.font = font;
    commands.spawn(Camera2dBundle::default());
    let entity_id = commands
        .spawn(
            (NodeBundle {
                background_color: AMBER_500.into(),
                style: Style {
                    left: Val::Px(300.0),
                    top: Val::Px(300.0),
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                ..default()
            }),
        )
        .id();
    active_style_inspection.entity = entity_id.into();
    let dd1 = create_dropdown(
        &mut commands,
        &icons,
        &theme,
        vec![
            DropdownItem {
                label: "Item 1".to_string(),
                value: Box::new(1usize),
            },
            DropdownItem {
                label: "Item 2".to_string(),
                value: Box::new(2),
            },
            DropdownItem {
                label: "Item 3".to_string(),
                value: Box::new(3),
            },
            DropdownItem {
                label: "Item 4".to_string(),
                value: Box::new(4),
            },
            DropdownItem {
                label: "Item 5".to_string(),
                value: Box::new(5),
            },
        ],
        Dropdown {
            open: false,
            selected: DropdownItem {
                label: "Item 5".to_string(),
                value: Box::new(5),
            },
        },
    );
    let dd2 = create_dropdown::<Display>(
        &mut commands,
        &icons,
        &theme,
        vec![
            DropdownItem {
                label: "Flex".to_string(),
                value: Box::new(Display::Flex),
            },
            DropdownItem {
                label: "Grid".to_string(),
                value: Box::new(Display::Grid),
            },
            DropdownItem {
                label: "Block".to_string(),
                value: Box::new(Display::Block),
            },
            DropdownItem {
                label: "None".to_string(),
                value: Box::new(Display::None),
            },
        ],
        Dropdown {
            open: false,
            selected: DropdownItem {
                label: "Flex".to_string(),
                value: Box::new(Display::Flex),
            },
        },
    );
    let input = create_input(&mut commands, &theme);
    let val_input = create_val_thing(
        &mut commands,
        &icons,
        &theme,
        Dropdown {
            open: false,
            selected: DropdownItem {
                label: "px".to_string(),
                value: ValTypes::Px.into(),
            },
        },
        ValTypeLink::MarginLeft,
    );
    let mut dd_container = commands.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                row_gap: Val::Px(12.0),
                width: Val::Px(500.0),
                ..default()
            },
            ..default()
        },
        Name::new("DropdownContainer"),
    ));

    dd_container.add_child(dd1);
    dd_container.add_child(dd2);
    dd_container.add_child(val_input);
}
#[derive(Component)]
enum ValTypeLink {
    MarginLeft,
}

#[derive(Resource, Default)]
struct ActiveStyleInspection {
    entity: Option<Entity>,
}

fn update_style_property(
    mut style_q: Query<&mut Style>,
    active_style_inspection: Res<ActiveStyleInspection>,
    val_input_q: Query<(&ValInput, &ValTypeLink, Entity)>,
    val_input_dropdown_q: Query<(Ref<Dropdown<ValTypes>>, Entity)>,
    text_input_q: Query<(Ref<TextInput>, Entity)>,
    children_q: Query<&Children>,
    parent_q: Query<&Parent>,
) {
    val_input_dropdown_q
        .iter()
        .for_each(|(dropdown, dropdown_e)| {
            if let Some((val_input, val_input_link, val_input_e)) = parent_q
                .iter_ancestors(dropdown_e)
                .find_map(|parent| val_input_q.get(parent).ok())
            {
                children_q.iter_descendants(val_input_e).for_each(|child| {
                    if let Ok((text_input, text_input_e)) = text_input_q.get(child) {
                        if text_input.is_changed() || dropdown.is_changed() {
                            println!("one changed ");
                            if let Some(mut style) = active_style_inspection
                                .entity
                                .and_then(|e| style_q.get_mut(e).ok())
                            {
                                let input_val = text_input.value.parse::<f32>().unwrap_or_default();
                                let val = match *dropdown.selected.value {
                                    ValTypes::Px => Val::Px(input_val),
                                    ValTypes::Percent => Val::Percent(input_val),
                                    ValTypes::Vw => Val::Vw(input_val),
                                    ValTypes::Vh => Val::Vh(input_val),
                                    ValTypes::VMin => Val::VMin(input_val),
                                    ValTypes::VMax => Val::VMax(input_val),
                                    ValTypes::Auto => Val::Auto,
                                };
                                match val_input_link {
                                    ValTypeLink::MarginLeft => style.margin.left = val,
                                }
                            }
                        }
                    }
                });
            }
        });
}

fn val_input_width_fixer(
    val_input_q: Query<(Entity, &ValInput)>,
    mut val_input_dropdown_q: Query<
        (&Dropdown<ValTypes>, &mut Style, Entity),
        (Changed<Dropdown<ValTypes>>, Without<TextInput>),
    >,
    mut text_input_q: Query<(&TextInput, &mut Style), Without<ValInputDropdown>>,
    children_q: Query<&Children>,
    parent_q: Query<&Parent>,
) {
    val_input_dropdown_q
        .iter_mut()
        .for_each(|(dropdown, mut dropdown_s, dropdown_e)| {
            let (val_input_e, val_input) = parent_q
                .iter_ancestors(dropdown_e)
                .find_map(|parent| val_input_q.get(parent).ok())
                .unwrap();
            children_q
                .iter_descendants(val_input_e)
                .find_map(|child| {
                    if let Ok((text_input, mut text_input_s)) = text_input_q.get_mut(child) {
                        if *dropdown.selected.value == ValTypes::Auto {
                            dropdown_s.width = Val::Px(54.0);
                            text_input_s.width = Val::Px(0.0);
                        } else {
                            dropdown_s.width = Val::Px(30.0);
                            text_input_s.width = Val::Px(26.0);
                        }
                        Some(())
                    } else {
                        None
                    }
                })
                .unwrap();
        });
}

#[derive(Component)]
struct ValInput {}

#[derive(Component)]
struct ValInputDropdown {}
pub fn create_val_thing<T: PartialEq + Send + Sync + Clone + 'static>(
    commands: &mut Commands,
    icons: &Icons,
    theme: &Theme,
    dropdown: Dropdown<T>,
    linked_to: ValTypeLink,
) -> Entity {
    let open = dropdown.open;
    let options = [
        DropdownItem {
            label: "auto".to_string(),
            value: ValTypes::Auto.into(),
        },
        DropdownItem {
            label: "px".to_string(),
            value: ValTypes::Px.into(),
        },
        DropdownItem {
            label: "%".to_string(),
            value: ValTypes::Percent.into(),
        },
        DropdownItem {
            label: "vw".to_string(),
            value: ValTypes::Vw.into(),
        },
        DropdownItem {
            label: "vh".to_string(),
            value: ValTypes::Vh.into(),
        },
        DropdownItem {
            label: "vmin".to_string(),
            value: ValTypes::VMin.into(),
        },
        DropdownItem {
            label: "vmax".to_string(),
            value: ValTypes::VMax.into(),
        },
    ];
    let number_input = create_input(commands, theme);
    let dropdown_e = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(30.),
                    height: Val::Px(22.),
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::horizontal(Val::Px(6.0)),

                    ..Default::default()
                },
                background_color: BackgroundColor(theme.input.background_color),
                ..Default::default()
            },
            Interaction::None,
            ValInputDropdown {},
            Name::new("Dropdown"),
            dropdown,
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    "default value",
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                        ..default()
                    },
                ),
                DropdownSelected {},
                Name::new("Selected"),
            ));
            builder.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(8.0),
                    height: Val::Px(8.0),
                    ..default()
                },
                image: UiImage::new(icons.chevron_down.clone()),
                ..default()
            });
            builder
                .spawn((
                    DropdownBox {},
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(22.0),
                            flex_direction: FlexDirection::Column,
                            width: Val::Px(140.0),
                            padding: UiRect::vertical(Val::Px(6.0)),
                            display: match open {
                                true => Display::Flex,
                                false => Display::None,
                            },
                            ..default()
                        },
                        background_color: theme.input.background_color.into(),
                        ..default()
                    },
                    Name::new("Options"),
                ))
                .with_children(|builder| {
                    for dropdown_item in options {
                        let label = dropdown_item.label.clone();
                        builder
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        padding: UiRect::vertical(Val::Px(3.0)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Option"),
                                Interaction::None,
                                dropdown_item,
                            ))
                            .with_children(|builder| {
                                builder.spawn((TextBundle::from_section(
                                    label,
                                    TextStyle {
                                        font: theme.font.clone(),
                                        font_size: theme.input.size,
                                        color: theme.input.color,
                                    },
                                ),));
                            });
                    }
                });
        })
        .id();

    let val_input_e = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            ValInput {},
            linked_to,
        ))
        .push_children(&[number_input, dropdown_e])
        .id();

    val_input_e
}
fn create_input(commands: &mut Commands, theme: &Theme) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Px(22.0),
                    width: Val::Px(120.0),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: theme.input.background_color.into(),
                ..Default::default()
            },
            Name::new("TextInput"),
            Interaction::default(),
            TextInput {
                value: String::new(),
                cursor: 0,
                focussed: false,
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            });
        })
        .id()
}

#[derive(Component)]
struct TextInput {
    value: String,
    cursor: usize,
    focussed: bool,
}
fn update_text_input(
    query: Query<(&Children, &TextInput)>,
    mut text_input_text_q: Query<&mut Text>,
) {
    for (children, text_input) in query.iter() {
        children.iter().for_each(|child| {
            if let Ok(mut text) = text_input_text_q.get_mut(*child) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text.sections[0].value = if text_input.focussed {
                    format!("{}|{}", start, end)
                } else {
                    text_input.value.clone()
                };
            }
        });
    }
}

fn text_input_focus(
    mut query: Query<(&mut TextInput, Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    query.iter_mut().for_each(
        |(mut text_input, focussed_e, interaction)| match interaction {
            Interaction::Pressed => {
                text_input.focussed = true;
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::NONE),
                        z_index: ZIndex::Global(-1),
                        ..Default::default()
                    },
                    Interaction::default(),
                    BackgroundTextFocus { focussed_e },
                ));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        },
    );
}
#[derive(Component)]
struct BackgroundTextFocus {
    focussed_e: Entity,
}

fn background_click_system_input_focus(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction, &BackgroundTextFocus), (Changed<Interaction>,)>,
    mut text_input_q: Query<&mut TextInput>,
) {
    for (entity, interaction, dropdown_bg) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut text_input) = text_input_q.get_mut(dropdown_bg.focussed_e) {
                text_input.focussed = false;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn handle_keyboard_input(mut query: Query<&mut TextInput>, keys: Res<ButtonInput<KeyCode>>) {
    for mut text_input in query.iter_mut() {
        if text_input.focussed {
            if keys.just_pressed(KeyCode::Digit0) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '0', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit1) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '1', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit2) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '2', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit3) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '3', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit4) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '4', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit5) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '5', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit6) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '6', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit7) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '7', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit8) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '8', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit9) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '9', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::ArrowLeft) {
                text_input.cursor = 0.max(text_input.cursor.saturating_sub(1));
            }
            if keys.just_pressed(KeyCode::ArrowRight) {
                text_input.cursor = text_input.value.len().min(text_input.cursor + 1);
            }
            if keys.just_pressed(KeyCode::Backspace) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                let without_last_char: String =
                    start.chars().take(start.chars().count() - 1).collect();
                text_input.value = format!("{}{}", without_last_char, end);
                text_input.cursor = 0.max(text_input.cursor.saturating_sub(1));
            }
            if keys.just_pressed(KeyCode::Delete) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                let without_first_char: String = end.chars().skip(1).collect();
                text_input.value = format!("{}{}", start, without_first_char);
            }
        }
    }
}
