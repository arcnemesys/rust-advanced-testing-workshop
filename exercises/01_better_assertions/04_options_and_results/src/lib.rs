#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::{anything, err, none, ok, some, eq};

    #[googletest::test]

    fn failed_is_none() {
        let x = Some(1);
        assert_that!(x, none());
    }

    #[googletest::test]
    fn failed_is_some() {
        let x: Option<usize> = None;
        assert_that!(Some(x), some(eq(x)));
    }

    #[googletest::test]
    fn failed_is_ok() {
        let x: Result<u32, &str> = Err("Something went wrong");
        assert_that!(Ok(x), ok(eq("Something went wrong")));
    }

    #[googletest::test]
    fn failed_is_err() {
        let x: Result<u32, &str> = Ok(42);
        assert_that!(Err(x), err(eq(x)));
    }
}
