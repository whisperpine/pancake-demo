use pancake::{retain_expr, useless, IdentMeta};

fn main() {
    let pan = retain_expr!(Pan::default());
    useless!(let pan = pan;);
    println!("{}", pan.get_name());
}

#[derive(Default, IdentMeta)]
struct Pan;
