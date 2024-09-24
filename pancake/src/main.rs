use pancake::*;
use std::process::ExitCode;

fn main() -> ExitCode {
    // let pan = retain_expr!(Pan::default());
    // useless!(let pan = pan;);
    // println!("{}", pan.get_name());
    // println!("{:?}", pan.get_items_name());

    ExitCode::SUCCESS
}

#[allow(dead_code)]
#[derive(Default, IdentMeta)]
struct Pan {
    name: String,
    baked: bool,
}
