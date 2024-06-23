#[no_mangle]
pub extern "C" fn rust_add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rust_add(2, 2);
        assert_eq!(result, 4);
    }
}
