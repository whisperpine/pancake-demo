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

    let pan = retain_expr!(Pan::default());
    useless!(let pan = pan;);
    println!("{}", pan.get_name());
    println!("{:?}", pan.get_items_name());
}
