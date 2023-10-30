use super::*;

#[test]
fn vec_string() {
    let v = vec_strings!("amiao", true, 3.55f32);
    println!("{:?}", v);
}

#[test]
fn times_five() {
    let value = times_five!(5);
    assert_eq!(value, 25);
    println!("value: {}", value);
}

#[test]
fn nice() {
    #[allow(dead_code)]
    #[derive(Default, IdentMeta)]
    struct Pan {
        name: String,
        baked: bool,
    }

    // let pan = retain_expr!(Pan::default());
    // useless!(let pan = pan;);
    // println!("{}", pan.get_name());
    // println!("{:?}", pan.get_items_name());
}

#[test]
fn labeled_blocks() {
    let amiao_is_a_cat = false;
    let meow_is_a_cat = true;
    let result = 'block: {
        if amiao_is_a_cat {
            break 'block 1;
        }
        if meow_is_a_cat {
            break 'block 2;
        }
        3
    };

    assert_eq!(result, 2);
}

#[test]
fn scoped_thread() {
    let mut v = vec![1, 2, 3];
    let mut x = 0;
    std::thread::scope(|s| {
        s.spawn(|| {
            println!("hello from first scoped thread");
            dbg!(&v);
        });
        s.spawn(|| {
            println!("hello from second scoped thread");
            x += v[0] + v[2];
        });
    });

    v.push(4);
    assert_eq!(x, v.len());
}

#[test]
fn is_terminal() {
    use std::io::{self, IsTerminal};
    let is_terminal = io::stdout().is_terminal();
    println!("is_terminal: {}", is_terminal);
}

#[test]
fn demo() {
    //
}
