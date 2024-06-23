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
use rand::rngs::SmallRng;
use ratatui::{
    layout::Rect,
    prelude::{Line, Span, Stylize, Terminal},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap, *},
};
use web_time::{Instant, SystemTime};

use rand::{Rng, SeedableRng};

//create resource to hold the ratatui terminal
#[derive(Resource)]
pub struct BevyTerminal<RataguiBackend: ratatui::backend::Backend> {
    pub terminal_info: Terminal<RataguiBackend>,
}

// Implement default on the resource to initialize it
impl Default for BevyTerminal<RataguiBackend> {
    fn default() -> Self {
        let mut backend1 = RataguiBackend::new(20, 20);
        backend1.set_font_size(14);
        let mut terminal1 = Terminal::new(backend1).unwrap();

        BevyTerminal {
            terminal_info: terminal1,
        }
    }
}

#[derive(Resource)]
pub struct Masterik {
    pub total_stars: i64,
    pub gen_seed: u64,
    pub spiral_arm_count: i64,
    pub camera_move_speed: f32,
    pub o_class: bool,
    pub b_class: bool,
    pub a_class: bool,
    pub f_class: bool,
    pub g_class: bool,
    pub k_class: bool,
    pub m_class: bool,
    pub rng: SmallRng,
    pub positions: PositionsVec,
}

impl Masterik {
    pub fn partial_reset(&mut self) {
        self.total_stars = 10000;
        self.gen_seed = self.rng.gen_range(1000..9000000000);
        self.rng = SmallRng::seed_from_u64(self.gen_seed);
        self.positions = Vec::new();
    }
}

impl Default for Masterik {
    fn default() -> Self {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut lol = SmallRng::seed_from_u64(ts.clone());
        let beep: u64 = lol.gen_range(1000..9000000000);
        let real = SmallRng::seed_from_u64(beep);

        Self {
            total_stars: 10000,
            rng: real,
            gen_seed: beep,
            spiral_arm_count: 2,
            camera_move_speed: 10.0,
            o_class: true,
            b_class: true,
            a_class: true,
            f_class: true,
            g_class: true,
            k_class: true,
            m_class: true,

            positions: Vec::new(),
        }
    }
}

//https://en.wikipedia.org/wiki/Stellar_classification#Harvard_spectral_classification
#[derive(Resource)]
pub struct StarData {
    pub o_class_radius: f32,
    pub b_class_radius: f32,
    pub a_class_radius: f32,
    pub f_class_radius: f32,
    pub g_class_radius: f32,
    pub k_class_radius: f32,
    pub m_class_radius: f32,
    pub o_class_rarity: i64,
    pub b_class_rarity: i64,
    pub a_class_rarity: i64,
    pub f_class_rarity: i64,
    pub g_class_rarity: i64,
    pub k_class_rarity: i64,
    pub m_class_rarity: i64,
}

impl Default for StarData {
    fn default() -> Self {
        Self {
            o_class_radius: 1600.0,
            b_class_radius: 500.0,
            a_class_radius: 200.0,
            f_class_radius: 150.0,
            g_class_radius: 100.0,
            k_class_radius: 50.0,
            m_class_radius: 10.0,
            o_class_rarity: 30,
            b_class_rarity: 2400,
            a_class_rarity: 9000,
            f_class_rarity: 30000,
            g_class_rarity: 70000,
            k_class_rarity: 120000,
            m_class_rarity: 760000,
        }
    }
}

#[derive(Event)]
pub struct SpawnStars(pub i64);

#[derive(Event)]
pub struct StarsRemoved(pub i64); //contains the previous amount of stars
#[derive(Event)]
pub struct StarsAdded(pub i64); //contains the previous amount of stars

#[derive(Event)]
pub struct ChangeSeed; //contains the previous amount of stars

#[derive(Event)]
pub struct RespawnStars; //contains the previous amount of stars

#[derive(Component)]
pub struct StarCount(pub i64);

pub type PositionsVec = Vec<(f32, f32, f32)>; // x y radius
