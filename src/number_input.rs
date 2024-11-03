use crate::{element::Element, input_helpers::Initializing, theme::Theme, val::ValExt};
use bevy::{color::palettes::css::BLACK, prelude::*};
pub struct NumberInputPlugin;
impl Plugin for NumberInputPlugin{
    fn build(&self, app: &mut App) {
        app.register_type::<TextInput>();
        app.add_systems(Update, update_text_input);
        app.add_systems(Update, text_input_focus);
        app.add_systems(Update, handle_keyboard_input);
        app.add_systems(Update, background_click_system_input_focus);
    }
}

pub fn input(theme: &Theme) -> Element {
    let element = Element::default()
        .with_style(|style| {
            style.height = 22.0.px();
            style.width = 120.0.px();
            style.justify_content = JustifyContent::Center;
            style.border = UiRect::new(1.0.px(), 0.0.px(), 1.0.px(), 1.0.px());
            style.overflow = Overflow::clip_x();
            style.align_items = AlignItems::Center;
        })
        .border_color(BLACK)
        .background_color(theme.input.background_color)
        .add_component(Name::new("Input"))
        .add_component(Initializing)
        .add_component(TextInput {
            value: String::new(),
            cursor: 0,
            focussed: false,
        })
        .add_component(Interaction::default())
        .with_text(
            "",
            TextStyle {
                font: theme.font.clone(),
                font_size: theme.input.size,
                color: theme.input.color,
            },
        );
    element
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TextInput {
    pub value: String,
    cursor: usize,
    focussed: bool,
}
fn update_text_input(
    query: Query<(&Children, &TextInput)>,
    mut text_input_text_q: Query<&mut Text>,
) {
    for (children, text_input) in query.iter() {
        children.iter().for_each(|child| {
            if let Ok(mut text) = text_input_text_q.get_mut(*child) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text.sections[0].value = if text_input.focussed {
                    format!("{}|{}", start, end)
                } else {
                    text_input.value.clone()
                };
            }
        });
    }
}

fn text_input_focus(
    mut query: Query<(&mut TextInput, Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    query.iter_mut().for_each(
        |(mut text_input, focussed_e, interaction)| match interaction {
            Interaction::Pressed => {
                text_input.focussed = true;
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
                    BackgroundTextFocus { focussed_e },
                ));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        },
    );
}
#[derive(Component)]
struct BackgroundTextFocus {
    focussed_e: Entity,
}

fn background_click_system_input_focus(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction, &BackgroundTextFocus), (Changed<Interaction>,)>,
    mut text_input_q: Query<&mut TextInput>,
) {
    for (entity, interaction, dropdown_bg) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut text_input) = text_input_q.get_mut(dropdown_bg.focussed_e) {
                text_input.focussed = false;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}


fn handle_keyboard_input(mut query: Query<&mut TextInput>, keys: Res<ButtonInput<KeyCode>>) {
    for mut text_input in query.iter_mut() {
        if text_input.focussed {
            if keys.just_pressed(KeyCode::Digit0) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '0', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit1) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '1', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit2) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '2', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit3) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '3', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit4) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '4', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit5) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '5', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit6) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '6', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit7) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '7', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit8) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '8', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::Digit9) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                text_input.value = format!("{}{}{}", start, '9', end);
                text_input.cursor += 1;
            }
            if keys.just_pressed(KeyCode::ArrowLeft) {
                text_input.cursor = 0.max(text_input.cursor.saturating_sub(1));
            }
            if keys.just_pressed(KeyCode::ArrowRight) {
                text_input.cursor = text_input.value.len().min(text_input.cursor + 1);
            }
            if keys.just_pressed(KeyCode::Backspace) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                let without_last_char: String =
                    start.chars().take(start.chars().count() - 1).collect();
                text_input.value = format!("{}{}", without_last_char, end);
                text_input.cursor = 0.max(text_input.cursor.saturating_sub(1));
            }
            if keys.just_pressed(KeyCode::Delete) {
                let (start, end) = text_input.value.split_at(text_input.cursor);
                let without_first_char: String = end.chars().skip(1).collect();
                text_input.value = format!("{}{}", start, without_first_char);
            }
        }
    }
}

