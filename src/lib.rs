pub mod cacher;
use crate::cacher::cacher::Cacher;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut c = Cacher::new(|x| x);
        let v1 = c.value(1);
        let v2 = c.value(2);
        let v3 = c.value(3);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
        assert_ne!(v3, 2);
    }
}
