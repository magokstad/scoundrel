use std::sync::atomic::{AtomicU16, Ordering};

use bevy_ecs::system::Query;

use crate::scoundrel::{Action, Board, DrawDeck, Health, Selection, WeaponSlot};

/// Interface for systems related to views
pub trait View {
    fn init_display();
    fn deinit_display();
    fn display(
        healths: Query<&Health>,
        boards: Query<&Board>,
        weapons: Query<&WeaponSlot>,
        draws: Query<&DrawDeck>,
        selections: Query<&Selection>,
    );
}

/// Interface for systems related to input
pub trait Input {
    fn process_input(actions: Query<&mut Action>);
}

/// Trait for load and store of u16
pub trait LdSt {
    fn ld(&self) -> u16;
    fn st(&self, n: u16);
    fn inc(&self) -> u16;
    fn dec(&self) -> u16;
}

/// Made for AtomicU16
impl LdSt for AtomicU16 {
    fn ld(&self) -> u16 {
        self.load(Ordering::Relaxed)
    }
    fn st(&self, n: u16) {
        self.store(n, Ordering::Relaxed);
    }

    fn inc(&self) -> u16 {
        self.fetch_add(1, Ordering::Relaxed)
    }

    fn dec(&self) -> u16 {
        self.fetch_sub(1, Ordering::Relaxed)
    }
}
