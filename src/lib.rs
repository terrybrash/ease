use glam::Vec2;

pub trait FloatExt<Component> {
    /// Linearly interpolates between `a` and `b` by `self`
    fn lerp(self, a: Self, b: Self) -> Self;

    /// Framerate independant "lerp" from `from` to `to`.
    /// It's not really a lerp but more of an exponential easing where `self` is the
    /// strength of the dampening. Can be greater than 1.0.
    /// https://www.rorydriscoll.com/2016/03/07/frame-rate-independent-damping-using-lerp/
    /// Also called "decay"
    fn damp(self, from: Self, to: Self, dt: Component) -> Self;

    /// Returns 0.0 if `self < edge`, otherwise 1.0.
    fn step(self, edge: Self) -> Self;

    /// A linear ease, equal to the identity function. Linear eases often appear mechanical and unphysical.
    fn linear(self) -> Self;

    /// Blends two angles by interpolating their unit vector components.
    /// Naturally handles wrapping. Non-constant angular velocity.
    fn nlerp_rad(self, a: Self, b: Self) -> Self;

    /// Interpolates between two angles along the shortest arc
    /// at constant angular velocity.
    fn lerp_rad(self, from: Self, to: Self) -> Self;

    fn damp_rad(self, from: Self, to: Self, dt: Component) -> Self;

    /// Normalize angle to [-π, π] range
    fn normalize_rad(self) -> Self;

    /// Returns a value from `0..1` along a parabolic curve.
    fn parabolic(self) -> Self;

    /// Returns a value from `0..1` along a hyperbolic curve.
    fn hyperbolic(self) -> Self;

    /// `edge0` shouldn't be greater than or equal to `edge1`
    fn smoothstep(self, edge0: Self, edge1: Self) -> Self;

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

    /// v is input value, m is modulus. wrap(a, b) is a replacement for a % b.
    /// This won't work for negative or zero modulo values.
    /// Try this instead (if necessary):
    /// https://www.imaginary-institute.com/resources/TechNote12/TechNote12.html
    // https://github.com/rust-lang/rust/issues/87970
    // https://news.ycombinator.com/item?id=34540353
    fn wrap(self, m: Self) -> Self;
    fn wrap_every(self, interval: Self) -> Self;

    /// Transform an unsigned unit (0..1) into a signed unit (-1..1)
    fn into_bipolar(self) -> Self;

    /// Transform a signed unit (-1..1) into an unsigned unit (0..1)
    fn into_unipolar(self) -> Self;
}

macro_rules! impl_float_1d {
    ($($ty:ty => $namespace:ident),* $(,)?) => {
        $(
            impl FloatExt<$ty> for $ty {
                fn lerp(self, from: Self, to: Self) -> Self {
                    from + (to - from) * self
                }

                fn damp(self, from: Self, to: Self, dt: $ty) -> Self {
                    (1.0 - Self::exp(-self * dt)).lerp(from, to)
                }

                fn step(self, edge: Self) -> Self {
                    if self < edge { 0.0 } else { 1.0 }
                }

                fn nlerp_rad(self, a: Self, b: Self) -> Self {
                    let cs = (1.0 - self) * Self::cos(a) + self * Self::cos(b);
                    let sn = (1.0 - self) * Self::sin(a) + self * Self::sin(b);
                    Self::atan2(sn, cs)
                }

                fn lerp_rad(self, from: Self, to: Self) -> Self {
                    let from = from.normalize_rad();
                    let to = to.normalize_rad();

                    let mut delta = to - from;

                    const PI: $namespace = core::$namespace::consts::PI;
                    if delta > PI {
                        delta -= 2.0 * PI;
                    } else if delta < -PI {
                        delta += 2.0 * PI;
                    }

                    (from + delta * self).normalize_rad()
                }

                fn damp_rad(self, from: Self, to: Self, dt: $ty) -> Self {
                    Self::lerp_rad(1.0 - Self::exp(-self * dt), from, to)
                }

                fn normalize_rad(self) -> Self {
                    const PI: $namespace = core::$namespace::consts::PI;
                    let mut normalized = self % (2.0 * PI);
                    if normalized > PI {
                        normalized -= 2.0 * PI;
                    } else if normalized < -PI {
                        normalized += 2.0 * PI;
                    }
                    normalized
                }

                fn linear(self) -> Self {
                    self
                }

                fn parabolic(self) -> Self {
                    1.0 - Self::powf(self * 2.0 - 1.0, 2.0)
                }

                fn hyperbolic(self) -> Self {
                    1.0 - 1.0 / (1.0 + self)
                }

                fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
                    let t = Self::clamp((self - edge0) / (edge1 - edge0), 0.0, 1.0);
                    t * t * (3.0 - 2.0 * t)
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
                        1.0
                    } else {
                        -Self::powf(2.0, 10.0 * self - 10.0) * Self::sin((self * 10.0 - 10.75) * c4)
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
                        1.0
                    } else {
                        Self::powf(2.0, -10.0 * self) * Self::sin((self * 10.0 - 0.75) * c4) + 1.0
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
                    1.0 - Self::abs(self * 2.0 - 1.0)
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
                        1.0
                    } else if self < 0.5 {
                        Self::powf(2.0, 20.0 * self - 10.0) / 2.0
                    } else {
                        (2.0 - Self::powf(2.0, -20.0 * self + 10.0)) / 2.0
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
                        1.0 - 1.0 / (self * (4.0 / 3.0) + 1.0)
                    } else {
                        -1.0 / (self * 3.0 - 4.0)
                    }
                }

                fn wrap(self, m: Self) -> Self {
                    let zero = Default::default();
                    if m <= zero { zero } else { ((self % m) + m) % m }
                }

                fn wrap_every(self, interval: Self) -> Self {
                    Self::wrap(self, interval) / interval
                }

                fn into_bipolar(self) -> Self {
                    self * 2.0 - 1.0
                }

                fn into_unipolar(self) -> Self {
                    (self + 1.0) * 0.5
                }
            }
        )*
    };
}

