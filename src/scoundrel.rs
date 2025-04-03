use anyhow::Result;
use bevy_ecs::prelude::*;

use crate::cards::Deck;

#[derive(Component, Default)]
pub struct DrawDeck(pub Deck);

#[derive(Component, Default)]
pub struct DiscardDeck(pub Deck);

#[derive(Component, Default)]
pub struct Board([Option<Entity>; 4]);

#[derive(Component, Default)]
pub enum WeaponSlot {
    #[default]
    Empty,
    Weapon {
        card: Entity,
        enemies: Vec<Entity>,
    },
}

#[derive(Component)]
pub struct Health(pub u8);
impl Default for Health {
    fn default() -> Self {
        Health(20)
    }
}

#[derive(Bundle, Default)]
pub struct ScoundrelBundle {
    draw_deck: DrawDeck,
    discard_deck: DiscardDeck,
    board: Board,
    weapon: WeaponSlot,
    health: Health,
}

impl ScoundrelBundle {
    pub fn new(start_deck: Deck) -> Self {
        Self {
            draw_deck: DrawDeck(start_deck),
            ..Default::default()
        }
    }
}

pub fn draw_card(
    mut commands: Commands,
    mut draws: Query<&mut DrawDeck>,
    mut boards: Query<&mut Board>,
) {
    let draw = draws.single();
    let board = boards.single();
}
