#[allow(dead_code)]
fn prime_factors(n: i64) -> String {
    let mut new_n: u64 = n as u64;
    let mut count;
    let mut result: String = String::new();

    for prime in PrimeGenerator::new(2, new_n) {
        count = 0;
        if new_n != 1 {
            while new_n % prime == 0 {
                count += 1;
                new_n /= prime;
            }
            match count {
                0 => (),
                1 => result.push_str(format!("({})", prime).as_ref()),
                _ => result.push_str(format!("({}**{})", prime, count).as_ref()),
            };
        } else {
            break;
        }
    }
    result
}

struct PrimeGenerator {
    end: u64,
    n: u64,
}

#[allow(dead_code)]
impl PrimeGenerator {
    fn new(start: u64, end: u64) -> Self {
        Self { end, n: start }
    }

    fn is_prime(&self, n: u64) -> bool {
        match n {
            2 | 3 => true,
            _ if n % 2 == 0 || n % 3 == 0 => false,
            _ => {
                let mut result: bool = true;
                let (mut i, mut w) = (5, 2);
                while i * i <= n {
                    if n % i == 0 {
                        result = false;
                        break;
                    } else {
                        i += w;
                        w = 6 - w;
                    }
                }
                result
            }
        }
    }
}

impl Iterator for PrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: Option<Self::Item> = None;
        while self.n <= self.end {
            if self.is_prime(self.n) {
                next = Some(self.n);
                self.n += 1;
                break;
            } else {
                self.n += 1;
            }
        }
        next
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn testing(n: i64, exp: &str) -> () {
        assert_eq!(&prime_factors(n), exp)
    }

    #[test]
    fn test_basics_prime_factors() {
        testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
        testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
    }
}
