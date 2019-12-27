use std::ops::{Add, Mul};

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
