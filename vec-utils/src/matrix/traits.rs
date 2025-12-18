use crate::complex::Complex;

pub trait Zeroable {
    fn zero() -> Self;

    fn is_zero(&self) -> bool;
}

pub trait Oneable {
    fn one() -> Self;

    fn is_one(&self) -> bool;
}

pub trait Twoable {
    fn two() -> Self;

    fn is_two(&self) -> bool;

    fn half(&self) -> Self;

    fn sqr(&self) -> Self;

    fn sqrt(&self) -> Self;
}

pub trait Fourable {
    fn four() -> Self;
}

pub trait Signed {
    fn abs(&self) -> Self;

    fn flip(&mut self);
}

impl Zeroable for f64 {
    fn is_zero(&self) -> bool {
        *self == 0.0
    }

    fn zero() -> Self {
        0.0
    }
}

impl Oneable for f64 {
    fn is_one(&self) -> bool {
        (self - 1.0).abs() < f64::EPSILON
    }

    fn one() -> Self {
        1.0
    }
}

impl Twoable for f64 {
    fn two() -> Self {
        2.0
    }

    fn is_two(&self) -> bool {
        (self - 2.0).abs() < f64::EPSILON
    }

    fn half(&self) -> Self {
        self / 2.0
    }

    fn sqr(&self) -> Self {
        self.powi(2)
    }

    fn sqrt(&self) -> Self {
        #[cfg(not(feature = "std"))]
        return core::f64::math::sqrt(self);
        #[cfg(feature = "std")]
        return f64::sqrt(*self);
    }
}

impl Fourable for f64 {
    fn four() -> Self {
        4.0
    }
}

impl Signed for f64 {
    fn abs(&self) -> Self {
        f64::abs(*self)
    }

    fn flip(&mut self) {
        *self *= -1.0;
    }
}

impl Zeroable for Complex {
    fn is_zero(&self) -> bool {
        self.real.abs() < f64::EPSILON && self.imaginary.abs() < f64::EPSILON
    }

    fn zero() -> Self {
        Self {
            real: 0.0,
            imaginary: 0.0
        }
    }
}

impl Oneable for Complex {
    fn is_one(&self) -> bool {
        (self.real - 1.0).abs() < f64::EPSILON && self.imaginary.abs() < f64::EPSILON
    }

    fn one() -> Self {
        Self {
            real: 1.0,
            imaginary: 0.0
        }
    }
}

impl Twoable for Complex {
    fn two() -> Self {
        Self {
            real: 2.0,
            imaginary: 0.0
        }
    }

    fn is_two(&self) -> bool {
        (self.real - 2.0).abs() < f64::EPSILON && self.imaginary.abs() < f64::EPSILON
    }

    fn half(&self) -> Self {
        Self {
            real: self.real / 2.0,
            imaginary: self.imaginary / 2.0
        }
    }

    fn sqr(&self) -> Self {
        Self {
            real: self.real.powi(2) - self.imaginary.powi(2),
            imaginary: 2.0 * self.real * self.imaginary
        }
    }

    fn sqrt(&self) -> Self {
        Complex::sqrt(self)
    }
}

impl Fourable for Complex {
    fn four() -> Self {
        Self {
            real: 4.0,
            imaginary: 0.0
        }
    }
}

impl Signed for Complex {
    fn abs(&self) -> Self {
        Self {
            real: self.magnitude(),
            imaginary: 0.0
        }
    }

    fn flip(&mut self) {
        *self = *self * -1.0;
    }
}
