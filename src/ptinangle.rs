#[derive(Copy, Clone)]
struct Pt {
    x: f64,
    y: f64,
}

impl Pt {
    fn new() -> Self {
        Pt { x: 0.0, y: 0.0 }
    }

    fn with_coords(x: f64, y: f64) -> Self {
        Pt { x, y }
    }
}

impl std::ops::Add for Pt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pt {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Pt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Pt {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul for Pt {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl std::ops::BitXor for Pt {
    type Output = f64;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.x * rhs.y - rhs.x * self.y
    }
}

fn angle_between(a: Pt, b: Pt) -> f64 {
    f64::atan2(a ^ b, a * b)
}

fn is_in(a: Pt, b: Pt, c: Pt, v: Pt) -> bool {
    let ab = b - a;
    let ac = c - a;
    let av = v - a;
    let sign_ab_av = sign(ab ^ av);
    let sign_ac_av = sign(ac ^ av);
    (sign_ab_av * sign_ac_av <= 0) && sign(ab * av) >= 0
}

fn sign(x: f64) -> i32 {
    if x > 0.0 {
        1
    } else if x < 0.0 {
        -1
    } else {
        0
    }
}
