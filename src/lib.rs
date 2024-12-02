use bevy::{
    color::palettes::{css::RED, tailwind::BLUE_100},
    prelude::*,
    render::view::RenderLayers,
    ui::NodeQuery,
};
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts, EguiPlugin,
};
// pub mod dropdown;
// pub mod element;
// pub mod icons;
// pub mod input_helpers;
// pub mod node_hierarchy;
// pub mod number_input;
// pub mod theme;
// pub mod val;
// pub mod val_input;

#[derive(Default, Copy, PartialEq, Eq, Clone, Debug, Reflect)]
pub enum ValTypes {
    #[default]
    Auto,
    Px,
    Percent,
    Vw,
    Vh,
    VMin,
    VMax,
}
impl std::fmt::Display for ValTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValTypes::Auto => f.write_str("a"),
            ValTypes::Px => f.write_str("px"),
            ValTypes::Percent => f.write_str("%"),
            ValTypes::Vw => f.write_str("vw"),
            ValTypes::Vh => f.write_str("vh"),
            ValTypes::VMin => f.write_str("vmin"),
            ValTypes::VMax => f.write_str("vmax"),
        }
    }
}
#[derive(Resource, Default)]
pub struct RestorePreviousResource {
    pub selected: Option<Entity>,
    hovered: Option<Entity>,
}
#[derive(Resource, Default)]
pub struct ActiveStyleInspection {
    pub entity: Option<Entity>,
}
#[derive(Resource, Default)]
struct PickingUiNode {
    is_picking: bool,
}

fn val_dropdown(ui: &mut Ui, val: &mut ValTypes, id: &str) -> bool {
    let mut has_changed = false;
    egui::ComboBox::from_id_salt(id)
        .selected_text(format!("{}", val))
        .width(12.0)
        .show_ui(ui, |ui| {
            has_changed |= ui.selectable_value(val, ValTypes::Auto, "auto").changed();
            has_changed |= ui.selectable_value(val, ValTypes::Px, "px").changed();
            has_changed |= ui.selectable_value(val, ValTypes::Percent, "%").changed();
            has_changed |= ui.selectable_value(val, ValTypes::Vw, "vw").changed();
            has_changed |= ui.selectable_value(val, ValTypes::Vh, "vh").changed();
        });
    has_changed
}

fn val_input(ui: &mut Ui, val: &mut Val, id: &str) {
    let mut original_val = match val {
        Val::Px(_) => ValTypes::Px,
        Val::Percent(_) => ValTypes::Percent,
        Val::Vw(_) => ValTypes::Vw,
        Val::Vh(_) => ValTypes::Vh,
        Val::VMin(_) => ValTypes::VMin,
        Val::VMax(_) => ValTypes::VMax,
        Val::Auto => ValTypes::Auto,
    };
    let default: f32 = 0.0;
    let mut v = match val {
        Val::Px(val) => *val,
        Val::Percent(val) => *val,
        _ => default,
    };

    let input = ui.add(egui::DragValue::new(&mut v));
    let type_changed = val_dropdown(ui, &mut original_val, id);
    if input.changed() || type_changed {
        *val = match original_val {
            ValTypes::Auto => Val::Auto,
            ValTypes::Px => Val::Px(v),
            ValTypes::Percent => Val::Percent(v),
            ValTypes::Vw => Val::Vw(v),
            ValTypes::Vh => Val::Vh(v),
            ValTypes::VMin => Val::VMin(v),
            ValTypes::VMax => Val::VMax(v),
        };
    }
}

macro_rules! enum_dropdown {
    ($ui:expr, $label:expr, $enum_type:ty, $selected_value:expr, $($variant:ident),*) => {{
        egui::ComboBox::from_id_salt($label)
            .selected_text(format!("{:?}", $selected_value))
            .show_ui($ui, |ui| {
                $(
                    ui.selectable_value(
                        &mut $selected_value,
                        <$enum_type>::$variant,
                        stringify!($variant),
                    );
                )*
            });
    }};
}

