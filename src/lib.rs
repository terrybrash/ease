pub trait Ease {
    /// A linear ease, equal to the identity function. Linear eases often appear mechanical and unphysical.
    fn linear(self) -> Self;

    fn in_back(self) -> Self;
    fn in_bounce(self) -> Self;
    fn in_circ(self) -> Self;
    fn in_elastic(self) -> Self;
    fn in_expo(self) -> Self;
    fn in_pow2(self) -> Self;
    fn in_pow3(self) -> Self;
    fn in_pow4(self) -> Self;
    fn in_pow5(self) -> Self;
    fn in_pow6(self) -> Self;
    fn in_pow7(self) -> Self;
    fn in_pow8(self) -> Self;
    fn in_sine(self) -> Self;

    fn out_back(self) -> Self;
    fn out_bounce(self) -> Self;
    fn out_circ(self) -> Self;
    fn out_elastic(self) -> Self;
    fn out_expo(self) -> Self;
    fn out_pow(self, y: Self) -> Self;
    fn out_pow2(self) -> Self;
    fn out_pow3(self) -> Self;
    fn out_pow4(self) -> Self;
    fn out_pow5(self) -> Self;
    fn out_pow6(self) -> Self;
    fn out_pow7(self) -> Self;
    fn out_pow8(self) -> Self;
    fn out_sine(self) -> Self;

    /// Non-sinusoidal waveform that linearly ramps up and down in a symmetric, sawtooth-like pattern
    fn in_out_triangle(self) -> Self;
    fn in_out_back(self) -> Self;
    fn in_out_bounce(self) -> Self;
    fn in_out_circ(self) -> Self;
    fn in_out_elastic(self) -> Self;
    fn in_out_expo(self) -> Self;
    fn in_out_pow2(self) -> Self;
    fn in_out_pow3(self) -> Self;
    fn in_out_pow4(self) -> Self;
    fn in_out_pow5(self) -> Self;
    fn in_out_sine(self) -> Self;

    fn out_in_hard(self) -> Self;
    fn out_in_soft(self) -> Self;
}

