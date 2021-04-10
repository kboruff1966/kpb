// replicate the std::option module

// This is a separate function to reduce the code size of .expect() itself.
#[inline(never)]
#[cold]
#[track_caller]
fn expect_failed(msg: &str) -> ! {
    panic!("{}", msg)
}

// This is a separate function to reduce the code size of .expect_none() itself.
#[inline(never)]
#[cold]
#[track_caller]
fn expect_none_failed(msg: &str, value: &dyn std::fmt::Debug) -> ! {
    panic!("{}: {:?}", msg, value)
}

#[derive(Debug, PartialEq)]
pub enum MyOption<T> {
    None,
    Some(T),
}

impl<T> MyOption<T> {
    pub const fn is_some(&self) -> bool {
        // returns true of option is a some value
        std::matches!(self, MyOption::Some(_))
    }

    pub const fn is_none(&self) -> bool {
        // returns true of option is none
        !self.is_some()
    }

    pub const fn as_ref(&self) -> MyOption<&T> {
        match self {
            MyOption::Some(ref t) => MyOption::Some(t),
            MyOption::None => MyOption::None,
        }
    }

    pub fn as_mut(&mut self) -> MyOption<&mut T> {
        match self {
            MyOption::Some(ref mut t) => MyOption::Some(t),
            MyOption::None => MyOption::None,
        }
    }

    // Maps an Option<T> to Option<U> by applying a function to a contained value.
    pub fn map<U, F>(self, f: F) -> MyOption<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            MyOption::Some(t) => MyOption::Some(f(t)),
            MyOption::None => MyOption::None,
        }
    }

    // applies a function to the contained value (if any), or returns the provided default (if not)
    pub fn map_or<F, U>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        match self {
            MyOption::Some(val) => f(val),
            MyOption::None => default,
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            MyOption::Some(t) => t,
            MyOption::None => panic!("called `MyOption::unwrap()` on a `None` value"),
        }
    }

    // returns Some value or default
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            MyOption::Some(val) => val,
            MyOption::None => default,
        }
    }

    // returns contained Some or computes from closure
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            MyOption::Some(val) => val,
            MyOption::None => f(),
        }
    }
}

#[cfg(test)]
pub mod options_test {

    use super::MyOption;

    #[test]
    fn is_some_and_is_none() {
        let x: MyOption<u32> = MyOption::Some(2);
        assert_eq!(x.is_some(), true);

        let x: MyOption<u32> = MyOption::None;
        assert_eq!(x.is_some(), false);
    }

    #[test]
    fn map_test() {
        let maybe_some_string = MyOption::Some(String::from("Hello, Map Test!"));

        // `Option::map` takes self *by value*, consuming `maybe_some_string`
        let maybe_some_len = maybe_some_string.map(|s| s.len());

        assert_eq!(maybe_some_len, MyOption::Some(16));
    }

    #[test]
    fn map_or_test() {
        let x = MyOption::Some("foo");
        assert_eq!(x.map_or(42, |v| v.len()), 3);

        let x: MyOption<&str> = MyOption::None;
        assert_eq!(x.map_or(42, |v| v.len()), 42);
    }

    #[test]
    fn as_ref_test() {
        let text: MyOption<String> = MyOption::Some("Hello, Ref Test!".to_string());
        // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
        // then consume *that* with `map`, leaving `text` on the stack.
        let text_length: MyOption<usize> = text.as_ref().map(|s| s.len());
        println!("still can print text: {:?}", text);
    }

    #[test]
    fn as_mut_test() {
        let mut x = MyOption::Some(2);
        match x.as_mut() {
            MyOption::Some(v) => *v = 42,
            MyOption::None => {}
        }
        assert_eq!(x, MyOption::Some(42));
    }

    #[test]
    fn unwrap_test() {
        let x = MyOption::Some("air");
        assert_eq!(x.unwrap(), "air");
    }

    #[test]
    #[should_panic]
    fn unwrap_test_panic() {
        let x: Option<&str> = None;
        assert_eq!(x.unwrap(), "air"); // fails
    }

    #[test]
    fn unwrap_or_test() {
        assert_eq!(MyOption::Some("car").unwrap_or("bike"), "car");
        assert_eq!(MyOption::None.unwrap_or("bike"), "bike");
    }

    #[test]
    fn unwrap_or_else_test() {
        let k = 10;
        assert_eq!(MyOption::Some(4).unwrap_or_else(|| 2 * k), 4);
        assert_eq!(MyOption::None.unwrap_or_else(|| 2 * k), 20);
    }
}
