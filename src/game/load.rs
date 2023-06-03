use noise::{NoiseFn, Perlin};
use rand::{seq::SliceRandom, Rng};

use crate::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;
const VOXEL_SIZE: f32 = 1.0;

pub fn spawn_balls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    voxel_data: &Vec<Vec<Vec<bool>>>,
) {
    let width = voxel_data.len();
    let height = voxel_data[0].len();
    let depth = voxel_data[0][0].len();
    let center_offset = Vec3::new(
        -(width as f32 * VOXEL_SIZE / 2.0),
        0.5,
        -(depth as f32 * VOXEL_SIZE / 2.0),
    );

    let mut positions: Vec<(usize, usize, usize)> = Vec::new();
    for x in 0..width {
        for z in 0..depth {
            let mut has_block_below = false;
            for y in 0..height {
                if voxel_data[x][y][z] {
                    has_block_below = true;
                } else if has_block_below {
                    positions.push((x, y, z));
                    break;
                }
            }
        }
    }

    positions.shuffle(&mut rand::thread_rng());

    let num_blue_balls = 5;
    let positions_to_spawn = positions.iter().take(num_blue_balls);

    for (x, y, z) in positions_to_spawn {
        let position = Vec3::new(*x as f32, *y as f32, *z as f32) * VOXEL_SIZE + center_offset;

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(shape::UVSphere::default().into()),
                material: materials.add(Color::BLUE.into()),
                visibility: Visibility::Visible,
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::ball(VOXEL_SIZE))
            .insert(Reboot)
            .insert(Renderable)
            .insert(BlueBall);
    }

    for (x, y, z) in positions.iter().skip(num_blue_balls) {
        if rand::thread_rng().gen_range(0..200) < 2 {
            let position = Vec3::new(*x as f32, *y as f32, *z as f32) * VOXEL_SIZE + center_offset;

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(shape::UVSphere::default().into()),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(position),
                    ..Default::default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(VOXEL_SIZE))
                .insert(Restitution::coefficient(2.1))
                .insert(Reboot)
                .insert(Enemy);
        }
    }
}

pub fn spawn_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let collider_size = Vec3::new(64.0, 0.9, 100.0);

    let mesh = Mesh::from(shape::Box::new(
        collider_size.x,
        collider_size.y,
        collider_size.z,
    ));

    let mut transform = Transform::from_xyz(32.0, 16.0, 0.0);
    transform.rotate_x(std::f32::consts::PI / 2.0);
    transform.rotate_y(std::f32::consts::PI / 2.0);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh.clone()),
            transform,
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(Reboot)
        .insert(Collider::cuboid(
            collider_size.x,
            collider_size.y,
            collider_size.z,
        ))
        .insert(RigidBody::Fixed);

    let mut transform = Transform::from_xyz(0.0, 16.0, -32.0);
    transform.rotate_x(std::f32::consts::PI / 2.0);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh.clone()),
            transform,
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(Reboot)
        .insert(Collider::cuboid(
            collider_size.x,
            collider_size.y,
            collider_size.z,
        ))
        .insert(RigidBody::Fixed);

    let mut transform = Transform::from_xyz(-32.0, 16.0, 0.0);
    transform.rotate_x(std::f32::consts::PI / 2.0);
    transform.rotate_y(std::f32::consts::PI / 2.0);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh.clone()),
            transform,
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(Reboot)
        .insert(Collider::cuboid(
            collider_size.x,
            collider_size.y,
            collider_size.z,
        ))
        .insert(RigidBody::Fixed);

    let mut transform = Transform::from_xyz(0.0, 16.0, 32.0);
    transform.rotate_x(std::f32::consts::PI / 2.0);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            transform,
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(Reboot)
        .insert(Collider::cuboid(
            collider_size.x,
            collider_size.y,
            collider_size.z,
        ))
        .insert(RigidBody::Fixed);
}

