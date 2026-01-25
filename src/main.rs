use bevy::{prelude::*, window::WindowResolution};
mod button;
use button::{BG_COLOR, BORDER_COLOR, ButtonPlugin};
mod calc;
use calc::Calc;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy calculator".to_string(),
                resolution: WindowResolution::new(450, 600),
                resizable: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(Calc::new())
        .add_systems(Startup, setup_calc_ui)
        .add_systems(Update, display_system)
        .add_plugins(ButtonPlugin)
        .run();
}

#[derive(Debug, Component)]
struct DisplayText;

fn display_system(calc: Res<calc::Calc>, mut query: Query<(&DisplayText, &mut Text)>) {
    for (_, mut text) in query.iter_mut() {
        text.0 = calc.display();
    }
}

fn setup_calc_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
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
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            parent
                // display (border)
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(30.0),
                        border: UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::FlexEnd,
                        ..Default::default()
                    },
                    BackgroundColor(BG_COLOR),
                    BorderColor::all(BORDER_COLOR),
                ))
                .with_children(|parent| {
                    parent
                        // display fill (content)
                        .spawn((
                            DisplayText,
                            Text::new("0".to_string()),
                            TextFont {
                                font: font.clone(),
                                font_size: 60.0,
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
                        height: Val::Percent(70.0),
                        display: Display::Grid,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|parent| {
                    ButtonPlugin::spawn_buttons(parent, font);
                });
        });
}
