use std::any::TypeId;

use bevy::{prelude::*, utils::HashMap};
pub struct Element {
    pub node: NodeBundle,
    pub components: HashMap<TypeId, Box<dyn Reflect + Send + Sync>>,
    pub children: Vec<Element>,
}

impl Element {
    pub fn add_component<T: Component + Reflect>(mut self, thing: T) -> Self {
        let component_data: Box<dyn Reflect> = Box::new(thing);
        let type_id = TypeId::of::<T>();

        self.components.insert(type_id, component_data);
        self
    }

    pub fn add_children(mut self, children: impl Into<Vec<Element>>) -> Self {
        self.children.extend(children.into());
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
    let id = world.spawn(my_struct.node).id();
    if let Some(p) = parent {
        let mut parent_e = world.entity_mut(p);
        parent_e.push_children(&[id]);
    }
    for (type_id, data) in my_struct.components.into_iter() {
        insert_component_to_element(id, data, type_id, world);
    }
    for child in my_struct.children.into_iter() {
        spawn_element_hierarchy(child, world, id.into());
    }
    id
}

#[derive(Default)]
pub struct ComponentArgs {
    pub children: Vec<Element>,
}

impl<T> From<T> for ComponentArgs
where
    T: Into<Vec<Element>>,
{
    fn from(value: T) -> Self {
        ComponentArgs {
            children: value.into(),
        }
    }
}
