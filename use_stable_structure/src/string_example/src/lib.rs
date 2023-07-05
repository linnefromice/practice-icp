use std::cell::RefCell;

use chainsight_cdk_macros::did_export;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableCell, StableVec, Storable};

#[derive(Default)]
struct StringWrapper(String);
impl Storable for StringWrapper {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.as_bytes().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(String::from_utf8(bytes.into_owned()).unwrap())
    }
}
impl BoundedStorable for StringWrapper {
    const MAX_SIZE: u32 = 100;
    const IS_FIXED_SIZE: bool = false;
}

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<ic_stable_structures::DefaultMemoryImpl>>  =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static CELL: RefCell<StableCell<StringWrapper, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(0))),
            StringWrapper::default()
        ).unwrap()
    );

    static VEC: RefCell<StableVec<StringWrapper,Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(1)))
        ).unwrap()
    )
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_cell() -> String {
    CELL.with(|cell| cell.borrow().get().0.clone())
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn set_cell(value: String) -> Result<(), String> {
    let res = CELL.with(|cell| cell.borrow_mut().set(StringWrapper(value)));
    res.map(|_| ()).map_err(|e| format!("{:?}", e))
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_vec(idx: u64) -> Option<String> {
    VEC.with(|vec| vec.borrow().get(idx).map(|s| s.0))
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn insert_to_vec(value: String) -> Result<(), String> {
    let res = VEC.with(|vec| vec.borrow_mut().push(&StringWrapper(value)));
    res.map_err(|e| format!("{:?}", e))
}

did_export!("string_example");
