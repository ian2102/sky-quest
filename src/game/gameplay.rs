use crate::game::load::ScenePlugin;
use crate::game::physics::PhysicsPlugin;
use crate::game::text::TextPlugin;
use crate::game::sound::SoundPlugin;
use crate::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TextPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(SoundPlugin)
            .add_system(check_win_condition.in_set(OnUpdate(GameState::InGame)))
            .add_system(menu_input_system.in_set(OnUpdate(GameState::InGame)))
            .add_system(new_game.in_schedule(OnEnter(GameState::NewGame)))
            .insert_resource(GameInfo {
                is_won: false,
                wins: 0,
                collected: 0,
            })
            .add_plugin(ScenePlugin);
    }
}

fn menu_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
    mut pause: ResMut<Pause>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(GameState::Menu);
        pause.paused = true;
    }
}

fn check_win_condition(
    mut commands: Commands,
    mut game_state: ResMut<GameInfo>,
    query: Query<Entity, With<Reboot>>,
    player: Query<Entity, With<Player>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    displayquality: ResMut<DisplayQuality>,
) {
    if game_state.is_won {
        game_state.is_won = false;

        for player in player.iter() {
            commands.entity(player).remove::<TransformBundle>();
            commands.entity(player).insert(TransformBundle::from(
                Transform::from_xyz(0.0, 80.0, 0.0).looking_at(Vec3::new(-1.0, -1.0, 0.0), Vec3::Y),
            ));
            for entity in query.iter() {
                if entity != player {
                    commands.entity(entity).despawn();
                }
            }
        }
        crate::game::load::setup(commands, meshes, materials, displayquality.into());
    }
}

fn new_game(mut game_state: ResMut<NextState<GameState>>,) {
    game_state.set(GameState::InGame)
}

