use crate::prelude::*;
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_rapier3d::prelude::*;

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.0,
        }
    }
}

#[derive(Resource)]
pub struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_ascend: KeyCode,
    pub move_descend: KeyCode,
    pub toggle_grab_cursor: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::W,
            move_backward: KeyCode::S,
            move_left: KeyCode::A,
            move_right: KeyCode::D,
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::LShift,
            toggle_grab_cursor: KeyCode::Escape,
        }
    }
}

#[derive(Component)]
pub struct FlyCam;

fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

fn initial_grab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

fn calculate_fov(value: f32) -> f32 {
    // Define the input range (1-10) and the desired output range (40-120)
    let input_min = 1.0;
    let input_max = 10.0;
    let output_min = 40.0;
    let output_max = 160.0;

    // Scale the input value to the output range
    let scaled_value = (value - input_min) / (input_max - input_min);
    let output_value = output_min + (output_max - output_min) * scaled_value;
    println!("{}", output_value);
    output_value.round()
}

pub fn setup_player(mut commands: Commands, fov: Res<Fov>) {
    commands
        .spawn((
            Camera3dBundle {
                projection: PerspectiveProjection {
                    fov: calculate_fov(fov.0 as f32).to_radians(),
                    ..default()
                }
                .into(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .looking_at(Vec3::new(-1.0, -1.0, 0.0), Vec3::Y),
                ..Default::default()
            },
            FlyCam,
        ))
        .insert(PointLightBundle {
            point_light: PointLight {
                intensity: 3000.0,
                color: Color::WHITE,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(TransformBundle::from(
            Transform::from_xyz(0.0, 80.0, 0.0).looking_at(Vec3::new(-1.0, -1.0, 0.0), Vec3::Y),
        ))
        .insert(Player)
        .insert(GravityScale(3.0))
        .insert(Ccd::enabled())
        .insert(Collider::ball(1.0));
}

fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut query: Query<(&FlyCam, &mut Transform)>,
    mut jump: ResMut<Jump>,
) {
    if let Ok(window) = primary_window.get_single() {
        for (_camera, mut transform) in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);
            for key in keys.get_pressed() {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let key = *key;
                        if key == key_bindings.move_forward {
                            velocity += forward;
                        } else if key == key_bindings.move_backward {
                            velocity -= forward;
                        } else if key == key_bindings.move_left {
                            velocity -= right;
                        } else if key == key_bindings.move_right {
                            velocity += right;
                        } else if key == key_bindings.move_ascend && jump.avalible {
                            jump.jumping = true;
                        }
                    }
                }
            }

            velocity = velocity.normalize_or_zero();
            velocity *= settings.speed;

            if jump.jumping {
                velocity *= 1.3;
                velocity.y = 40.0 * jump.elapsed;
            }

            transform.translation += velocity * time.delta_seconds();
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

fn player_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.iter(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(
    keys: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_grab_cursor) {
            toggle_grab_cursor(&mut window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

fn jump_system(mut jump: ResMut<Jump>, time: Res<Time>) {
    if jump.jumping {
        jump.avalible = false;
        jump.elapsed -= time.delta_seconds();
        if jump.elapsed < 0.0 {
            jump.jumping = false;
            jump.elapsed = 0.5;
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .add_system(setup_player.in_schedule(OnEnter(GameState::InGame)))
            //.add_system(cleanup::<Player>.in_schedule(OnEnter(MenuState::SettingsFov)))
            //.add_system(setup_player.in_schedule(OnExit(MenuState::SettingsFov)))
            .add_system(initial_grab_cursor.in_schedule(OnEnter(GameState::InGame)))
            .add_system(cleanup::<Player>.in_schedule(OnExit(GameState::InGame)))
            .insert_resource(MovementSettings {
                sensitivity: 0.00005, // default: 0.00012
                speed: 12.0,          // default: 12.0
            })
            .add_system(jump_system.in_set(OnUpdate(GameState::InGame)))
            .add_system(player_move.in_set(OnUpdate(GameState::InGame)))
            .add_system(player_look.in_set(OnUpdate(GameState::InGame)))
            .add_system(cursor_grab.in_set(OnUpdate(GameState::InGame)));
    }
}
