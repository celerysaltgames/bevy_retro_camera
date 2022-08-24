use bevy::prelude::*;
use bevy_retro_camera::prelude::*;

// Create a stage label that will be used for our game logic stage
#[derive(StageLabel, Debug, Eq, Hash, PartialEq, Clone)]
struct GameStage;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Retro Camera - Hello World".into(),
            ..Default::default()
        })
        .add_plugins(RetroCameraPlugin::default())
        .add_startup_system(setup)
        .add_system(move_player)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load our sprites
    let red_radish_image = asset_server.load("redRadish.png");

    // Spawn the camera with a fixed height of 80 in-game pixels and a width determined by the
    // window aspect.
    commands.spawn_bundle(RetroCameraBundle::fixed_height(80.0));

    // Spawn a red radish
    commands
        .spawn_bundle(SpriteBundle {
            texture: red_radish_image,
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        // Add our player marker component so we can move it
        .insert(Player);
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let speed: f32 = 20.0 * time.delta_seconds();

        let mut direction = Vec3::new(0., 0., 0.);

        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-speed, 0., 0.);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(speed, 0., 0.);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0., speed, 0.);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0., -speed, 0.);
        }

        if direction != Vec3::new(0., 0., 0.) {
            transform.translation += direction;
        }
    }
}