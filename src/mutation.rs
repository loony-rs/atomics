use std::cell::{Cell, RefCell};

pub(crate) fn l_interior_mut() {
    let a = Cell::new(10);
    l_cell(&a, &a);

    let a = RefCell::new(100);
    l_ref_cell(&a);
}

fn l_cell(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();

    if before != after {
        println!("Hello World");
    }
}

fn l_ref_cell(a: &RefCell<i32>) {
    let mut_ref1 = a.borrow_mut();
let mut_ref2 = a.borrow_mut();
    // println!("{:?}", a);
}