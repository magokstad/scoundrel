use std::{cmp::min, ops::Add};

use bevy_ecs::prelude::*;

use crate::{
    RUNNING,
    cards::{Card, Deck, Suit},
};

#[derive(Component, Default)]
pub struct DrawDeck(pub Deck);

#[derive(Component, Default)]
pub struct DiscardDeck(pub Deck);

#[derive(Component, Default)]
pub struct Board(pub [Option<Card>; 4]);

#[derive(Component, Default)]
pub struct Selection(pub usize);

#[derive(Component, Default)]
pub struct Status {
    queue_next_round: bool,
    has_healed: bool,
    avoided_last: bool,
}

#[derive(Component, Default, Clone)]
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

    BareHand,

    SkipRoom,
    EndTurn,
}

#[derive(Component)]
pub struct Health(pub i8);
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
    Selection,
    Status
)]
pub struct Scoundrel;

pub fn start_game(mut boards: Query<&mut Board>, mut draws: Query<&mut DrawDeck>) {
    let mut board = boards.single_mut();
    let mut draw = draws.single_mut();

    for i in 0..4 {
        board.0[i] = draw.0.draw();
    }
}

pub fn select_card(
    // mut commands: Commands,
    actions: Query<&mut Action>,
    selections: Query<&mut Selection>,
    // mut draws: Query<&mut DrawDeck>,
    mut boards: Query<&mut Board>,
    mut weapons: Query<&mut WeaponSlot>,
    mut healths: Query<&mut Health>,
    mut statuses: Query<&mut Status>,
) {
    let action = actions.single();
    let selection = selections.single();
    // let mut draw = draws.single_mut();
    let mut board = boards.single_mut();
    let mut health = healths.single_mut();
    let mut weapon_slot = weapons.single_mut();
    let mut status = statuses.single_mut();

    // Have to spare last card
    let n_cards = board.0.iter().filter(|card| card.is_some()).count();
    if n_cards <= 1 {
        return;
    };

    if *action != Action::Select || *action != Action::BareHand {
        return;
    }

    match board.0[selection.0] {
        Some(selected) => match selected.suit {
            // HEAL
            Suit::Hearts => {
                if !status.has_healed {
                    health.0 += selected.rank as i8;
                }
                if health.0 > 20 {
                    health.0 = 20
                }
                status.has_healed = true;
            }
            // RETRIEVE WEAPON
            Suit::Diamond => {
                *weapon_slot = WeaponSlot::Weapon {
                    weapon: selected,
                    enemies: Deck::empty(),
                };
            }
            // FIGHT
            Suit::Clubs | Suit::Spades => {
                if *action == Action::BareHand {
                    health.0 -= selected.rank as i8;
                }

                match weapon_slot.clone() {
                    WeaponSlot::Empty => health.0 -= selected.rank as i8,
                    WeaponSlot::Weapon {
                        weapon,
                        mut enemies,
                    } => {
                        let weapon_rank = match enemies.peek() {
                            Some(s) => min(s.rank as i8, weapon.rank as i8),
                            None => weapon.rank as i8,
                        };
                        // weapon_dam - enemy_health
                        // min(weapon_dam, stabbed_enemy_health) - (enemy_health)
                        match weapon_rank.cmp(&(selected.rank as i8)) {
                            std::cmp::Ordering::Less => {
                                health.0 -= selected.rank as i8 - weapon_rank
                            }
                            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                                enemies.add_top(selected);
                            }
                        }
                        *weapon_slot = WeaponSlot::Weapon { weapon, enemies }
                    }
                }
            }
        },
        None => return,
    }
    board.0[selection.0] = None;
    // TODO: discard pile?
}

pub fn manage_selection(mut actions: Query<&mut Action>, mut selections: Query<&mut Selection>) {
    let action = actions.single_mut();
    let mut selection = selections.single_mut();

    match *action {
        Action::Slot(s) => *selection = Selection(s),
        Action::Left => *selection = Selection(selection.0.wrapping_sub(1) % 4),
        Action::Right => *selection = Selection(selection.0.add(1) % 4),
        _ => {}
    };
}

pub fn next_round(
    mut actions: Query<&mut Action>,
    mut boards: Query<&mut Board>,
    mut draws: Query<&mut DrawDeck>,
    mut statuses: Query<&mut Status>,
) {
    let action = actions.single_mut();
    let mut draw = draws.single_mut();
    let mut board = boards.single_mut();
    let mut status = statuses.single_mut();

    match *action {
        Action::SkipRoom => {
            // TODO: change magic number to MAX_SKIPS
            if status.avoided_last {
                return;
            }

            // Clone here???
            for (i, c) in board.0.clone().iter().enumerate() {
                if let Some(c) = c {
                    draw.0.insert(*c);
                }
                board.0[i] = draw.0.draw();
            }

            status.avoided_last = true;
            status.queue_next_round = true;
        }
        Action::EndTurn => {
            let cards_left = board.0.iter().filter(|slot| slot.is_some()).count();
            if cards_left != 1 {
                return;
            }
            // Clone here???
            for (i, c) in board.0.clone().iter().enumerate() {
                if c.is_none() {
                    board.0[i] = draw.0.draw();
                }
            }
            status.avoided_last = false;
            status.queue_next_round = true;
        }
        _ => {}
    }
}

pub fn check_health(mut healths: Query<&mut Health>) {
    let mut health = healths.single_mut();

    if health.0 > 20 {
        health.0 = 20
    } else if health.0 <= 0 {
        *RUNNING.lock().unwrap() = false
    }
}

pub fn clear_action(mut actions: Query<&mut Action>) {
    let mut action = actions.single_mut();
    *action = Action::Idle;
}

pub fn handle_new_round(mut statuses: Query<&mut Status>) {
    let mut status = statuses.single_mut();

    if status.queue_next_round {
        status.has_healed = false;
        status.queue_next_round = false;
    }
}
