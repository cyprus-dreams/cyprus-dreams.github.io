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
    prelude::{Line, Span, Stylize, Terminal},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap, *},
};

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
    pub gen_seed: i64,
    pub spiral_arm_count: i64,
    pub camera_move_speed: f32,
    pub o_class: bool,
    pub b_class: bool,
    pub a_class: bool,
    pub f_class: bool,
    pub g_class: bool,
    pub k_class: bool,
    pub m_class: bool,
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
            camera_move_speed: 10.0,
            o_class: true,
            b_class: true,
            a_class: true,
            f_class: true,
            g_class: true,
            k_class: true,
            m_class: true,
        }
    }
}

#[derive(Event)]
pub struct SpawnStars(pub i64);

#[derive(Event)]
pub struct StarsRemoved(pub i64); //contains the previous amount of stars
#[derive(Event)]
pub struct StarsAdded(pub i64);//contains the previous amount of stars

#[derive(Component)]
pub struct StarCount(pub i64);
