// pub mod opt {
use std::{fmt::Display, path::Iter};
use tbge::{Act, Move, State};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
    Joker,
}
impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Spades => f.write_str("♠️"),
            Suit::Hearts => f.write_str("♥️"),
            Suit::Clubs => f.write_str("♣️"),
            Suit::Diamonds => f.write_str("♦️"),
            Suit::Joker => f.write_str("🃏"),
        }
    }
}
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Card {
    suit: Suit,
    value: u8,
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card { suit: s, value: 11 } => f.write_fmt(format_args!("J {}", s)),
            Card { suit: s, value: 12 } => f.write_fmt(format_args!("Q {}", s)),
            Card { suit: s, value: 13 } => f.write_fmt(format_args!("K {}", s)),
            Card { suit: s, value: v } => f.write_fmt(format_args!("{} {}", v, s)),
        }
    }
}
impl Default for Card {
    fn default() -> Self {
        Self {
            suit: Suit::Clubs,
            value: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    inner: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Deck {
            inner: Vec::with_capacity(52),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Deck {
            inner: Vec::with_capacity(capacity),
        }
    }
    pub fn new_shuffled() -> Self {
        use rand::rngs::mock::StepRng;
        use shuffle::irs::Irs;
        use shuffle::shuffler::Shuffler;
        let mut rng = StepRng::new(2, 13);
        let mut irs = Irs::default();

        let mut inner: Vec<u8> = (0..52).collect();
        irs.shuffle(&mut inner, &mut rng).unwrap();
        Deck {
            inner: inner
                .iter()
                .map(|val| Card {
                    suit: match val / 13 {
                        0 => Suit::Spades,
                        1 => Suit::Hearts,
                        2 => Suit::Clubs,
                        3 => Suit::Diamonds,
                        _ => Suit::Joker,
                    },
                    value: val % 13 + 1,
                })
                .collect(),
        }
    }
    fn pop(mut self) -> Push {
        Push {
            card: self.inner.pop().unwrap(),
            on: Box::new(self),
        }
    }
    fn push(mut self, card: Card) -> Self {
        self.inner.push(card);
        self
    }
}
impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.inner.iter().map(|x| x.to_string()))
            .finish()
    }
}
impl State for Deck {
    fn diff(self, other: &Self) -> Box<dyn Act>
    where
        Self: Sized,
    {
        if self.inner == other.inner {
            if self.inner.ends_with(&other.inner) {
                return Box::new(Nothing {
                    state: Box::new(self),
                }) as Box<dyn Act>;
            }
        }
        if other.inner.len() > self.inner.len() {
            return Box::new(Pop {
                state: Box::new(self),
            }) as Box<dyn Act>;
        } else {
            match other.inner.last().unwrap() {
                Card { suit: s, value: v } => Box::new(Push {
                    card: Card {
                        suit: *s,
                        value: *v,
                    },
                    on: Box::new(self),
                }) as Box<dyn Act>,
            }
        }
    }

    // fn update(&mut self, act: Box<dyn Act>) {
    //     act.effect();
    // }
}

#[derive(Debug, Clone)]
struct Pop {
    state: Box<Deck>,
}
impl Display for Pop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Pop {} from {}",
            self.state.inner.last().unwrap(),
            self.state
        ))
    }
}
impl Act for Pop {
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }

    fn effect(self) -> Box<dyn State> {
        Box::new(*(self.state.pop().on))
    }
}

#[derive(Debug)]
struct Push {
    card: Card,
    on: Box<Deck>,
}

impl Display for Push {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Add {} to {}", self.card, self.on))
    }
}
impl Act for Push {
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }

    fn effect(self) -> Box<dyn State> {
        Box::new(self.on.push(self.card))
    }
}

#[derive(Debug)]
struct Nothing {
    state: Box<Deck>,
}
impl Display for Nothing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Nothing")
    }
}
impl Act for Nothing {
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }

    fn effect(self) -> Box<dyn State> {
        Box::new(*(self.state))
    }
}

#[derive(Debug)]
struct Hit {
    deck1: Box<Deck>,
    deck2: Box<Deck>,
    deck3: Box<Deck>,
    deck4: Box<Deck>,
    stack: Box<Deck>,
}
impl Display for Hit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Hit")
    }
}

impl IntoIterator for Hit {
    type Item = Box<dyn Act>;

    type IntoIter = std::array::IntoIter<Box<dyn Act>, 5>;

    fn into_iter(self) -> Self::IntoIter {
        // self.stack.pop().iter();
        let Push {
            card: c1,
            on: state,
        } = self.stack.pop();
        let Push {
            card: c2,
            on: state,
        } = state.pop();
        let Push {
            card: c3,
            on: state,
        } = state.pop();
        let Push {
            card: c4,
            on: state,
        } = state.pop();
        [
            Box::new(Push {
                card: c1,
                on: self.deck1,
            }),
            Box::new(Push {
                card: c2,
                on: self.deck2,
            }),
            Box::new(Push {
                card: c3,
                on: self.deck3,
            }),
            Box::new(Push {
                card: c4,
                on: self.deck4,
            }) as Box<dyn Act>,
            Box::new(Nothing { state: state }) as Box<dyn Act>,
        ]
        .into_iter()
    }
}
impl Move for Hit {}
// }
