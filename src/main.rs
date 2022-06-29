// mod iter;
// mod option;

// use iter::MyIterator;
// use option::MyOption;

fn main() {
    let a = [0, 1, 2];

    let mut iter = a.iter().filter(|&&x| x > 1); // need two *s!

    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
}
