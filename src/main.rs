use bevy::prelude::*;
use rand::Rng;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const STAR_SIZE: f32 = 2.0;

struct Star;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Spiral Galaxy".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(generate_spiral_galaxy.system())
        .add_system(handle_input.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn generate_spiral_galaxy(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let star_material = materials.add(ColorMaterial::color(Color::WHITE));
    let mut rng = rand::thread_rng();
    let arms = 4;
    let arm_offset = std::f32::consts::PI * 2.0 / arms as f32;

    for i in 0..1000 {
        let arm = i % arms;
        let theta = i as f32 * 0.1 + arm as f32 * arm_offset;
        let radius = (i as f32 * 0.05).sqrt();
        let x = SCREEN_WIDTH / 2.0 + theta.cos() * radius + rng.gen_range(-5.0..5.0);
        let y = SCREEN_HEIGHT / 2.0 + theta.sin() * radius + rng.gen_range(-5.0..5.0);
        
        commands.spawn_bundle(SpriteBundle {
            material: star_material.clone(),
            transform: Transform::from_xyz(x - SCREEN_WIDTH / 2.0, y - SCREEN_HEIGHT / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(STAR_SIZE, STAR_SIZE)),
            ..Default::default()
        })
        .insert(Star);
    }
}

fn handle_input(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        let star_material = materials.add(ColorMaterial::color(Color::WHITE));
        let mut rng = rand::thread_rng();
        let arms = 4;
        let arm_offset = std::f32::consts::PI * 2.0 / arms as f32;

        for i in 0..1000 {
            let arm = i % arms;
            let theta = i as f32 * 0.1 + arm as f32 * arm_offset;
            let radius = (i as f32 * 0.05).sqrt();
            let x = SCREEN_WIDTH / 2.0 + theta.cos() * radius + rng.gen_range(-5.0..5.0);
            let y = SCREEN_HEIGHT / 2.0 + theta.sin() * radius + rng.gen_range(-5.0..5.0);

            commands.spawn_bundle(SpriteBundle {
                material: star_material.clone(),
                transform: Transform::from_xyz(x - SCREEN_WIDTH / 2.0, y - SCREEN_HEIGHT / 2.0, 0.0),
                sprite: Sprite::new(Vec2::new(STAR_SIZE, STAR_SIZE)),
                ..Default::default()
            })
            .insert(Star);
        }
    }
}
