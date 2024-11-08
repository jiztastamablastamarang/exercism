pub fn private_key(p: u64) -> u64 {
    return (2..p).find(|&x| is_prime(x)).unwrap();
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    return mod_pow(g, a, p);
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    return mod_pow(b_pub, a, p);
}


fn mod_pow(mut b: u64, mut e: u64, m: u64) -> u64 {
    if m == 1 { return 0; };

    let mut r = 1;
    b = b % m;

    while e > 0 {
        if e % 2 == 1 {
            r = (r * b) % m;
        }

        b = (b * b) % m;
        e = e >> 1;
    }

    return r;
}

fn is_prime(n: u64) -> bool {
    return match n {
        0 | 1 => false,
        2 => true,
        _ if n % 2 == 0 => false,
        _ => (3..=(n as f64).sqrt() as u64).step_by(2).all(|x| n % x != 0),
    };
}
