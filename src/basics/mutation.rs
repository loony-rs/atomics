use std::cell::{Cell, RefCell};

pub(crate) fn l_interior_mut() {
    let a = Cell::new(10);
    l_cell(&a, &a);

    let a = RefCell::new(100);
    l_ref_cell(&a, &a);
}

fn l_cell(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();

    if before != after {
        println!("Cell: {}", before != after);
    }
}

fn l_ref_cell(a: &RefCell<i32>, b: &RefCell<i32>) {
    let before_val = {
        let before = a.borrow(); // This borrow ends at the end of this block
        *before
    };

    {
        let mut x = b.borrow_mut();
        *x += 1;
    }

    let after_val = {
        let after = a.borrow(); // Reborrow after the mutable borrow
        *after
    };

    if before_val != after_val {
        println!("RefCell: {}", before_val != after_val);
    }
}