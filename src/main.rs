use std::io;
use std::string::String;

#[derive(Copy, Clone)]
struct PlayerAttrs {
    agility : i32,
    attack: i32,
    armor: i32
}

#[derive(Copy, Clone)]
struct PlayerState {
    hp : i32
}

struct Player {
    state : PlayerState,
    attrs : PlayerAttrs,
    recvr : PlayerAttrs
}

impl Player {
    fn create(&self, state: PlayerState, attrs: PlayerAttrs) -> Player {
        Player {
            state : state,
            attrs : attrs.clone(),
            recvr : attrs.clone()
        }
    }
}


fn battle_terminate(offender: & Player, defender: & Player) -> bool {
    offender.state.hp < 0 || defender.state.hp < 0
}

fn main() {

    loop{
        println!("Hello, world!");
    }
}
