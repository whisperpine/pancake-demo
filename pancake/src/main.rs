use pancake::*;

fn main() {
    let pan = retain_expr!(Pan::default());
    useless!(let pan = pan;);
    println!("{}", pan.get_name());
    println!("{:?}", pan.get_items_name());
}

#[allow(dead_code)]
#[derive(Default, IdentMeta)]
struct Pan {
    name: String,
    baked: bool,
}
