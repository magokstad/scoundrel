use bevy_ecs::system::Query;

use crate::scoundrel::{Board, DrawDeck, Health, WeaponSlot};

pub trait View {
    fn init_display();
    fn deinit_display();
    fn display(
        healths: Query<&Health>,
        boards: Query<&Board>,
        weapons: Query<&WeaponSlot>,
        draws: Query<&DrawDeck>,
    );
}
