use std::{clone, fmt};

use bevy::color::palettes::css::BLACK;
use bevy::ecs::system::SystemState;
use bevy::ui::FocusPolicy;
use bevy::{color::palettes::tailwind::*, prelude::*, window::WindowResolution};
use bevy_ui_inspector::dropdown::{
    self, create_dropdown, Dropdown, DropdownBox, DropdownItem, DropdownPlugin, DropdownSelected,
};
use bevy_ui_inspector::element::{spawn_element_hierarchy, ComponentArgs, Element};
use bevy_ui_inspector::icons::{setup_icons, Icons};
use bevy_ui_inspector::theme::Theme;
use bevy_ui_inspector::val::ValExt;
use bevy_ui_inspector::{UiInspectorPlugin, ValTypes};
#[derive(Event)]
struct SpawnUiEvent {
    element: Element,
    parent: Option<Entity>,
    index: Option<usize>,
}
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
        .add_event::<SpawnUiEvent>()
        .insert_resource(ActiveStyleInspection::default())
        .add_plugins(UiInspectorPlugin)
        .add_plugins(DropdownPlugin)
        .register_type::<SpacingMarkerPosition>()
        .register_type::<SpacingMarker>()
        .register_type::<ValTypes>()
        .register_type::<SpacingDimensionsMarker>()
        .add_systems(Startup, setup_icons)
        .add_systems(Startup, spawn_layout)
        .add_systems(Update, spawn_ui_on_event)
        .add_systems(
            Update,
            ((
                update_spacing_markers,
                update_style_panel,
                val_input_width_fixer,
                update_text_input,
                text_input_focus,
                handle_keyboard_input,
                background_click_system_input_focus,
                update_style_property,
            )
                .chain()),
        )
        .run();
}

fn spawn_ui_on_event(world: &mut World) {
    let mut spawn_ui_events = world.get_resource_mut::<Events<SpawnUiEvent>>().unwrap();

    let events: Vec<SpawnUiEvent> = spawn_ui_events.update_drain().collect();

    // Clear the events after reading
    for spawn_ui_e in events {
        spawn_element_hierarchy(
            spawn_ui_e.element,
            world,
            spawn_ui_e.parent,
            spawn_ui_e.index,
        );
    }
}

