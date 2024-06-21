

//! Illustrates bloom post-processing in 2d.

use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use bevy_egui::{
    egui::{self, Frame},
    EguiContexts, EguiPlugin,
};
use egui::{FontDefinitions,FontData,FontFamily};
use ratatui::{
    layout::Rect,
    prelude::{Line, Stylize, Terminal},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap, *},
};
use egui_ratatui::RataguiBackend;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Masterik>()
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
       
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

   

    // Circle mesh
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(100.)).into(),
        // 4. Put something bright in a dark environment to see the effect
        material: materials.add(Color::rgb(7.5, 0.0, 7.5)),
        transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
        ..default()
    });

 

}


#[derive(Resource)]
struct Masterik {
    total_stars: i64,
    gen_seed: i64,
    spiral_arm_count: i64,

  
}

impl Masterik {
    pub fn refresh_menus(&mut self) {
    
    }
}

impl Default for Masterik {
    fn default() -> Self {
        Self {
            total_stars: 10000,
            gen_seed: 1111111,
            spiral_arm_count: 2,
     
        }
    }
}


fn keyboard_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut masterok: ResMut<Masterik>,
  
) {
    let char_up = input.any_pressed([KeyCode::KeyW]);
    let char_down = input.any_pressed([KeyCode::KeyS]);
    let char_left = input.any_pressed([KeyCode::KeyA]);
    let char_right = input.any_pressed([KeyCode::KeyD]);

    let cursor_up = input.any_just_pressed([KeyCode::KeyI]);
    let cursor_down = input.any_just_pressed([KeyCode::KeyK]);
    let cursor_left = input.any_just_pressed([KeyCode::KeyJ]);
    let cursor_right = input.any_just_pressed([KeyCode::KeyL]);

    let char_attack = input.any_just_pressed([KeyCode::KeyY]); // jęti (jme) / vzeti
    let char_take = input.any_just_pressed([KeyCode::KeyG]); // jęti (jme) / vzeti metnuti  imej target range do ktorogo mozno metati dla praktiki zeby povysati skil be ubijstva
    let char_drop = input.any_just_pressed([KeyCode::KeyB]); //izbaviti se
    let char_help = input.any_just_pressed([KeyCode::KeyT]); //pokazati pomoc ?

    let char_one = input.any_just_pressed([KeyCode::Digit1]);
    let char_two = input.any_just_pressed([KeyCode::Digit2]);
    let char_three = input.any_just_pressed([KeyCode::Digit3]);
    let char_four = input.any_just_pressed([KeyCode::Digit4]);
    let char_five = input.any_just_pressed([KeyCode::Digit5]);
    let char_six = input.any_just_pressed([KeyCode::Digit6]);
    let char_seven = input.any_just_pressed([KeyCode::Digit7]);
    let char_eight = input.any_just_pressed([KeyCode::Digit8]);
    let char_nine = input.any_just_pressed([KeyCode::Digit9]);
    let char_zero = input.any_just_pressed([KeyCode::Digit0]);

    let char_backspace = input.any_pressed([KeyCode::Backspace, KeyCode::Delete]);
    let char_quit = input.any_just_pressed([KeyCode::KeyQ]);

   

    if char_quit {
        panic!("BYE");
    }
  
}
