#![crate_name = "tbge"]
 //! This carte is a turn-base game engine in Rust. 
use std::fmt::{Debug, Display};

/// The Act trait marks a type as a game act
pub trait Act: Display + Debug + Send + Sync {}

/// The State trait defines the behavior of game states
pub trait State {
    /// determine the required act (as trait object) that alter the current state to given state
    fn diff(&self, other: &Self) -> dyn Act
    where
        Self: Sized;
    
    /// update the current state according to given act
    fn update(&mut self, act: &dyn Act);
}

/// The Move trait marks an iterator of act as some sequential acts of player in each turn
pub trait Move: Display + Debug + Send + Sync + Iterator<Item = dyn Act> {}

/// Defines the behavior of overall game situation as an iterator of some states
/// for example in Poker states can be: each player cards, revealed cards, unrevealed cards, current bet, ...
// Todo: must consider public and private states. [this may be avoided with not providing the whole game status to players]
pub trait Status: Display + Debug + Send + Sync + Iterator<Item = dyn State> {
    /// tells the score of specified player in current situation
    fn score(&self, player: dyn Player) -> dyn ToString;
    /// specifies which player's turn is.
    fn turn(&self) -> dyn Player;
    /// validates the given move due the game status returns true if the move is valid
    fn validate(&self, mov: dyn Move) -> bool;
}

/// This trait defines behavior of a player
/// Players must be able to think and move according to game status
pub trait Player: Display + Debug + Send + Sync {
    /// Tells the player name. We do not play with strangers
    fn name(&self) -> String;
    /// The thinking function to decide the move with given game status
    fn play(&self, status: &dyn Status) -> dyn Move;
}


/// Defines behavior of a turn-based game
/// Game iterates over sequence of situations and ends after the iteration
pub trait Game: Iterator<Item = dyn Status> {
    /// Specifies the players
    fn players(&self) -> dyn Iterator<Item = dyn Player>;
    /// Specifies the current situation of game
    fn status(&self) -> dyn Status;
}