fn print_debug(reflect_value: &dyn Reflect) {
    let mut output = String::new();
    let _ = std::fmt::write(&mut output, format_args!("{:?}", reflect_value));
    println!("Reflected debug output: {}", output);
}
fn spawn_layout(world: &mut World) {
    let mut system_state: SystemState<(
        Commands,
        Res<AssetServer>,
        Res<Icons>,
        ResMut<Theme>,
        ResMut<ActiveStyleInspection>,
        EventWriter<SpawnUiEvent>,
    )> = SystemState::new(world);

    let element_to_inspect = Element::default()
        .background_color(AMBER_500)
        .with_style(|style| {
            style.left = 300.0.px();
            style.top = 300.0.px();
            style.width = 150.0.px();
            style.height = 50.0.px();
            style.margin = UiRect::all(12.0.px());
            style.padding = UiRect::all(Val::Auto);
            style.border = UiRect::all(1.0.px());
        })
        .add_component(Name::new("OrangeSquare"))
        .add_child_elements([Element::default()
            .with_style(|style| {
                style.width = 100.0.pct();
                style.justify_content = JustifyContent::Center;
            })
            .add_component(Name::new("ChildSquare"))]);
    let entity_id = spawn_element_hierarchy(element_to_inspect, world, None, None);
    {
        let (
            mut commands,
            asset_server,
            icons,
            mut theme,
            mut active_style_inspection,
            mut spawn_ui_event_writer,
        ) = system_state.get_mut(world);
        let font: Handle<Font> = asset_server.load("fonts/SourceCodePro-Regular.ttf");
        theme.font = font;
        commands.spawn((
            Camera2dBundle::default(),
            bevy::render::view::RenderLayers::layer(0),
        ));

        let margin_left = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::MarginLeft,
        );
        let margin_right = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::MarginRight,
        );
        let margin_top = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::MarginTop,
        );
        let margin_bottom = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::MarginBottom,
        );
        let padding_left = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::PaddingLeft,
        );
        let padding_right = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::PaddingRight,
        );
        let padding_top = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::PaddingTop,
        );
        let padding_bottom = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::PaddingBottom,
        );
        let border_left = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::BorderLeft,
        );
        let border_right = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::BorderRight,
        );
        let border_top = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::BorderTop,
        );
        let border_bottom = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::BorderBottom,
        );
        let position_left = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::PositionLeft,
        );
        let position_right = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::PositionRight,
        );
        let position_top = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::PositionTop,
        );
        let position_bottom = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::PositionBottom,
        );
        let width = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::Width,
        );
        let min_width = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::MinWidth,
        );
        let max_width = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::MaxWidth,
        );
        let height = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::Height,
        );
        let min_height = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::MinHeight,
        );
        let max_height = create_val_thing(
            &mut commands,
            &icons,
            &theme,
            Dropdown {
                open: false,
                selected: 0,
            },
            ValTypeLink::MaxHeight,
        );
        let width_title = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "width".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            })
            .id();
        let height_title = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "height".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            })
            .id();
        let min_title = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "min".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            })
            .id();
        let max_title = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "max".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            })
            .id();

        let margin_title = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "margin".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            })
            .id();
        let padding_title = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "padding".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            })
            .id();
        let border_title = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "border".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            })
            .id();
        let position_title = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "position".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            })
            .id();
        let left_title = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "left".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            })
            .id();
        let right_title = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "right".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            })
            .id();
        let top_title = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "top".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            })
            .id();
        let bottom_title = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "bottom".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: theme.input.color,
                    },
                ),
                ..Default::default()
            })
            .id();
        let empty = commands.spawn(NodeBundle::default()).id();
        let empty2 = commands.spawn(NodeBundle::default()).id();
        let empty3 = commands.spawn(NodeBundle::default()).id();
        let mut dd_container = commands.spawn((
            NodeBundle {
                background_color: theme.background.into(),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    position_type: PositionType::Absolute,
                    padding: UiRect::all(Val::Px(12.0)),
                    row_gap: Val::Px(12.0),
                    width: Val::Px(500.0),
                    height: Val::Percent(100.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            Name::new("StylePanel"),
        ));
        // dd_container.push_children(&[position_thing]);
        dd_container.with_children(|builder| {
            let mut ui_rect_grid = builder.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::min_content(5),
                        grid_template_rows: RepeatedGridTrack::min_content(5),
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("UiRectGrid"),
            ));
            ui_rect_grid.push_children(&[
                // title
                empty,
                left_title,
                right_title,
                top_title,
                bottom_title,
                //margin
                margin_title,
                margin_left,
                margin_right,
                margin_top,
                margin_bottom,
                //padding
                padding_title,
                padding_left,
                padding_right,
                padding_top,
                padding_bottom,
                //border
                border_title,
                border_left,
                border_right,
                border_top,
                border_bottom,
                //position
                position_title,
                position_left,
                position_right,
                position_top,
                position_bottom,
            ]);
            let mut ui_rect_grid = builder.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::min_content(4),
                        grid_template_rows: RepeatedGridTrack::min_content(3),
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Dimensions"),
            ));
            ui_rect_grid.push_children(&[
                empty2,
                empty3,
                min_title,
                max_title,
                //width
                width_title,
                width,
                min_width,
                max_width,
                //height
                height_title,
                height,
                min_height,
                max_height,
            ]);
        });
        spacing(&mut spawn_ui_event_writer, &theme, dd_container.id().into());
        active_style_inspection.entity = entity_id.into();
    }

    system_state.apply(world);
}
fn get_val_type(val: Val) -> ValTypes {
    match val {
        Val::Px(_) => ValTypes::Px,
        Val::Percent(_) => ValTypes::Percent,
        Val::Vw(_) => ValTypes::Vw,
        Val::Vh(_) => ValTypes::Vh,
        Val::VMin(_) => ValTypes::VMin,
        Val::VMax(_) => ValTypes::VMax,
        Val::Auto => ValTypes::Auto,
    }
}
fn get_number_val(val: Val) -> f32 {
    match val {
        Val::Px(num) => num,
        Val::Percent(num) => num,
        Val::Vw(num) => num,
        Val::Vh(num) => num,
        Val::VMin(num) => num,
        Val::VMax(num) => num,
        Val::Auto => 0.0,
    }
}
enum CalculateDirection {
    Horizontal,
    Vertical,
}
fn get_calculated_pixel_val(
    val: Val,
    parent_size: Vec2,
    window_size: Vec2,
    direction: CalculateDirection,
) -> f32 {
    match val {
        Val::Px(num) => num,
        Val::Percent(num) => match direction {
            CalculateDirection::Horizontal => parent_size.x * num / 100.0,
            CalculateDirection::Vertical => parent_size.y * num / 100.0,
        },
        Val::Vw(num) => window_size.x / 100.0 * num,
        Val::Vh(num) => window_size.y / 100.0 * num,
        Val::VMin(num) => todo!(),
        Val::VMax(num) => todo!(),
        Val::Auto => 0.0,
    }
}

