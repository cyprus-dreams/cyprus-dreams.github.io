//! Illustrates bloom post-processing in 2d.

use bevy::asset::embedded_asset;
use bevy::core_pipeline::bloom::BloomPrefilterSettings;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
};
use bevy_egui::{
    egui::{self, Frame},
    EguiContexts, EguiPlugin,
};

use egui_ratatui::RataguiBackend;
use ratatui::{
    layout::Rect,
    prelude::{Line, Modifier, Span, Stylize, Terminal},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap, *},
};

use rand::{Rng, SeedableRng};

mod resources;
use resources::{
    BevyTerminal, ChangeSeed, Masterik, PositionsVec, RespawnStars, SpawnStars, StarCount,
    StarData, StarsAdded, StarsRemoved,
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<Masterik>()
        .init_resource::<StarData>()
        .init_resource::<BevyTerminal<RataguiBackend>>()
        .insert_resource(ClearColor(Color::rgb(0.01, 0.01, 0.01)))
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
        .add_event::<RespawnStars>();

    embedded_asset!(app, "star.png"); //embedding assets to exe

    app.run();
}

fn setup(
    mut commands: Commands,
    mut ev_respawn: EventWriter<RespawnStars>,
    server: Res<AssetServer>,
    mut masterok: ResMut<Masterik>,
) {
    let bloom_set = BloomSettings {
        intensity: 0.75,
        low_frequency_boost: 0.5,
        low_frequency_boost_curvature: 0.5,
        high_pass_frequency: 1.0,
        prefilter_settings: BloomPrefilterSettings {
            threshold: 0.0,
            threshold_softness: 0.0,
        },
        composite_mode: BloomCompositeMode::Additive,
    };
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            projection: OrthographicProjection {
                far: 90000.0,
                near: -90000.0,
                scale: 1400.0,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        bloom_set, // 3. Enable bloom for the camera
    ));

    // Send event to spawn stars
    ev_respawn.send(RespawnStars);
}

