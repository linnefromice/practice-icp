use ic_cdk_macros::{query, update};

#[query]
fn query_1() -> u64 {
    let mut result = 0;
    for _ in 1..=1 {
        result += 1;
    }
    result
}
#[query]
fn query_1_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=1_000_000 {
        result += 1;
    }
    result
}
#[query]
fn query_2_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=2_000_000 {
        result += 1;
    }
    result
}
#[query]
fn query_3_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=3_000_000 {
        result += 1;
    }
    result
}
#[query]
fn query_4_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=4_000_000 {
        result += 1;
    }
    result
}
#[query]
fn query_1_000_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=1_000_000_000u32 {
        result += 1;
    }
    result
}
#[query]
fn query_2_000_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=2_000_000_000u32 {
        result += 1;
    }
    result
}
#[query]
fn query_3_000_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=3_000_000_000u32 {
        result += 1;
    }
    result
}
#[query]
fn query_4_000_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=4_000_000_000u32 {
        result += 1;
    }
    result
}

#[update]
fn update_1() -> u64 {
    let mut result = 0;
    for _ in 1..=1 {
        result += 1;
    }
    result
}
#[update]
fn update_1_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=1_000_000 {
        result += 1;
    }
    result
}
#[query]
fn update_2_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=2_000_000 {
        result += 1;
    }
    result
}
#[query]
fn update_3_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=3_000_000 {
        result += 1;
    }
    result
}
#[query]
fn update_4_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=4_000_000 {
        result += 1;
    }
    result
}
#[update]
fn update_1_000_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=1_000_000_000u32 {
        result += 1;
    }
    result
}
#[update]
fn update_2_000_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=2_000_000_000u32 {
        result += 1;
    }
    result
}
#[update]
fn update_3_000_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=3_000_000_000u32 {
        result += 1;
    }
    result
}
#[update]
fn update_4_000_000_000() -> u64 {
    let mut result = 0;
    for _ in 1..=4_000_000_000u32 {
        result += 1;
    }
    result
}