impl_float_1d!(
    f32 => f32,
    f64 => f64,
);

impl FloatExt<f32> for Vec2 {
    fn lerp(self, from: Self, to: Self) -> Self {
        Vec2::new(self.x.lerp(from.x, to.x), self.y.lerp(from.y, to.y))
    }

    fn damp(self, from: Self, to: Self, dt: f32) -> Self {
        Vec2::new(self.x.damp(from.x, to.x, dt), self.y.damp(from.y, to.y, dt))
    }

    fn step(self, edge: Self) -> Self {
        Vec2::new(self.x.step(edge.x), self.y.step(edge.y))
    }

    fn nlerp_rad(self, a: Self, b: Self) -> Self {
        Vec2::new(self.x.nlerp_rad(a.x, b.x), self.y.nlerp_rad(a.y, b.y))
    }

    fn lerp_rad(self, from: Self, to: Self) -> Self {
        Vec2::new(self.x.lerp_rad(from.x, to.x), self.y.lerp_rad(from.y, to.y))
    }

    fn damp_rad(self, from: Self, to: Self, dt: f32) -> Self {
        Vec2::new(
            self.x.damp_rad(from.x, to.x, dt),
            self.y.damp_rad(from.y, to.y, dt),
        )
    }

    fn normalize_rad(self) -> Self {
        Vec2::new(self.x.normalize_rad(), self.y.normalize_rad())
    }

    fn linear(self) -> Self {
        self
    }

    fn parabolic(self) -> Self {
        Vec2::new(self.x.parabolic(), self.y.parabolic())
    }

    fn hyperbolic(self) -> Self {
        Vec2::new(self.x.hyperbolic(), self.y.hyperbolic())
    }

    fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
        Vec2::new(
            self.x.smoothstep(edge0.x, edge1.x),
            self.y.smoothstep(edge0.y, edge1.y),
        )
    }

    fn in_back(self) -> Self {
        Vec2::new(self.x.in_back(), self.y.in_back())
    }

    fn in_bounce(self) -> Self {
        Vec2::new(self.x.in_bounce(), self.y.in_bounce())
    }

    fn in_circ(self) -> Self {
        Vec2::new(self.x.in_circ(), self.y.in_circ())
    }

    fn in_elastic(self) -> Self {
        Vec2::new(self.x.in_elastic(), self.y.in_elastic())
    }

    fn in_expo(self) -> Self {
        Vec2::new(self.x.in_expo(), self.y.in_expo())
    }

    fn in_pow2(self) -> Self {
        Vec2::new(self.x.in_pow2(), self.y.in_pow2())
    }

    fn in_pow3(self) -> Self {
        Vec2::new(self.x.in_pow3(), self.y.in_pow3())
    }

    fn in_pow4(self) -> Self {
        Vec2::new(self.x.in_pow4(), self.y.in_pow4())
    }

    fn in_pow5(self) -> Self {
        Vec2::new(self.x.in_pow5(), self.y.in_pow5())
    }

    fn in_pow6(self) -> Self {
        Vec2::new(self.x.in_pow6(), self.y.in_pow6())
    }

    fn in_pow7(self) -> Self {
        Vec2::new(self.x.in_pow7(), self.y.in_pow7())
    }

    fn in_pow8(self) -> Self {
        Vec2::new(self.x.in_pow8(), self.y.in_pow8())
    }

    fn in_sine(self) -> Self {
        Vec2::new(self.x.in_sine(), self.y.in_sine())
    }

    fn out_back(self) -> Self {
        Vec2::new(self.x.out_back(), self.y.out_back())
    }

    fn out_bounce(self) -> Self {
        Vec2::new(self.x.out_bounce(), self.y.out_bounce())
    }

    fn out_circ(self) -> Self {
        Vec2::new(self.x.out_circ(), self.y.out_circ())
    }

    fn out_elastic(self) -> Self {
        Vec2::new(self.x.out_elastic(), self.y.out_elastic())
    }

    fn out_expo(self) -> Self {
        Vec2::new(self.x.out_expo(), self.y.out_expo())
    }

    fn out_pow(self, y: Self) -> Self {
        Vec2::new(self.x.out_pow(y.x), self.y.out_pow(y.y))
    }

    fn out_pow2(self) -> Self {
        Vec2::new(self.x.out_pow2(), self.y.out_pow2())
    }

    fn out_pow3(self) -> Self {
        Vec2::new(self.x.out_pow3(), self.y.out_pow3())
    }

    fn out_pow4(self) -> Self {
        Vec2::new(self.x.out_pow4(), self.y.out_pow4())
    }

    fn out_pow5(self) -> Self {
        Vec2::new(self.x.out_pow5(), self.y.out_pow5())
    }

    fn out_pow6(self) -> Self {
        Vec2::new(self.x.out_pow6(), self.y.out_pow6())
    }

    fn out_pow7(self) -> Self {
        Vec2::new(self.x.out_pow7(), self.y.out_pow7())
    }

    fn out_pow8(self) -> Self {
        Vec2::new(self.x.out_pow8(), self.y.out_pow8())
    }

    fn out_sine(self) -> Self {
        Vec2::new(self.x.out_sine(), self.y.out_sine())
    }

    fn in_out_triangle(self) -> Self {
        Vec2::new(self.x.in_out_triangle(), self.y.in_out_triangle())
    }

    fn in_out_back(self) -> Self {
        Vec2::new(self.x.in_out_back(), self.y.in_out_back())
    }

    fn in_out_bounce(self) -> Self {
        Vec2::new(self.x.in_out_bounce(), self.y.in_out_bounce())
    }

    fn in_out_circ(self) -> Self {
        Vec2::new(self.x.in_out_circ(), self.y.in_out_circ())
    }

    fn in_out_elastic(self) -> Self {
        Vec2::new(self.x.in_out_elastic(), self.y.in_out_elastic())
    }

    fn in_out_expo(self) -> Self {
        Vec2::new(self.x.in_out_expo(), self.y.in_out_expo())
    }

    fn in_out_pow2(self) -> Self {
        Vec2::new(self.x.in_out_pow2(), self.y.in_out_pow2())
    }

    fn in_out_pow3(self) -> Self {
        Vec2::new(self.x.in_out_pow3(), self.y.in_out_pow3())
    }

    fn in_out_pow4(self) -> Self {
        Vec2::new(self.x.in_out_pow4(), self.y.in_out_pow4())
    }

    fn in_out_pow5(self) -> Self {
        Vec2::new(self.x.in_out_pow5(), self.y.in_out_pow5())
    }

    fn in_out_sine(self) -> Self {
        Vec2::new(self.x.in_out_sine(), self.y.in_out_sine())
    }

    fn out_in_hard(self) -> Self {
        Vec2::new(self.x.out_in_hard(), self.y.out_in_hard())
    }

    fn out_in_soft(self) -> Self {
        Vec2::new(self.x.out_in_soft(), self.y.out_in_soft())
    }

    fn wrap(self, m: Self) -> Self {
        Vec2::new(self.x.wrap(m.x), self.y.wrap(m.y))
    }

    fn wrap_every(self, interval: Self) -> Self {
        Vec2::new(self.x.wrap_every(interval.x), self.y.wrap_every(interval.y))
    }

    fn into_bipolar(self) -> Self {
        Vec2::new(self.x.into_bipolar(), self.y.into_bipolar())
    }

    fn into_unipolar(self) -> Self {
        Vec2::new(self.x.into_unipolar(), self.y.into_unipolar())
    }
}
