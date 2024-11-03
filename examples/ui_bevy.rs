use std::{clone, fmt};

use bevy::color::palettes::css::BLACK;
use bevy::ecs::reflect;
use bevy::ecs::system::SystemState;
use bevy::ui::widget::UiImageSize;
use bevy::ui::{ContentSize, FocusPolicy};
use bevy::{color::palettes::tailwind::*, prelude::*, window::WindowResolution};
use bevy_ui_inspector::dropdown::{
    self, Dropdown, DropdownBox, DropdownItem, DropdownPlugin, DropdownSelected,
};
use bevy_ui_inspector::element::{spawn_element_hierarchy, ComponentArgs, Components, Element};
use bevy_ui_inspector::icons::{setup_icons, Icons};
use bevy_ui_inspector::input_helpers::{Initializing, Stable};
use bevy_ui_inspector::number_input::{NumberInputPlugin, TextInput};
use bevy_ui_inspector::theme::Theme;
use bevy_ui_inspector::val::ValExt;
use bevy_ui_inspector::val_input::{
    val_input, get_dropdown_value_for_val_type, get_number_val, get_val_type, get_val_type_for_dropdown_value, ValInput, ValInputDropdown, ValInputDropdownPlugin, ValTypes
};
use bevy_ui_inspector::{ActiveStyleInspection, UiInspectorPlugin};
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
        .add_plugins(UiInspectorPlugin)
        .add_plugins(DropdownPlugin)
        .add_plugins(ValInputDropdownPlugin)
        .add_plugins(NumberInputPlugin)
        .register_type::<SpacingMarkerPosition>()
        .register_type::<SpacingMarker>()
        .register_type::<SpacingDimensionsMarker>()
        .register_type::<ValTypeLink>()
        .register_type::<Initializing>()
        .add_systems(Startup, setup_icons)
        .add_systems(Startup, spawn_layout.after(setup_icons))
        .add_systems(Startup, spawn_test.after(spawn_layout))
        .add_systems(Update, spawn_ui_on_event)
        .add_systems(
            Update,
            ((
                update_spacing_markers,
                update_style_panel,
                update_style_property,
            )
                .chain()),
        )
        .run();
}

fn spawn_ui_on_event(world: &mut World) {
    let mut spawn_ui_events = world.get_resource_mut::<Events<SpawnUiEvent>>().unwrap();

    let events: Vec<SpawnUiEvent> = spawn_ui_events.update_drain().collect();

    for spawn_ui_e in events {
        spawn_element_hierarchy(
            spawn_ui_e.element,
            world,
            spawn_ui_e.parent,
            spawn_ui_e.index,
        );
    }
}

