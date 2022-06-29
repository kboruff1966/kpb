use crate::option::MyOption;

pub trait MyIterator {
    type Item;
    fn next(&mut self) -> MyOption<Self::Item>;
}
