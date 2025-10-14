use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

#[derive(Component)]
struct Movement {
    velocity: f64,
    acceleration: f64, // Rate of gaining speed
    inertia: f64,      // Rate of slowing
    direction: f64,
    turn_speed: f64,
    accelerating: bool, // True if speeding up, false if slowing down
}

#[derive(Component)]
struct Follower;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.add_systems(Update, update_movement);
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
        Transform::from_xyz(0.0, 0.0, 0.0),
        Movement {
            velocity: 0.0,
            acceleration: 1.0,
            inertia: 2.0,
            direction: 0.0,
            turn_speed: 10.0,
            accelerating: true,
        },
    ));
}

fn update_movement(
    mut query: Query<(&mut Movement, &GlobalTransform), With<Follower>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = get_cursor(camera_query, window_query) {
        for entity in query.iter_mut().collect::<Vec<_>>() {
            let x = entity.1.translation().x;
            let y = entity.1.translation().y;
            let entity_pos = Vec2::new(x, y);
            println!("{}", get_bearing(entity_pos, position));
        }
    }
}

fn get_bearing(a: Vec2, b: Vec2) -> f32 {
    a.angle_to(b)
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
    return Some(cursor_world_pos);
}
