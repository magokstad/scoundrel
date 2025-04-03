use std::fmt::Display;

use anyhow::Result;
use bevy_ecs::prelude::*;
use rand::{rng, seq::SliceRandom};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter};

pub const ACE_VAL: isize = 1;

#[derive(Debug, Default, EnumCount, EnumIter, Display, Clone, Copy, PartialEq, Eq)]
enum Rank {
    #[default]
    Ace = ACE_VAL,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl Rank {
    fn is_face(&self) -> bool {
        vec![Rank::Ace, Rank::Jack, Rank::Queen, Rank::King].contains(&self)
    }
}

#[derive(Debug, Default, EnumCount, EnumIter, Display, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Hearts,
    Diamond,
    Clubs,
    #[default]
    Spades,
}

#[derive(Component, Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn is_face_card(&self) -> bool {
        self.rank.is_face()
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

#[derive(Component, Default)]
pub struct Deck {
    cards: Vec<Entity>,
}

// #[derive(Bundle)]
// pub struct DeckBundle {
//     deck: Deck,
// }

impl Deck {
    pub fn new_classic(commands: &mut Commands) -> Self {
        let mut deck = vec![];

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                let card = Card::new(suit.to_owned(), rank.to_owned());
                let card_entity = commands.spawn(card).id();
                deck.push(card_entity);
            }
        }

        Self { cards: deck }
    }

    pub fn new_scoundrel(commands: &mut Commands) -> Deck {
        let mut deck = vec![];

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                match (rank.to_owned(), suit.to_owned()) {
                    (r, Suit::Diamond) if r.is_face() => {}
                    (r, Suit::Hearts) if r.is_face() => {}
                    (r, s) => {
                        let card = Card::new(s, r);
                        let card_entity = commands.spawn(card).id();
                        deck.push(card_entity);
                    }
                }
            }
        }

        Self { cards: deck }
    }

    pub fn empty() -> Self {
        Self { cards: vec![] }
    }
}

pub fn shuffle(mut decks: Query<&mut Deck>) -> Result<()> {
    let mut deck = decks.get_single_mut()?;
    let mut rng = rng();

    deck.cards.shuffle(&mut rng);
    println!("Deck Shuffled!");

    Ok(())
}
