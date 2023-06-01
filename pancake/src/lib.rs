pub use pancake_macro::{useless, IdentMeta};

pub trait IdentMeta {
    fn get_name(&self) -> &'static str;
    // fn get_items_name(&self) -> &'static str;
}

#[macro_export]
macro_rules! retain_expr {
    ($e:expr) => {
        $e
    };
}
