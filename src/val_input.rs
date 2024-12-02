use crate::{
    dropdown::{dropdown, Dropdown, DropdownItem},
    element::Element,
    icons::Icons,
    input_helpers::Initializing,
    number_input::{input, TextInput},
    theme::Theme,
};
use bevy::prelude::*;

pub struct ValInputDropdownPlugin;
impl Plugin for ValInputDropdownPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ValInputDropdown>();
        app.register_type::<ValTypes>();
        app.register_type::<ValInput>();
        app.add_systems(Update, val_input_width_fixer);

    }
}
pub fn val_input(icons: &Icons, theme: &Theme) -> Element {
    let options = [
        DropdownItem {
            label: "auto".to_string(),
            value: 0,
        },
        DropdownItem {
            label: "px".to_string(),
            value: 1,
        },
        DropdownItem {
            label: "%".to_string(),
            value: 2,
        },
        DropdownItem {
            label: "vw".to_string(),
            value: 3,
        },
        DropdownItem {
            label: "vh".to_string(),
            value: 4,
        },
        DropdownItem {
            label: "vmin".to_string(),
            value: 5,
        },
        DropdownItem {
            label: "vmax".to_string(),
            value: 6,
        },
    ];
    let number_input_el = input(theme);
    let dropdown_el = dropdown(theme, icons, options, 0).add_component(ValInputDropdown {});

    Element::default()
        .with_style(|style| {
            style.flex_direction = FlexDirection::Row;
        })
        .add_component(ValInput {})
        .add_child_elements([number_input_el, dropdown_el])
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ValInputDropdown {}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ValInput {}

fn val_input_width_fixer(
    val_input_q: Query<(Entity, &ValInput)>,
    mut val_input_dropdown_q: Query<
        (&Dropdown, &mut Style, Entity),
        (
            Changed<Dropdown>,
            Without<TextInput>,
            With<ValInputDropdown>,
            Without<Initializing>,
        ),
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
pub fn get_val_type(val: Val) -> ValTypes {
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
pub fn get_number_val(val: Val) -> f32 {
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
pub fn get_dropdown_value_for_val_type(val: ValTypes) -> usize {
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
pub fn get_val_type_for_dropdown_value(i: usize) -> ValTypes {
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
