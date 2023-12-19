use std::{
    collections::BTreeSet,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub,
        SubAssign,
    },
};

pub trait ApplyTo: Sized {
    fn app<F, T>(self, f: F) -> T
    where
        F: FnOnce(Self) -> T,
    {
        f(self)
    }

    fn appr<'a, F, T>(&'a self, f: F) -> T
    where
        F: FnOnce(&'a Self) -> T,
    {
        f(self)
    }

    fn appm<'a, F, T>(&'a mut self, f: F) -> T
    where
        F: FnOnce(&'a mut Self) -> T,
    {
        f(self)
    }
}
impl<T> ApplyTo for T {}

pub trait Chain: Sized {
    fn ch<F, T>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self) -> T,
    {
        f(&mut self);
        self
    }
}
impl<T> Chain for T {}

pub trait ChainDeref: Sized + std::ops::DerefMut {
    fn chd<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self::Target) -> (),
    {
        f(&mut *self);
        self
    }
}
impl<T: std::ops::DerefMut> ChainDeref for T {}

pub trait CollectVec: Iterator + Sized {
    fn cv(self) -> Vec<Self::Item> {
        self.collect()
    }
    fn cs(self) -> BTreeSet<Self::Item>
    where
        Self::Item: Ord,
    {
        self.collect()
    }
}
impl<I: Iterator> CollectVec for I {}

pub trait Transpose {
    type Out;
    fn transpose(&self) -> Self::Out;
}
impl<T: Clone> Transpose for Vec<Vec<T>> {
    type Out = Vec<Vec<T>>;
    fn transpose(&self) -> Vec<Vec<T>> {
        let h = self.len();
        let w = self[0].len();
        let mut r = vec![];
        for i in 0..h {
            r.push(vec![]);
            for j in 0..w {
                r.last_mut().unwrap().push(self[j][i].clone());
            }
        }
        r
    }
}

pub struct Dsu {
    pub p: Vec<usize>,
    pub s: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        let mut p = vec![0; n];
        for (i, v) in p.iter_mut().enumerate() {
            *v = i;
        }
        Dsu { p, s: vec![1; n] }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.p[i] == i {
            return i;
        }
        let v = self.find(self.p[i]);
        self.p[i] = v;
        return self.p[i];
    }

    pub fn merge(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return;
        }
        if self.s[j] > self.s[i] {
            self.p[i] = j;
            self.s[j] += self.s[i];
        } else {
            self.p[j] = i;
            self.s[i] += self.s[j];
        }
    }
}

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct P<T>(pub T, pub T);

macro_rules! impl_p {
    ($t:ty) => {
        impl P<$t> {
            pub fn au(self) -> P<usize> {
                P(self.0 as _, self.1 as _)
            }

            pub fn ai(self) -> P<i64> {
                P(self.0 as _, self.1 as _)
            }

            pub fn af(self) -> P<f64> {
                P(self.0 as _, self.1 as _)
            }
        }

        impl Add for P<$t> {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
                self += rhs;
                self
            }
        }

        impl AddAssign for P<$t> {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
                self.1 += rhs.1;
            }
        }

        impl Sub for P<$t> {
            type Output = P<$t>;

            fn sub(mut self, rhs: Self) -> Self::Output {
                self -= rhs;
                self
            }
        }

        impl SubAssign for P<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
                self.1 -= rhs.1;
            }
        }

        impl MulAssign<$t> for P<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                self.0 *= rhs;
                self.1 *= rhs;
            }
        }

        impl Mul<$t> for P<$t> {
            type Output = Self;
            fn mul(mut self, rhs: $t) -> Self {
                self *= rhs;
                self
            }
        }

        impl Mul<P<$t>> for $t {
            type Output = P<$t>;
            fn mul(self, mut rhs: P<$t>) -> P<$t> {
                rhs *= self;
                rhs
            }
        }

        impl DivAssign<$t> for P<$t> {
            fn div_assign(&mut self, rhs: $t) {
                self.0 /= rhs;
                self.1 /= rhs;
            }
        }

        impl Div<$t> for P<$t> {
            type Output = Self;
            fn div(mut self, rhs: $t) -> Self {
                self /= rhs;
                self
            }
        }

        impl RemAssign<$t> for P<$t> {
            fn rem_assign(&mut self, rhs: $t) {
                self.0 %= rhs;
                self.1 %= rhs;
            }
        }

        impl Rem<$t> for P<$t> {
            type Output = Self;
            fn rem(mut self, rhs: $t) -> Self {
                self %= rhs;
                self
            }
        }
    };
}

impl_p!(i64);
impl_p!(f64);
impl_p!(usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    pub fn p(self) -> P<i64> {
        match self {
            Dir::U => P(0, -1),
            Dir::D => P(0, 1),
            Dir::L => P(-1, 0),
            Dir::R => P(1, 0),
        }
    }
    pub fn l(self) -> Dir {
        match self {
            Dir::U => Dir::L,
            Dir::D => Dir::R,
            Dir::L => Dir::D,
            Dir::R => Dir::U,
        }
    }
    pub fn r(self) -> Dir {
        match self {
            Dir::U => Dir::R,
            Dir::D => Dir::L,
            Dir::L => Dir::U,
            Dir::R => Dir::D,
        }
    }

    /// Reflect along positive slope
    pub fn rp(self) -> Dir {
        match self {
            Dir::U => Dir::R,
            Dir::D => Dir::L,
            Dir::L => Dir::D,
            Dir::R => Dir::U,
        }
    }
    /// Reflect along negative slope
    pub fn rn(self) -> Dir {
        match self {
            Dir::U => Dir::L,
            Dir::D => Dir::R,
            Dir::L => Dir::U,
            Dir::R => Dir::D,
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "U" | "N" => Dir::U,
            "D" | "S" => Dir::D,
            "L" | "W" => Dir::L,
            "R" | "E" => Dir::R,
            _ => unreachable!(),
        }
    }
}

pub struct Grid<T> {
    pub array: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(array: Vec<T>, width: usize) -> Self {
        let height = array.len() / width;
        Self {
            array,
            width,
            height,
        }
    }
    pub fn len(&self) -> usize {
        self.height
    }
}

impl Grid<u8> {
    pub fn from_ascii(s: &str) -> Self {
        let w = s.lines().next().unwrap().len();
        let a = s.lines().flat_map(|x| x.as_bytes()).copied().cv();
        Self::new(a, w)
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &[T] {
        &self.array[index * self.width..][..self.width]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        &mut self.array[index * self.width..][..self.width]
    }
}

impl<T> Index<P<usize>> for Grid<T> {
    type Output = T;
    fn index(&self, index: P<usize>) -> &T {
        &self[index.1][index.0]
    }
}

impl<T> IndexMut<P<usize>> for Grid<T> {
    fn index_mut(&mut self, index: P<usize>) -> &mut T {
        &mut self[index.1][index.0]
    }
}
