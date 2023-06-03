use crate::prelude::*;

#[derive(Resource)]
struct MusicController(Handle<AudioSink>);

pub struct SoundPlugin;
impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnExit(GameState::Splash)))
        .add_system(update_volume.in_set(OnUpdate(MenuState::SettingsSound)));
    }
}


fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
    volume: Res<Volume>,
) {
    let music = asset_server.load("sounds/music.ogg");
    let handle = audio_sinks.get_handle(audio.play_with_settings(
        music,
        PlaybackSettings::LOOP.with_volume(volume.0 as f32 * 0.1),
    ));
    commands.insert_resource(MusicController(handle));
}


fn update_volume (
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Res<MusicController>,
    volume: Res<Volume>,
) {
    if let Some(sink) = audio_sinks.get(&music_controller.0) {
        sink.set_volume(volume.0 as f32 * 0.1);
    }
}


pub fn play_hit_sound(asset_server: &Res<AssetServer>, audio: &Res<Audio>, volume: &Res<Volume>) {
    let music = asset_server.load("sounds/hit.ogg");
    audio.play_with_settings(
        music,
        PlaybackSettings {
            volume: volume.0 as f32 * 0.1,
            speed: 0.5,
            ..default()
        }
    );
}


pub fn play_score_sound(asset_server: &Res<AssetServer>, audio: &Res<Audio>, volume: &Res<Volume>) {
    let music = asset_server.load("sounds/score.ogg");
    audio.play_with_settings(
        music,
        PlaybackSettings {
            volume: volume.0 as f32 * 0.1,
            ..default()
        }
    );
}

pub fn play_death_sound(asset_server: &Res<AssetServer>, audio: &Res<Audio>, volume: &Res<Volume>) {
    let music = asset_server.load("sounds/die.ogg");
    audio.play_with_settings(
        music,
        PlaybackSettings {
            volume: volume.0 as f32 * 0.1,
            ..default()
        }
    );
}