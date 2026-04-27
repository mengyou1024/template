
unsafe extern "C" {
    #[link_name = "Add"]
    #[allow(unused)]
    unsafe fn add(a: i32, b: i32) -> i32;
    #[link_name = "Hello"]
    #[allow(unused)]
    unsafe fn hello();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(unsafe { add(1, 2) }, 3);
    }

    #[test]
    fn test_hello() {
        unsafe { hello() };
    }
}

