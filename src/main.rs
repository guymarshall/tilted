use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    post_process::bloom::Bloom,
    prelude::*,
    text::FontSmoothing,
};

const PLAYER_SPEED: f32 = 100.;
const CAMERA_DECAY_RATE: f32 = 2.;

#[derive(Component)]
struct Player;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));

    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(25.))),
        MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz(0., 0., 2.),
    ));
}

fn setup_instructions(mut commands: Commands) {
    commands.spawn((
        Text::new("Move the light with WASD.\nThe camera will smoothly track the light."),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Bloom::NATURAL));
}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. }: Vec3 = player.translation;
    let direction: Vec3 = Vec3::new(x, y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction: Vec2 = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let move_delta: Vec2 = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player.translation += move_delta.extend(0.);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 42.0,
                        font: default(),
                        font_smoothing: FontSmoothing::default(),
                        ..default()
                    },
                    text_color: Color::srgb(0.0, 1.0, 0.0),
                    refresh_interval: core::time::Duration::from_millis(100),
                    enabled: true,
                    frame_time_graph_config: FrameTimeGraphConfig {
                        enabled: true,
                        min_fps: 30.0,
                        target_fps: 144.0,
                    },
                },
            },
        ))
        .add_systems(Startup, (setup_scene, setup_instructions, setup_camera))
        .add_systems(Update, (move_player, update_camera).chain())
        .run();
}