fn ui_node_hit_test_system(
    windows: Query<&Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    node_query: Query<
        (Entity, &GlobalTransform, &ComputedNode),
        (Without<HoverUiElementWrapperMarker>, Without<HoverUiElementMarker>),
    >,
    node_q: Query<(&ComputedNode, &GlobalTransform)>,
    mut previous_resource: ResMut<RestorePreviousResource>,
    mut style_under_inspection: ResMut<ActiveStyleInspection>,
    hovered_ui_wrapper_q: Query<Entity, With<HoverUiElementWrapperMarker>>,
    hovered_ui_q: Query<(Entity, &Node), With<HoverUiElementMarker>>,
    mut picking_ui_node: ResMut<PickingUiNode>,
    mut gizmos: Gizmos,
    mut commands: Commands,
    ui_scale: Res<UiScale>,
) {
    let window = windows.get_single().unwrap();
    let entity_m: Option<Entity> = if picking_ui_node.is_picking {
        window.cursor_position().and_then(|cursor_position| {
            let mut nodes_under_cursor = Vec::new();

            for (entity, global_transform, node) in node_query.iter() {
                let rect =
                    Rect::from_center_size(global_transform.translation().truncate(), node.size());
                let scale: f32 = window.scale_factor() / ui_scale.0;
                let position = Rect {
                    min: Vec2::new(rect.min.x / scale, rect.min.y / scale),
                    max: Vec2::new(rect.max.x / scale, rect.max.y / scale),
                };

                dbg!(scale, position, node.size());
                if position.contains(cursor_position) {
                    let z = node.stack_index();

                    nodes_under_cursor.push((entity, z));
                }
            }

            nodes_under_cursor.sort_by(|a, b| b.1.cmp(&a.1));

            nodes_under_cursor.first().map(|(top_entity, _)| {
                if mouse_button_input.just_pressed(MouseButton::Left) {
                    previous_resource.selected = Some(*top_entity);
                    style_under_inspection.entity = Some(*top_entity);
                    previous_resource.hovered = None;
                }
                *top_entity
            })
        })
    } else {
        previous_resource.hovered.or(previous_resource.selected)
    };
    if let Some(entity) = entity_m {
        if let Ok((node, tf)) = node_q.get(entity) {
            let rect = Rect::from_center_size(tf.translation().truncate(), node.size());
            let scale: f32 = window.scale_factor() / ui_scale.0;
            let pos = Rect {
                min: Vec2::new(rect.min.x / scale, rect.min.y / scale),
                max: Vec2::new(rect.max.x / scale, rect.max.y / scale),
            };
            let left = window.width() / -2.0;

            let top = window.height() / 2.0;
            gizmos.line_2d(
                Vec2::new(left + pos.min.x, top),
                Vec2::new(left + pos.min.x, -top),
                Color::srgba(0.0, 0.0, 1.0, 0.3),
            );
            gizmos.line_2d(
                Vec2::new(left + pos.max.x, top),
                Vec2::new(left + pos.max.x, -top),
                Color::srgba(0.0, 0.0, 1.0, 0.3),
            );
            gizmos.line_2d(
                Vec2::new(left, top - pos.min.y),
                Vec2::new(-left, top - pos.min.y),
                Color::srgba(0.0, 0.0, 1.0, 0.3),
            );
            gizmos.line_2d(
                Vec2::new(left, top - pos.max.y),
                Vec2::new(-left, top - pos.max.y),
                Color::srgba(0.0, 0.0, 1.0, 0.3),
            );

            show_hovered_ui(&mut commands, &hovered_ui_wrapper_q, &hovered_ui_q, pos)
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Left) {
        picking_ui_node.is_picking = false;
    }
}

fn create_ui(
    mut contexts: EguiContexts,
    ui_root_q: Query<
        (Entity, Option<&Children>),
        (
            Without<Parent>,
            Without<HoverUiElementWrapperMarker>,
            With<Node>,
        ),
    >,
    parents_q: Query<&Parent, With<Node>>,
    ui_q: Query<(Entity, Option<&Children>, Option<&Name>), (With<Node>)>,
    mut style_q: Query<(&mut Node, &mut BorderColor, &mut BackgroundColor)>,
    mut previous_resource: ResMut<RestorePreviousResource>,
    mut style_under_inspection: ResMut<ActiveStyleInspection>,
    mut selected_node: Local<Option<Entity>>,
    mut collapse_all: Local<Option<bool>>,
    mut picking_ui_node: ResMut<PickingUiNode>,
) {
    // Used to open or close Node hierarchy when picking a UI node
    let mut open_on_change: Option<Entity> = None;
    if *selected_node != previous_resource.selected {
        *selected_node = previous_resource.selected;
        open_on_change = *selected_node;
    }

    let mut parents_of_selected = Vec::new();
    if let Some(selected) = previous_resource.selected {
        parents_of_selected.push(selected);
        let mut current_entity = selected;
        while let Ok(parent) = parents_q.get(current_entity) {
            parents_of_selected.push(parent.get());
            current_entity = parent.get();
        }
    }
    let mut something_hovered = false;
    egui::Window::new("UI Inspector").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("pick element").clicked() {
                picking_ui_node.is_picking = true;
            }
            let collapse = ui.button("collapse all");
            let expand = ui.button("expand all");

            if collapse.clicked() {
                *collapse_all = Some(false);
            } else if expand.clicked() {
                *collapse_all = Some(true);
            } else {
                *collapse_all = None;
            }
        });

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(200.0);
                ui.set_height(600.0);
                ui_root_q
                    .iter()
                    .enumerate()
                    .for_each(|(i, (root_e, children))| {
                        ui.label("Root");
                        egui::ScrollArea::vertical()
                            .id_salt(i.to_string())
                            .auto_shrink([false, true]) // Ensure the ScrollArea doesn't shrink automatically
                            .show(ui, |ui| {
                                let button = ui.button("Select");
                                if button.clicked() {
                                    previous_resource.selected = Some(root_e);
                                    style_under_inspection.entity = Some(root_e);
                                }

                                let something_within_root_hoverd = render_nested_elements(
                                    ui,
                                    &ui_q,
                                    children.into(),
                                    &mut previous_resource,
                                    &mut style_under_inspection,
                                    &parents_of_selected,
                                    &open_on_change,
                                    &collapse_all,
                                );
                                if something_within_root_hoverd {
                                    something_hovered = true;
                                }
                            });
                    });
            });
            ui.vertical(|ui| {
                if let Some((mut selected_style, _, _)) = previous_resource
                    .selected
                    .and_then(|selected_e| style_q.get_mut(selected_e).ok())
                {
                    ui.set_width(320.0);
                    ui.horizontal(|ui| {
                        ui.label("width:");
                        val_input(ui, &mut selected_style.width, "width");
                        ui.label("min:");
                        val_input(ui, &mut selected_style.min_width, "min-width");
                        ui.label("max:");
                        val_input(ui, &mut selected_style.max_width, "max-width");
                    });
                    ui.horizontal(|ui| {
                        ui.label("height:");
                        val_input(ui, &mut selected_style.height, "height");
                        ui.label("min:");
                        val_input(ui, &mut selected_style.min_height, "min-height");
                        ui.label("max:");
                        val_input(ui, &mut selected_style.max_height, "max-height");
                    });

                    ui.horizontal(|ui| {
                        ui.label("padding:");
                        val_input(ui, &mut selected_style.padding.left, "padding.left");
                        val_input(ui, &mut selected_style.padding.bottom, "padding.bottom");
                        val_input(ui, &mut selected_style.padding.right, "padding.right");
                        val_input(ui, &mut selected_style.padding.top, "padding.top");
                    });
                    ui.horizontal(|ui| {
                        ui.label("margin:");
                        val_input(ui, &mut selected_style.margin.left, "margin.left");
                        val_input(ui, &mut selected_style.margin.bottom, "margin.bottom");
                        val_input(ui, &mut selected_style.margin.right, "margin.right");
                        val_input(ui, &mut selected_style.margin.top, "margin.top");
                    });
                    ui.horizontal(|ui| {
                        ui.label("border:");
                        val_input(ui, &mut selected_style.border.left, "border.left");
                        val_input(ui, &mut selected_style.border.bottom, "border.bottom");
                        val_input(ui, &mut selected_style.border.right, "border.right");
                        val_input(ui, &mut selected_style.border.top, "border.top");
                    });
                    ui.horizontal(|ui| {
                        ui.label("pos:");
                        val_input(ui, &mut selected_style.left, "left");
                        val_input(ui, &mut selected_style.bottom, "bottom");
                        val_input(ui, &mut selected_style.right, "right");
                        val_input(ui, &mut selected_style.top, "top");
                    });

                    ui.horizontal(|ui| {
                        ui.label("display");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            enum_dropdown!(
                                ui,
                                "display",
                                Display,
                                selected_style.display,
                                Flex,
                                Grid,
                                Block,
                                None
                            );
                        });
                    });
                    ui.collapsing("flex", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("flex-direction");
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    enum_dropdown!(
                                        ui,
                                        "flex_direction",
                                        FlexDirection,
                                        selected_style.flex_direction,
                                        Row,
                                        Column
                                    );
                                },
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("align-items");
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            enum_dropdown!(
                                                ui,
                                                "align_items",
                                                AlignItems,
                                                selected_style.align_items,
                                                Default,
                                                Start,
                                                End,
                                                FlexStart,
                                                FlexEnd,
                                                Center,
                                                Baseline,
                                                Stretch
                                            );
                                        },
                                    );
                                },
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("justify-content");
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    enum_dropdown!(
                                        ui,
                                        "justify_content",
                                        JustifyContent,
                                        selected_style.justify_content,
                                        Default,
                                        Start,
                                        End,
                                        FlexStart,
                                        FlexEnd,
                                        Center,
                                        Stretch,
                                        SpaceBetween,
                                        SpaceEvenly,
                                        SpaceAround
                                    );
                                },
                            );
                        });
                    });
                } else {
                    ui.set_width(0.0);
                }
            });
        });
    });
    if !something_hovered {
        previous_resource.hovered = None;
    }
}

