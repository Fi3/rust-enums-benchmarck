#[derive(Copy, Clone)]
pub enum Wrapper1 {
    A(i32),
}

impl Wrapper1 {
    #[inline(always)]
    pub fn get_inner(self) -> i32 {
        match self {
            Wrapper1::A(x) => x,
        }
    }
}

// get_inner for wrapper 1 it took ~= 610 ps

#[derive(Copy, Clone)]
pub enum Wrapper2 {
    A([u8;4]),
}

impl Wrapper2 {
    #[inline(always)]
    pub fn get_inner(self) -> [u8;4] {
        match self {
            Wrapper2::A(x) => x,
        }
    }
}

// get_inner for wrapper 2 it took ~= 610 ps

#[derive(Copy, Clone)]
pub enum Wrapper3 {
    A([u8;4]),
    B([u8;4]),
}

impl Wrapper3 {
    #[inline(always)]
    pub fn get_inner(self) -> [u8;4] {
        match self {
            Wrapper3::A(x) => x,
            Wrapper3::B(x) => x,
        }
    }
}

// get_inner for wrapper 3 it took ~= 6.15 ns

#[derive(Copy, Clone)]
pub enum Wrapper4 {
    A([u8;4]),
    B([u8;4]),
    C([u8;4]),
    D([u8;4]),
    E([u8;4]),
}
impl Wrapper4 {
    #[inline(always)]
    pub fn get_inner(self) -> [u8;4] {
        match self {
            Wrapper4::A(x) => x,
            Wrapper4::B(x) => x,
            Wrapper4::C(x) => x,
            Wrapper4::D(x) => x,
            Wrapper4::E(x) => x,
        }
    }
}

// get_inner for wrapper 4 it took ~= 6.15 ns

#[derive(Copy, Clone)]
pub enum Wrapper5 {
    A(i32),
    B(i32),
}

impl Wrapper5 {
    #[inline(always)]
    pub fn get_inner(self) -> i32 {
        match self {
            Wrapper5::A(x) => x,
            Wrapper5::B(x) => x,
        }
    }
}

// get_inner for wrapper 5 it took ~= 900 ps