fn get_val_type_for_dropdown_value(i: usize) -> ValTypes {
    match i {
        1 => ValTypes::Px,
        2 => ValTypes::Percent,
        3 => ValTypes::Vw,
        4 => ValTypes::Vh,
        5 => ValTypes::VMin,
        6 => ValTypes::VMax,
        _ => ValTypes::Auto,
    }
}
fn get_dropdown_value_for_val_type(val: ValTypes) -> usize {
    match val {
        ValTypes::Auto => 0,
        ValTypes::Px => 1,
        ValTypes::Percent => 2,
        ValTypes::Vw => 3,
        ValTypes::Vh => 4,
        ValTypes::VMin => 5,
        ValTypes::VMax => 6,
    }
}
fn update_style_panel(
    style_inputs_q: Query<(&ValTypeLink, Entity)>,
    active_style_inspection: Res<ActiveStyleInspection>,
    children_q: Query<&Children>,
    mut val_input_dropdown_q: Query<(&mut Dropdown, Entity)>,
    dropdown_items_q: Query<&DropdownItem>,
    mut text_input_q: Query<(&mut TextInput)>,
    style_q: Query<&Style>,
) {
    if let Some(e) = active_style_inspection
        .entity
        .filter(|_| active_style_inspection.is_changed())
    {
        let style = style_q.get(e).unwrap();
        style_inputs_q
            .iter()
            .for_each(|(val_type_link, val_input_e)| {
                let val_type = match val_type_link {
                    ValTypeLink::MarginLeft => get_val_type(style.margin.left),
                    ValTypeLink::MarginRight => get_val_type(style.margin.right),
                    ValTypeLink::MarginTop => get_val_type(style.margin.top),
                    ValTypeLink::MarginBottom => get_val_type(style.margin.bottom),
                    ValTypeLink::PaddingLeft => get_val_type(style.padding.left),
                    ValTypeLink::PaddingRight => get_val_type(style.padding.right),
                    ValTypeLink::PaddingTop => get_val_type(style.padding.top),
                    ValTypeLink::PaddingBottom => get_val_type(style.padding.bottom),
                    ValTypeLink::BorderLeft => get_val_type(style.border.left),
                    ValTypeLink::BorderRight => get_val_type(style.border.right),
                    ValTypeLink::BorderTop => get_val_type(style.border.top),
                    ValTypeLink::BorderBottom => get_val_type(style.border.bottom),
                    ValTypeLink::PositionLeft => get_val_type(style.left),
                    ValTypeLink::PositionRight => get_val_type(style.right),
                    ValTypeLink::PositionTop => get_val_type(style.top),
                    ValTypeLink::PositionBottom => get_val_type(style.bottom),
                    ValTypeLink::Width => get_val_type(style.width),
                    ValTypeLink::MinWidth => get_val_type(style.min_width),
                    ValTypeLink::MaxWidth => get_val_type(style.max_width),
                    ValTypeLink::Height => get_val_type(style.height),
                    ValTypeLink::MinHeight => get_val_type(style.min_height),
                    ValTypeLink::MaxHeight => get_val_type(style.max_height),
                };
                children_q.iter_descendants(val_input_e).for_each(|child| {
                    if let Ok((mut dropdown, dropdown_e)) = val_input_dropdown_q.get_mut(child) {
                        dropdown.selected = get_dropdown_value_for_val_type(val_type);
                    }
                    if let Ok(mut text_input) = text_input_q.get_mut(child) {
                        let number_val = match val_type_link {
                            ValTypeLink::MarginLeft => get_number_val(style.margin.left),
                            ValTypeLink::MarginRight => get_number_val(style.margin.right),
                            ValTypeLink::MarginTop => get_number_val(style.margin.top),
                            ValTypeLink::MarginBottom => get_number_val(style.margin.bottom),
                            ValTypeLink::PaddingLeft => get_number_val(style.padding.left),
                            ValTypeLink::PaddingRight => get_number_val(style.padding.right),
                            ValTypeLink::PaddingTop => get_number_val(style.padding.top),
                            ValTypeLink::PaddingBottom => get_number_val(style.padding.bottom),
                            ValTypeLink::BorderLeft => get_number_val(style.border.left),
                            ValTypeLink::BorderRight => get_number_val(style.border.right),
                            ValTypeLink::BorderTop => get_number_val(style.border.top),
                            ValTypeLink::BorderBottom => get_number_val(style.border.bottom),
                            ValTypeLink::PositionLeft => get_number_val(style.left),
                            ValTypeLink::PositionRight => get_number_val(style.right),
                            ValTypeLink::PositionTop => get_number_val(style.top),
                            ValTypeLink::PositionBottom => get_number_val(style.bottom),
                            ValTypeLink::Width => get_number_val(style.width),
                            ValTypeLink::MinWidth => get_number_val(style.min_width),
                            ValTypeLink::MaxWidth => get_number_val(style.max_width),
                            ValTypeLink::Height => get_number_val(style.height),
                            ValTypeLink::MinHeight => get_number_val(style.min_height),
                            ValTypeLink::MaxHeight => get_number_val(style.max_height),
                        };
                        text_input.value = format!("{}", number_val);
                    }
                });
            });
    }
}