fn render_nested_elements(
    ui: &mut Ui,
    ui_q: &Query<(Entity, Option<&Children>, Option<&Name>), With<Node>>,
    children: Option<&Children>,
    previous_resource: &mut ResMut<RestorePreviousResource>,
    style_under_inspection: &mut ResMut<ActiveStyleInspection>,
    parents_of_selected: &Vec<Entity>,
    open_on_change: &Option<Entity>,
    collapse_all: &Option<bool>,
) -> bool {
    let mut something_hovered = false;
    if let Some(children) = children {
        children.iter().for_each(|child| {
            if let Ok((e, children, name)) = ui_q.get(*child) {
                let name = match name {
                    Some(n) => format!("{} ({})", n.as_str(), e),
                    None => format!("{}", e),
                };
                let is_selected = previous_resource.selected == Some(*child);

                let style = ui.style().clone();

                ui.set_style(style);
                let mut label = egui::RichText::new(name);
                if is_selected {
                    label = label
                        .color(egui::Color32::from_rgb(255, 255, 255))
                        .background_color(egui::Color32::from_rgb(100, 149, 237));
                }
                let thing = egui::CollapsingHeader::new(label)
                    .open(
                        collapse_all
                            .or(open_on_change.map(|_| parents_of_selected.contains(child))),
                    )
                    .show(ui, |ui| {
                        let button = ui.button("Select");
                        if button.clicked() {
                            previous_resource.selected = Some(*child);
                            style_under_inspection.entity = Some(*child);
                        }

                        let something_already_hovered = render_nested_elements(
                            ui,
                            ui_q,
                            children,
                            previous_resource,
                            style_under_inspection,
                            parents_of_selected,
                            open_on_change,
                            collapse_all,
                        );
                        if something_already_hovered {
                            something_hovered = true;
                        }
                    });
                if thing.header_response.hovered() {
                    previous_resource.hovered = Some(*child);
                    something_hovered = true;
                }
            }
        });
    }
    something_hovered
}
#[derive(Component)]
struct HoverUiElementWrapperMarker;
#[derive(Component)]
struct HoverUiElementMarker;