fn keyboard_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut masterok: ResMut<Masterik>,
    mut query_camera: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    mut ev_spawn_stars: EventWriter<SpawnStars>,
    mut ev_change_seed: EventWriter<ChangeSeed>,
) {
    //Block input during star spawning process
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
        let reset_to_default = input.any_just_pressed([KeyCode::KeyF]);

        let add_arm = input.any_just_pressed([KeyCode::KeyP]);
        let delete_arm = input.any_just_pressed([KeyCode::Semicolon]);

        //change_seed gets set to true to trigger regeneration of stars to apply new settings, because star position generation is dependent on settings

        if reset_to_default {
            *masterok = Masterik::default();
            change_seed = true;
        }

        if add_angle {
            masterok.angle_mod += 0.0001;
            change_seed = true;
        }
        if remove_angle && (masterok.angle_mod > 0.0001) {
            masterok.angle_mod -= 0.0001;
            change_seed = true;
        }
        if add_radius {
            masterok.radius_mod += 300.0;
            change_seed = true;
        }
        if remove_radius && (masterok.radius_mod > 310.0) {
            masterok.radius_mod -= 200.0;
            change_seed = true;
        }

        if add_distance {
            masterok.distance_mod += 10.0;
            change_seed = true;
        }
        if remove_distance && (masterok.distance_mod > 11.0) {
            masterok.distance_mod -= 10.0;
            change_seed = true;
        }

        //max 4 arms using simple algorithim
        if add_arm && (masterok.spiral_arm_count < 4) {
            masterok.spiral_arm_count += 1;
            change_seed = true;
        } else if delete_arm && (masterok.spiral_arm_count > 1) {
            masterok.spiral_arm_count -= 1;
            change_seed = true;
        }

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
            projection.scale *= 2.0;
        }
        if char_e {
            // zoom in
            projection.scale /= 2.0;
        }

        //block input during star spawning, reset positions and star count, send respawn event
        if change_seed {
            masterok.block_input = true;
            masterok.partial_reset();
            ev_change_seed.send(ChangeSeed);
        }

        let char_backspace = input.any_pressed([KeyCode::Backspace, KeyCode::Delete]);

        //quit app hack, disable for wasm build
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

    //draws info to ratatui terminal
    draw_info_menu(&mut termres.terminal_info, &masterok, fps);

    let mut frame = egui::Frame::default()
        .inner_margin(1.0)
        .outer_margin(1.0)
        .fill(egui::Color32::BLACK);

    //limit panel to certain size that is guaranteed to fit text
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
                Line::from(format!("FPS: {} ", fps as i64)),
                Line::from(" "),
                Line::from("[WASD] - Move Camera "),
                Line::from("[Q/E] - Zoom Out/In"),
                Line::from(" "),
                Line::from(format!("Seed: {} ", masterok.gen_seed)),
                Line::from("[R] - Change Seed"),
                Line::from("[F] - Default Settings"),
                Line::from(" "),
                Line::from(format!("Stars: {} ", masterok.total_stars + 20000)), //adding 30000 here because I spawn 30000 stars to act as the backdrop of the galaxy
                Line::from("[I/K] - Add/Delete 1000 Stars"),
                Line::from("[O/L] - Add/Remove 10000 Stars"),
                Line::from(" "),
                Line::from(format!("Spiral Arms: {} ", masterok.spiral_arm_count)),
                Line::from("[P/;] - Add/Remove Spiral Arm"),
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
                    .block(Block::new().title("Kosmos").gray().borders(Borders::ALL)),
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
    let rand_range = 20000.0 as f32;
    for mut star_index in start..end {
        //this keeps stars closer to center when spawning more stars, since the spawning alternates arms
        star_index = star_index / (masterok.spiral_arm_count);
        //randomness to make it look natural
        let random_angle: f32 = masterok.rng.gen_range(0.0..(masterok.angle_mod));

        let angle = (star_index as f32) * (0.0002 + random_angle);

        let random_radius: f32 = masterok.rng.gen_range(2.0..(masterok.radius_mod));
        let radius = (masterok.radius_mod + random_radius) * angle;
        let mut xik = radius * angle.cos() * masterok.distance_mod;
        let mut yik = radius * angle.sin() * masterok.distance_mod;

        let random_star = masterok.rng.gen_range(0..1000000);

        let spawning_radius = if (random_star > star_data.k_class_rarity) && (masterok.m_class) {
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

        let random_offset_x: f32 = masterok.rng.gen_range(-rand_range..rand_range);
        let random_offset_y: f32 = masterok.rng.gen_range(-rand_range..rand_range);

        xik += random_offset_x;
        yik += random_offset_y;

        //this creates the spiral arms
        if (star_index % 5 == 0) && masterok.spiral_arm_count > 3 {
            let holder = yik.clone();
            yik = xik;
            xik = -holder;
        } else if (star_index % 3 == 0) && masterok.spiral_arm_count > 2 {
            let holder = xik.clone();
            xik = yik;
            yik = -holder;
        } else if (star_index % 2 == 0) && masterok.spiral_arm_count > 1 {
            xik = -xik;
            yik = -yik;
        }

        // Ensure the new circle does not overlap with any existing circles
        let mut attempts = 0;
        while masterok.positions.iter().any(|&(px, py, checking_radius)| {
            let dx = xik - px;
            let dy = yik - py;
            (((dx * dx) + (dy * dy)) as f64).sqrt() < (checking_radius + spawning_radius) as f64
        }) && attempts < 20
        {
            xik += masterok.rng.gen_range(-50000.0..50000.0);
            yik += masterok.rng.gen_range(-50000.0..50000.0);
            attempts += 1;
        }

        if attempts < 19 {
            // Store the new circle position
            masterok.positions.push((xik, yik, spawning_radius));
        }
    }
}

fn star_color_from_radius(radius: &f32, star_data: &StarData) -> Color {
    let test_radius = radius + 10.0;

    if test_radius > star_data.b_class_radius {
        Color::rgb_u8(5, 5, 250)
    } else if test_radius > star_data.a_class_radius {
        Color::rgb_u8(10, 10, 240)
    } else if test_radius > star_data.f_class_radius {
        Color::rgb_u8(250, 250, 250)
    } else if test_radius > star_data.g_class_radius {
        Color::rgb_u8(200, 100, 100)
    } else if test_radius > star_data.k_class_radius {
        Color::rgb_u8(254, 170, 52)
    } else {
        Color::rgb_u8(30, 0, 0)
    }
}

