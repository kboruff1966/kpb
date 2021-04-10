mod option;
use crate::option::MyOption;

fn main() {
    let x: MyOption<u32> = MyOption::Some(2);
    assert_eq!(x.is_some(), true);

    let x: MyOption<u32> = MyOption::None;
    assert_eq!(x.is_some(), false);
}
