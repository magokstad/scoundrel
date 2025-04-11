use std::{collections::VecDeque, fmt::Display};

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

    pub fn emojify(&self) -> String {
        let mut tmp: String = String::new();

        let suit_str = match self.suit {
            Suit::Hearts => "♥ ",
            Suit::Diamond => "♦ ",
            Suit::Spades => "♠ ",
            Suit::Clubs => "♣ ",
        };
        let rank_str = match self.rank {
            Rank::Ace => " A",
            Rank::King => " K",
            Rank::Queen => " Q",
            Rank::Jack => " J",
            Rank::Ten => "10",
            rank => {
                tmp = format!(" {}", rank as u8);
                tmp.as_str()
            }
        };

        format!("{}{}", suit_str, rank_str)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

#[derive(Component, Default)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    pub fn new_classic() -> Self {
        let mut deck = VecDeque::new();

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                let card = Card::new(suit.to_owned(), rank.to_owned());
                deck.push_front(card);
            }
        }

        Self { cards: deck }
    }

    pub fn new_scoundrel() -> Self {
        let mut deck = VecDeque::new();

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                match (rank.to_owned(), suit.to_owned()) {
                    (r, Suit::Diamond) if r.is_face() => {}
                    (r, Suit::Hearts) if r.is_face() => {}
                    (r, s) => {
                        let card = Card::new(s, r);
                        deck.push_front(card);
                    }
                }
            }
        }

        Self { cards: deck }
    }

    pub fn shuffled_scoundrel() -> Self {
        let mut deck = Self::new_scoundrel();
        deck.shuffle();

        deck
    }

    pub fn empty() -> Self {
        Self {
            cards: VecDeque::new(),
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        let cards: Vec<_> = self.cards.drain(..).collect();
        let mut shuffled = cards;
        shuffled.shuffle(&mut rng);
        self.cards = VecDeque::from(shuffled);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    pub fn insert(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    pub fn peek(&self) -> Option<&Card> {
        self.cards.front()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn count(&self) -> usize {
        self.cards.len()
    }
}
