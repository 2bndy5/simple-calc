use crate::calc::Calc;
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

pub const BG_COLOR: Color = Color::srgb(0.153, 0.153, 0.153);
pub const BORDER_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const PRESSED_COLOR: Color = Color::srgb(0.7, 0.4, 0.0);
pub const CC_COLOR: Color = Color::srgb(0.7, 0.0, 0.0);

pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
    }
}

fn button_press(calc: &mut Calc, val: String) {
    match &val[..] {
        "0" => calc.add_display(0.0),
        "1" => calc.add_display(1.0),
        "2" => calc.add_display(2.0),
        "3" => calc.add_display(3.0),
        "4" => calc.add_display(4.0),
        "5" => calc.add_display(5.0),
        "6" => calc.add_display(6.0),
        "7" => calc.add_display(7.0),
        "8" => calc.add_display(8.0),
        "9" => calc.add_display(9.0),
        "+" => calc.add_symbol("+".to_string()),
        "-" => calc.add_symbol("-".to_string()),
        "*" => calc.add_symbol("*".to_string()),
        "/" => calc.add_symbol("/".to_string()),
        "=" => {
            let sym = calc.symbol();
            match &sym[..] {
                "+" => calc.add(),
                "-" => calc.sub(),
                "*" => calc.mult(),
                "/" => calc.div(),
                _ => (),
            }
        }
        "C" => calc.reset(),
        _ => (),
    }
}

#[allow(clippy::type_complexity)]
fn button_system(
    mut calc: ResMut<Calc>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut button_color, children) in interaction_query.iter_mut() {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                button_color.0 = PRESSED_COLOR;
                button_press(&mut calc, text.0.clone());
            }
            Interaction::Hovered => {
                if text.0 == "C" {
                    button_color.0 = Color::BLACK.mix(&CC_COLOR, 0.4);
                } else {
                    button_color.0 = Color::BLACK.mix(&BG_COLOR, 0.4);
                }
            }
            Interaction::None => {
                if text.0 == "C" {
                    button_color.0 = CC_COLOR;
                } else {
                    button_color.0 = BG_COLOR;
                }
            }
        }
        button_color.set_changed();
    }
}

impl ButtonPlugin {
    pub fn spawn_buttons(parent: &mut RelatedSpawnerCommands<'_, ChildOf>, font: Handle<Font>) {
        let btn_symbols = vec![
            "7", "8", "9", "C", "4", "5", "6", "-", "1", "2", "3", "+", "0", "*", "/", "=",
        ];
        for i in btn_symbols {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        // center button
                        border: UiRect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor::all(BORDER_COLOR),
                    BackgroundColor(BG_COLOR),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(i.to_string()),
                        TextFont {
                            font: font.clone(),
                            font_size: 40.0,
                            ..Default::default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                });
        }
    }
}
