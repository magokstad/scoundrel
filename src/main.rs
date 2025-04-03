use bevy_ecs::prelude::*;
use cards::Deck;
use scoundrel::{ScoundrelBundle, draw_card};

pub mod cards;
pub mod common;
pub mod scoundrel;
pub mod view;

fn setup(mut commands: Commands) {
    let deck = Deck::new_classic(&mut commands);
    let scound = ScoundrelBundle::new(deck);
    commands.spawn(scound);
}

fn main() {
    let mut world = World::new();
    let mut init = Schedule::default();
    let mut update = Schedule::default();

    init.add_systems(setup);
    update.add_systems(draw_card);

    init.run(&mut world);
}
