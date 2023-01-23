use librypt_entropy::EntropySource;
use librypt_int::u4096;

fn sundaram_sieve(start: usize, end: usize, idx: usize) -> Option<usize> {
    assert!(start > 1 && start < end);
    let range_nums = end - start;

    assert!(range_nums > idx);

    let max_primes = (range_nums - 3) / 2 + 1;

    let mut nums = vec![true; max_primes];
    for i in 0..((range_nums as f64).sqrt().trunc() as usize) / 2 + 1 {
        let step = 2 * i + 3;
        let cull_start = (step * step - 3) / 2;

        for j in (cull_start..max_primes).step_by(step) {
            nums[j] = false;
        }
    }

    let mut cnt = 0;
    for (i, &is_prime) in nums.iter().enumerate() {
        if is_prime {
            cnt += 1;
        }

        if cnt == idx + 1 {
            return Some(i + start);
        }
    }

    None
}

fn random_prime<S: EntropySource>(source: &S) -> Result<usize, S::EntropySourceError> {
    let mut range_bufs = [[0; usize::BITS as usize / 8], [0; usize::BITS as usize / 8]];
    let mut idx_buf = [0; usize::BITS as usize / 8];

    loop {
        let (start, end) = {
            let mut start = 0;
            let mut end = 0;
            while start == end {
                source.read_bytes(&mut range_bufs[0])?;
                source.read_bytes(&mut range_bufs[1])?;

                start = usize::from_ne_bytes(range_bufs[1]);
                end = usize::from_ne_bytes(range_bufs[0]);
            }

            if start > end {
                (end, start)
            } else {
                (start, end)
            }
        };

        source.read_bytes(&mut idx_buf)?;
        let idx = usize::from_ne_bytes(idx_buf) % (end - start);

        if let Some(x) = sundaram_sieve(start, end, idx) {
            return Ok(x);
        }
    }
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        (max, min) = (min, max);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        (max, min) = (min, res);
    }
}

fn carmichael(n: usize) -> usize {
    let mut coprimes = vec![];
    for i in 0..n {
        if gcd(i, n) == 1 {
            coprimes.push(i);
        }
    }
    let mut k = 0;
    loop {
        for coprime in coprimes.clone() {
            if coprime.pow(k) % n != 1 {
                break;
            } else {
                return k as usize;
            }
        }
        k += 1
    }
}

pub fn gen<S: EntropySource>(source: &S) -> Result<u4096, S::EntropySourceError> {
    let mut n;
    loop {
        let p = random_prime(source)?;
        let q = random_prime(source)?;
        let (x, carry) = p.overflowing_mul(q);
        n = x;
        if !carry {
            break;
        }
    }
    println!("{}", carmichael(n));

    Ok(42.into())
}
