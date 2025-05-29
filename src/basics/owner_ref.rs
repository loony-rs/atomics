use std::rc::Rc;
use std::sync::Arc;

fn l_rc() {
    let rc_1 = Rc::new([1, 2, 3]);
    let _rc_2 = rc_1.clone();
    println!("Rc count: {}", Rc::clone(&rc_1).len())
}

fn l_arc() {
    let rc_1 = Arc::new([1, 2, 3]);
    let _rc_2 = rc_1.clone();
    println!("Arc count: {}", Arc::clone(&rc_1).len())
}

pub(crate) fn l_owner_ref() {
    l_rc();
    l_arc();
}

