// #[macro_use]
// extern crate mockall;

use mockall::automock;

#[automock]
mod foo {
    pub fn bar() -> i32 {
        return 0;
    }
    pub fn baz() -> i32 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_foo;
    use mock_foo::{ bar, baz };

    #[test]
    fn test_a() {
        let ctx = mock_foo::bar_context();
        ctx.expect().returning(||7);
        for _ in 0..1000000 {
            let x = bar();
            println!("Got {}", x);
            assert_eq!(x, 7);
        }
    }

    #[test]
    fn test_b() {
        let ctx = mock_foo::bar_context();
        ctx.expect().returning(||22);
        for _ in 0..1000000 {
            let x = bar();
            println!("Got {}", x);
            assert_eq!(x, 22);
        }
    }
}
