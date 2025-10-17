use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

#[derive(Component)]
struct Movement {
    velocity: f32,
    acceleration: f32,  // Rate of gaining speed
    accelerating: bool, // True if speeding up, false if slowing down
}

#[derive(Component)]
struct RotateToCursor {
    turn_speed: f32,
}

#[derive(Component)]
struct Follower;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.add_systems(Update, (update_movement, rotate_to_cursor));
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());
    let mut rng = rand::rng();

    commands.spawn((
        Follower,
        Mesh2d(meshes.add(Ellipse::new(5.0, 10.0))),
        MeshMaterial2d(materials.add(Color::hsv(
            rng.random_range(0.0..=1.0),
            rng.random_range(0.0..=1.0),
            rng.random_range(0.0..=1.0),
        ))),
        Transform::from_xyz(100.0, 1.0, 1.0),
        Movement {
            velocity: 0.0,
            acceleration: 10.0,
            accelerating: true,
        },
        RotateToCursor { turn_speed: 1.5 },
    ));
}

fn update_movement(
    mut query: Query<(&mut Movement, &mut Transform), With<Follower>>,
    time: Res<Time>,
) {
    for entity in query.iter_mut().collect::<Vec<_>>() {
        let (mut movement, mut transform) = entity;
        let direction = transform.local_y();
        movement.velocity += movement.acceleration * time.delta_secs();
        if !movement.accelerating {
            movement.velocity = -movement.acceleration * time.delta_secs();
            if movement.velocity < 0.0 {
                movement.velocity = 0.0;
            }
        }
        transform.translation += direction * movement.velocity * time.delta_secs();
    }
}

fn get_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> Option<Vec2> {
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return None;
    };
    let Ok(window) = window_query.single() else {
        return None;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return None;
    };
    // Convert cursor position to world coordinates using viewport_to_world_2d
    let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    else {
        return None;
    };
    //println!("cursor position: {:?}", cursor_world_pos);
    Some(cursor_world_pos)
}

fn rotate_to_cursor(
    time: Res<Time>,
    mut query: Query<(&RotateToCursor, &mut Transform, &mut Movement), With<Follower>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let cursor_translation = get_cursor(camera_query, window_query).unwrap_or(Vec2::ZERO);

    for (config, mut follower_transform, mut movement) in &mut query {
        let follower_forward = (follower_transform.rotation * Vec3::Y).xy();
        let to_player = (cursor_translation - follower_transform.translation.xy()).normalize();
        let forward_dot_player = follower_forward.dot(to_player);
        if forward_dot_player.abs() < 0.19209290e-07_f32 {
            movement.accelerating = true;
            continue;
        }
        movement.accelerating = false;
        let follower_right = (follower_transform.rotation * Vec3::X).xy();
        let right_dot_player = follower_right.dot(to_player);
        let rotation_sign = -f32::copysign(1.0, right_dot_player);
        let max_angle = ops::acos(forward_dot_player.clamp(-1.0, 1.0));
        let rotation_angle = rotation_sign * (config.turn_speed * time.delta_secs()).min(max_angle);
        follower_transform.rotate_z(rotation_angle);
    }
}
