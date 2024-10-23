use bevy::{
    color::palettes::tailwind::{GREEN_100, GREEN_400},
    prelude::*,
};
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts, EguiPlugin,
};
use std::mem;

#[derive(Resource, Default)]
struct RestorePreviousResource {
    previous: Option<PreviousElementHighlighted>,
    selected: Option<Entity>,
}
#[derive(Resource, Default)]
struct PickingUiNode {
    is_picking: bool,
}
struct PreviousElementHighlighted {
    entity: Entity,
    border_color: Color,
    background_color: Color,
    border_width: UiRect,
}

#[derive(Default, PartialEq, Eq)]
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
    node_query: Query<(Entity, &GlobalTransform, &Node)>,
    mut style_q: Query<(&mut Style, &mut BorderColor, &mut BackgroundColor)>,
    mut previous_resource: ResMut<RestorePreviousResource>,
    mut picking_ui_node: ResMut<PickingUiNode>,
) {
    if !picking_ui_node.is_picking {
        return;
    }

    let window = windows.get_single().unwrap();
    if let Some(cursor_position) = window.cursor_position() {
        let mut nodes_under_cursor = Vec::new();

        for (entity, global_transform, node) in node_query.iter() {
            let position = node.logical_rect(global_transform);

            if position.contains(cursor_position) {
                let z = node.stack_index();

                nodes_under_cursor.push((entity, z));
            }
        }

        nodes_under_cursor.sort_by(|a, b| b.1.cmp(&a.1));

        if let Some((top_entity, _)) = nodes_under_cursor.first() {
            color_hovered_element(&mut previous_resource, &mut style_q, top_entity);
            if mouse_button_input.just_pressed(MouseButton::Left) {
                previous_resource.selected = Some(*top_entity);
            }
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        picking_ui_node.is_picking = false;
    }
}

fn create_ui(
    mut contexts: EguiContexts,
    ui_root_q: Query<(Entity, &Node, &Children), Without<Parent>>,
    parents_q: Query<&Parent, With<Node>>,
    ui_q: Query<(Entity, &Node, Option<&Children>, Option<&Name>)>,
    mut style_q: Query<(&mut Style, &mut BorderColor, &mut BackgroundColor)>,
    mut previous_resource: ResMut<RestorePreviousResource>,
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
                ui.set_min_height(600.0);
                ui_root_q.iter().for_each(|(_, _, children)| {
                    ui.label("Root");
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false]) // Ensure the ScrollArea doesn't shrink automatically
                        .show(ui, |ui| {
                            render_nested_elements(
                                ui,
                                &ui_q,
                                children.into(),
                                &mut style_q,
                                &mut previous_resource,
                                &parents_of_selected,
                                &open_on_change,
                                &collapse_all,
                            );
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
}
fn color_hovered_element(
    previous_resource: &mut ResMut<RestorePreviousResource>,
    style_q: &mut Query<(&mut Style, &mut BorderColor, &mut BackgroundColor)>,
    ui_entity: &Entity,
) {
    // Restore previous
    let previous = mem::take(&mut previous_resource.previous);
    if let Some(prev) = previous {
        if let Ok((mut style, mut border, mut background_color)) = style_q.get_mut(prev.entity) {
            style.border = prev.border_width;
            border.0 = prev.border_color;
            background_color.0 = prev.background_color;
        }
    }

    if let Ok((mut style, mut border, mut background_color)) = style_q.get_mut(*ui_entity) {
        // Store current styles before changing
        previous_resource.previous = Some(PreviousElementHighlighted {
            entity: *ui_entity,
            border_color: border.0,
            background_color: background_color.0,
            border_width: style.border,
        });

        // Update current style
        style.border = UiRect::all(Val::Px(2.0));
        border.0 = GREEN_400.into();
        background_color.0 = GREEN_100.into();
    }
}
fn render_nested_elements(
    ui: &mut Ui,
    ui_q: &Query<(Entity, &Node, Option<&Children>, Option<&Name>)>,
    children: Option<&Children>,
    style_q: &mut Query<(&mut Style, &mut BorderColor, &mut BackgroundColor)>,
    previous_resource: &mut ResMut<RestorePreviousResource>,
    parents_of_selected: &Vec<Entity>,
    open_on_change: &Option<Entity>,
    collapse_all: &Option<bool>,
) {
    if let Some(children) = children {
        children.iter().for_each(|child| {
            if let Ok((e, _, children, name)) = ui_q.get(*child) {
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
                        }

                        render_nested_elements(
                            ui,
                            ui_q,
                            children,
                            style_q,
                            previous_resource,
                            parents_of_selected,
                            open_on_change,
                            collapse_all,
                        );
                    });
                if thing.header_response.hovered() {
                    color_hovered_element(previous_resource, style_q, child);
                }
            }
        });
    }
}
pub struct UiInspectorPlugin;
impl Plugin for UiInspectorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        app.insert_resource(RestorePreviousResource::default());
        app.insert_resource(PickingUiNode::default());
        app.add_systems(Update, (create_ui, ui_node_hit_test_system));
    }
}
