#[no_mangle]
pub extern fn add(n: i32, m: i32) -> i32 {
    n + m
}


#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(2 + 2, 4);
    }
}