pub fn generate_voxels(width: usize, height: usize, depth: usize) -> Vec<Vec<Vec<bool>>> {
    let random_int: u32 = rand::thread_rng().gen_range(1..=100);

    let perlin = Perlin::new(random_int);

    let mut voxels = vec![vec![vec![false; depth]; height]; width];

    for x in 0..width {
        for y in 0..height {
            for z in 0..depth {
                let noise_value = perlin.get([
                    (x as f64 - (width as f64 / 2.0)) / 10.0,
                    (y as f64 - (height as f64 / 2.0)) / 10.0,
                    (z as f64 - (depth as f64 / 2.0)) / 10.0,
                ]);

                let threshold = 0.2;

                if noise_value > threshold {
                    voxels[x][y][z] = true;
                }
            }
        }
    }
    voxels
}

pub fn spawn_cubes(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    voxel_data: &Vec<Vec<Vec<bool>>>,
) {
    let width = voxel_data.len();
    let height = voxel_data[0].len();
    let depth = voxel_data[0][0].len();
    let center_offset = Vec3::new(
        -(width as f32 * VOXEL_SIZE / 2.0),
        0.5,
        -(depth as f32 * VOXEL_SIZE / 2.0),
    );

    let cube_mesh = meshes.add(shape::Cube::new(VOXEL_SIZE).into());

    //let cube_mesh = meshes.add(create_low_resolution_cube_mesh(VOXEL_SIZE, 10));

    let instance_material = materials.add(StandardMaterial {
        base_color: Color::BEIGE,
        perceptual_roughness: 0.3,
        ..Default::default()
    });
    for x in 0..width {
        for y in 0..height {
            for z in 0..depth {
                if voxel_data[x][y][z] {
                    let position =
                        Vec3::new(x as f32, y as f32, z as f32) * VOXEL_SIZE + center_offset;

                    let has_neighbors_on_all_sides = (x > 0 && voxel_data[x - 1][y][z])  // Left
                        && (x < width - 1 && voxel_data[x + 1][y][z])  // Right
                        && (y > 0 && voxel_data[x][y - 1][z])  // Bottom
                        && (y < height - 1 && voxel_data[x][y + 1][z])  // Top
                        && (z > 0 && voxel_data[x][y][z - 1])  // Back
                        && (z < depth - 1 && voxel_data[x][y][z + 1]); // Front

                    if !has_neighbors_on_all_sides {
                        commands
                            .spawn(PbrBundle {
                                mesh: cube_mesh.clone(),
                                material: instance_material.clone(),
                                transform: Transform::from_translation(position),
                                ..Default::default()
                            })
                            .insert(Reboot)
                            .insert(Cube)
                            .insert(Renderable)
                            .insert(Collider::cuboid(
                                VOXEL_SIZE / 2.0,
                                VOXEL_SIZE / 2.0,
                                VOXEL_SIZE / 2.0,
                            ));
                    }
                }
            }
        }
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    displayquality: Res<DisplayQuality>,
) {
    let height;
    match *displayquality {
        DisplayQuality::Low => {
            height = 8;
        },
        DisplayQuality::Medium => {
            height = 32;
        },
        DisplayQuality::High => {
            height = 64;
        },
    }

    let voxel_data = generate_voxels(64, height, 64);

    spawn_cubes(&mut commands, &mut meshes, &mut materials, &voxel_data); // + 24

    spawn_balls(&mut commands, &mut meshes, &mut materials, &voxel_data); // + .5

    commands.spawn(crate::prelude::FPSTimer { elapsed: 0.0 }).insert(Reboot);

    spawn_walls(&mut commands, &mut meshes, &mut materials); // + .5

    commands
        .spawn(Collider::cuboid(64.0, 0.1, 64.0))
        .insert(Cube)
        .insert(Reboot)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.1, 0.0)));

    commands
        .spawn(Collider::cuboid(64.0, 10.0, 64.0))
        .insert(Reboot)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 90.0, 0.0)));

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(64.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    }).insert(Reboot);

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.6,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 3200.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        ..default()
    }).insert(Reboot);
}


fn game_info(mut game_info: ResMut<GameInfo>,) {
    game_info.is_won = true;
    game_info.collected = 0;

}
pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(cleanup::<Reboot>.in_schedule(OnEnter(GameState::NewGame)))
        .add_system(game_info.in_schedule(OnEnter(GameState::NewGame)))
        .add_system(setup.in_schedule(OnEnter(GameState::NewGame)));
    }
}
