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
    prelude::{Line, Modifier, Span, Stylize, Terminal},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap, *},
};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

mod resources;
use resources::{BevyTerminal, Masterik, SpawnStars, StarCount, StarsAdded, StarsRemoved,PositionsVec, StarData};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<Masterik>()
        .init_resource::<StarData>()
        .init_resource::<BevyTerminal<RataguiBackend>>()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .add_systems(Startup, setup)
        .add_systems(PostStartup, spawn_initial_stars)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, star_watcher)
        .add_systems(Update, star_adder)
        .add_event::<SpawnStars>()
        .add_event::<StarsAdded>()
        .add_event::<StarsRemoved>()
        .run();
}

fn setup(
    mut commands: Commands,

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
    mut ev_spawn_stars: EventWriter<SpawnStars>,
) {
    let (mut projection, mut transform) = query_camera.single_mut();

    let char_up = input.any_pressed([KeyCode::KeyW]);
    let char_down = input.any_pressed([KeyCode::KeyS]);
    let char_left = input.any_pressed([KeyCode::KeyA]);
    let char_right = input.any_pressed([KeyCode::KeyD]);

    let char_q = input.any_just_pressed([KeyCode::KeyQ]); //zoom out
    let char_e = input.any_just_pressed([KeyCode::KeyE]); //zoom in

    let o_class = input.any_just_pressed([KeyCode::KeyZ]);
    let b_class = input.any_just_pressed([KeyCode::KeyX]);
    let a_class = input.any_just_pressed([KeyCode::KeyC]);
    let f_class = input.any_just_pressed([KeyCode::KeyV]);
    let g_class = input.any_just_pressed([KeyCode::KeyB]);
    let k_class = input.any_just_pressed([KeyCode::KeyN]);
    let m_class = input.any_just_pressed([KeyCode::KeyM]);

    let add_1000 = input.any_just_pressed([KeyCode::KeyU]);
    let remove_1000 = input.any_just_pressed([KeyCode::KeyJ]);
    let add_10000 = input.any_just_pressed([KeyCode::KeyI]);
    let remove_10000 = input.any_just_pressed([KeyCode::KeyK]);

    if add_1000 {
        ev_spawn_stars.send(SpawnStars(1000));
    } else if remove_1000 {
        ev_spawn_stars.send(SpawnStars(-1000));
    } else if add_10000 {
        ev_spawn_stars.send(SpawnStars(10000));
    } else if remove_10000 {
        ev_spawn_stars.send(SpawnStars(-10000));
    } else {
        ();
    }

    if o_class {
        masterok.o_class = !masterok.o_class;
    }
    if b_class {
        masterok.b_class = !masterok.b_class;
    }
    if a_class {
        masterok.a_class = !masterok.a_class;
    }
    if f_class {
        masterok.f_class = !masterok.f_class;
    }
    if g_class {
        masterok.g_class = !masterok.g_class;
    }
    if k_class {
        masterok.k_class = !masterok.k_class;
    }
    if m_class {
        masterok.m_class = !masterok.m_class;
    }

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
    masterok: Res<Masterik>,
) {
    draw_info_menu(&mut termres.terminal_info, &masterok);

    let mut frame = egui::Frame::default()
        .inner_margin(1.0)
        .outer_margin(1.0)
        .fill(egui::Color32::BLACK);

    egui::SidePanel::right("my_left_panel")
        .frame(frame)
        .min_width(322.0)
        .max_width(322.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(termres.terminal_info.backend_mut());
        });
}