macro_rules! impl_ease_1d {
    ($($ty:ty => $namespace:ident),* $(,)?) => {
        $(
            impl Ease for $ty {
                fn linear(self) -> Self {
                    self
                }

                fn in_sine(self) -> Self {
                    1.0 - Self::cos((self * core::$namespace::consts::PI) / 2.0)
                }

                fn in_pow2(self) -> Self {
                    self * self
                }

                fn in_pow3(self) -> Self {
                    self * self * self
                }

                fn in_pow4(self) -> Self {
                    self * self * self * self
                }

                fn in_pow5(self) -> Self {
                    self * self * self * self * self
                }

                fn in_pow6(self) -> Self {
                    self * self * self * self * self * self
                }

                fn in_pow7(self) -> Self {
                    self * self * self * self * self * self * self
                }

                fn in_pow8(self) -> Self {
                    self * self * self * self * self * self * self * self
                }

                fn in_circ(self) -> Self {
                    1.0 - Self::sqrt(1.0 - Self::powf(self, 2.0))
                }

                fn in_elastic(self) -> Self {
                    let c4 = (2.0 * core::$namespace::consts::PI) / 3.0;
                    if self == 0.0 {
                        0.0
                    } else if self == 1.0 {
                        return 1.0;
                    } else {
                        return -Self::powf(2.0, 10.0 * self - 10.0) * Self::sin((self * 10.0 - 10.75) * c4);
                    }
                }

                fn in_expo(self) -> Self {
                    if self == 0.0 {
                        0.0
                    } else {
                        Self::powf(2.0, 10.0 * self - 10.0)
                    }
                }

                fn in_back(self) -> Self {
                    let strength = 1.70158;
                    let c3 = strength + 1.0;
                    c3 * self * self * self - strength * self * self
                }

                fn in_bounce(self) -> Self {
                    1.0 - Self::out_bounce(1.0 - self)
                }

                fn out_sine(self) -> Self {
                    Self::sin((self * core::$namespace::consts::PI) / 2.0)
                }

                fn out_pow(self, y: Self) -> Self {
                    1.0 - Self::powf(1.0 - self, y)
                }

                fn out_pow2(self) -> Self {
                    1.0 - Self::in_pow2(1.0 - self)
                }

                fn out_pow3(self) -> Self {
                    1.0 - Self::in_pow3(1.0 - self)
                }

                fn out_pow4(self) -> Self {
                    1.0 - Self::in_pow4(1.0 - self)
                }

                fn out_pow5(self) -> Self {
                    1.0 - Self::in_pow5(1.0 - self)
                }

                fn out_pow6(self) -> Self {
                    1.0 - Self::in_pow6(1.0 - self)
                }

                fn out_pow7(self) -> Self {
                    1.0 - Self::in_pow7(1.0 - self)
                }

                fn out_pow8(self) -> Self {
                    1.0 - Self::in_pow8(1.0 - self)
                }

                fn out_circ(self) -> Self {
                    Self::sqrt(1.0 - Self::powf(self - 1.0, 2.0))
                }

                fn out_elastic(self) -> Self {
                    let c4 = (2.0 * core::$namespace::consts::PI) / 3.0;
                    if self == 0.0 {
                        0.0
                    } else if self == 1.0 {
                        return 1.0;
                    } else {
                        return Self::powf(2.0, -10.0 * self) * Self::sin((self * 10.0 - 0.75) * c4) + 1.0;
                    }
                }

                fn out_expo(self) -> Self {
                    if self == 1.0 {
                        1.0
                    } else {
                        1.0 - Self::powf(2.0, -10.0 * self)
                    }
                }

                fn out_back(self) -> Self {
                    let c1 = 1.70158;
                    let c3 = c1 + 1.0;

                    1.0 + c3 * Self::powf(self - 1.0, 3.0) + c1 * Self::powf(self - 1.0, 2.0)
                }

                fn out_bounce(mut self) -> Self {
                    let n1 = 7.5625;
                    let d1 = 2.75;

                    if self < 1.0 / d1 {
                        n1 * self * self
                    } else if self < 2.0 / d1 {
                        self -= 1.5 / d1;
                        n1 * self * self + 0.75
                    } else if self < 2.5 / d1 {
                        self -= 2.25 / d1;
                        n1 * self * self + 0.9375
                    } else {
                        self -= 2.625 / d1;
                        n1 * self * self + 0.984375
                    }
                }

                fn in_out_triangle(self) -> Self {
                    Self::abs(self * 2.0 - 1.0) * -1.0 + 1.0
                }

                fn in_out_sine(self) -> Self {
                    -(Self::cos(core::$namespace::consts::PI * self) - 1.0) / 2.0
                }

                fn in_out_pow2(self) -> Self {
                    if self < 0.5 {
                        2.0 * self * self
                    } else {
                        1.0 - Self::powf(-2.0 * self + 2.0, 2.0) / 2.0
                    }
                }

                fn in_out_pow3(self) -> Self {
                    if self < 0.5 {
                        4.0 * self * self * self
                    } else {
                        1.0 - Self::powf(-2.0 * self + 2.0, 3.0) / 2.0
                    }
                }

                fn in_out_pow4(self) -> Self {
                    if self < 0.5 {
                        8.0 * self * self * self * self
                    } else {
                        1.0 - Self::powf(-2.0 * self + 2.0, 4.0) / 2.0
                    }
                }

                fn in_out_pow5(self) -> Self {
                    if self < 0.5 {
                        16.0 * self * self * self * self * self
                    } else {
                        1.0 - Self::powf(-2.0 * self + 2.0, 5.0) / 2.0
                    }
                }

                fn in_out_circ(self) -> Self {
                    if self < 0.5 {
                        (1.0 - Self::sqrt(1.0 - Self::powf(2.0 * self, 2.0))) / 2.0
                    } else {
                        (Self::sqrt(1.0 - Self::powf(-2.0 * self + 2.0, 2.0)) + 1.0) / 2.0
                    }
                }

                fn in_out_elastic(self) -> Self {
                    let c5 = (2.0 * core::$namespace::consts::PI) / 4.5;
                    if self == 0.0 {
                        0.0
                    } else if self == 1.0 {
                        1.0
                    } else if self < 0.5 {
                        -(Self::powf(2.0, 20.0 * self - 10.0) * Self::sin((20.0 * self - 11.125) * c5)) / 2.0
                    } else {
                        (Self::powf(2.0, -20.0 * self + 10.0) * Self::sin((20.0 * self - 11.125) * c5)) / 2.0 + 1.0
                    }
                }

                fn in_out_expo(self) -> Self {
                    if self == 0.0 {
                        0.0
                    } else if self == 1.0 {
                        return 1.0;
                    } else if self < 0.5 {
                        return Self::powf(2.0, 20.0 * self - 10.0) / 2.0;
                    } else {
                        return (2.0 - Self::powf(2.0, -20.0 * self + 10.0)) / 2.0;
                    }
                }

                fn in_out_back(self) -> Self {
                    let c1 = 1.70158;
                    let c2 = c1 * 1.525;
                    if self < 0.5 {
                        (Self::powf(2.0 * self, 2.0) * ((c2 + 1.0) * 2.0 * self - c2)) / 2.0
                    } else {
                        (Self::powf(2.0 * self - 2.0, 2.0) * ((c2 + 1.0) * (self * 2.0 - 2.0) + c2) + 2.0) / 2.0
                    }
                }

                fn in_out_bounce(self) -> Self {
                    if self < 0.5 {
                        (1.0 - Self::out_bounce(1.0 - 2.0 * self)) / 2.0
                    } else {
                        (1.0 + Self::out_bounce(2.0 * self - 1.0)) / 2.0
                    }
                }

                fn out_in_hard(self) -> Self {
                    if self < 0.5 {
                        1.0 - 1.0 / (self * 2.0 + 1.0)
                    } else {
                        -1.0 / (self * 2.0 - 3.0)
                    }
                }

                fn out_in_soft(self) -> Self {
                    if self < 0.5 {
                        1.0 - 1.0 / (self * 1.3333333 + 1.0)
                    } else {
                        -1.0 / (self * 3.0 - 4.0)
                    }
                }
            }
        )*
    };
}

impl_ease_1d!(
    f32 => f32,
    f64 => f64,
);
