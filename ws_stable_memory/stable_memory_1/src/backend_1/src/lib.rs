use std::{cell::RefCell, borrow::Cow};

use candid::{CandidType, Deserialize, Decode, Encode};
use ic_cdk_macros::{init, pre_upgrade, post_upgrade};
use ic_stable_structures::{memory_manager::{VirtualMemory, MemoryManager, MemoryId}, DefaultMemoryImpl, Storable, BoundedStorable, Cell, writer::Writer, Memory};

type MemoryType = VirtualMemory<DefaultMemoryImpl>;

const MEMORY_ID_FOR_UPGRADE: MemoryId = MemoryId::new(0);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    
    // stable memory
    static PLAYERS: RefCell<ic_stable_structures::Vec<Player, MemoryType>> = RefCell::new(
        ic_stable_structures::Vec::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))).unwrap()
    );
    static SUM_AGE: RefCell<Cell<u32, MemoryType>> = RefCell::new(Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 0).unwrap());

    // heap
    static LENGTH: RefCell<usize> = RefCell::new(0);
    static LATEST_PLAYER: std::cell::RefCell<Option<Player>> = std::cell::RefCell::new(None);
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, PartialEq)]
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
fn get_player(index: u64) -> Option<Player> {
    PLAYERS.with(|p| p.borrow().get(index)).clone()
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_latest_player_from_heap() -> Option<Player> {
    LATEST_PLAYER.with(|l| l.borrow().clone()).clone()
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_latest_player_from_stable() -> Option<Player> {
    PLAYERS.with(|p| p.borrow().get(p.borrow().len() as u64 - 1)).clone()
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

pub fn get_upgrades_memory() -> MemoryType {
    MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_FOR_UPGRADE))
}
#[derive(serde::Serialize, Deserialize)]
pub struct UpgradeState {
    length: u64,
    latest_player: Player,
}
#[pre_upgrade]
fn pre_upgrade() {
    let mut state_bytes = vec![];
    ciborium::ser::into_writer(&UpgradeState {
        length: get_players_len_from_heap(),
        latest_player: get_latest_player_from_heap().unwrap(),
    }, &mut state_bytes).unwrap();

    let len = state_bytes.len() as u32;
    let mut memory = get_upgrades_memory();
    let mut writer = Writer::new(&mut memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap()
}
#[post_upgrade]
fn post_upgrade() {
    let memory = get_upgrades_memory();

    // Read the length of the state bytes.
    let mut state_len_bytes = [0; 4];
    memory.read(0, &mut state_len_bytes);
    let state_len = u32::from_le_bytes(state_len_bytes) as usize;

    // Read the bytes
    let mut state_bytes = vec![0; state_len];
    memory.read(4, &mut state_bytes);

    let state: UpgradeState = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
    LENGTH.with(|l| *l.borrow_mut() = state.length as usize);
    LATEST_PLAYER.with(|l| *l.borrow_mut() = Some(state.latest_player));
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
        assert_eq!(get_latest_player_from_heap().unwrap(), expected_latest_player.clone());
        assert_eq!(get_latest_player_from_stable().unwrap(), expected_latest_player.clone());

        let player = add_player("Alice".to_string(), 20);
        assert_eq!(get_latest_player_from_heap().unwrap(), player.clone());
        assert_eq!(get_latest_player_from_stable().unwrap(), player.clone());

        assert_eq!(get_players_len_from_heap(), 2);
        assert_eq!(get_players_len_from_stable(), 2);
    }

    #[test]
    fn use_cbor() {
        let _ = init();
        let length = get_players_len_from_heap();
        let latest_player = get_latest_player_from_heap().unwrap();

        let mut state_bytes = vec![];
        ciborium::ser::into_writer(&UpgradeState {
            length,
            latest_player: latest_player.clone(),
        }, &mut state_bytes).unwrap();

        let state: UpgradeState = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
        assert_eq!(state.length, length);
        assert_eq!(state.latest_player, latest_player);
    }
}
