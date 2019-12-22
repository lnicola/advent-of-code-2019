use std::ops::{Add, Mul};

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

#[derive(Copy, Clone)]
pub struct Zp {
    pub n: i128,
    pub p: i128,
}

impl Zp {
    pub fn new(n: i128, p: i128) -> Self {
        let n = (n + p) % p;
        Self { n, p }
    }
}

impl Add for Zp {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.p, rhs.p);
        Self {
            n: (self.n + rhs.n) % self.p,
            p: self.p,
        }
    }
}

impl Add<&Self> for Zp {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        assert_eq!(self.p, rhs.p);
        Self {
            n: (self.n + rhs.n) % self.p,
            p: self.p,
        }
    }
}

impl Mul for Zp {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.p, rhs.p);
        Self {
            n: self.n * rhs.n % self.p,
            p: self.p,
        }
    }
}

impl Mul<&Self> for Zp {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        assert_eq!(self.p, rhs.p);
        Self {
            n: self.n * rhs.n % self.p,
            p: self.p,
        }
    }
}