fn show_hovered_ui(
    commands: &mut Commands,
    hovered_ui_wrapper_q: &Query<Entity, With<HoverUiElementWrapperMarker>>,
    hovered_ui_q: &Query<(Entity, &Node), With<HoverUiElementMarker>>,
    pos: Rect,
) {
    let new_style = Node { ..default() };

    if let Ok((e, old_style)) = hovered_ui_q.get_single() {
        if old_style.left == new_style.left
            && old_style.top == new_style.top
            && old_style.width == new_style.width
            && old_style.height == new_style.height
        {
            return;
        }
    }

    for entity in hovered_ui_wrapper_q {
        commands.entity(entity).despawn_recursive();
    }

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            GlobalZIndex(i32::MAX),
            HoverUiElementWrapperMarker,
        ))
        .with_children(|builder| {
            builder.spawn((
                Node {
                    left: Val::Px(pos.min.x),
                    top: Val::Px(pos.min.y),
                    width: Val::Px(pos.max.x - pos.min.x),
                    height: Val::Px(pos.max.y - pos.min.y),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 1.0, 0.3)),
                HoverUiElementMarker,
            ));
        });
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::None,
            order: 4,
            ..default()
        },
        RenderLayers::layer(10),
        Name::new("Plugin camera"),
    ));
}
pub struct UiInspectorPlugin;
impl Plugin for UiInspectorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        app.insert_gizmo_config(
            DefaultGizmoConfigGroup,
            GizmoConfig {
                render_layers: RenderLayers::layer(10),
                ..default()
            },
        );
        app.insert_resource(RestorePreviousResource::default());
        app.insert_resource(ActiveStyleInspection::default());
        app.insert_resource(PickingUiNode::default());
        app.add_systems(Update, (create_ui, ui_node_hit_test_system));
        app.add_systems(Startup, (setup));
    }
}
