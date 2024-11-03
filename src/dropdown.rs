use crate::{
    element::{Components, Element},
    icons::Icons,
    input_helpers::{stabilize_inputs, Initializing},
    theme::Theme,
    val::ValExt,
    ValTypes,
};
use ::bevy::prelude::*;
use bevy::{
    color::palettes::css::BLACK,
    ui::{widget::UiImageSize, ContentSize},
};
pub struct DropdownPlugin;
impl Plugin for DropdownPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, manage_dropdown_state)
            .add_systems(Update, background_click_system)
            .add_systems(Update, interact_with_dropdown)
            .add_systems(PostUpdate, stabilize_inputs)
            .register_type::<Dropdown>()
            .register_type::<DropdownItem>()
            .register_type::<DropdownBox>()
            .register_type::<DropdownSelected>()
            .add_systems(Update, click_dropdown_item);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DropdownItem {
    pub label: String,
    pub value: usize,
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Dropdown {
    pub open: bool,
    pub selected: usize,
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DropdownBox {}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DropdownSelected {}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DropdownBackground {
    pub dropdown_e: Entity,
}

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
            style.width = 60.0.px();
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
        .add_component(Initializing)
        .add_component(Dropdown {
            open: false,
            selected,
        })
        .add_component(Name::new("dropdown"));

    let selected_value = Element::default()
        .with_text_and_components(
            "default value",
            TextStyle {
                font: theme.font.clone(),
                font_size: theme.input.size,
                color: theme.input.color,
                ..default()
            },
            Components::new().add(DropdownSelected {}),
        )
        .add_component(Name::new("Selected"));

    let chevron_icon = Element::default()
        .with_style(|style| {
            style.width = 10.0.px();
        })
        .add_component(UiImage::new(icons.chevron_down.clone()))
        .add_component(UiImageSize::default())
        .add_component(ContentSize::default());

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

fn interact_with_dropdown(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Style,
            &mut BackgroundColor,
            &mut Dropdown,
            Entity,
        ),
        (Changed<Interaction>, With<Node>),
    >,
    theme: Res<Theme>,
    mut commands: Commands,
) {
    for (interaction, mut style, mut bg, mut dropdown, dropdown_e) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg = theme.input.hover_background_color.into();
                dropdown.open = !dropdown.open;
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
                    DropdownBackground { dropdown_e },
                ));
            }
            Interaction::Hovered => *bg = theme.input.hover_background_color.into(),
            Interaction::None => *bg = theme.input.background_color.into(),
        };
    }
}
fn click_dropdown_item(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Style,
            &mut BackgroundColor,
            &DropdownItem,
            Entity,
        ),
        (Changed<Interaction>, With<Node>),
    >,
    parent_q: Query<&Parent>,
    children_q: Query<&Children>,
    theme: Res<Theme>,
    mut dropdown_q: Query<(&mut Dropdown, Entity)>,
    dropdown_item_q: Query<Entity, With<DropdownItem>>,
) {
    for (interaction, mut style, mut bg, mut dropdown_item, dropdown_item_e) in
        interaction_query.iter_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                *bg = theme.input.hover_background_color.into();
                for ancestor in parent_q.iter_ancestors(dropdown_item_e) {
                    if let Ok((mut dropdown, dropdown_e)) = dropdown_q.get_mut(ancestor) {
                        let mut index = 0;
                        for descendant in children_q.iter_descendants(dropdown_e) {
                            if let Ok(dropdown_item_iter_e) = dropdown_item_q.get(descendant) {
                                if dropdown_item_iter_e == dropdown_item_e {
                                    dropdown.selected = index;
                                }
                                index += 1;
                            }
                        }

                        dropdown.open = false;
                    }
                }
            }
            Interaction::Hovered => *bg = theme.input.hover_background_color.into(),
            Interaction::None => *bg = theme.input.background_color.into(),
        };
    }
}

fn manage_dropdown_state(
    changed_dropdown_q: Query<(Entity, &Dropdown), (Changed<Dropdown>)>,
    dropdown_item_q: Query<(Entity, &DropdownItem)>,
    children_q: Query<&Children>,
    mut dropdown_box_q: Query<&mut Style, (With<DropdownBox>, Without<DropdownSelected>)>,
    mut dropdown_selected_q: Query<&mut Text, (With<DropdownSelected>, Without<DropdownBox>)>,
) {
    changed_dropdown_q
        .iter()
        .for_each(|(dropdown_e, dropdown)| {
            let mut index = 0;
            let mut val = None;
            for descendant in children_q.iter_descendants(dropdown_e) {
                if let Ok((_, dropdown_item)) = dropdown_item_q.get(descendant) {
                    if dropdown.selected == index {
                        val = Some(dropdown_item.label.clone());
                    }
                    index += 1;
                }
            }
            for descendant in children_q.iter_descendants(dropdown_e) {
                if let Ok(mut dropdown_box_s) = dropdown_box_q.get_mut(descendant) {
                    dropdown_box_s.display = match dropdown.open {
                        true => Display::Flex,
                        false => Display::None,
                    };
                } else if let Ok(mut text) = dropdown_selected_q.get_mut(descendant) {
                    text.sections[0].value = val.clone().unwrap();
                }
            }
        });
}

fn background_click_system(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction, &DropdownBackground), (Changed<Interaction>,)>,
    mut dropdown_q: Query<&mut Dropdown>,
) {
    for (entity, interaction, dropdown_bg) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut dropdown) = dropdown_q.get_mut(dropdown_bg.dropdown_e) {
                dropdown.open = false;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}
