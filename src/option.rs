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

    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        match self {
            MyOption::Some(val) => f(val),
            MyOption::None => default(),
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

    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            MyOption::Some(val) => Ok(val),
            MyOption::None => Err(err),
        }
    }

    pub fn ok_or_else<E, F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            MyOption::Some(val) => Ok(val),
            MyOption::None => Err(f()),
        }
    }

    pub fn and<U>(self, optb: MyOption<U>) -> MyOption<U> {
        match self {
            MyOption::Some(val) => optb,
            MyOption::None => MyOption::None,
        }
    }

    pub fn and_then<U, F>(self, f: F) -> MyOption<U>
    where
        F: FnOnce(T) -> MyOption<U>,
    {
        match self {
            MyOption::Some(val) => f(val),
            MyOption::None => MyOption::None,
        }
    }

    pub fn filter<P>(self, predicate: P) -> MyOption<T>
    where
        P: FnOnce(&T) -> bool,
    {
        match self {
            MyOption::Some(t) if predicate(&t) => MyOption::Some(t),
            _ => MyOption::None,
        }
    }

    pub fn or(self, optb: MyOption<T>) -> MyOption<T> {
        match self {
            MyOption::Some(_) => self,
            MyOption::None => optb,
        }
    }

    pub fn or_else<F>(self, f: F) -> MyOption<T>
    where
        F: FnOnce() -> MyOption<T>,
    {
        match self {
            MyOption::Some(_) => self,
            MyOption::None => f(),
        }
    }

    pub fn xor(self, optb: MyOption<T>) -> MyOption<T> {
        match (self, optb) {
            (MyOption::Some(a), MyOption::None) => MyOption::Some(a),
            (MyOption::None, MyOption::Some(b)) => MyOption::Some(b),
            _ => MyOption::None,
        }
    }
}

#[cfg(test)]
pub mod options_test {

    use super::MyOption;

    #[test]
    fn xor_test() {
        let x = MyOption::Some(2);
        let y: MyOption<u32> = MyOption::None;
        assert_eq!(x.xor(y), MyOption::Some(2));

        let x: MyOption<u32> = MyOption::None;
        let y = MyOption::Some(2);
        assert_eq!(x.xor(y), MyOption::Some(2));

        let x = MyOption::Some(2);
        let y = MyOption::Some(2);
        assert_eq!(x.xor(y), MyOption::None);

        let x: MyOption<u32> = MyOption::None;
        let y: MyOption<u32> = MyOption::None;
        assert_eq!(x.xor(y), MyOption::None);
    }

    #[test]
    fn or_else_test() {
        fn nobody() -> MyOption<&'static str> {
            MyOption::None
        }
        fn vikings() -> MyOption<&'static str> {
            MyOption::Some("vikings")
        }

        assert_eq!(
            MyOption::Some("barbarians").or_else(vikings),
            MyOption::Some("barbarians")
        );
        assert_eq!(MyOption::None.or_else(vikings), MyOption::Some("vikings"));
        assert_eq!(MyOption::None.or_else(nobody), MyOption::None);
    }

    #[test]
    fn or_test() {
        let x = MyOption::Some(2);
        let y = MyOption::None;
        assert_eq!(x.or(y), MyOption::Some(2));

        let x = MyOption::None;
        let y = MyOption::Some(100);
        assert_eq!(x.or(y), MyOption::Some(100));

        let x = MyOption::Some(2);
        let y = MyOption::Some(100);
        assert_eq!(x.or(y), MyOption::Some(2));

        let x: MyOption<i32> = MyOption::None;
        let y = MyOption::None;
        assert_eq!(x.or(y), MyOption::None);
    }

    #[test]
    fn filter_test() {
        fn is_even(n: &i32) -> bool {
            n % 2 == 0
        }

        assert_eq!(MyOption::None.filter(is_even), MyOption::None);
        assert_eq!(MyOption::Some(3).filter(is_even), MyOption::None);
        assert_eq!(MyOption::Some(4).filter(is_even), MyOption::Some(4));
    }

    #[test]
    fn and_then_test() {
        fn sq(x: u32) -> MyOption<u32> {
            MyOption::Some(x * x)
        }
        fn nope(_: u32) -> MyOption<u32> {
            MyOption::None
        }

        assert_eq!(
            MyOption::Some(2).and_then(sq).and_then(sq),
            MyOption::Some(16)
        );
        assert_eq!(
            MyOption::Some(2).and_then(sq).and_then(nope),
            MyOption::None
        );
        assert_eq!(
            MyOption::Some(2).and_then(nope).and_then(sq),
            MyOption::None
        );
        assert_eq!(MyOption::None.and_then(sq).and_then(sq), MyOption::None);
    }

    #[test]
    fn and_test() {
        let x = MyOption::Some(2);
        let y: MyOption<&str> = MyOption::None;
        assert_eq!(x.and(y), MyOption::None);

        let x: MyOption<u32> = MyOption::None;
        let y = MyOption::Some("foo");
        assert_eq!(x.and(y), MyOption::None);

        let x = MyOption::Some(2);
        let y = MyOption::Some("foo");
        assert_eq!(x.and(y), MyOption::Some("foo"));

        let x: MyOption<u32> = MyOption::None;
        let y: MyOption<&str> = MyOption::None;
        assert_eq!(x.and(y), MyOption::None);
    }

    #[test]
    fn ok_or_else_test() {
        let x = MyOption::Some("foo");
        assert_eq!(x.ok_or_else(|| 0), Ok("foo"));

        let x: MyOption<&str> = MyOption::None;
        assert_eq!(x.ok_or_else(|| 0), Err(0));
    }

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
    fn map_or_else_test() {
        let k = 21;

        let x = MyOption::Some("foo");
        assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);

        let x: MyOption<&str> = MyOption::None;
        assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);
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

    #[test]
    fn ok_or_test() {
        let x = MyOption::Some("foo");
        assert_eq!(x.ok_or(0), Ok("foo"));

        let x: MyOption<&str> = MyOption::None;
        assert_eq!(x.ok_or(0), Err(0));
    }
}
