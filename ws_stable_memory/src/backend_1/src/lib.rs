use std::{cell::RefCell, borrow::Cow};

use candid::{CandidType, Deserialize, Decode, Encode};
use ic_cdk_macros::init;
use ic_stable_structures::{memory_manager::{VirtualMemory, MemoryManager, MemoryId}, DefaultMemoryImpl, Storable, BoundedStorable, Cell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    
    // stable memory
    static PLAYERS: RefCell<ic_stable_structures::Vec<Player, Memory>> = RefCell::new(
        ic_stable_structures::Vec::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))).unwrap()
    );
    static SUM_AGE: RefCell<Cell<u32, Memory>> = RefCell::new(Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 0).unwrap());

    // heap
    static LENGTH: RefCell<usize> = RefCell::new(0);
    static LATEST_PLAYER: std::cell::RefCell<Option<Player>> = std::cell::RefCell::new(None);
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Player {
    index: u32,
    name: String,
    age: u32,
}
impl Storable for Player {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
}
impl BoundedStorable for Player {
    const MAX_SIZE: u32 = 100;
    const IS_FIXED_SIZE: bool = false;
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_player(index: u64) -> Player {
    PLAYERS.with(|p| p.borrow().get(index).unwrap())
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_latest_player_from_heap() -> Player {
    LATEST_PLAYER.with(|l| l.borrow().clone().unwrap())
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_latest_player_from_stable() -> Player {
    PLAYERS.with(|p| p.borrow().get(p.borrow().len() as u64 - 1).unwrap())
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_players_len_from_heap() -> u64 {
    LENGTH.with(|l| *l.borrow() as u64)
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_players_len_from_stable() -> u64 {
    PLAYERS.with(|p| p.borrow().len() as u64)
}

#[ic_cdk::update]
#[candid::candid_method]
fn add_default_player() -> Player {
    _add_player("Anonymous".to_string(), 99)
}

#[ic_cdk::update]
#[candid::candid_method]
fn add_player(name: String, age: u32) -> Player {
    _add_player(name, age)
}

fn _add_player(name: String, age: u32) -> Player {
    let index = PLAYERS.with(|p| p.borrow().len() as u32);
    let player = Player { index, name, age };

    PLAYERS.with(|p| p.borrow_mut().push(&player.clone()).unwrap());
    SUM_AGE.with(|s| {
        let before = s.borrow().get().clone();
        let after = before + age;
        s.borrow_mut().set(after).unwrap();
    });

    LENGTH.with(|l| {
        let before = l.borrow().clone();
        *l.borrow_mut() = before + 1;
    });
    LATEST_PLAYER.with(|l| *l.borrow_mut() = Some(player.clone()));

    player
}


#[init]
fn init() {
    add_default_player();
}

#[cfg(test)]
mod tests {
    use super::*;
    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }

    #[test]
    fn scenario() {
        let _ = init();
        assert_eq!(get_players_len_from_stable(), 1);
        assert_eq!(get_players_len_from_heap(), 1);
        let expected_latest_player = Player { index: 0, name: "Anonymous".to_string(), age: 99 };
        assert_eq!(get_latest_player_from_heap(), expected_latest_player.clone());
        assert_eq!(get_latest_player_from_stable(), expected_latest_player.clone());

        let player = add_player("Alice".to_string(), 20);
        assert_eq!(get_latest_player_from_heap(), player.clone());
        assert_eq!(get_latest_player_from_stable(), player.clone());

        assert_eq!(get_players_len_from_heap(), 2);
        assert_eq!(get_players_len_from_stable(), 2);
    }
}