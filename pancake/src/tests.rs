use super::*;

#[test]
fn test_vec_string() {
    let v = vec_strings!("amiao", true, 3.55f32);
    println!("{v:?}");
}

#[test]
fn test_times_five() {
    let value = times_five!(5);
    assert_eq!(value, 25);
    println!("value: {value}");
}

#[test]
fn test_ident_meta() {
    #[allow(dead_code)]
    #[derive(Default, IdentMeta)]
    struct Pan {
        name: String,
        baked: bool,
    }

    let pan = Pan::default();
    println!("{}", pan.get_name());
    println!("{:?}", pan.get_items_name());
}

#[test]
fn test_comp() {
    let v1: Vec<i32> = comp![x * 2 for x in [1, 2, 3] if x != 3 if x != 2].collect();
    assert_eq!(v1, [2]);

    let v2 = Vec::from([
        ("amiao".to_owned(), 5),
        ("yahaha".to_owned(), 66),
        ("zzz".to_owned(), 12),
    ]);
    let v2 = comp![format!("{s}@{n}") for (s, n) in v2 if n < 50].collect::<Vec<String>>();
    assert_eq!(v2, ["amiao@5", "zzz@12"]);
}
