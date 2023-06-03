use crate::prelude::*;
use crate::Player;
use crate::game::sound::{play_hit_sound, play_score_sound, play_death_sound};
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn collision_system(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    enemy_query: Query<Entity, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
    cube_query: Query<Entity, With<Cube>>,
    blue_ball_query: Query<Entity, With<BlueBall>>,
    mut jump: ResMut<Jump>,
    mut game_state: ResMut<GameInfo>,
    asset_server: Res<AssetServer>, 
    audio: Res<Audio>, 
    volume: Res<Volume>,
) {
    for entity in player_query.iter() {
        for contact_pair in rapier_context.contacts_with(entity) {
            let other_collider = if contact_pair.collider1() == entity {
                contact_pair.collider2()
            } else {
                contact_pair.collider1()
            };
            let mut ava = false;
            for cube in cube_query.iter() {
                if other_collider == cube {
                    ava = true;
                }
            }
            if ava {
                jump.avalible = true;
            } else {
                jump.avalible = false;
            }

            for enemy in enemy_query.iter() {
                if other_collider == enemy {
                    game_state.is_won = true;
                    play_death_sound(&asset_server, &audio, &volume);
                    game_state.collected = 0;
                }
            }
            for ball in blue_ball_query.iter() {
                if other_collider == ball {
                    game_state.collected += 1;
                    play_score_sound(&asset_server, &audio, &volume);
                    commands.entity(ball).despawn();
                    if game_state.collected == 5 {
                        game_state.is_won = true;
                        game_state.collected = 0;
                        game_state.wins += 1;
                    }
                }
            }
        }
    }
}

pub fn ray_cast(
    mut commands: Commands,
    hit: ResMut<Hit>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    rapier_context: Res<RapierContext>,
    mouse_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>, 
    audio: Res<Audio>, 
    volume: Res<Volume>,
) {
    if mouse_input.pressed(MouseButton::Left) && hit.hit {
        for (player_entity, player_transform) in player_query.iter() {
            let player_forward = player_transform.forward();
            let ray_origin = player_transform.translation;
            let ray_direction = player_forward;
            let max_toi = 7.0;
            let hit = rapier_context.cast_ray(
                ray_origin,
                ray_direction,
                max_toi,
                true,
                QueryFilter::exclude_fixed()
                    .exclude_rigid_body(player_entity)
                    .exclude_collider(player_entity),
            );
            if let Some((entity, _toi)) = hit {
                let pushback_direction = ray_direction;
                play_hit_sound(&asset_server, &audio, &volume);
                commands.insert_resource(HitTimer(Timer::from_seconds(0.5, TimerMode::Once)));
                commands.entity(entity).insert(Velocity {
                    linvel: pushback_direction * 30.0,
                    angvel: Vec3::ZERO,
                });
            }
        }
    }
    
}

fn hit_countdown(
    mut hit: ResMut<Hit>,
    time: Res<Time>,
    mut timer: ResMut<HitTimer>,
) {
    if timer.0.tick(time.delta()).finished() {
        hit.hit = true;
    } else {
        hit.hit = false;
    }
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_system.in_set(OnUpdate(GameState::InGame)))
        .add_system(hit_countdown.in_set(OnUpdate(GameState::InGame)))
        .init_resource::<HitTimer>()
        .insert_resource(Hit {
            hit: false,
        })
            .insert_resource(Jump {
                jumping: false,
                elapsed: 0.0,
                avalible: false,
            })
            .add_system(ray_cast.in_set(OnUpdate(GameState::InGame)));
    }
}
