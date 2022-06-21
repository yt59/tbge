#![crate_name = "tbge"]
//! This carte is a turn-base game engine in Rust.
use std::{fmt::{Debug, Display}, any::Any, vec::IntoIter};

/// The Act trait marks a type as a game act
pub trait Act: Display + Debug + Send + Sync + Any{
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any>;
    fn effect(self) -> Box<dyn State>;
}

/// The State trait defines the behavior of game states
pub trait State: Display + Debug + Send + Sync {
    /// determine the required act (as trait object) that alter the current state to given state
    fn diff(self, other: &Self) -> Box<dyn Act>
    where
        Self: Sized;

    /*/// update the current state according to given act
    fn update(&mut self, act: Box<dyn Act>);
    */
}

/// The Move trait marks an iterator of act as some sequential acts of player in each turn
pub trait Move: Display + Debug + Send + Sync + IntoIterator<Item = Box<dyn Act>> {}

/// Defines the behavior of overall game situation as an iterator of some states
/// for example in Poker states can be: each player cards, revealed cards, unrevealed cards, current bet, ...
// Todo: must consider public and private states. [this may be avoided with not providing the whole game status to players]
pub trait Status: Display + Debug + Send + Sync + Iterator<Item = Box<dyn State>> {
    /// tells the score of specified player in current situation
    fn score(&self, player: Box<dyn Player>) -> Box<dyn ToString>;
    /// specifies which player's turn is.
    fn turn(&self) -> Box<dyn Player>;
    /// validates the given move due the game status returns true if the move is valid
    fn validate(&self, mov: Box<dyn Move<IntoIter = IntoIter<Box<dyn Act>>, Item = Box<dyn Act>>>) -> bool;
}

/// This trait defines behavior of a player
/// Players must be able to think and move according to game status
pub trait Player: Display + Debug + Send + Sync {
    /// Tells the player name. We do not play with strangers
    fn name(&self) -> String;
    /// The thinking function to decide the move with given game status
    fn play(&self, status: Box<dyn Status>) -> Box<dyn Move<IntoIter = IntoIter<Box<dyn Act>>, Item = Box<dyn Act>>>;
}

/// Defines behavior of a turn-based game
/// Game iterates over sequence of situations and ends after the iteration
pub trait Game: Iterator<Item = Box<dyn Status>> + Display + Debug + Send + Sync {
    /// Specifies the players
    fn players(&self) -> Box<dyn Iterator<Item = Box<dyn Player>>>;
    /// Specifies the current situation of game
    fn status(&self) -> Box<dyn Status>;
}
