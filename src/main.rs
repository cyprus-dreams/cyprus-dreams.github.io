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
use egui::{FontData, FontDefinitions, FontFamily};
use egui_ratatui::RataguiBackend;
use ratatui::{
    layout::Rect,
    prelude::{Line, Span, Stylize, Terminal,Modifier},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap, *},
};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

mod resources;
use resources::{BevyTerminal, Masterik};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<Masterik>()
        .init_resource::<BevyTerminal<RataguiBackend>>()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .add_systems(Startup, setup)
        .add_systems(PostStartup, spawn_all_stars)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, ui_example_system)
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
            projection: OrthographicProjection {
                far: 1000.0,
                near: -1000.0,
                scale: 350.0,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));
}

fn keyboard_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut masterok: ResMut<Masterik>,
    mut query_camera: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
) {
    let (mut projection, mut transform) = query_camera.single_mut();

    let char_up = input.any_pressed([KeyCode::KeyW]);
    let char_down = input.any_pressed([KeyCode::KeyS]);
    let char_left = input.any_pressed([KeyCode::KeyA]);
    let char_right = input.any_pressed([KeyCode::KeyD]);

    if char_up {
        transform.translation.y += (masterok.camera_move_speed * projection.scale);
    }
    if char_down {
        transform.translation.y -= (masterok.camera_move_speed * projection.scale);
    }
    if char_left {
        transform.translation.x -= (masterok.camera_move_speed * projection.scale);
    }
    if char_right {
        transform.translation.x += (masterok.camera_move_speed * projection.scale);
    }

    let char_q = input.any_just_pressed([KeyCode::KeyQ]); //zoom out
    let char_e = input.any_just_pressed([KeyCode::KeyE]); //zoom in

    if char_q {
        // zoom out
        projection.scale *= 1.25;
    }
    if char_e {
        // zoom in
        projection.scale /= 1.25;
    }

    let char_backspace = input.any_pressed([KeyCode::Backspace, KeyCode::Delete]);

    if char_backspace {
        panic!("BYE");
    }
}

// Render to the terminal and to egui , both are immediate mode
fn ui_example_system(
    mut contexts: EguiContexts,
    mut termres: ResMut<BevyTerminal<RataguiBackend>>,
    mut masterok: ResMut<Masterik>,
) {
    draw_info_menu(&mut termres.terminal_info, &mut masterok);

    let mut frame = egui::Frame::default().inner_margin(1.0).outer_margin(1.0).fill(egui::Color32::BLACK);
 

    egui::SidePanel::right("my_left_panel")
        .frame(frame)
        .min_width(322.0)
        .max_width(322.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(termres.terminal_info.backend_mut());
        });
}

fn draw_info_menu(terminal: &mut Terminal<RataguiBackend>, masterok: &mut Masterik) {
    terminal
        .draw(|frame| {
            let area = frame.size();

            let mut lines = (Text::from(vec![
                Line::from("FPS: TODO!! "),
                Line::from(" "),
                Line::from("[WASD] - Move Camera "),
                Line::from("[Q/E] - Zoom Out/In"),
                Line::from(" "),
             
                Line::from("Current Seed: TODO!!"),
                Line::from("[T] - Change Seed"),
                Line::from(" "),
               
                Line::from("Stars: TODO!! "),
                Line::from("[U/J] - Add/Delete 1000 Stars"),
                Line::from("[I/K] - Add/Remove 10000 Stars"),
                Line::from(" "),
           
                Line::from("Spiral Arms: TODO!!"),
                Line::from("[O/L] - Add/Remove Spiral Arm"),
                Line::from(" "),
                Line::from("Toggle Star Types in Galaxy"),
                Line::from(" "),
                Line::from("[Z] - O-Class (Blue Giant)").style( if masterok.o_class {Modifier::empty()}else {Modifier::CROSSED_OUT}),
                Line::from("[X] - B-Class (Blue-White)").style( if masterok.b_class {Modifier::empty()}else {Modifier::CROSSED_OUT}),
                Line::from("[C] - A-Class (White)").style( if masterok.a_class {Modifier::empty()}else {Modifier::CROSSED_OUT}),
                Line::from("[V] - F-Class (Yellow-White Dwarf)").style( if masterok.f_class {Modifier::empty()}else {Modifier::CROSSED_OUT}),
                Line::from("[B] - G-Class (Sun)").style( if masterok.g_class {Modifier::empty()}else {Modifier::CROSSED_OUT}),
                Line::from("[N] - K-Class (Orange Dwarf)").style( if masterok.k_class {Modifier::empty()}else {Modifier::CROSSED_OUT}),
                Line::from("[M] - M-Class (Red Dwarf)").style( if masterok.m_class {Modifier::empty()}else {Modifier::CROSSED_OUT}),
            ]));

            frame.render_widget(
                Paragraph::new(lines)
                    .on_black()
                    .block(Block::new().title("Sirius-7").gray().borders(Borders::ALL)),
                area,
            );
        })
        .expect("epic fail");
}

fn spawn_all_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let circle_radius: f32 = 100.0;
    let mut positions = Vec::new();

    for boop in 1..3000 {
        let angle = boop as f32 * 0.002;
        let radius = 90.0 * angle;
        let mut xik = radius * angle.cos() * 200.0;
        let mut yik = radius * angle.sin() * 200.0;

        // Create a small RNG and add randomness
        let mut rng = SmallRng::from_entropy();
        let random_offset_x: f32 = rng.gen_range(-20000.0..20000.0);
        let random_offset_y: f32 = rng.gen_range(-20000.0..20000.0);

        xik += random_offset_x;
        yik += random_offset_y;

        if boop % 2 == 0 {
            xik = -xik;
            yik = -yik;
        }

        // Ensure the new circle does not overlap with any existing circles
        let mut attempts = 0;
        while positions.iter().any(|&(px, py)| {
            let dx = xik - px;
            let dy = yik - py;
            (((dx * dx) + (dy * dy)) as f64).sqrt() < (2.0 * circle_radius) as f64
        }) && attempts < 100
        {
            xik += rng.gen_range(-circle_radius..circle_radius);
            yik += rng.gen_range(-circle_radius..circle_radius);
            attempts += 1;
        }

        // Store the new circle position
        positions.push((xik, yik));
        // Circle mesh
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(circle_radius)).into(),
            // 4. Put something bright in a dark environment to see the effect
            material: materials.add(Color::rgb(7.5, 0.0, 7.5)),
            transform: Transform::from_translation(Vec3::new(xik as f32, yik as f32, 0.)),
            ..default()
        });
    }
}
