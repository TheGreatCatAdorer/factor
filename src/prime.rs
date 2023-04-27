pub fn generate_primes(size: u64) -> Vec<u64> {
    if size < 26 {
        let result: &'static [u64] = if size < 3 {
            &[2]
        } else if size < 5 {
            &[2, 3]
        } else if size < 7 {
            &[2, 3, 5]
        } else if size < 11 {
            &[2, 3, 5, 7]
        } else if size < 13 {
            &[2, 3, 5, 7, 11]
        } else if size < 17 {
            &[2, 3, 5, 7, 11, 13]
        } else if size < 19 {
            &[2, 3, 5, 7, 11, 13, 17]
        } else if size < 23 {
            &[2, 3, 5, 7, 11, 13, 17, 19]
        } else {
            &[2, 3, 5, 7, 11, 13, 17, 19, 23]
        };
        return result.to_vec();
    }
    let mut wheel = vec![1, 5];
    let mut length = 6;
    let mut prime = 5;
    let mut result = vec![2, 3];
    while length < size {
        let next_length = length * prime;
        let mut next_wheel = vec![];
        for i in 0..prime {
            for &j in &wheel {
                let k = i * length + j;
                if k > size {
                    break;
                }
                if k % prime != 0 {
                    next_wheel.push(k);
                }
            }
        }
        wheel = next_wheel;
        length = next_length;
        result.push(prime);
        prime = wheel[1];
    }
    while prime * prime < size {
        let mut filters = wheel.iter();
        filters.next();
        let filters: Vec<_> = match prime.checked_pow(4).map(|i| i.min(size)) {
            Some(max) => filters.map_while(|&i| (i * i < max).then_some(i)).collect(),
            None => {
                let prime2 = prime.checked_mul(prime).expect("integer overflow!");
                filters
                    .map_while(|&i| (i < prime2 && i * i < size).then_some(i))
                    .collect()
            }
        };
        let mut next_wheel = vec![];
        for &i in &wheel {
            if filters.iter().all(|&j| i % j != 0) {
                next_wheel.push(i);
            }
        }
        wheel = next_wheel;
        prime = wheel[1];
        result.extend(filters.into_iter());
    }
    let mut wheel = wheel.into_iter();
    wheel.next();
    result.extend(wheel);
    result
}

pub fn factor<'a>(mut i: u64, primes: &'a [u64], mut each: impl FnMut(u64)) {
    let mut primes = primes.iter();
    while i != 1 {
        let &p = primes.next().unwrap();
        while i % p == 0 {
            i /= p;
            each(p);
        }
    }
}

pub fn factors<'a>(i: u64, primes: &'a [u64]) -> Vec<u64> {
    let mut result = vec![];
    factor(i, primes, |p| result.push(p));
    result
}

pub fn gcd(mut i: u64, mut j: u64) -> u64 {
    while j != 0 {
        let next = i % j;
        i = j;
        j = next;
    }
    i
}

pub fn lcm(i: u64, j: u64) -> u64 {
    i * j / gcd(i, j)
}
