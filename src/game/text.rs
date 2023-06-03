use crate::prelude::*;
use bevy::diagnostic::{
    Diagnostics, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};

#[derive(Component)]
struct TimerText;

fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((
        TextBundle::from_sections([TextSection::new(
            " fps,  ms/frame\nElapsed Time: \n Wins",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::BLACK,
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        TextChanges,
    ));
    commands
        .spawn((TextBundle::from_sections([TextSection::new(
            "+",
            TextStyle {
                font,
                font_size: 100.0,
                color: Color::BLACK,
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Percent(48.8),
                top: Val::Percent(45.5),
                ..Default::default()
            },
            ..Default::default()
        }),))
        .insert(Cursor);
}

fn change_text_system(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<TextChanges>>,
    game_state: ResMut<GameInfo>,
    mut timer_query: Query<&mut crate::prelude::FPSTimer>,
) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }
        let mut cpu_usage = 0.0;
        if let Some(cpu_usage_diagnostic) =
            diagnostics.get(SystemInformationDiagnosticsPlugin::CPU_USAGE)
        {
            if let Some(cpu_usage_smoothed) = cpu_usage_diagnostic.smoothed() {
                cpu_usage = cpu_usage_smoothed;
            }
        }

        let mut mem_usage = time.delta_seconds_f64();
        if let Some(mem_usage_diagnostic) =
            diagnostics.get(SystemInformationDiagnosticsPlugin::MEM_USAGE)
        {
            if let Some(mem_usage_smoothed) = mem_usage_diagnostic.smoothed() {
                mem_usage = mem_usage_smoothed;
            }
        }
        let mut elapsed_time = 0.0;
        for mut clock in &mut timer_query {
            clock.elapsed += time.delta_seconds();
            elapsed_time = clock.elapsed;
        }
        let formatted_string = format!(
            "{:.1} fps, {:.3} ms/frame\ncpu_usage {}%\nmem_usage {}%\nElapsed Time: {:.2}\n{} Wins\nCollected {}/5",
            fps, frame_time, cpu_usage.round(), mem_usage.round(), elapsed_time, game_state.wins, game_state.collected
        );
        text.sections[0].value = formatted_string;
    }
}

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(infotext_system.in_schedule(OnEnter(GameState::InGame)))
            .add_system(change_text_system.in_set(OnUpdate(GameState::InGame)));
    }
}
