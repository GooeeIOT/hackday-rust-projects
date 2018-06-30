use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<'a, F, K: 'a, V: 'a> {
    calculation: F,
    values: HashMap<&'a K, &'a V>,
}

impl<'a, F, K, V> Cacher<'a, F, K, V>
where
    F: Fn(&K) -> &V,
    K: Eq + Hash,
{
    pub fn new(calculation: F) -> Cacher<'a, F, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: &'a K) -> &'a V {
        match self.values.get(&arg) {
            Some(&v) => &v,
            None => {
                let v = (self.calculation)(&arg);
                self.values.insert(&arg, v);
                &v
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|_| &"hello");

        let _ = c.value(&3);
        let v2 = c.value(&3);

        assert_eq!(v2, &"hello");
    }

}
