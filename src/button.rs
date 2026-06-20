use crate::calc::Calc;
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

pub const BG_COLOR: Color = Color::srgb(0.1255, 0.1255, 0.1255);
pub const PRESSED_COLOR: Color = Color::srgb(0.7, 0.4, 0.0);
pub const EQ_COLOR: Color = Color::srgb(0.7, 0.0, 0.0);

pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
    }
}

fn button_press(calc: &mut Calc, val: String) {
    match val.as_str() {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            calc.push_operand(val.as_str())
        }
        ButtonPlugin::ADD => calc.push_operator("+"),
        ButtonPlugin::SUBTRACT => calc.push_operator("-"),
        ButtonPlugin::MULTIPLY => calc.push_operator("*"),
        ButtonPlugin::DIVIDE => calc.push_operator("/"),
        ButtonPlugin::MODULO => calc.push_operator("%"),
        ButtonPlugin::SQUARED => {
            calc.push_operator("^");
            calc.push_operand("2");
        }
        ButtonPlugin::SQRT => {
            calc.push_operator("^");
            calc.push_operator("(");
            calc.push_operand("1");
            calc.push_operator("/");
            calc.push_operand("2");
            calc.push_operator(")");
        }
        ButtonPlugin::INVERSE => {
            calc.push_operator("^");
            calc.push_operand("-1");
        }
        ButtonPlugin::EQUALS => {
            if let Err(e) = calc.solve() {
                eprintln!("{e}");
            }
        }
        ButtonPlugin::NEGATE => calc.push_operand("-"),
        "." if calc.operand.as_ref().is_none_or(|op| !op.has_dot()) => calc.push_operand("."),
        "C" => calc.operand = None,
        "CE" => calc.expression.clear(),
        ButtonPlugin::BACKSPACE => calc.pop(),
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
        match text_query.get_mut(children[0]) {
            Ok(text) => {
                match *interaction {
                    Interaction::Pressed => {
                        button_color.0 = PRESSED_COLOR;
                        button_press(&mut calc, text.0.clone());
                    }
                    Interaction::Hovered => {
                        if text.0 == ButtonPlugin::EQUALS {
                            button_color.0 = Color::BLACK.mix(&EQ_COLOR, 0.6);
                        } else if ButtonPlugin::OPERANDS.contains(&text.0.as_str()) {
                            button_color.0 =
                                Color::BLACK.mix(&ButtonPlugin::OPERAND_BUTTON_COLOR, 0.75);
                        } else {
                            button_color.0 =
                                Color::BLACK.mix(&ButtonPlugin::OPERATOR_BUTTON_COLOR, 0.75);
                        }
                    }
                    Interaction::None => {
                        if text.0 == ButtonPlugin::EQUALS {
                            button_color.0 = EQ_COLOR;
                        } else if ButtonPlugin::OPERANDS.contains(&text.0.as_str()) {
                            button_color.0 = ButtonPlugin::OPERAND_BUTTON_COLOR;
                        } else {
                            button_color.0 = ButtonPlugin::OPERATOR_BUTTON_COLOR;
                        }
                    }
                }
                button_color.set_changed();
            }
            Err(e) => eprintln!("Error getting button text: {e}"),
        }
    }
}

impl ButtonPlugin {
    pub const MODULO: &'static str = "\u{E94C}";
    pub const BACKSPACE: &'static str = "\u{E94F}";
    pub const DIVIDE: &'static str = "\u{E94A}";
    pub const MULTIPLY: &'static str = "\u{E947}";
    pub const SUBTRACT: &'static str = "\u{E949}";
    pub const ADD: &'static str = "\u{E948}";
    pub const NEGATE: &'static str = "\u{E94D}";
    pub const EQUALS: &'static str = "\u{E94E}";
    pub const INVERSE: &'static str = "\u{00B9}\u{2044}x";
    pub const SQUARED: &'static str = "x\u{00B2}";
    pub const SQRT: &'static str = "\u{00B2}\u{221A}x";
    const SPECIAL_CHARS: [&'static str; 8] = [
        Self::MODULO,
        Self::BACKSPACE,
        Self::DIVIDE,
        Self::MULTIPLY,
        Self::SUBTRACT,
        Self::ADD,
        Self::NEGATE,
        Self::EQUALS,
    ];
    pub const OPERAND_BUTTON_COLOR: Color = Color::srgb(0.2314, 0.2314, 0.2314);
    pub const OPERATOR_BUTTON_COLOR: Color = Color::srgb(0.1908, 0.1908, 0.1908);
    pub const OPERANDS: [&'static str; 12] = [
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        ".",
        Self::NEGATE,
    ];

    pub fn spawn_buttons(
        parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
        font_ui: Handle<Font>,
        font_norm: Handle<Font>,
    ) {
        #[rustfmt::skip]
        let btn_symbols = vec![
            Self::MODULO, "CE", "C", Self::BACKSPACE,
            Self::INVERSE, Self::SQUARED, Self::SQRT, Self::DIVIDE,
            "7", "8", "9", Self::MULTIPLY,
            "4", "5", "6", Self::SUBTRACT,
            "1", "2", "3", Self::ADD,
            Self::NEGATE, "0", ".", Self::EQUALS,
        ];
        for i in btn_symbols {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        margin: UiRect::all(Val::Px(2.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(if Self::OPERANDS.contains(&i) {
                        Self::OPERAND_BUTTON_COLOR
                    } else {
                        Self::OPERATOR_BUTTON_COLOR
                    }),
                    BorderColor::all(Color::NONE),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(i.to_string()),
                        TextFont {
                            font: FontSource::Handle(if ButtonPlugin::SPECIAL_CHARS.contains(&i) {
                                font_ui.clone()
                            } else {
                                font_norm.clone()
                            }),
                            font_size: FontSize::Px(30.0),
                            ..Default::default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                });
        }
    }
}
