use std::clone;

use bevy::color::palettes::css::BLACK;
use bevy::ui::FocusPolicy;
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

fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    icons: Res<Icons>,
    mut theme: ResMut<Theme>,
    mut active_style_inspection: ResMut<ActiveStyleInspection>,
) {
    let font: Handle<Font> = asset_server.load("fonts/SourceCodePro-Regular.ttf");
    theme.font = font;
    let cam = commands.spawn((
        Camera2dBundle::default(),
        bevy::render::view::RenderLayers::layer(0),
    )).id();
    let position_thing = spacing(&mut commands, &theme);
    let entity_id = commands
        .spawn((
            NodeBundle {
                background_color: AMBER_500.into(),
                style: Style {
                    left: Val::Px(300.0),
                    top: Val::Px(300.0),
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Percent(12.0)),
                    padding: UiRect::all(Val::Auto),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                ..default()
            },
            Name::new("OrangeSquare"),
            TargetCamera(cam)
        ))
        .with_children(|builder| {
            builder.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                Name::new("ChildSquare"),
            ));
        })
        .id();
    let margin_left = create_val_thing(
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
    let margin_right = create_val_thing(
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
        ValTypeLink::MarginRight,
    );
    let margin_top = create_val_thing(
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
        ValTypeLink::MarginTop,
    );
    let margin_bottom = create_val_thing(
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
        ValTypeLink::MarginBottom,
    );
    let padding_left = create_val_thing(
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
        ValTypeLink::PaddingLeft,
    );
    let padding_right = create_val_thing(
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
        ValTypeLink::PaddingRight,
    );
    let padding_top = create_val_thing(
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
        ValTypeLink::PaddingTop,
    );
    let padding_bottom = create_val_thing(
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
        ValTypeLink::PaddingBottom,
    );
    let border_left = create_val_thing(
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
        ValTypeLink::BorderLeft,
    );
    let border_right = create_val_thing(
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
        ValTypeLink::BorderRight,
    );
    let border_top = create_val_thing(
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
        ValTypeLink::BorderTop,
    );
    let border_bottom = create_val_thing(
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
        ValTypeLink::BorderBottom,
    );
    let position_left = create_val_thing(
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
        ValTypeLink::PositionLeft,
    );
    let position_right = create_val_thing(
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
        ValTypeLink::PositionRight,
    );
    let position_top = create_val_thing(
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
        ValTypeLink::PositionTop,
    );
    let position_bottom = create_val_thing(
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
        ValTypeLink::PositionBottom,
    );
    let width = create_val_thing(
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
        ValTypeLink::Width,
    );
    let min_width = create_val_thing(
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
        ValTypeLink::MinWidth,
    );
    let max_width = create_val_thing(
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
        ValTypeLink::MaxWidth,
    );
    let height = create_val_thing(
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
        ValTypeLink::Height,
    );
    let min_height = create_val_thing(
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
        ValTypeLink::MinHeight,
    );
    let max_height = create_val_thing(
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
    dd_container.push_children(&[position_thing]);
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
    active_style_inspection.entity = entity_id.into();
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
fn update_style_panel(
    style_inputs_q: Query<(&ValTypeLink, Entity)>,
    active_style_inspection: Res<ActiveStyleInspection>,
    children_q: Query<&Children>,
    mut val_input_dropdown_q: Query<(&mut Dropdown<ValTypes>, Entity)>,
    dropdown_items_q: Query<&DropdownItem<ValTypes>>,
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
                        let selected = children_q.iter_descendants(dropdown_e).find_map(|child| {
                            if let Ok(dropdown_item) = dropdown_items_q.get(child) {
                                if val_type == *dropdown_item.value {
                                    return Some(dropdown_item.clone());
                                }
                            } else {
                            }
                            None
                        });
                        dropdown.selected = selected.unwrap();
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
    spacing_markers_q: Query<(&SpacingMarker, &SpacingMarkerPosition, Entity)>,
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
            .for_each(|(spacing_type, position, e)| {
                let mut text = text_q.get_mut(e).unwrap();
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
            });
    }
}
const SPACING_MARGIN: f32 = 12.0;
const SPACING_MARGIN_Y: f32 = 6.0;
fn created_nested_spacing_indicator(
    type_of_spacing: &str,
    builder: &mut Commands,
    theme: &Theme,
    box_color: &str,
    child: Option<Entity>,
    marker: SpacingMarker,
) -> Entity {
    let left = builder
        .spawn((
            TextBundle {
                style: Style {
                    margin: UiRect::right(Val::Px(SPACING_MARGIN)),
                    ..default()
                },
                text: Text::from_section(
                    "6".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: Srgba::hex("F8F8FA").unwrap().into(),
                    },
                ),
                ..Default::default()
            },
            marker.clone(),
            SpacingMarkerPosition::Left,
            Name::new("Left"),
        ))
        .id();
    let right = builder
        .spawn((
            TextBundle {
                style: Style {
                    margin: UiRect::left(Val::Px(SPACING_MARGIN)),
                    ..default()
                },
                text: Text::from_section(
                    "7".to_string(),
                    TextStyle {
                        font: theme.font.clone(),
                        font_size: theme.input.size,
                        color: Srgba::hex("F8F8FA").unwrap().into(),
                    },
                ),
                ..Default::default()
            },
            marker.clone(),
            SpacingMarkerPosition::Right,
            Name::new("Right"),
        ))
        .id();
    builder
        .spawn((
            NodeBundle {
                style: Style {
                    padding: UiRect::new(
                        Val::Px(SPACING_MARGIN),
                        Val::Px(SPACING_MARGIN),
                        Val::Px(0.0),
                        Val::Px(0.0),
                    ),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Srgba::hex(box_color).unwrap().into(),
                ..default()
            },
            Name::new(type_of_spacing.to_string()),
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            ..default()
                        },
                        text: Text::from_section(
                            type_of_spacing.to_string(),
                            TextStyle {
                                font: theme.font.clone(),
                                font_size: theme.input.size,
                                color: Srgba::hex("F8F8FA").unwrap().into(),
                            },
                        ),
                        ..Default::default()
                    });

                    builder.spawn((
                        TextBundle {
                            style: Style {
                                margin: UiRect::vertical(Val::Px(SPACING_MARGIN_Y)),
                                ..default()
                            },
                            text: Text::from_section(
                                "8".to_string(),
                                TextStyle {
                                    font: theme.font.clone(),
                                    font_size: theme.input.size,
                                    color: Srgba::hex("F8F8FA").unwrap().into(),
                                },
                            ),
                            ..Default::default()
                        },
                        marker.clone(),
                        SpacingMarkerPosition::Top,
                        Name::new("Top"),
                    ));
                });
            let mut wrapper = builder.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            });

            if let Some(e) = child {
                wrapper.push_children(&[left, e, right]);
            }

            builder.spawn((
                TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect::vertical(Val::Px(SPACING_MARGIN_Y)),
                        ..default()
                    },
                    text: Text::from_section(
                        "9".to_string(),
                        TextStyle {
                            font: theme.font.clone(),
                            font_size: theme.input.size,
                            color: Srgba::hex("F8F8FA").unwrap().into(),
                        },
                    ),
                    ..Default::default()
                },
                marker.clone(),
                SpacingMarkerPosition::Bottom,
                Name::new("Bottom"),
            ));
        })
        .id()
}
#[derive(Component, Clone)]
enum SpacingMarker {
    Position,
    Margin,
    Border,
    Padding,
}
#[derive(Component)]
enum SpacingMarkerPosition {
    Left,
    Right,
    Top,
    Bottom,
}
#[derive(Component)]
struct SpacingDimensionsMarker;
fn spacing(commands: &mut Commands, theme: &Theme) -> Entity {
    let node_size = commands
        .spawn((
            NodeBundle {
                style: Style {
                    padding: UiRect::new(
                        Val::Px(12.0),
                        Val::Px(12.0),
                        Val::Px(12.0),
                        Val::Px(12.0),
                    ),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                border_color: BLACK.into(),
                background_color: Srgba::hex("407AA4").unwrap().into(),
                ..default()
            },
            Name::new("NodeSize"),
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "1000x370".to_string(),
                        TextStyle {
                            font: theme.font.clone(),
                            font_size: theme.input.size,
                            color: Srgba::hex("F8F8FA").unwrap().into(),
                        },
                    ),
                    ..Default::default()
                },
                SpacingDimensionsMarker,
            ));
        })
        .id();

    let padding = created_nested_spacing_indicator(
        "padding",
        commands,
        theme,
        "6657A6",
        node_size.into(),
        SpacingMarker::Padding,
    );
    let border = created_nested_spacing_indicator(
        "border",
        commands,
        theme,
        "38383D",
        padding.into(),
        SpacingMarker::Border,
    );
    let margin = created_nested_spacing_indicator(
        "margin",
        commands,
        theme,
        "73764A",
        border.into(),
        SpacingMarker::Margin,
    );
    let position = created_nested_spacing_indicator(
        "position",
        commands,
        theme,
        "222222",
        margin.into(),
        SpacingMarker::Position,
    );

    let mut container = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        Name::new("SpacingContainer"),
    ));
    container.push_children(&[position]);
    container.id()
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

#[derive(Component)]
struct ValInput {}

#[derive(Component)]
struct ValInputDropdown {}
pub fn create_val_thing(
    commands: &mut Commands,
    icons: &Icons,
    theme: &Theme,
    dropdown: Dropdown<ValTypes>,
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
