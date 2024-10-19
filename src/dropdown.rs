use crate::{icons::Icons, theme::Theme, ValTypes};
use ::bevy::prelude::*;
pub struct DropdownPlugin;
impl Plugin for DropdownPlugin {
    fn build(&self, app: &mut App) {
        register_dropdown_systems::<usize>(app);
        register_dropdown_systems::<Display>(app);
        register_dropdown_systems::<ValTypes>(app);
    }
}

fn register_dropdown_systems<T: 'static + PartialEq + Clone + Send + Sync>(app: &mut App) {
    app.add_systems(Update, manage_dropdown_state::<T>)
        .add_systems(Update, background_click_system::<T>)
        .add_systems(Update, interact_with_dropdown::<T>)
        .add_systems(Update, click_dropdown_item::<T>);
}

#[derive(Component, Clone)]
pub struct DropdownItem<T: PartialEq + Clone> {
    pub label: String,
    pub value: Box<T>,
}
#[derive(Component)]
pub struct Dropdown<T: PartialEq + Clone> {
    pub open: bool,
    pub selected: DropdownItem<T>,
}
#[derive(Component)]
pub struct DropdownBox {}

#[derive(Component)]
pub struct DropdownSelected {}

#[derive(Component)]
pub struct DropdownBackground {
    pub dropdown_e: Entity,
}
fn interact_with_dropdown<T: PartialEq + Clone + Send + Sync + 'static>(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Style,
            &mut BackgroundColor,
            &mut Dropdown<T>,
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
fn click_dropdown_item<T: PartialEq + Send + Sync + Clone + 'static>(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Style,
            &mut BackgroundColor,
            &DropdownItem<T>,
            Entity,
        ),
        (Changed<Interaction>, With<Node>),
    >,
    parent_q: Query<&Parent>,
    theme: Res<Theme>,
    mut dropdown_q: Query<&mut Dropdown<T>>,
) {
    for (interaction, mut style, mut bg, mut dropdown_item, dropdown_item_e) in
        interaction_query.iter_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                *bg = theme.input.hover_background_color.into();
                for ancestor in parent_q.iter_ancestors(dropdown_item_e) {
                    if let Ok(mut dropdown) = dropdown_q.get_mut(ancestor) {
                        dropdown.selected = dropdown_item.clone();
                        dropdown.open = false;
                    }
                }
            }
            Interaction::Hovered => *bg = theme.input.hover_background_color.into(),
            Interaction::None => *bg = theme.input.background_color.into(),
        };
    }
}

fn manage_dropdown_state<T: PartialEq + Send + Sync + Clone + 'static>(
    changed_dropdown_q: Query<(Entity, &Dropdown<T>), (Changed<Dropdown<T>>)>,
    children_q: Query<&Children>,
    mut dropdown_box_q: Query<&mut Style, (With<DropdownBox>, Without<DropdownSelected>)>,
    mut dropdown_selected_q: Query<&mut Text, (With<DropdownSelected>, Without<DropdownBox>)>,
) {
    changed_dropdown_q
        .iter()
        .for_each(|(dropdown_e, dropdown)| {
            for descendant in children_q.iter_descendants(dropdown_e) {
                if let Ok(mut dropdown_box_s) = dropdown_box_q.get_mut(descendant) {
                    dropdown_box_s.display = match dropdown.open {
                        true => Display::Flex,
                        false => Display::None,
                    };
                } else if let Ok(mut text) = dropdown_selected_q.get_mut(descendant) {
                    text.sections[0].value = dropdown.selected.label.clone();
                }
            }
        });
}

fn background_click_system<T: PartialEq + Send + Sync + Clone + 'static>(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction, &DropdownBackground), (Changed<Interaction>,)>,
    mut dropdown_q: Query<&mut Dropdown<T>>,
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

pub fn create_dropdown<T: PartialEq + Send + Sync + Clone + 'static>(
    commands: &mut Commands,
    icons: &Icons,
    theme: &Theme,
    options: Vec<DropdownItem<T>>,
    dropdown: Dropdown<T>,
) -> Entity {
    let open = dropdown.open;
    let dropdown_e = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(120.),
                    height: Val::Px(22.),
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::horizontal(Val::Px(6.0)),
                    ..Default::default()
                },
                background_color: BackgroundColor(theme.input.background_color),
                ..Default::default()
            },
            Interaction::None,
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
                    width: Val::Px(16.0),
                    height: Val::Px(16.0),
                    ..default()
                },
                image: UiImage::new(icons.chevron_down.clone()),
                ..default()
            });
            builder
                .spawn((
                    DropdownBox {},
                    NodeBundle {
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

    dropdown_e
}
