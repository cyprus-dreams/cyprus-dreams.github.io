//! Illustrates bloom post-processing in 2d.

use bevy::ecs::system::RunSystemOnce;
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

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use web_time::Instant;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

mod resources;
use resources::{
    BevyTerminal, ChangeSeed, Masterik, PositionsVec, RespawnStars, SpawnStars, StarCount,
    StarData, StarsAdded, StarsRemoved,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<Masterik>()
        .init_resource::<StarData>()
        .init_resource::<BevyTerminal<RataguiBackend>>()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, spawn_initial_stars)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, star_watcher)
        .add_systems(Update, star_adder)
        .add_systems(Update, star_remover)
        .add_systems(Update, despawn_all_stars)
        .add_event::<SpawnStars>()
        .add_event::<StarsAdded>()
        .add_event::<StarsRemoved>()
        .add_event::<ChangeSeed>()
        .add_event::<RespawnStars>()
        .run();
}

fn setup(mut commands: Commands, mut ev_respawn: EventWriter<RespawnStars>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            projection: OrthographicProjection {
                far: 1000.0,
                near: -1000.0,
                scale: 250.0,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));
    ev_respawn.send(RespawnStars);
}

fn keyboard_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut masterok: ResMut<Masterik>,
    mut query_camera: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    mut ev_spawn_stars: EventWriter<SpawnStars>,
    mut ev_change_seed: EventWriter<ChangeSeed>,
) {

    if !masterok.block_input {
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
    
        let add_angle = input.any_just_pressed([KeyCode::KeyT]);
        let remove_angle = input.any_just_pressed([KeyCode::KeyG]);
        let add_radius = input.any_just_pressed([KeyCode::KeyY]);
        let remove_radius = input.any_just_pressed([KeyCode::KeyH]);
        let add_distance = input.any_just_pressed([KeyCode::KeyU]);
        let remove_distance = input.any_just_pressed([KeyCode::KeyJ]);
    
        let add_1000 = input.any_just_pressed([KeyCode::KeyI]);
        let remove_1000 = input.any_just_pressed([KeyCode::KeyK]);
        let add_10000 = input.any_just_pressed([KeyCode::KeyO]);
        let remove_10000 = input.any_just_pressed([KeyCode::KeyL]);
    
        let mut change_seed = input.any_just_pressed([KeyCode::KeyR]);
        let  reset_to_default = input.any_just_pressed([KeyCode::KeyF]);

        if reset_to_default {
            *masterok = Masterik::default();
            change_seed = true;
        }
    
        if add_angle {

            masterok.angle_mod += 0.00011;
            change_seed = true;
        }
        if remove_angle && (masterok.angle_mod >0.00006){
            masterok.angle_mod -= 0.00005;
            change_seed = true;
        }
        if add_radius {
            masterok.radius_mod += 30.0;
            change_seed = true;
        }
        if remove_radius && (masterok.radius_mod >30.0){
            masterok.radius_mod -= 20.0;
            change_seed = true;
        }

        if add_distance {
            masterok.distance_mod += 30.0;
            change_seed = true;
        }
        if remove_distance && (masterok.distance_mod >30.0){
            masterok.distance_mod -= 20.0;
            change_seed = true;
        }
    
   
    
           let add_arm = input.any_just_pressed([KeyCode::BracketRight]);
            let delete_arm = input.any_just_pressed([KeyCode::BracketLeft]);
    
         if add_arm && (masterok.spiral_arm_count <4) {masterok.spiral_arm_count+=1;
                change_seed=true;}
            else    if delete_arm && (masterok.spiral_arm_count >1) {masterok.spiral_arm_count-=1;
                    change_seed=true;}
        
    
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
            change_seed = true;
        }
        if b_class {
            masterok.b_class = !masterok.b_class;
            change_seed = true;
        }
        if a_class {
            masterok.a_class = !masterok.a_class;
            change_seed = true;
        }
        if f_class {
            masterok.f_class = !masterok.f_class;
            change_seed = true;
        }
        if g_class {
            masterok.g_class = !masterok.g_class;
            change_seed = true;
        }
        if k_class {
            masterok.k_class = !masterok.k_class;
            change_seed = true;
        }
        if m_class {
            masterok.m_class = !masterok.m_class;
            change_seed = true;
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
    
        if change_seed {
            masterok.block_input = true;
            masterok.partial_reset();
            ev_change_seed.send(ChangeSeed);
        }
    
        let char_backspace = input.any_pressed([KeyCode::Backspace, KeyCode::Delete]);
    
        if char_backspace {
            panic!("BYE");
        }
    }

}

