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
    prelude::{Line, Stylize, Terminal},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap, *},
};

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<Masterik>()
        .init_resource::<BevyTerminal<RataguiBackend>>()
        .add_systems(Startup, setup)
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
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    for boop in 1..3000 {
        let angle = boop as f32 * 0.01;
        let radius = 40.0 * angle;
        let mut xik = radius * angle.cos() * 50.0;
        let mut yik = radius * angle.sin() * 50.0;

        // Create a small RNG and add randomness
    let mut rng = SmallRng::from_entropy();
    let random_offset_x: f32 = rng.gen_range(-1000.0..1000.0);
    let random_offset_y: f32 = rng.gen_range(-1000.0..1000.0);

    xik += random_offset_x;
    yik += random_offset_y;

        if boop % 2 == 0 {
            xik = -xik;
            yik = -yik;

        }

        // Circle mesh
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(5.)).into(),
            // 4. Put something bright in a dark environment to see the effect
            material: materials.add(Color::rgb(7.5, 0.0, 7.5)),
            transform: Transform::from_translation(Vec3::new(xik as f32, yik as f32, 0.)),
            ..default()
        });
    }
}

#[derive(Resource)]
struct Masterik {
    total_stars: i64,
    gen_seed: i64,
    spiral_arm_count: i64,
    camera_move_speed: f32,
   
}

impl Masterik {
    pub fn refresh_menus(&mut self) {}
}

impl Default for Masterik {
    fn default() -> Self {
        Self {
            total_stars: 10000,
            gen_seed: 1111111,
            spiral_arm_count: 2,
            camera_move_speed:10.0
           
        }
    }
}

fn keyboard_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut masterok: ResMut<Masterik>,
    mut query_camera: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
) {
    let  (mut projection,mut transform) = query_camera.single_mut();


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

    egui::SidePanel::right("my_left_panel")
        .min_width(300.0)
        .max_width(300.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(termres.terminal_info.backend_mut());
        });
}

fn draw_info_menu(terminal: &mut Terminal<RataguiBackend>, masterok: &mut Masterik) {
    terminal
        .draw(|frame| {
            let area = frame.size();

            //neccesary beccause drawing is from the top

            frame.render_widget(
                Paragraph::new("ASDASDASDSSSS").on_gray().block(
                    Block::new()
                        .title("press number to choose item to pick up")
                        .blue()
                        .borders(Borders::ALL),
                ),
                area,
            );
        })
        .expect("epic fail");
}

//create resource to hold the ratatui terminal
#[derive(Resource)]
struct BevyTerminal<RataguiBackend: ratatui::backend::Backend> {
    terminal_info: Terminal<RataguiBackend>,
}

// Implement default on the resource to initialize it
impl Default for BevyTerminal<RataguiBackend> {
    fn default() -> Self {
        let mut backend1 = RataguiBackend::new(20, 20);
        backend1.set_font_size(20);
        let mut terminal1 = Terminal::new(backend1).unwrap();

        BevyTerminal {
            terminal_info: terminal1,
        }
    }
}
