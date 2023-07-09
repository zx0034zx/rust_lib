use std::collections::HashMap;
use std::hash::Hash;

#[allow(dead_code)]
pub struct Cacher<T, X: Eq + Hash + Clone>
where
    T: Fn(X) -> X,
{
    _calculation: T,
    _value: Option<HashMap<X, X>>,
}
#[allow(dead_code)]
impl<T, X: Eq + Hash + Clone> Cacher<T, X>
where
    T: Fn(X) -> X,
{
    pub fn new(calculation: T) -> Cacher<T, X> {
        Cacher {
            _calculation: calculation,
            _value: None,
        }
    }
    pub fn value(&mut self, arg: X) -> X {
        match self._value {
            Some(ref mut v) => match v.get(&arg) {
                Some(v) => v.clone(),
                None => {
                    let v = (self._calculation)(arg);
                    v
                }
            },
            None => {
                let mut _v: HashMap<X, X> = HashMap::new();
                let _v = (self._calculation)(arg);
                _v
            }
        }
    }
}