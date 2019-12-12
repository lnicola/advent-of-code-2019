pub fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

pub fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

pub fn lcm3(x: u64, y: u64, z: u64) -> u64 {
    lcm(lcm(x, y), z)
}
