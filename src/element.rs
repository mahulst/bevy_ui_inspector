use std::any::TypeId;

use bevy::{prelude::*, utils::HashMap};
#[derive(Debug)]
pub enum ElementChildren {
    Elements(Vec<Element>),
    Text(String, TextStyle),
}
impl Default for ElementChildren {
    fn default() -> Self {
        Self::Elements(vec![])
    }
}
pub struct Element {
    pub node: NodeBundle,
    pub components: HashMap<TypeId, Box<dyn Reflect + Send + Sync>>,
    pub children: ElementChildren,
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.children {
            ElementChildren::Elements(elements) => {
                for ele in elements {
                    f.write_str(&format!("Element:\n{:?}", ele));
                }
            }
            ElementChildren::Text(text, _) => {
                f.write_str(&format!("text: {:?}", text));
            }
        }
        Ok(())
    }
}

impl Element {
    pub fn add_component<T: Component + Reflect>(mut self, thing: T) -> Self {
        let component_data: Box<dyn Reflect> = Box::new(thing);
        let type_id = TypeId::of::<T>();

        self.components.insert(type_id, component_data);
        self
    }

    pub fn add_children(mut self, children: impl Into<Vec<Element>>) -> Self {
        match &mut self.children {
            ElementChildren::Elements(current_children) => current_children.extend(children.into()),
            ElementChildren::Text(_, _) => {
                println!("Can't add elements to text child");
            }
        }
        self
    }
    pub fn with_text(mut self, text: impl Into<String>, style: TextStyle) -> Self {
        self.children = ElementChildren::Text(text.into(), style);
        self
    }

    pub fn with_style<F>(mut self, mut closure: F) -> Self
    where
        F: FnMut(&mut Style),
    {
        closure(&mut self.node.style);
        self
    }
}

pub fn insert_component_to_element(
    entity: Entity,
    component_data: Box<dyn Reflect>,
    type_id: TypeId,
    world: &mut World,
) {
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let registry = type_registry.read();

    if let Some(registration) = registry.get(type_id) {
        if let Some(reflect_component) = registration.data::<ReflectComponent>() {
            reflect_component.insert(&mut world.entity_mut(entity), &*component_data, &registry);
        } else {
            println!(
                "TypeId {:?} does not correspond to a ReflectComponent",
                type_id
            );
        }
    } else {
        println!("TypeId {:?} not found in TypeRegistry", type_id);
    }
}

pub fn spawn_element_hierarchy(
    my_struct: Element,
    world: &mut World,
    parent: Option<Entity>,
) -> Entity {
    let node_id = world.spawn(my_struct.node).id();
    if let Some(p) = parent {
        let mut parent_e = world.entity_mut(p);
        parent_e.push_children(&[node_id]);
    }
    for (type_id, data) in my_struct.components.into_iter() {
        insert_component_to_element(node_id, data, type_id, world);
    }
    match my_struct.children {
        ElementChildren::Elements(children) => {
            for child in children.into_iter() {
                spawn_element_hierarchy(child, world, node_id.into());
            }
        }
        ElementChildren::Text(text, style) => {
            let text_bundle = TextBundle {
                text: Text::from_section(text, style),
                ..Default::default()
            };

            let id = world.spawn(text_bundle).id();
            let mut parent_e = world.entity_mut(node_id);
            parent_e.push_children(&[id]);
        }
    }
    node_id
}

#[derive(Default)]
pub struct ComponentArgs {
    pub children: ElementChildren,
}

impl<T> From<T> for ComponentArgs
where
    T: Into<Vec<Element>>,
{
    fn from(value: T) -> Self {
        ComponentArgs {
            children: ElementChildren::Elements(value.into()),
        }
    }
}