fn spawn_initial_stars(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut masterok: ResMut<Masterik>,
    star_data: Res<StarData>,
    mut ev_respawn: EventReader<RespawnStars>,
) {
    for _ in ev_respawn.read() {
        generate_star_positions_in_range(
            1,
            masterok.total_stars.clone(),
            &mut masterok,
            &star_data,
        );

        let mut initial_counter = 0;

        //this is for embedding assets
        let crate_name = "spiral_galaxy";

        let path = std::path::Path::new(crate_name).join("star.png");
        let source = bevy::asset::io::AssetSourceId::from("embedded");
        let asset_path = bevy::asset::AssetPath::from_path(&path).with_source(source);

        let star: Handle<Image> = asset_server.load(asset_path);

        for (x, y, radius) in &masterok.positions {
            initial_counter += 1;

            let radius = radius.clone();
            let test_radius = radius + 10.0;

            let star_color = star_color_from_radius(&test_radius, &star_data);

            let mut transform = Transform::from_translation(Vec3::new(x.clone(), y.clone(), 0.));
            //rotate stars a bit to make it look more natural
            transform.rotate_local_z(x.clone());
            commands.spawn((
                SpriteBundle {
                    texture: star.clone(),
                    transform: transform,
                    sprite: Sprite {
                        color: star_color,
                        custom_size: Some(Vec2::splat(radius.clone() * 2.0)),
                        ..default()
                    },
                    ..default()
                },
                StarCount(initial_counter),
            ));
        }

        //spawns stars that act as backdrop of the galaxy
        for randomczik in 1..20000 {
            // initial_counter += 1;

            let spawning_radius: f32 = masterok
                .rng
                .gen_range(10.0..(star_data.k_class_radius + 100.0));

            let rand_range = randomczik as f32 * 90.0;

            let mut random_offset_x: f32 = masterok.rng.gen_range(-rand_range..rand_range);
            let mut random_offset_y: f32 = masterok.rng.gen_range(-rand_range..rand_range);

            let radius = spawning_radius.clone();
            //buffer radius for testing to avoid floating point errors
            let test_radius = radius + 10.0;

            let star_color = star_color_from_radius(&test_radius, &star_data);

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

                let mut transform = Transform::from_translation(Vec3::new(
                    random_offset_x.clone(),
                    random_offset_y.clone(),
                    0.,
                ));
                //rotate stars a bit to make it look more natural
                transform.rotate_local_z(random_offset_x.clone());

                commands.spawn((
                    SpriteBundle {
                        texture: star.clone(),
                        transform: transform,
                        sprite: Sprite {
                            color: star_color, // 4. Put something bright in a dark environment to see the effect
                            custom_size: Some(Vec2::splat(radius.clone() * 2.0)),
                            ..default()
                        },
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
    for ev in ev_spawn_stars.read() {
        let previous_value = masterok.total_stars.clone();

        let potential_value = (masterok.total_stars + ev.0);

        if (potential_value > 0) && (potential_value < 301000) {
            masterok.total_stars += ev.0;

            //removing stars
            if (ev.0 < 0) {
                ev_stars_remove.send(StarsRemoved(previous_value));
            } else
            //adding stars
            {
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

    asset_server: Res<AssetServer>,
) {
    for ev in ev_stars_add.read() {
        let previous_value = ev.0;

        let new_value = (masterok.total_stars);

        if new_value > previous_value {
            let amount_added = new_value - previous_value;

            generate_star_positions_in_range(previous_value, new_value, &mut masterok, &star_data);

            let mut positions_clone = masterok.positions.clone();

            let crate_name = "spiral_galaxy";

            let path = std::path::Path::new(crate_name).join("star.png");
            let source = bevy::asset::io::AssetSourceId::from("embedded");
            let asset_path = bevy::asset::AssetPath::from_path(&path).with_source(source);

            let star: Handle<Image> = asset_server.load(asset_path);

            for new_star in 0..amount_added {
                let (x, y, radius) = positions_clone.pop().unwrap_or((0.0, 0.0, 0.0));

                let radius = radius.clone();
                let test_radius = radius + 10.0;

                let star_color = star_color_from_radius(&test_radius, &star_data);

                let mut transform =
                    Transform::from_translation(Vec3::new(x.clone(), y.clone(), 0.));
                transform.rotate_local_z(x.clone());

                commands.spawn((
                    SpriteBundle {
                        texture: star.clone(),
                        transform: transform,
                        sprite: Sprite {
                            color: star_color, // 4. Put something bright in a dark environment to see the effect
                            custom_size: Some(Vec2::splat(radius.clone() * 2.0)),
                            ..default()
                        },
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

    mut ev_stars_remove: EventReader<StarsRemoved>,

    mut commands: Commands,
    query: Query<(Entity, &StarCount)>,
) {
    for ev in ev_stars_remove.read() {
        let previous_value = ev.0;

        let new_value = (masterok.total_stars);

        let amount_remove = previous_value - new_value;

        if amount_remove > 0 {
            for _ in 1..amount_remove {
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
