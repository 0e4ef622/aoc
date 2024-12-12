use std::{
    collections::BTreeSet,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub,
        SubAssign,
    }, slice::{ChunksExact, ChunksExactMut},
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

#[derive(Debug)]
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

    pub fn merge(&mut self, i: usize, j: usize) -> bool {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return false;
        }
        if self.s[j] > self.s[i] {
            self.p[i] = j;
            self.s[j] += self.s[i];
        } else {
            self.p[j] = i;
            self.s[i] += self.s[j];
        }
        true
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
#[repr(u8)]
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
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::U, Self::D, Self::L, Self::R].into_iter()
    }
}

impl Add<Dir> for P<i64> {
    type Output = Self;
    fn add(mut self, rhs: Dir) -> P<i64> {
        self.0 += rhs.p().0;
        self.1 += rhs.p().1;
        self
    }
}

impl Add<Dir> for P<f64> {
    type Output = Self;
    fn add(mut self, rhs: Dir) -> P<f64> {
        self.0 += rhs.p().0 as f64;
        self.1 += rhs.p().1 as f64;
        self
    }
}

impl Add<Dir> for P<usize> {
    type Output = Self;
    fn add(mut self, rhs: Dir) -> P<usize> {
        self.0 = (self.0 as i64 + rhs.p().0) as usize;
        self.1 = (self.1 as i64 + rhs.p().1) as usize;
        self
    }
}

#[derive(Clone)]
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
    pub fn iter(&self) -> impl Iterator<Item = &[T]> {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.into_iter()
    }
}

impl<T: Eq> Grid<T> {
    pub fn count(&self, value: &T) -> usize {
        self.array.iter().filter(|&x| x == value).count()
    }
}

impl<T: Eq + Clone> Grid<T> {
    pub fn replace(&mut self, old: &T, new: T) {
        self.array.iter_mut().filter(|x| *x == old).for_each(|v| *v = new.clone());
    }
}

impl Grid<u8> {
    pub fn from_ascii(s: &str) -> Self {
        let w = s.lines().next().unwrap().len();
        let a = s.lines().flat_map(|x| x.as_bytes()).copied().cv();
        Self::new(a, w)
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                eprint!("{}", self[y][x] as char);
            }
            eprintln!();
        }
    }
    pub fn print_ints(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                eprint!("{:4}", self[y][x]);
            }
            eprintln!();
        }
    }
}

pub trait GridIndex<T> {
    type Output: ?Sized;
    fn get(&self, index: T) -> Option<&Self::Output>;
    fn get_mut(&mut self, index: T) -> Option<&mut Self::Output>;
}

impl<T> GridIndex<usize> for Grid<T> {
    type Output = [T];
    fn get(&self, index: usize) -> Option<&[T]> {
        Some(self.array.get(index * self.width..)?.get(..self.width)?)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut [T]> {
        Some(
            self.array
                .get_mut(index * self.width..)?
                .get_mut(..self.width)?,
        )
    }
}

impl<T> GridIndex<P<usize>> for Grid<T> {
    type Output = T;
    fn get(&self, index: P<usize>) -> Option<&T> {
        Some(self.get(index.1)?.get(index.0)?)
    }

    fn get_mut(&mut self, index: P<usize>) -> Option<&mut T> {
        Some(self.get_mut(index.1)?.get_mut(index.0)?)
    }
}

impl<T> GridIndex<P<i64>> for Grid<T> {
    type Output = T;
    fn get(&self, index: P<i64>) -> Option<&T> {
        if index.0 < 0 || index.1 < 0 {
            None
        } else {
            Some(self.get(index.1 as usize)?.get(index.0 as usize)?)
        }
    }

    fn get_mut(&mut self, index: P<i64>) -> Option<&mut T> {
        if index.0 < 0 || index.1 < 0 {
            None
        } else {
            Some(self.get_mut(index.1 as usize)?.get_mut(index.0 as usize)?)
        }
    }
}

impl<I, T> Index<I> for Grid<T>
where
    Grid<T>: GridIndex<I>,
{
    type Output = <Grid<T> as GridIndex<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<I, T> IndexMut<I> for Grid<T>
where
    Grid<T>: GridIndex<I>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a [T];
    type IntoIter = ChunksExact<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.array.chunks_exact(self.width)
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut [T];
    type IntoIter = ChunksExactMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.array.chunks_exact_mut(self.width)
    }
}
