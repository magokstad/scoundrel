use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::*;
use common::{Input, View};
use lazy_static::lazy_static;
use scoundrel::{DrawDeck, Scoundrel, select_card};
use tui::Tui;

pub mod cards;
pub mod common;
pub mod scoundrel;
pub mod tui;

lazy_static! {
    static ref RUNNING: Mutex<bool> = Mutex::new(true);
}

fn setup(mut commands: Commands) {
    commands.spawn(Scoundrel);
}

fn main() {
    let mut world = World::new();

    let mut init = Schedule::default();
    let mut deinit = Schedule::default();
    let mut update = Schedule::default();

    init.add_systems((setup, Tui::init_display).chain());
    update.add_systems((Tui::process_input, Tui::display, select_card).chain());
    deinit.add_systems((Tui::deinit_display).chain());

    init.run(&mut world);

    while *RUNNING.lock().expect("main couldnt check RUNNING") {
        update.run(&mut world);
    }

    deinit.run(&mut world);
}
