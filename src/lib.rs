use std::fmt::{Debug, Display};


pub trait Act: Display + Debug + Send + Sync {
    fn affect(self, status:&mut dyn Status) -> &mut dyn Status;
}

pub trait Player: Display + Debug + Send + Sync {
    fn name(&self) -> String;
    fn acts(&self) -> dyn Iterator<Item = dyn Act>;
    fn play(&self, status: &dyn Status) -> dyn Act;
}

pub trait Status: Display + Debug + Send + Sync {
    fn score(&self, player: dyn Player) -> String;
    fn turn(&self) -> dyn Player;
    fn previlages(
        &self,
        acts: dyn Iterator<Item = dyn Act>,
        player: dyn Player,
    ) -> dyn Iterator<Item = dyn Act>;
} 


pub trait Game: Iterator<Item = dyn Status>{
    fn players(&self) -> dyn Iterator<Item = dyn Player>;
    fn status(&self) -> dyn Status;
}