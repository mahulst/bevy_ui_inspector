use crate::{
    element::{Element, SpawnElement, SpawnUiEvent},
    icons::Icons,
    node_hierarchy,
    theme::Theme,
    val::ValExt,
};
use bevy::{
    prelude::*,
    ui::{widget::UiImageSize, ContentSize},
    utils::HashMap,
};
pub struct NodeHierarchyPlugin;
impl Plugin for NodeHierarchyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            create_ui_node_hierarchy.before(update_hierarchy_on_children_change),
        )
        .add_systems(Update, update_hierarchy_on_children_change)
        .add_systems(PostUpdate, detect_removals)
        .add_systems(Update, update_hierarchy)
        .add_systems(Update, toggle_chevrons)
        .register_type::<RootHierarchyContainer>()
        .register_type::<HierarchyViewIgnore>()
        .register_type::<NodeChevron>()
        .register_type::<NodeHierarchyMarker>()
        .register_type::<NodeHierarchyChildrenMarker>()
        .insert_resource(Roots::default())
        .insert_resource(OpenedNodes::default())
        .add_event::<HierarchyChangedEvent>();
    }
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HierarchyViewIgnore;

struct NodeParent(Entity);
#[derive(Event)]
enum HierarchyChangedEvent {
    RootAdded(Entity),
    ChildAdded(Entity, NodeParent, usize),
    ChildRemoved(Entity),
}

