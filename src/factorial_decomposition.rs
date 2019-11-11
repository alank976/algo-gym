//! https://www.codewars.com/kata/5a045fee46d843effa000070/train/rust
use std::collections::{HashMap, HashSet, BTreeMap};
use std::ops::{Deref, DerefMut};
use core::ops;
use std::iter::FromIterator;
use std::rc::Rc;


fn decomp(n: i32) -> String {
    let mut mf = CachedFactorizer::new();
    (2..n + 1)
        .rev()
        .fold(PrimeFactorCount::new(), |acc, i| {
            acc + mf.factorize(i)
        })
        .to_string()
}

struct CachedFactorizer { cache: HashMap<i32, PrimeFactorCount> }

impl CachedFactorizer {
    fn new() -> Self { Self { cache: HashMap::new() } }

    fn factorize(&mut self, n: i32) -> PrimeFactorCount {
        self.cache.get(&n)
            // lookup cached value if any
            .map(|pfc| pfc.clone())
            // decompose and recursively compute
            .or_else(|| (2..n)
                .find(|&x| n % x == 0)
                .map(|x| self.factorize(x) + self.factorize(n / x)))
            // prime number: n^1 if no match above
            .unwrap_or(PrimeFactorCount::prime(n))
    }
}

#[derive(Debug)]
struct PrimeFactorCount(Rc<HashMap<i32, i32>>);

impl PrimeFactorCount {
    fn new() -> Self { Self::new_with_map(HashMap::new()) }
    fn new_with_map(map: HashMap<i32, i32>) -> Self { Self(Rc::new(map)) }

    fn prime(n: i32) -> Self {
        let mut m: HashMap<i32, i32> = HashMap::new();
        m.insert(n, 1);
        Self::new_with_map(m)
    }

    fn to_string(&self) -> String {
        let sorted: BTreeMap<&i32, &i32> = BTreeMap::from_iter(self.iter());
        sorted
            .iter()
            .map(|(&&k, &&v)| if v == 1 { k.to_string() } else { format!("{}^{}", k, v) })
            .collect::<Vec<String>>()
            .join(" * ")
    }
}

impl Deref for PrimeFactorCount {
    type Target = Rc<HashMap<i32, i32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PrimeFactorCount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Clone for PrimeFactorCount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl ops::Add<Self> for PrimeFactorCount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result: HashMap<i32, i32> = HashMap::new();

        self.keys()
            .chain(rhs.keys())
            .collect::<HashSet<&i32>>()
            .iter()
            .for_each(|&k| {
                let v1 = *self.get(k).unwrap_or(&0);
                let v2 = *rhs.get(k).unwrap_or(&0);
                result.insert(*k, v1 + v2);
            });
        Self::new_with_map(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memo_factorizer_works() {
        let mut mf = CachedFactorizer::new();
        let result = mf.factorize(6);

        assert_eq!(1, *result.get(&2).unwrap());
        assert_eq!(1, *result.get(&3).unwrap());
    }

    #[test]
    fn test_prime_factor_count_add_ops() {
        let mut m1: HashMap<i32, i32> = HashMap::new();
        m1.insert(1, 3);
        m1.insert(3, 1);

        let mut m2: HashMap<i32, i32> = HashMap::new();
        m2.insert(2, 5);
        m2.insert(1, 1);

        let aggregated = PrimeFactorCount::new_with_map(m1) + PrimeFactorCount::new_with_map(m2);
        assert_eq!(4, *aggregated.get(&1).unwrap());
        assert_eq!(5, *aggregated.get(&2).unwrap());
        assert_eq!(1, *aggregated.get(&3).unwrap());
    }

    fn dotest(n: i32, exp: &str) -> () {
        println!("n:{:?}", n);
        let ans = decomp(n);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp.to_string());
        println!("{}", ans == exp.to_string());
        assert_eq!(ans, exp.to_string());
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(5, "2^3 * 3 * 5");
        dotest(22, "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19");
        dotest(14, "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13");
        dotest(25, "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23");
    }

}
