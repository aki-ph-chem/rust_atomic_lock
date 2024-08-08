use std::cell::{Cell, RefCell};

// Cell<T>: TにはCopyが実装されている必要がある
// シングルスレッドのみ
// 変更可能
fn f_cell(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);

    //a.set(a.get() + 1);
    let after = a.get();
    if before != after {
        println!("before != after");
    }
}

fn cell_vec(v: &Cell<Vec<i32>>) {
    let mut v_2 = v.take();
    v_2.push(11);
    v.set(v_2);
}

// RefCell<T>
// シングルスレッドのみ
// 変更可能
fn f_refcell(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(10);
}

fn main() {
    let (a_cell, b_cell) = (Cell::new(4), Cell::new(3));
    f_cell(&a_cell, &b_cell);

    let v = Cell::new(vec![1, 2]);
    cell_vec(&v);
    println!("v: {:?}", v.take());

    let v = RefCell::new(vec![1, 2]);
    f_refcell(&v);
    println!("v: {:?}", v.take());
}
