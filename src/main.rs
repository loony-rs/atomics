mod atomics;
use atomics::{load_store, fetch};

fn main() {
    fetch::l_fetch_add();
}
