// WARNING: there is a remaining TODO for the `powi` function

pub(crate) trait FloatPolyfill
where
    Self: core::ops::Rem<Output = Self>
        + core::ops::Add<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::Neg<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Div<Output = Self>
        + core::cmp::PartialEq
        + core::cmp::PartialOrd
        + Sized
        + Copy,
{
    fn one() -> Self;
    fn zero() -> Self;

    fn floor(self) -> Self;

    fn ceil(self) -> Self {
        match self == self.floor() {
            true => self,
            false => self.floor() + Self::one(),
        }
    }

    fn round(self) -> Self {
        match self == self.floor() {
            true => self,
            false => self.floor() + Self::one().copysign(self),
        }
    }

    fn copysign(self, of_num: Self) -> Self {
        if of_num >= Self::zero() {
            self.abs()
        } else {
            -self.abs()
        }
    }

    fn abs(self) -> Self {
        if self >= Self::zero() {
            self
        } else {
            -self
        }
    }

    fn powi(self, n: i32) -> Self {
        if n == 0 {
            Self::one()
        } else if n == 1 {
            self
        } else if n < 0 {
            self.powi(-n).recip()
        } else {
            let mut result = Self::one();
            let mut base = self;

            // shadow
            let mut n = n;
            while n > 0 {
                if n % 2 == 0 {
                    base = base * base;
                    n = n / 2;
                } else {
                    result = result * base;
                    n = n - 1;
                }
            }
            result
        }
    }

    fn recip(self) -> Self {
        Self::one() / self
    }
}

impl FloatPolyfill for f64 {
    fn one() -> Self {
        1.0
    }

    fn zero() -> Self {
        0.0
    }

    fn floor(self) -> Self {
        // cannot use `self.floor()` because it is not available in no_std
        self - (self % 1.0)
    }
}

impl FloatPolyfill for f32 {
    fn one() -> Self {
        1.0
    }

    fn zero() -> Self {
        0.0
    }

    fn floor(self) -> Self {
        // cannot use `self.floor()` because it is not available in no_std
        self - (self % 1.0)
    }
}
