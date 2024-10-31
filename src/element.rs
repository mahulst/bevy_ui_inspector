use std::any::TypeId;

use bevy::{prelude::*, utils::HashMap};
#[derive(Debug)]
pub enum ElementChildren {
    Element(Element),
    Text(String, TextStyle),
    None,
}
impl Default for ElementChildren {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Default)]
pub struct Element {
    pub node: NodeBundle,
    pub components: HashMap<TypeId, Box<dyn Reflect>>,
    pub children: Vec<ElementChildren>,
}


impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for child in self.children.iter() {
            match child {
                ElementChildren::Element(element) => {
                    f.write_str(&format!("Element:\n{:?}", element));
                }
                ElementChildren::Text(text, _) => {
                    f.write_str(&format!("text: {:?}", text));
                }
                ElementChildren::None => {
                    f.write_str(&format!("none"));
                }
            }
        }
        Ok(())
    }
}
pub struct MyUiRect(Val, Val, Val, Val);

impl From<Val> for MyUiRect {
    fn from(value: Val) -> Self {
        Self(value, value, value, value)
    }
}
impl From<(Val, Val)> for MyUiRect {
    fn from(value: (Val, Val)) -> Self {
        Self(value.0, value.0, value.1, value.1)
    }
}
impl From<(Val, Val, Val, Val)> for MyUiRect {
    fn from(value: (Val, Val, Val, Val)) -> Self {
        Self(value.0, value.1, value.2, value.3)
    }
}

impl From<MyUiRect> for UiRect {
    fn from(val: MyUiRect) -> Self {
        UiRect::new(val.0, val.1, val.2, val.3)
    }
}

impl Element {
    pub fn add_component<T: Component + Reflect>(mut self, thing: T) -> Self {
        let component_data: Box<dyn Reflect> = Box::new(thing);
        let type_id = TypeId::of::<T>();

        self.components.insert(type_id, component_data);
        self
    }

    pub fn add_child_elements(mut self, children: impl Into<Vec<Element>>) -> Self {
        self.children
            .extend(children.into().into_iter().map(ElementChildren::Element));
        self
    }
    pub fn with_text(mut self, text: impl Into<String>, style: TextStyle) -> Self {
        self.children = vec![ElementChildren::Text(text.into(), style)];
        self
    }

    pub fn border_radius(self, rect: impl Into<MyUiRect>) -> Self {
        let rect = rect.into();
        let border_radius = BorderRadius::new(rect.0, rect.1, rect.2, rect.3);

        self.add_component(border_radius)
    }

    pub fn border_color(self, color: impl Into<Color>) -> Self {
        let border_color = BorderColor(color.into());

        self.add_component(border_color)
    }
    pub fn background_color(self, color: impl Into<Color>) -> Self {
        let bg_color = BackgroundColor(color.into());

        self.add_component(bg_color)
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
    index: Option<usize>,
) -> Entity {
    let node_id = world.spawn(my_struct.node).id();
    if let Some(p) = parent {
        let mut parent_e = world.entity_mut(p);
        if let Some(i) = index {
            parent_e.insert_children(i, &[node_id]);
        } else {
            parent_e.push_children(&[node_id]);
        }
    }
    for (type_id, data) in my_struct.components.into_iter() {
        insert_component_to_element(node_id, data, type_id, world);
    }
    for child in my_struct.children.into_iter() {
        match child {
            ElementChildren::Element(element) => {
                spawn_element_hierarchy(element, world, node_id.into(), None);
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
            ElementChildren::None => {}
        }
    }

    node_id
}

#[derive(Default)]
pub struct ComponentArgs {
    pub children: Vec<ElementChildren>,
}

impl<T> From<T> for ComponentArgs
where
    T: Into<Vec<Element>>,
{
    fn from(value: T) -> Self {
        ComponentArgs {
            children: value
                .into()
                .into_iter()
                .map(ElementChildren::Element)
                .collect(),
        }
    }
}
