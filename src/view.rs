use bevy_ecs::prelude::*;
use crossterm::{event::*, execute};
use lazy_static::lazy_static;

use crate::{
    common::View,
    scoundrel::{Board, DrawDeck, Health, WeaponSlot},
};

lazy_static! {
    static ref BAWLS: u32 = 3;
}

struct CrosstermView;

impl View for CrosstermView {
    fn init_display() {
        execute!(std::io::stdout(), EnableFocusChange, EnableMouseCapture).unwrap();
    }

    fn deinit_display() {
        execute!(std::io::stdout(), DisableFocusChange, DisableMouseCapture).unwrap();
    }

    fn display(
        healths: Query<&Health>,
        boards: Query<&Board>,
        weapons: Query<&WeaponSlot>,
        draws: Query<&DrawDeck>,
    ) {
        let health = healths.get_single();
        let board = boards.get_single();
        let weapon = weapons.get_single();
        let draw = draws.get_single();
    }
}