// Render to the terminal and to egui , both are immediate mode
fn ui_example_system(
    mut contexts: EguiContexts,
    mut termres: ResMut<BevyTerminal<RataguiBackend>>,
    masterok: Res<Masterik>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
        .unwrap_or(60.0);

    draw_info_menu(&mut termres.terminal_info, &masterok, fps);

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

fn draw_info_menu(terminal: &mut Terminal<RataguiBackend>, masterok: &Masterik, fps: f64) {
    terminal
        .draw(|frame| {
            let area = frame.size();

            let mut lines = (Text::from(vec![
                Line::from(format!("FPS: {} ", fps)),
                Line::from(" "),
                Line::from("[WASD] - Move Camera "),
                Line::from("[Q/E] - Zoom Out/In"),
                Line::from(" "),
                Line::from(format!("Seed: {} ", masterok.gen_seed)),
                Line::from("[R] - Change Seed"),
                Line::from("[F] - Default Settings"),
                Line::from(" "),
                Line::from(format!("Stars: {} ", masterok.total_stars + 10000)), //adding 10000 here because I spawn 10000 stars to act as the backdrop of the galaxy
                Line::from("[I/K] - Add/Delete 1000 Stars"),
                Line::from("[O/L] - Add/Remove 10000 Stars"),
                Line::from(" "),
                Line::from(format!("Spiral Arms: {} ", masterok.spiral_arm_count)),
                Line::from("[[/]] - Add/Remove Spiral Arm"),
                Line::from(" "),
                Line::from("Increase / Decrease"),
                Line::from(" "),
                Line::from(format!("[T/G] Galaxy Angle Mod: {} ", masterok.angle_mod)),
                Line::from(format!("[Y/H] Galaxy Radius Mod: {} ", masterok.radius_mod)),
                Line::from(format!(
                    "[U/J] Galaxy Distance Mod: {} ",
                    masterok.distance_mod
                )),
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

fn generate_star_positions_in_range(
    start: i64,
    end: i64,
    masterok: &mut Masterik,
    star_data: &StarData,
) {
    for boop in start..end {
        let random_angle: f32 = masterok.rng.gen_range(0.0..(masterok.angle_mod ));

        let mut angle = (boop as f32) * (0.0002 + random_angle);
       

        let random_radius: f32 = masterok.rng.gen_range(2.0..(masterok.radius_mod ));
        let radius = (masterok.radius_mod + random_radius) * angle;
        let mut xik = radius * angle.cos() * masterok.distance_mod;
        let mut yik = radius * angle.sin() * masterok.distance_mod;

        // Create a small RNG and add randomness

        let random_star = masterok.rng.gen_range(0..1000000);

        let mut spawning_radius = if (random_star > star_data.k_class_rarity) && (masterok.m_class)
        {
            star_data.m_class_radius
        } else if (random_star > star_data.g_class_rarity) && (masterok.k_class) {
            star_data.k_class_radius
        } else if (random_star > star_data.f_class_rarity) && (masterok.g_class) {
            star_data.g_class_radius
        } else if (random_star > star_data.a_class_rarity) && (masterok.f_class) {
            star_data.f_class_radius
        } else if (random_star > star_data.b_class_rarity) && (masterok.a_class) {
            star_data.a_class_radius
        } else if (random_star > star_data.o_class_rarity) && (masterok.b_class) {
            star_data.b_class_radius
        } else if (masterok.o_class) {
            star_data.o_class_radius
        } else {
            70.0
        };

        //  spawning_radius += random_radius as f32;

        let rand_range = 5000.0 as f32;

        let random_offset_x: f32 = masterok.rng.gen_range(-rand_range..rand_range);
        let random_offset_y: f32 = masterok.rng.gen_range(-rand_range..rand_range);

        xik += random_offset_x;
        yik += random_offset_y;


        if (boop % 3 == 0) && masterok.spiral_arm_count > 2 {
            let holder = xik.clone();
            xik = yik;
            yik = -holder;
        }else if (boop % 2 == 0) && masterok.spiral_arm_count > 1 {
            
            xik = -xik;
            yik = -yik;
        }

       /*
        if (boop % 2 == 0) && masterok.spiral_arm_count > 1 {
            xik = -xik;
            yik = -yik;
        }
        */

        // Ensure the new circle does not overlap with any existing circles
        let mut attempts = 0;
        while masterok.positions.iter().any(|&(px, py, checking_radius)| {
            let dx = xik - px;
            let dy = yik - py;
            (((dx * dx) + (dy * dy)) as f64).sqrt() < (checking_radius + spawning_radius) as f64
        }) && attempts < 10
        {
            xik += masterok.rng.gen_range(-5000.0..5000.0);
            yik += masterok.rng.gen_range(-5000.0..5000.0);
            attempts += 1;
        }

        if attempts < 9 {
            // Store the new circle position
            masterok.positions.push((xik, yik, spawning_radius));
        }
    }
}

fn spawn_initial_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut masterok: ResMut<Masterik>,
    star_data: Res<StarData>,
    mut ev_respawn: EventReader<RespawnStars>,
    //  mut query_camera: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
) {
    for resp in ev_respawn.read() {
        //  let (mut projection, mut transform) = query_camera.single_mut();
        //   transform.rotate_local_z(masterok.initial_angle);

        generate_star_positions_in_range(
            1,
            masterok.total_stars.clone(),
            &mut masterok,
            &star_data,
        );

        let mut initial_counter = 0;

        for (x, y, radius) in &masterok.positions {
            initial_counter += 1;

            let radius = radius.clone();

            let star_color = if radius > 500.0 {
                Color::rgb_u8(159, 162, 222)
            } else if radius > 200.0 {
                Color::rgb_u8(240, 240, 254)
            } else if radius > 140.0 {
                Color::rgb_u8(248, 254, 252)
            } else if radius > 99.0 {
                Color::rgb_u8(247, 254, 144)
            } else if radius > 45.0 {
                Color::rgb_u8(254, 170, 52)
            } else {
                Color::rgb_u8(254, 70, 70)
            };

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(radius.clone())).into(),
                    // 4. Put something bright in a dark environment to see the effect
                    material: materials.add(star_color),
                    transform: Transform::from_translation(Vec3::new(x.clone(), y.clone(), 0.)),
                    ..default()
                },
                StarCount(initial_counter),
            ));
        }

        for randomczik in 1..10000 {
            // initial_counter += 1;

            let spawning_radius: f32 = masterok.rng.gen_range(10.0..60.0);
            let radius = spawning_radius.clone();

            let rand_range = randomczik as f32 * 30.0;

            let mut random_offset_x: f32 = masterok.rng.gen_range(-rand_range..rand_range);
            let mut random_offset_y: f32 = masterok.rng.gen_range(-rand_range..rand_range);

            let star_color = if radius > 500.0 {
                Color::rgb_u8(159, 162, 222)
            } else if radius > 200.0 {
                Color::rgb_u8(240, 240, 254)
            } else if radius > 140.0 {
                Color::rgb_u8(248, 254, 252)
            } else if radius > 99.0 {
                Color::rgb_u8(247, 254, 144)
            } else if radius > 45.0 {
                Color::rgb_u8(254, 170, 52)
            } else {
                Color::rgb_u8(254, 70, 70)
            };

            // Ensure the new circle does not overlap with any existing circles
            let mut attempts = 0;
            while masterok.positions.iter().any(|&(px, py, checking_radius)| {
                let dx = random_offset_x - px;
                let dy = random_offset_y - py;
                (((dx * dx) + (dy * dy)) as f64).sqrt() < (checking_radius + radius) as f64
            }) && attempts < 10
            {
                random_offset_x += masterok.rng.gen_range(-radius..radius);
                random_offset_y += masterok.rng.gen_range(-radius..radius);
                attempts += 1;
            }

            if attempts < 9 {
                // Store the new circle position
                masterok
                    .positions
                    .push((random_offset_x, random_offset_y, radius));

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::new(radius.clone())).into(),
                        // 4. Put something bright in a dark environment to see the effect
                        material: materials.add(star_color),
                        transform: Transform::from_translation(Vec3::new(
                            random_offset_x,
                            random_offset_y,
                            0.,
                        )),
                        ..default()
                    },
                    StarCount(11),
                ));
            }
        }
        masterok.block_input = false;
    }
}