fn draw_info_menu(terminal: &mut Terminal<RataguiBackend>, masterok: &Masterik) {
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
                Line::from(format!("Stars: {} ",masterok.total_stars)),
                Line::from("[U/J] - Add/Delete 1000 Stars"),
                Line::from("[I/K] - Add/Remove 10000 Stars"),
                Line::from(" "),
                Line::from("Spiral Arms: TODO!!"),
                Line::from("[O/L] - Add/Remove Spiral Arm"),
                Line::from(" "),
                Line::from("Toggle Star Types in Galaxy"),
                Line::from(" "),
                Line::from("[Z] - O-Class (Blue Giant)").style(if masterok.o_class {
                    Modifier::empty()
                } else {
                    Modifier::CROSSED_OUT
                }),
                Line::from("[X] - B-Class (Blue-White)").style(if masterok.b_class {
                    Modifier::empty()
                } else {
                    Modifier::CROSSED_OUT
                }),
                Line::from("[C] - A-Class (White)").style(if masterok.a_class {
                    Modifier::empty()
                } else {
                    Modifier::CROSSED_OUT
                }),
                Line::from("[V] - F-Class (Yellow-White Dwarf)").style(if masterok.f_class {
                    Modifier::empty()
                } else {
                    Modifier::CROSSED_OUT
                }),
                Line::from("[B] - G-Class (Sun)").style(if masterok.g_class {
                    Modifier::empty()
                } else {
                    Modifier::CROSSED_OUT
                }),
                Line::from("[N] - K-Class (Orange Dwarf)").style(if masterok.k_class {
                    Modifier::empty()
                } else {
                    Modifier::CROSSED_OUT
                }),
                Line::from("[M] - M-Class (Red Dwarf)").style(if masterok.m_class {
                    Modifier::empty()
                } else {
                    Modifier::CROSSED_OUT
                }),
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



fn generate_star_positions_in_range(start:i64, end:i64, masterok: &mut Masterik, star_data: &StarData ) {

    for boop in start..end {
        let angle = boop as f32 * 0.002;
        let radius = 90.0 * angle;
        let mut xik = radius * angle.cos() * 200.0;
        let mut yik = radius * angle.sin() * 200.0;

        // Create a small RNG and add randomness

        let random_star = masterok.rng.gen_range(0..1000000);

        let mut spawning_radius = if random_star > star_data.k_class_rarity {
            star_data.m_class_radius
        } else if random_star > star_data.g_class_rarity {
            star_data.k_class_radius
        } else if random_star > star_data.f_class_rarity {
            star_data.g_class_radius
        } else if random_star > star_data.a_class_rarity {
            star_data.f_class_radius
        } else if random_star > star_data.b_class_rarity {
            star_data.a_class_radius
        } else if random_star > star_data.o_class_rarity {
            star_data.b_class_radius
        } else {
            star_data.o_class_radius
        };
        
        let random_radius = masterok.rng.gen_range(0..50);
        spawning_radius += random_radius as f32;

        let rand_range = 10000.0 + (boop) as f32;

        let random_offset_x: f32 = masterok.rng.gen_range(-rand_range..rand_range);
        let random_offset_y: f32 = masterok.rng.gen_range(-rand_range..rand_range);

        xik += random_offset_x;
        yik += random_offset_y;

        if boop % 2 == 0 {
            xik = -xik;
            yik = -yik;
        }

        // Ensure the new circle does not overlap with any existing circles
        let mut attempts = 0;
        while masterok.positions.iter().any(|&(px, py , checking_radius)| {
            let dx = xik - px;
            let dy = yik - py;
            (((dx * dx) + (dy * dy)) as f64).sqrt() < (checking_radius + spawning_radius) as f64
        }) && attempts < 100
        {
            xik += masterok.rng.gen_range(-spawning_radius..spawning_radius);
            yik += masterok.rng.gen_range(-spawning_radius..spawning_radius);
            attempts += 1;
        }

        // Store the new circle position
        masterok.positions.push((xik, yik,spawning_radius));
     
      
    }


    



    
}

fn spawn_initial_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut masterok: ResMut<Masterik>,
    star_data: Res<StarData>,
) {
   

   generate_star_positions_in_range(1, masterok.total_stars.clone(), &mut masterok, &star_data);


   for (x,y,radius) in &masterok.positions {

    let radius = radius.clone();

    let star_color = if radius > 500.0 {Color::rgb_u8(159, 162, 222)} else if radius > 200.0 {Color::rgb_u8(240, 240, 254)} else if radius > 140.0 {Color::rgb_u8(248, 254, 252)} else  if radius > 99.0 {Color::rgb_u8(247, 254, 144)} else if radius > 45.0 {Color::rgb_u8(254, 170, 52)} else {Color::rgb_u8(254, 70, 70)};

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(radius.clone())).into(),
            // 4. Put something bright in a dark environment to see the effect
            material: materials.add(star_color),
            transform: Transform::from_translation(Vec3::new(x.clone(), y.clone(), 0.)),
            ..default()
        },
    ));


   }
    

   
    
}

fn star_watcher(mut ev_spawn_stars: EventReader<SpawnStars>, mut masterok: ResMut<Masterik>,  mut ev_stars_add: EventWriter<StarsAdded>,  mut ev_stars_remove: EventWriter<StarsRemoved>,) {
    //cant naively respawn all stars because it crashes if trying to spawn too many entities at once

    for ev in ev_spawn_stars.read() {
        let previous_value = masterok.total_stars.clone();

        let potential_value = (masterok.total_stars + ev.0);

        if (potential_value > 10000) && (potential_value < 1000000) {
            masterok.total_stars += ev.0;
            println!("add stars {:?} ", ev.0);
            println!("current stars {:?} ", masterok.total_stars);

            //removing stars
            if (ev.0 < 0) {
                ev_stars_remove.send(StarsRemoved(previous_value));
            } else {
                ev_stars_add.send(StarsAdded(previous_value));
            }
        }
    }
}


fn star_adder( mut masterok: ResMut<Masterik>, star_data: Res<StarData>, mut ev_stars_add: EventReader<StarsAdded>, mut commands: Commands,    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,) {
    //cant naively respawn all stars because it crashes if trying to spawn too many entities at once

    for ev in ev_stars_add.read() {
        let previous_value = ev.0;

        let new_value = (masterok.total_stars );

        

        if new_value > previous_value {
            let amount_added = new_value - previous_value;

            generate_star_positions_in_range(previous_value, new_value,&mut masterok, &star_data);

            let mut positions_clone = masterok.positions.clone();

            for new_star in 0..amount_added {

                let (x,y,radius) = positions_clone.pop().unwrap_or((0.0,0.0,0.0));

                let radius = radius.clone();

                let star_color = if radius > 500.0 {Color::rgb_u8(159, 162, 222)} else if radius > 200.0 {Color::rgb_u8(240, 240, 254)} else if radius > 140.0 {Color::rgb_u8(248, 254, 252)} else  if radius > 99.0 {Color::rgb_u8(247, 254, 144)} else if radius > 45.0 {Color::rgb_u8(254, 170, 52)} else {Color::rgb_u8(254, 70, 70)};
            
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::new(radius.clone())).into(),
                        // 4. Put something bright in a dark environment to see the effect
                        material: materials.add(star_color),
                        transform: Transform::from_translation(Vec3::new(x.clone(), y.clone(), 0.)),
                        ..default()
                    },
                ));


            }

        }


      
    }
}
