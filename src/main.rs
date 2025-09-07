use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::*;
use common::{Input, View};
use lazy_static::lazy_static;
use scoundrel::{
    DrawDeck, Scoundrel, check_health, clear_action, handle_new_round, manage_selection,
    next_round, select_card, start_game,
};
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

    init.add_systems((setup, start_game, Tui::init_display).chain());
    update.add_systems(
        (
            Tui::process_input,
            Tui::display,
            manage_selection,
            select_card,
            check_health,
            next_round,
            handle_new_round,
            clear_action,
        )
            .chain(),
    );
    deinit.add_systems((Tui::deinit_display).chain());

    init.run(&mut world);

    while *RUNNING.lock().expect("main couldnt check RUNNING") {
        update.run(&mut world);
    }

    deinit.run(&mut world);
}
