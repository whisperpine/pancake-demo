pub use pancake_macro::IdentMeta;

pub trait IdentMeta {
    fn get_name(&self) -> &'static str;
    // fn get_items_name(&self) -> &'static str;
}
