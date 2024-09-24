#[cfg(test)]
mod tests;

pub use pancake_macro::{useless, IdentMeta};

pub trait IdentMeta {
    fn get_name(&self) -> &'static str;
    fn get_items_name(&self) -> Vec<&'static str>;
}

/// Demo GAT - Generic Associated Type
pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&mut self) -> Option<Self::Item<'a>>;
}

#[macro_export]
macro_rules! retain_expr {
    ($e:expr) => {
        $e
    };
}

#[macro_export]
macro_rules! times_five {
    ($e:expr) => {
        $e * 5
    };
}

#[macro_export]
macro_rules! vec_strings {
    ($($e:expr),*) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $e));
            )*
            v
        }
    };
}