#[derive(Component, Debug)]
enum ValTypeLink {
    MarginLeft,
    MarginRight,
    MarginTop,
    MarginBottom,

    PaddingLeft,
    PaddingRight,
    PaddingTop,
    PaddingBottom,

    BorderLeft,
    BorderRight,
    BorderTop,
    BorderBottom,

    PositionLeft,
    PositionRight,
    PositionTop,
    PositionBottom,

    Width,
    MinWidth,
    MaxWidth,

    Height,
    MinHeight,
    MaxHeight,
}

#[derive(Resource, Default)]
struct ActiveStyleInspection {
    entity: Option<Entity>,
}

fn update_style_property(
    mut style_q: Query<&mut Style>,
    active_style_inspection: Res<ActiveStyleInspection>,
    val_input_q: Query<(&ValInput, &ValTypeLink, Entity)>,
    val_input_dropdown_q: Query<(Ref<Dropdown>, Entity)>,
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
                            if let Some(mut style) = active_style_inspection
                                .entity
                                .and_then(|e| style_q.get_mut(e).ok())
                            {
                                let input_val = text_input.value.parse::<f32>().unwrap_or_default();
                                let val_type = get_val_type_for_dropdown_value(dropdown.selected);
                                let val = match val_type {
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
                                    ValTypeLink::MarginRight => style.margin.right = val,
                                    ValTypeLink::MarginTop => style.margin.top = val,
                                    ValTypeLink::MarginBottom => style.margin.bottom = val,
                                    ValTypeLink::PaddingLeft => style.padding.left = val,
                                    ValTypeLink::PaddingRight => style.padding.right = val,
                                    ValTypeLink::PaddingTop => style.padding.top = val,
                                    ValTypeLink::PaddingBottom => style.padding.bottom = val,
                                    ValTypeLink::BorderLeft => style.border.left = val,
                                    ValTypeLink::BorderRight => style.border.right = val,
                                    ValTypeLink::BorderTop => style.border.top = val,
                                    ValTypeLink::BorderBottom => style.border.bottom = val,
                                    ValTypeLink::PositionLeft => style.left = val,
                                    ValTypeLink::PositionRight => style.right = val,
                                    ValTypeLink::PositionTop => style.top = val,
                                    ValTypeLink::PositionBottom => style.bottom = val,
                                    ValTypeLink::Width => style.width = val,
                                    ValTypeLink::MinWidth => style.min_width = val,
                                    ValTypeLink::MaxWidth => style.max_width = val,
                                    ValTypeLink::Height => style.height = val,
                                    ValTypeLink::MinHeight => style.min_height = val,
                                    ValTypeLink::MaxHeight => style.max_height = val,
                                }
                            }
                        }
                    }
                });
            }
        });
}
fn update_spacing_markers(
    mut text_q: Query<&mut Text>,
    style_q: Query<(&Style, &Node, &GlobalTransform, Option<&Parent>)>,
    active_style_inspection: Res<ActiveStyleInspection>,
    spacing_markers_q: Query<(&SpacingMarker, &SpacingMarkerPosition, &Children)>,
    dimension_markers_q: Query<Entity, With<SpacingDimensionsMarker>>,
    windows_q: Query<(&Window)>,
) {
    if let Some(e) = active_style_inspection.entity {
        let (style, node, tf, parent) = style_q.get(e).unwrap();
        let window_size = windows_q.get_single().unwrap().size();
        let parent_size = parent
            .and_then(|parent_e| style_q.get(parent_e.get()).ok())
            .map(|(_, node, _, _)| node.size())
            .unwrap_or(window_size);
        let dimensions = node.size();
        let pos = node.logical_rect(tf);

        if let Some(mut text) = dimension_markers_q
            .get_single()
            .ok()
            .and_then(|marker_e| text_q.get_mut(marker_e).ok())
        {
            text.sections[0].value = format!("{}x{}", dimensions.x, dimensions.y);
        }
        spacing_markers_q
            .iter()
            .for_each(|(spacing_type, position, children)| {
                for e in children.iter() {
                    let mut text = text_q.get_mut(*e).unwrap();
                    let new = match (spacing_type, position) {
                        (SpacingMarker::Position, SpacingMarkerPosition::Left) => {
                            get_calculated_pixel_val(
                                style.left,
                                parent_size,
                                window_size,
                                CalculateDirection::Horizontal,
                            )
                        }
                        (SpacingMarker::Position, SpacingMarkerPosition::Right) => {
                            get_calculated_pixel_val(
                                style.right,
                                parent_size,
                                window_size,
                                CalculateDirection::Horizontal,
                            )
                        }
                        (SpacingMarker::Position, SpacingMarkerPosition::Top) => {
                            get_calculated_pixel_val(
                                style.top,
                                parent_size,
                                window_size,
                                CalculateDirection::Vertical,
                            )
                        }
                        (SpacingMarker::Position, SpacingMarkerPosition::Bottom) => {
                            get_calculated_pixel_val(
                                style.bottom,
                                parent_size,
                                window_size,
                                CalculateDirection::Vertical,
                            )
                        }

                        (SpacingMarker::Margin, SpacingMarkerPosition::Left) => {
                            get_calculated_pixel_val(
                                style.margin.left,
                                parent_size,
                                window_size,
                                CalculateDirection::Horizontal,
                            )
                        }
                        (SpacingMarker::Margin, SpacingMarkerPosition::Right) => {
                            get_calculated_pixel_val(
                                style.margin.right,
                                parent_size,
                                window_size,
                                CalculateDirection::Horizontal,
                            )
                        }
                        (SpacingMarker::Margin, SpacingMarkerPosition::Top) => {
                            get_calculated_pixel_val(
                                style.margin.top,
                                parent_size,
                                window_size,
                                CalculateDirection::Vertical,
                            )
                        }
                        (SpacingMarker::Margin, SpacingMarkerPosition::Bottom) => {
                            get_calculated_pixel_val(
                                style.margin.bottom,
                                parent_size,
                                window_size,
                                CalculateDirection::Vertical,
                            )
                        }
                        (SpacingMarker::Border, SpacingMarkerPosition::Left) => {
                            get_calculated_pixel_val(
                                style.border.left,
                                parent_size,
                                window_size,
                                CalculateDirection::Horizontal,
                            )
                        }
                        (SpacingMarker::Border, SpacingMarkerPosition::Right) => {
                            get_calculated_pixel_val(
                                style.border.right,
                                parent_size,
                                window_size,
                                CalculateDirection::Horizontal,
                            )
                        }
                        (SpacingMarker::Border, SpacingMarkerPosition::Top) => {
                            get_calculated_pixel_val(
                                style.border.top,
                                parent_size,
                                window_size,
                                CalculateDirection::Vertical,
                            )
                        }
                        (SpacingMarker::Border, SpacingMarkerPosition::Bottom) => {
                            get_calculated_pixel_val(
                                style.border.bottom,
                                parent_size,
                                window_size,
                                CalculateDirection::Vertical,
                            )
                        }
                        (SpacingMarker::Padding, SpacingMarkerPosition::Left) => {
                            get_calculated_pixel_val(
                                style.padding.left,
                                parent_size,
                                window_size,
                                CalculateDirection::Horizontal,
                            )
                        }
                        (SpacingMarker::Padding, SpacingMarkerPosition::Right) => {
                            get_calculated_pixel_val(
                                style.padding.right,
                                parent_size,
                                window_size,
                                CalculateDirection::Horizontal,
                            )
                        }
                        (SpacingMarker::Padding, SpacingMarkerPosition::Top) => {
                            get_calculated_pixel_val(
                                style.padding.top,
                                parent_size,
                                window_size,
                                CalculateDirection::Vertical,
                            )
                        }
                        (SpacingMarker::Padding, SpacingMarkerPosition::Bottom) => {
                            get_calculated_pixel_val(
                                style.padding.bottom,
                                parent_size,
                                window_size,
                                CalculateDirection::Vertical,
                            )
                        }
                    };
                    text.sections[0].value = format!("{}", new);
                }
            });
    }
}
const SPACING_MARGIN: f32 = 12.0;
const SPACING_MARGIN_Y: f32 = 6.0;
fn created_nested_spacing_indicator(
    type_of_spacing: &str,
    theme: &Theme,
    box_color: &str,
    child: Option<Element>,
    marker: SpacingMarker,
) -> Element {
    let left_el = Element::default()
        .with_style(|style| {
            style.margin = UiRect::right(SPACING_MARGIN.px());
            style.align_items = AlignItems::Center;
        })
        .with_text(
            "6",
            TextStyle {
                font: theme.font.clone(),
                font_size: theme.input.size,
                color: Srgba::hex("F8F8FA").unwrap().into(),
            },
        )
        .add_component(SpacingMarkerPosition::Left)
        .add_component(marker.clone())
        .add_component(Name::new("Left"));
    let right_el = Element::default()
        .with_style(|style| {
            style.margin = UiRect::left(SPACING_MARGIN.px());
            style.align_items = AlignItems::Center;
        })
        .with_text(
            "7",
            TextStyle {
                font: theme.font.clone(),
                font_size: theme.input.size,
                color: Srgba::hex("F8F8FA").unwrap().into(),
            },
        )
        .add_component(SpacingMarkerPosition::Right)
        .add_component(marker.clone())
        .add_component(Name::new("Right"));

    let box_el = Element::default()
        .with_style(|style| {
            style.padding = UiRect::horizontal(SPACING_MARGIN.px());
            style.flex_direction = FlexDirection::Column;
        })
        .background_color(Srgba::hex(box_color).unwrap())
        .add_component(Name::new(type_of_spacing.to_string()));

    let title = Element::default()
        .with_style(|style| {
            style.position_type = PositionType::Absolute;
            style.left = 0.0.px();
        })
        .with_text(
            type_of_spacing,
            TextStyle {
                font: theme.font.clone(),
                font_size: theme.input.size,
                color: Srgba::hex("F8F8FA").unwrap().into(),
            },
        );

    let top_el = Element::default()
        .with_style(|style| {
            style.margin = UiRect::vertical(SPACING_MARGIN_Y.px());
            style.justify_content = JustifyContent::Center;
        })
        .with_text(
            "8",
            TextStyle {
                font: theme.font.clone(),
                font_size: theme.input.size,
                color: Srgba::hex("F8F8FA").unwrap().into(),
            },
        )
        .add_component(SpacingMarkerPosition::Top)
        .add_component(marker.clone())
        .add_component(Name::new("Top"));
    let bottom_el = Element::default()
        .with_style(|style| {
            style.margin = UiRect::vertical(SPACING_MARGIN_Y.px());
            style.justify_content = JustifyContent::Center;
        })
        .with_text(
            "9",
            TextStyle {
                font: theme.font.clone(),
                font_size: theme.input.size,
                color: Srgba::hex("F8F8FA").unwrap().into(),
            },
        )
        .add_component(SpacingMarkerPosition::Bottom)
        .add_component(marker.clone())
        .add_component(Name::new("Bottom"));

    let wrapper = Element::default().with_style(|style| {
        style.flex_direction = FlexDirection::Row;
        style.justify_content = JustifyContent::Center;
    });
    let wrapper = if let Some(el) = child {
        wrapper.add_child_elements([left_el, el, right_el])
    } else {
        wrapper
    };
    let outer = Element::default().with_style(|style| {
        style.position_type = PositionType::Relative;
        style.flex_direction = FlexDirection::Row;
        style.justify_content = JustifyContent::Center;
    });
    let box_el = box_el.add_child_elements([title, top_el, wrapper, bottom_el]);

    outer.add_child_elements([box_el])
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
enum SpacingMarker {
    Position,
    Margin,
    Border,
    Padding,
}
#[derive(Component, Reflect)]
#[reflect(Component)]
enum SpacingMarkerPosition {
    Left,
    Right,
    Top,
    Bottom,
}
#[derive(Component, Reflect)]
#[reflect(Component)]
struct SpacingDimensionsMarker;
fn spacing(spawn_ui_e: &mut EventWriter<SpawnUiEvent>, theme: &Theme, parent: Option<Entity>) {
    let node_size = Element::default()
        .with_style(|style| {
            style.padding = UiRect::all(12.0.px());
            style.border = UiRect::all(1.0.px());
        })
        .border_color(BLACK)
        .background_color(Srgba::hex("407AA4").unwrap())
        .add_component(Name::new("NodeSize"))
        .with_text(
            "1000x370",
            TextStyle {
                font: theme.font.clone(),
                font_size: theme.input.size,
                color: Srgba::hex("F8F8FA").unwrap().into(),
            }, //todo SpacingDimensionsMarker
        );

    let padding = created_nested_spacing_indicator(
        "padding",
        theme,
        "6657A6",
        node_size.into(),
        SpacingMarker::Padding,
    );
    let border = created_nested_spacing_indicator(
        "border",
        theme,
        "38383D",
        padding.into(),
        SpacingMarker::Border,
    );
    let margin = created_nested_spacing_indicator(
        "margin",
        theme,
        "73764A",
        border.into(),
        SpacingMarker::Margin,
    );
    let position = created_nested_spacing_indicator(
        "position",
        theme,
        "222222",
        margin.into(),
        SpacingMarker::Position,
    );
    let container = Element::default()
        .with_style(|style| {
            style.width = 100.0.pct();
            style.justify_content = JustifyContent::Center;
        })
        .add_component(Name::new("SpacingContainer"));
    let container = container.add_child_elements([position]);
    println!("send");
    spawn_ui_e.send(SpawnUiEvent {
        element: container,
        parent,
        index: 0.into(),
    });
}

fn val_input_width_fixer(
    val_input_q: Query<(Entity, &ValInput)>,
    mut val_input_dropdown_q: Query<
        (&Dropdown, &mut Style, Entity),
        (Changed<Dropdown>, Without<TextInput>),
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
                        let place_holder = get_val_type_for_dropdown_value(dropdown.selected);
                        if place_holder == ValTypes::Auto {
                            dropdown_s.width = Val::Px(55.0);
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

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ValInput {}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ValInputDropdown {}
pub fn dropdown_option(option: DropdownItem, theme: &Theme) -> Element {
    Element::default()
        .with_style(|style| {
            style.padding = UiRect::vertical(3.0.px());
        })
        .add_component(Name::new(format!("Option -> {}", option.label)))
        .add_component(Interaction::None)
        .with_text(
            &option.label,
            TextStyle {
                font: theme.font.clone(),
                font_size: theme.input.size,
                color: theme.input.color,
            },
        )
        .add_component(option)
}

pub fn dropdown(
    theme: &Theme,
    icons: &Icons,
    options: impl Into<Vec<DropdownItem>>,
    selected: usize,
) -> Element {
    let root_el = Element::default()
        .with_style(|style| {
            style.width = 30.0.px();
            style.height = 22.0.px();
            style.position_type = PositionType::Relative;
            style.flex_direction = FlexDirection::Row;
            style.align_items = AlignItems::Center;
            style.justify_content = JustifyContent::SpaceBetween;
            style.padding = UiRect::horizontal(6.0.px());
            style.border = UiRect::new(0.0.px(), 1.0.px(), 1.0.px(), 1.0.px());
            style.overflow = Overflow::clip_x();
        })
        .background_color(theme.input.background_color)
        .border_color(BLACK)
        .add_component(Interaction::None)
        .add_component(Dropdown {
            open: false,
            selected,
        })
        .add_component(Name::new("dropdown"));

    let selected_value = Element::default()
        .with_text(
            "default value",
            TextStyle {
                font: theme.font.clone(),
                font_size: theme.input.size,
                color: theme.input.color,
                ..default()
            },
        )
        .add_component(DropdownSelected {})
        .add_component(Name::new("Selected"));

    let chevron_icon = Element::default().add_component(UiImage::new(icons.chevron_down.clone()));
    let dropdown_box = Element::default()
        .with_style(|style| {
            style.position_type = PositionType::Absolute;
            style.top = 22.0.px();
            style.flex_direction = FlexDirection::Column;
            style.width = 140.0.px();
            style.padding = UiRect::vertical(6.0.px());
            style.display = Display::None;
        })
        .add_component(Name::new("Options"))
        .add_component(DropdownBox {})
        .add_component(ZIndex::Global(1000))
        .background_color(theme.input.background_color);
    let option_els: Vec<_> = options
        .into()
        .into_iter()
        .map(|option| dropdown_option(option, theme))
        .collect();

    let dropdown_box = dropdown_box.add_child_elements(option_els);

    root_el.add_child_elements([selected_value, chevron_icon, dropdown_box])
}

pub fn create_val_thing(
    commands: &mut Commands,
    icons: &Icons,
    theme: &Theme,
    dropdown: Dropdown,
    linked_to: ValTypeLink,
) -> Entity {
    let open = dropdown.open;
    let options = [
        DropdownItem {
            label: "auto".to_string(),
            value: 0, //Box::new(ValTypes::Auto),
        },
        DropdownItem {
            label: "px".to_string(),
            value: 1, //Box::new(ValTypes::Px),
        },
        DropdownItem {
            label: "%".to_string(),
            value: 2, //Box::new(ValTypes::Percent),
        },
        DropdownItem {
            label: "vw".to_string(),
            value: 3, //Box::new(ValTypes::Vw),
        },
        DropdownItem {
            label: "vh".to_string(),
            value: 4, //Box::new(ValTypes::Vh),
        },
        DropdownItem {
            label: "vmin".to_string(),
            value: 5, //Box::new(ValTypes::VMin),
        },
        DropdownItem {
            label: "vmax".to_string(),
            value: 6, //Box::new(ValTypes::VMax),
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
                    border: UiRect::new(Val::Px(0.0), Val::Px(1.0), Val::Px(1.0), Val::Px(1.0)),
                    overflow: Overflow {
                        x: OverflowAxis::Hidden,
                        ..default()
                    },
                    ..Default::default()
                },
                border_radius: BorderRadius::new(
                    Val::Px(0.0),
                    Val::Px(4.0),
                    Val::Px(4.0),
                    Val::Px(0.0),
                ),
                border_color: BLACK.into(),
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
                        z_index: ZIndex::Global(1000),
                        focus_policy: FocusPolicy::Block,
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

fn input(args: impl Into<ComponentArgs>, theme: &Theme) -> Element {
    let element = Element::default()
        .with_style(|style| {
            style.height = 22.0.px();
            style.width = 120.0.px();
            style.justify_content = JustifyContent::Center;
            style.border = UiRect::new(1.0.px(), 0.0.px(), 1.0.px(), 1.0.px());
            style.overflow = Overflow::clip_x();
            style.align_items = AlignItems::Center;
        })
        .border_radius((4.0.px(), 0.0.px(), 0.0.px(), 4.0.px()))
        .border_color(BLACK)
        .background_color(theme.input.background_color)
        .add_component(Name::new("Input"))
        .add_component(TextInput {
            value: String::new(),
            cursor: 0,
            focussed: false,
        })
        .add_component(Interaction::default())
        .with_text(
            "",
            TextStyle {
                font: theme.font.clone(),
                font_size: theme.input.size,
                color: theme.input.color,
            },
        );
    element
}

fn create_input(commands: &mut Commands, theme: &Theme) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Px(22.0),
                    width: Val::Px(120.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::new(Val::Px(1.0), Val::Px(0.0), Val::Px(1.0), Val::Px(1.0)),
                    overflow: Overflow {
                        x: OverflowAxis::Hidden,
                        ..default()
                    },
                    ..Default::default()
                },
                border_radius: BorderRadius::new(
                    Val::Px(4.0),
                    Val::Px(0.0),
                    Val::Px(0.0),
                    Val::Px(4.0),
                ),
                border_color: BLACK.into(),
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

#[derive(Component, Reflect)]
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