fn star_watcher(
    mut ev_spawn_stars: EventReader<SpawnStars>,
    mut masterok: ResMut<Masterik>,
    mut ev_stars_add: EventWriter<StarsAdded>,
    mut ev_stars_remove: EventWriter<StarsRemoved>,
) {
    //cant naively respawn all stars because it crashes if trying to spawn too many entities at once

    for ev in ev_spawn_stars.read() {
        let previous_value = masterok.total_stars.clone();

        let potential_value = (masterok.total_stars + ev.0);

        if (potential_value > 0) && (potential_value < 101000) {
            masterok.total_stars += ev.0;
         

            //removing stars
            if (ev.0 < 0) {
                ev_stars_remove.send(StarsRemoved(previous_value));
            } else {
                ev_stars_add.send(StarsAdded(previous_value));
            }
        }
    }
}

fn star_adder(
    mut masterok: ResMut<Masterik>,
    star_data: Res<StarData>,
    mut ev_stars_add: EventReader<StarsAdded>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //cant naively respawn all stars because it crashes if trying to spawn too many entities at once

    for ev in ev_stars_add.read() {
        let previous_value = ev.0;

        let new_value = (masterok.total_stars);

        if new_value > previous_value {
            let amount_added = new_value - previous_value;

            generate_star_positions_in_range(previous_value, new_value, &mut masterok, &star_data);

            let mut positions_clone = masterok.positions.clone();

            for new_star in 0..amount_added {
                let (x, y, radius) = positions_clone.pop().unwrap_or((0.0, 0.0, 0.0));

                let radius = radius.clone();

                let star_color = if radius > 500.0 {
                    Color::rgb_u8(159, 162, 222)
                } else if radius > 200.0 {
                    Color::rgb_u8(240, 240, 254)
                } else if radius > 140.0 {
                    Color::rgb_u8(248, 254, 252)
                } else if radius > 99.0 {
                    Color::rgb_u8(247, 254, 144)
                } else if radius > 45.0 {
                    Color::rgb_u8(254, 170, 52)
                } else {
                    Color::rgb_u8(254, 70, 70)
                };

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::new(radius.clone())).into(),
                        // 4. Put something bright in a dark environment to see the effect
                        material: materials.add(star_color),
                        transform: Transform::from_translation(Vec3::new(x.clone(), y.clone(), 0.)),
                        ..default()
                    },
                    StarCount(previous_value + new_star),
                ));
            }
        }
    }
}

fn star_remover(
    mut masterok: ResMut<Masterik>,
    star_data: Res<StarData>,
    mut ev_stars_remove: EventReader<StarsRemoved>,

    mut commands: Commands,
    query: Query<(Entity, &StarCount)>,
) {
    //cant naively respawn all stars because it crashes if trying to spawn too many entities at once

    for ev in ev_stars_remove.read() {
        let previous_value = ev.0;

        let new_value = (masterok.total_stars);

        let amount_remove = previous_value-new_value;

        if amount_remove > 0 {

            for counter in 1..amount_remove {
                masterok.positions.pop();
            }


            for (entity, sc) in query.iter() {
                if sc.0 > new_value {
                    commands.entity(entity).despawn();
                }
            }


        }

       
    }
}

fn despawn_all_stars(
    mut ev_change_seed: EventReader<ChangeSeed>,
    mut ev_respawn: EventWriter<RespawnStars>,

    mut commands: Commands,
    query: Query<(Entity, &StarCount)>,
) {
    for ev in ev_change_seed.read() {
        for (entity, sc) in query.iter() {
            commands.entity(entity).despawn();
        }
        ev_respawn.send(RespawnStars);
    }
}
