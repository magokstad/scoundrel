use bevy_ecs::prelude::*;

use crate::cards::{Card, Deck};

#[derive(Component, Default)]
pub struct DrawDeck(pub Deck);

#[derive(Component, Default)]
pub struct DiscardDeck(pub Deck);

#[derive(Component, Default)]
pub struct Board(pub [Option<Card>; 4]);

#[derive(Component, Default)]
pub struct Selection(pub usize);

#[derive(Component, Default)]
pub enum WeaponSlot {
    #[default]
    Empty,
    Weapon {
        weapon: Card,
        enemies: Deck,
    },
}

#[derive(Component, Default, PartialEq, Eq)]
pub enum Action {
    #[default]
    Idle,

    Slot(usize),
    Select,
    Left,
    Right,

    SkipRoom,
    EndTurn,
}

#[derive(Component)]
pub struct Health(pub u8);
impl Default for Health {
    fn default() -> Self {
        Health(20)
    }
}

#[derive(Component)]
#[require(
    DrawDeck(|| DrawDeck(Deck::shuffled_scoundrel())),
    DiscardDeck,
    Board,
    WeaponSlot,
    Health,
    Action,
    Selection
)]
pub struct Scoundrel;

pub fn select_card(
    // mut commands: Commands,
    mut actions: Query<&mut Action>,
    mut selections: Query<&mut Selection>,
    mut draws: Query<&mut DrawDeck>,
    mut boards: Query<&mut Board>,
) {
    let action = actions.single();
    let selection = selections.single();
    let draw = draws.single();
    let board = boards.single();

    if *action != Action::Select {
        return;
    }
    let selec = board.0[selection.0];
    if selec.is_none() {
        return;
    }
    let selec = selec.unwrap();
}
