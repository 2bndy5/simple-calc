#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use std::time::Duration;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
    winit::{UpdateMode, WinitSettings},
};
mod button;
use button::{BG_COLOR, ButtonPlugin};
mod calc;
use calc::Calc;
mod cas;

fn main() {
    App::new()
        // Power-saving reactive rendering for applications.
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::reactive_low_power(Duration::from_millis(100)),
            unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs(5)),
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy calculator".to_string(),
                resolution: WindowResolution::new(323, 470),
                resizable: true,
                present_mode: PresentMode::AutoNoVsync,
                resize_constraints: WindowResizeConstraints {
                    min_width: 323.0,
                    min_height: 470.0,
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(Calc::default())
        .add_systems(Startup, setup_calc_ui)
        .add_systems(Update, display_expr_system)
        .add_systems(Update, display_operand_system)
        .add_plugins(ButtonPlugin)
        .run();
}

#[derive(Debug, Component)]
struct DisplayExpression;

#[derive(Debug, Component)]
struct DisplayOperand;

fn display_expr_system(calc: Res<calc::Calc>, mut query: Query<(&DisplayExpression, &mut Text)>) {
    for (_, mut text) in query.iter_mut() {
        text.0 = calc.expression.to_string();
    }
}

fn display_operand_system(calc: Res<calc::Calc>, mut query: Query<(&DisplayOperand, &mut Text)>) {
    for (_, mut text) in query.iter_mut() {
        text.0 = calc.display_operand();
    }
}

fn setup_calc_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_ui: Handle<Font> = asset_server.load("fonts/Segoe Fluent Icons.ttf");
    let font_norm: Handle<Font> = asset_server.load("fonts/RobotoMonoNerdFont-Medium.ttf");
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(BG_COLOR),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(35.0),
                        margin: UiRect::all(Val::Px(4.0)),
                        padding: UiRect::all(Val::Px(8.0)),
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::SpaceEvenly,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|parent| {
                    parent
                        // display fill (content)
                        .spawn((
                            DisplayExpression,
                            Text::new("".to_string()),
                            TextFont {
                                font: font_norm.clone(),
                                font_size: 32.0,
                                ..Default::default()
                            },
                            TextColor(Color::WHITE.with_alpha(0.5)),
                        ));
                    parent.spawn((
                        DisplayOperand,
                        Text::new("0".to_string()),
                        TextFont {
                            font: font_norm.clone(),
                            font_size: 42.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            // button panel
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(65.0),
                        display: Display::Grid,
                        padding: UiRect::all(Val::Px(3.0)),
                        justify_items: JustifyItems::Center,
                        align_items: AlignItems::Center,
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(6, 1.0),
                        ..Default::default()
                    },
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|parent| {
                    ButtonPlugin::spawn_buttons(parent, font_ui, font_norm);
                });
        });
}
