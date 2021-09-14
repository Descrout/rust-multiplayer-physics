use crate::connection::Connection;
use crate::proto::proto_all::*;

#[derive(Debug)]
pub enum GameEvents {
    Join(Connection),
    Quit(u32),
    Input(u32, GameInput),
}

#[derive(Debug)]
pub enum BroadcastEvents {
    Join(Connection),
    Quit(u32),
    StateOut(State),
}