fn spawn_test(theme: Res<Theme>, icons: Res<Icons>, mut spawn_ui_e: EventWriter<SpawnUiEvent>) {}

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
    spawn_element_hierarchy(element_to_inspect, world, None, None);
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


        let margin_left = val_input(&icons, &theme).add_component(ValTypeLink::MarginLeft);
        let margin_right = val_input(&icons, &theme).add_component(ValTypeLink::MarginRight);
        let margin_top = val_input(&icons, &theme).add_component(ValTypeLink::MarginTop);
        let margin_bottom =
            val_input(&icons, &theme).add_component(ValTypeLink::MarginBottom);
        let padding_left = val_input(&icons, &theme).add_component(ValTypeLink::PaddingLeft);
        let padding_right =
            val_input(&icons, &theme).add_component(ValTypeLink::PaddingRight);
        let padding_top = val_input(&icons, &theme).add_component(ValTypeLink::PaddingTop);
        let padding_bottom =
            val_input(&icons, &theme).add_component(ValTypeLink::PaddingBottom);
        let border_left = val_input(&icons, &theme).add_component(ValTypeLink::BorderLeft);
        let border_right = val_input(&icons, &theme).add_component(ValTypeLink::BorderRight);
        let border_top = val_input(&icons, &theme).add_component(ValTypeLink::BorderTop);
        let border_bottom =
            val_input(&icons, &theme).add_component(ValTypeLink::BorderBottom);
        let position_left =
            val_input(&icons, &theme).add_component(ValTypeLink::PositionLeft);
        let position_right =
            val_input(&icons, &theme).add_component(ValTypeLink::PositionRight);
        let position_top = val_input(&icons, &theme).add_component(ValTypeLink::PositionTop);
        let position_bottom =
            val_input(&icons, &theme).add_component(ValTypeLink::PositionBottom);
        let width = val_input(&icons, &theme).add_component(ValTypeLink::Width);
        let min_width = val_input(&icons, &theme).add_component(ValTypeLink::MinWidth);
        let max_width = val_input(&icons, &theme).add_component(ValTypeLink::MaxWidth);
        let height = val_input(&icons, &theme).add_component(ValTypeLink::Height);
        let min_height = val_input(&icons, &theme).add_component(ValTypeLink::MinHeight);
        let max_height = val_input(&icons, &theme).add_component(ValTypeLink::MaxHeight);
        let title_text_style = TextStyle {
            font: theme.font.clone(),
            font_size: theme.input.size,
            color: theme.input.color,
        };

        let width_title = Element::text_with_style("width", title_text_style.clone());
        let height_title = Element::text_with_style("height", title_text_style.clone());
        let min_title = Element::text_with_style("min", title_text_style.clone());
        let max_title = Element::text_with_style("max", title_text_style.clone());
        let margin_title = Element::text_with_style("margin", title_text_style.clone());
        let padding_title = Element::text_with_style("padding", title_text_style.clone());
        let border_title = Element::text_with_style("border", title_text_style.clone());
        let position_title = Element::text_with_style("position", title_text_style.clone());
        let left_title = Element::text_with_style("left", title_text_style.clone());
        let right_title = Element::text_with_style("right", title_text_style.clone());
        let top_title = Element::text_with_style("top", title_text_style.clone());
        let bottom_title = Element::text_with_style("bottom", title_text_style.clone());

        let empty = Element::empty();
        let empty2 = Element::empty();
        let empty3 = Element::empty();

        let spacing_thing = spacing(&theme);
        let grid = Element::default()
            .with_style(|style| {
                style.display = Display::Grid;
                style.grid_template_columns = RepeatedGridTrack::min_content(5);
                style.grid_template_rows = RepeatedGridTrack::min_content(5);
                style.row_gap = Val::Px(12.0);
                style.column_gap = Val::Px(12.0);
            })
            .add_component(Name::new("UiRectGrid"))
            .add_child_elements([
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
        let grid_dimensions = Element::default()
            .with_style(|style| {
                style.display = Display::Grid;
                style.grid_template_columns = RepeatedGridTrack::min_content(4);
                style.grid_template_rows = RepeatedGridTrack::min_content(3);
                style.row_gap = Val::Px(12.0);
                style.column_gap = Val::Px(12.0);
            })
            .add_component(Name::new("Dimensions"))
            .add_child_elements([
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
        let mut dd_container = Element::default()
            .background_color(theme.background)
            .with_style(|style| {
                style.flex_direction = FlexDirection::Column;
                style.align_items = AlignItems::FlexEnd;
                style.position_type = PositionType::Absolute;
                style.padding = UiRect::all(Val::Px(12.0));
                style.row_gap = Val::Px(12.0);
                style.width = Val::Px(500.0);
                style.height = Val::Percent(100.0);
                style.right = Val::Px(0.0);
                style.top = Val::Px(0.0);
            })
            .add_component(Name::new("StylePanel"))
            .add_child_elements([spacing_thing, grid, grid_dimensions]);

        spawn_ui_event_writer.send(SpawnUiEvent {
            element: dd_container,
            parent: None,
            index: None,
        });
    }
    system_state.apply(world);
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

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub enum ValTypeLink {
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

fn update_style_property(
    mut style_q: Query<&mut Style>,
    active_style_inspection: Res<ActiveStyleInspection>,
    val_input_q: Query<(&ValInput, &ValTypeLink, Entity)>,
    val_input_dropdown_q: Query<(Ref<Dropdown>, Entity), With<Stable>>,
    text_input_q: Query<(Ref<TextInput>, Entity), With<Stable>>,
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
                            println!("Setting style as val input changed");
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
fn spacing(theme: &Theme) -> Element {
    let node_size = Element::default()
        .with_style(|style| {
            style.padding = UiRect::all(12.0.px());
            style.border = UiRect::all(1.0.px());
        })
        .border_color(BLACK)
        .background_color(Srgba::hex("407AA4").unwrap())
        .add_component(Name::new("NodeSize"))
        .with_text_and_components(
            "1000x370",
            TextStyle {
                font: theme.font.clone(),
                font_size: theme.input.size,
                color: Srgba::hex("F8F8FA").unwrap().into(),
            },
            Components::new().add(SpacingDimensionsMarker {}),
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
    container
}
