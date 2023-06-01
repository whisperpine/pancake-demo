use pancake::IdentMeta;

fn main() {
    let pan = Pan::default();
    println!("{}", pan.get_name());
}

#[derive(Default, IdentMeta)]
struct Pan;