fn update_hierarchy(
    mut events: EventReader<HierarchyChangedEvent>,
    hierarchy_container_q: Query<Entity, With<RootHierarchyContainer>>,
    hierarchy_q: Query<(Entity, &NodeHierarchyMarker)>,
    theme: Res<Theme>,
    icons: Res<Icons>,
    roots: Res<Roots>,
    mut commands: Commands,
    mut opened_nodes: ResMut<OpenedNodes>,
) {
    if let Ok(hierarchy_container_e) = hierarchy_container_q.get_single() {
        for event in events.read() {
            match event {
                HierarchyChangedEvent::RootAdded(root_e) => {
                    println!("root added {}", root_e);
                    if let Some(hierarchy) = roots.roots.iter().find(|root| root.entity == *root_e)
                    {
                        let h = node_hierarchy(&theme, &icons, hierarchy, 0);

                        commands.add(SpawnElement {
                            element: h,
                            parent: hierarchy_container_e.into(),
                            index: None,
                        });
                        opened_nodes.nodes.insert(hierarchy.entity, false);
                    }
                }
                HierarchyChangedEvent::ChildAdded(_, _, _) => todo!(),
                HierarchyChangedEvent::ChildRemoved(node_e) => {
                    println!("removed {}", node_e);
                    let a = roots.roots.iter().find_map(|root| find_node(root, *node_e));
                    let a = hierarchy_q
                        .iter()
                        .find(|(el_entity, node_hierarchy_marker)| {
                            node_hierarchy_marker.entity == *node_e
                        });
                    if let Some((node_hierarchy_el_e, _)) = a {
                        commands.entity(node_hierarchy_el_e).despawn_recursive();
                    } else {
                        println!("Not found");
                    }
                }
            }
        }
    }
}
#[derive(Component, Reflect)]
#[reflect(Component)]
struct NodeChevron {
    entity: Entity,
}
#[derive(Component, Reflect)]
#[reflect(Component)]
struct NodeHierarchyMarker {
    entity: Entity,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct NodeHierarchyChildrenMarker {
    entity: Entity,
}

#[derive(Resource, Default)]
pub struct OpenedNodes {
    nodes: HashMap<Entity, bool>,
}

fn toggle_chevrons(
    mut chevrons_q: Query<(Entity, &Interaction, &NodeChevron, &mut UiImage), Changed<Interaction>>,
    parents_q: Query<&Parent>,
    children_q: Query<&Children>,
    node_hierarchy_q: Query<(Entity, &NodeHierarchyMarker)>,
    mut node_hierarchy_children_q: Query<(Entity, &mut Style, &NodeHierarchyChildrenMarker)>,
    mut opened_nodes: ResMut<OpenedNodes>,
    icons: Res<Icons>,
) {
    chevrons_q.iter_mut().for_each(
        |(entity, interaction, chevron, mut image)| match interaction {
            Interaction::Pressed => {
                let is_opened = opened_nodes.nodes.entry(chevron.entity).or_insert(true);
                *is_opened = !*is_opened;
                match is_opened {
                    true => {
                        *image = UiImage::new(icons.chevron_down.clone());
                    }
                    false => {
                        *image = UiImage::new(icons.chevron_up.clone());
                    }
                }

                parents_q.iter_ancestors(entity).for_each(|parent| {
                    if let Ok((node_hierarchy_el_e, marker)) = node_hierarchy_q.get(parent) {
                        if marker.entity != chevron.entity {
                            return;
                        }
                        children_q
                            .iter_descendants(node_hierarchy_el_e)
                            .for_each(|child| {
                                if let Ok((_, mut style, child_marker)) =
                                    node_hierarchy_children_q.get_mut(child)
                                {
                                    if child_marker.entity != chevron.entity {
                                        return;
                                    }
                                    style.display = match is_opened {
                                        true => Display::Flex,
                                        false => Display::None,
                                    }
                                }
                            });
                    }
                });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        },
    );
}

pub fn node_hierarchy(
    theme: &Theme,
    icons: &Icons,
    hierarchy: &NodeHierarchy,
    level: u32,
) -> Element {
    let name = hierarchy
        .name
        .clone()
        .unwrap_or(hierarchy.entity.to_string());
    let chevron_icon = Element::default()
        .with_style(|style| {
            style.width = 8.0.px();
            style.height = 8.0.px();
            style.margin = UiRect::right(4.0.px());
        })
        .add_component(UiImage::new(icons.chevron_down.clone()))
        .add_component(UiImageSize::default())
        .add_component(Interaction::None)
        .add_component(NodeChevron {
            entity: hierarchy.entity,
        })
        .add_component(ContentSize::default());

    let node_name = Element::default().with_text(
        name,
        TextStyle {
            color: theme.hierarchy.color,
            font_size: theme.hierarchy.size,
            ..default()
        },
    );
    let row = Element::default()
        .with_style(|style| {
            style.align_items = AlignItems::Center;
        })
        .add_child_elements([chevron_icon, node_name]);

    let children: Vec<Element> = hierarchy
        .children
        .iter()
        .map(|child| node_hierarchy(theme, icons, child, level + 1))
        .collect();
    let children_container = Element::default()
        .add_component(NodeHierarchyChildrenMarker {
            entity: hierarchy.entity,
        })
        .with_style(|style| {
            style.flex_direction = FlexDirection::Column;
            style.padding = UiRect::left(8.0.px());
            if level == 0 {
                style.display = Display::None;
            }
        })
        .add_child_elements(children);

    Element::default()
        .add_component(NodeHierarchyMarker {
            entity: hierarchy.entity,
        })
        .with_style(|style| {
            style.flex_direction = FlexDirection::Column;
        })
        .add_child_elements([row, children_container])
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RootHierarchyContainer;

#[derive(Default, Resource)]
pub struct Roots {
    pub roots: Vec<NodeHierarchy>,
}

fn find_node_mut(node: &mut NodeHierarchy, entity: Entity) -> Option<&mut NodeHierarchy> {
    if node.entity == entity {
        Some(node)
    } else {
        for child in &mut node.children {
            if let Some(found) = find_node_mut(child, entity) {
                return Some(found);
            }
        }
        None
    }
}

fn find_node(node: &NodeHierarchy, entity: Entity) -> Option<&NodeHierarchy> {
    if node.entity == entity {
        Some(node)
    } else {
        for child in &node.children {
            if let Some(found) = find_node(child, entity) {
                return Some(found);
            }
        }
        None
    }
}

pub fn detect_removals(
    mut removals: RemovedComponents<Node>,
    mut roots: ResMut<Roots>,
    node_hierarchy_q: Query<(&NodeHierarchyMarker)>,
    mut hierarchy_changed_e: EventWriter<HierarchyChangedEvent>,
) {
    for entity in removals.read() {
        if !node_hierarchy_q.get(entity).is_ok() {
            remove_node_from_hierarchy_stack(entity, &mut roots);
            hierarchy_changed_e.send(HierarchyChangedEvent::ChildRemoved(entity));
        }
    }
}

fn remove_node_from_hierarchy_stack(target_entity: Entity, roots: &mut Roots) -> bool {
    let mut stack = Vec::new();

    for i in (0..roots.roots.len()).rev() {
        stack.push((Vec::new(), i));
    }

    while let Some((parent_path, index)) = stack.pop() {
        let mut parent_nodes = &mut roots.roots;
        for &idx in &parent_path {
            parent_nodes = &mut parent_nodes[idx as usize].children;
        }

        if index >= parent_nodes.len() {
            continue;
        }

        let node = &parent_nodes[index];

        if node.entity == target_entity {
            parent_nodes.remove(index);
            return true;
        } else {
            let children_len = node.children.len();
            let mut child_parent_path = parent_path.clone();
            child_parent_path.push(index);
            for i in (0..children_len).rev() {
                stack.push((child_parent_path.clone(), i));
            }
        }
    }

    false
}
pub fn update_hierarchy_on_children_change(
    ui_q: Query<(Entity, Option<&Children>, Option<&Name>)>,
    changed_children_query: Query<Entity, Changed<Children>>,
    mut roots: ResMut<Roots>,
) {
    for entity in changed_children_query.iter() {
        for root in roots.roots.iter_mut() {
            if let Some(node) = find_node_mut(root, entity) {
                node.children.clear();
                if let Ok((_, children_opt, _)) = ui_q.get(entity) {
                    if let Some(children) = children_opt {
                        for &child_entity in children.iter() {
                            build_node_hierarchy(child_entity, node, &ui_q);
                        }
                    }
                }
                break;
            }
        }
    }

    if !changed_children_query.is_empty() {
        for root in &roots.roots {
            // print_node_hierarchy(root, 0);
        }
    }
}

fn build_node_hierarchy(
    entity: Entity,
    parent_node: &mut NodeHierarchy,
    ui_q: &Query<(Entity, Option<&Children>, Option<&Name>)>,
) {
    if let Ok((child_entity, children_opt, name_opt)) = ui_q.get(entity) {
        let mut child_node = NodeHierarchy {
            name: name_opt.map(|n| n.to_string()),
            entity: child_entity,
            children: Vec::new(),
        };
        if let Some(children) = children_opt {
            for &grandchild_entity in children.iter() {
                build_node_hierarchy(grandchild_entity, &mut child_node, ui_q);
            }
        }
        parent_node.children.push(child_node);
    }
}
pub struct NodeHierarchy {
    name: Option<String>,
    entity: Entity,
    children: Vec<NodeHierarchy>,
}

pub fn create_ui_node_hierarchy(
    ui_root_q: Query<(Entity, Option<&Children>, Option<&Name>), (Without<Parent>, Added<Node>)>,
    ui_q: Query<(Entity, Option<&Children>, Option<&Name>)>,
    mut roots: ResMut<Roots>,
    mut hierarchy_changed_e: EventWriter<HierarchyChangedEvent>,
) {
    let mut stack: Vec<(Entity, Vec<usize>)> = Vec::new(); // Stack containing (Entity, path to parent)

    // Initialize the roots
    for (root_entity, children_opt, name_opt) in ui_root_q.iter() {
        let root_node = NodeHierarchy {
            name: name_opt.map(|n| n.to_string()),
            entity: root_entity,
            children: Vec::new(),
        };
        roots.roots.push(root_node);
        let root_index = roots.roots.len() - 1;

        // Add root's children to the stack
        if let Some(children) = children_opt {
            for &child_entity in children.iter() {
                stack.push((child_entity, vec![root_index]));
            }
        }
    }

    // Process the stack
    while let Some((entity, parent_path)) = stack.pop() {
        if let Ok((child_entity, children_opt, name_opt)) = ui_q.get(entity) {
            let child_node = NodeHierarchy {
                name: name_opt.map(|n| n.to_string()),
                entity: child_entity,
                children: Vec::new(),
            };

            // Get mutable reference to the parent node using the path
            let parent_node = get_node_mut(&mut roots.roots, &parent_path);

            // Add the child to the parent's children
            parent_node.children.push(child_node);
            let child_index = parent_node.children.len() - 1;

            // If the child has its own children, add them to the stack with updated path
            if let Some(children) = children_opt {
                let mut child_path = parent_path.clone();
                child_path.push(child_index);
                for &grandchild_entity in children.iter() {
                    stack.push((grandchild_entity, child_path.clone()));
                }
            }
        }
    }

    for (root_entity, _, _) in ui_root_q.iter() {
        hierarchy_changed_e.send(HierarchyChangedEvent::RootAdded(root_entity));
    }
}

// Helper function to get mutable reference to a node using the path
fn get_node_mut<'a>(nodes: &'a mut [NodeHierarchy], path: &[usize]) -> &'a mut NodeHierarchy {
    let mut current_node = &mut nodes[path[0]];
    for &index in &path[1..] {
        current_node = &mut current_node.children[index];
    }
    current_node
}

// Optional: Function to print the hierarchy for debugging
fn print_node_hierarchy(node: &NodeHierarchy, level: usize) {
    let indent = "  ".repeat(level);
    println!("{}Entity: {:?}, Name: {:?}", indent, node.entity, node.name);
    for child in &node.children {
        print_node_hierarchy(child, level + 1);
    }
}
