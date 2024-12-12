use std::{
    collections::BTreeSet,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub,
        SubAssign,
    }, slice::{ChunksExact, ChunksExactMut},
};

trait CollectVec: Iterator + Sized {
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
#[repr(C)]
struct P<T>(T, T);

macro_rules! impl_p {
    ($t:ty) => {
        impl P<$t> {
            fn au(self) -> P<usize> {
                P(self.0 as _, self.1 as _)
            }

            fn ai(self) -> P<i16> {
                P(self.0 as _, self.1 as _)
            }

            fn af(self) -> P<f64> {
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

impl_p!(i16);
impl_p!(f64);
impl_p!(usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
#[repr(u8)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn p(self) -> P<i16> {
        match self {
            Dir::U => P(0, -1),
            Dir::D => P(0, 1),
            Dir::L => P(-1, 0),
            Dir::R => P(1, 0),
        }
    }
    fn l(self) -> Dir {
        match self {
            Dir::U => Dir::L,
            Dir::D => Dir::R,
            Dir::L => Dir::D,
            Dir::R => Dir::U,
        }
    }
    fn r(self) -> Dir {
        match self {
            Dir::U => Dir::R,
            Dir::D => Dir::L,
            Dir::L => Dir::U,
            Dir::R => Dir::D,
        }
    }

    /// Reflect along positive slope
    fn rp(self) -> Dir {
        match self {
            Dir::U => Dir::R,
            Dir::D => Dir::L,
            Dir::L => Dir::D,
            Dir::R => Dir::U,
        }
    }
    /// Reflect along negative slope
    fn rn(self) -> Dir {
        match self {
            Dir::U => Dir::L,
            Dir::D => Dir::R,
            Dir::L => Dir::U,
            Dir::R => Dir::D,
        }
    }
    fn from_str(s: &str) -> Self {
        match s {
            "U" | "N" => Dir::U,
            "D" | "S" => Dir::D,
            "L" | "W" => Dir::L,
            "R" | "E" => Dir::R,
            _ => unreachable!(),
        }
    }
    fn iter() -> impl Iterator<Item = Self> {
        [Self::U, Self::D, Self::L, Self::R].into_iter()
    }
}

impl Add<Dir> for P<i16> {
    type Output = Self;
    fn add(mut self, rhs: Dir) -> P<i16> {
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
        self.0 = (self.0 as i16 + rhs.p().0) as usize;
        self.1 = (self.1 as i16 + rhs.p().1) as usize;
        self
    }
}

#[derive(Clone)]
struct Grid<T> {
    array: Vec<T>,
    stride: usize,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn new(array: Vec<T>, width: usize, stride: usize) -> Self {
        let height = array.len() / stride;
        Self {
            array,
            width,
            stride,
            height,
        }
    }
    fn len(&self) -> usize {
        self.height
    }
    fn iter(&self) -> impl Iterator<Item = &[T]> {
        self.into_iter()
    }
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.into_iter()
    }
}

impl<T: Eq> Grid<T> {
    fn count(&self, value: &T) -> usize {
        self.array.iter().filter(|&x| x == value).count()
    }
}

impl<T: Eq + Clone> Grid<T> {
    fn replace(&mut self, old: &T, new: T) {
        self.array.iter_mut().filter(|x| *x == old).for_each(|v| *v = new.clone());
    }
}

impl Grid<u8> {
    fn from_ascii(s: &str) -> Self {
        let w = s.split_once('\n').unwrap().0.len();
        let a = s.as_bytes().to_vec();
        Self::new(a, w, w+1)
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                eprint!("{}", self[y][x] as char);
            }
            eprintln!();
        }
    }
    fn print_ints(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                eprint!("{:4}", self[y][x]);
            }
            eprintln!();
        }
    }
}

trait GridIndex<T> {
    type Output: ?Sized;
    fn get(&self, index: T) -> Option<&Self::Output>;
    fn get_mut(&mut self, index: T) -> Option<&mut Self::Output>;
    unsafe fn get_unchecked(&self, index: T) -> &Self::Output;
    unsafe fn get_unchecked_mut(&mut self, index: T) -> &mut Self::Output;
}

impl<T> GridIndex<usize> for Grid<T> {
    type Output = [T];
    fn get(&self, index: usize) -> Option<&[T]> {
        Some(self.array.get(index * self.stride..)?.get(..self.width)?)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut [T]> {
        Some(
            self.array
                .get_mut(index * self.stride..)?
                .get_mut(..self.width)?,
        )
    }

    unsafe fn get_unchecked(&self, index: usize) -> &[T] {
        self.array.get_unchecked(index * self.stride..).get_unchecked(..self.width)
    }

    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut [T] {
        self.array
            .get_unchecked_mut(index * self.stride..)
            .get_unchecked_mut(..self.width)
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

    unsafe fn get_unchecked(&self, index: P<usize>) -> &T {
        self.get_unchecked(index.1).get_unchecked(index.0)
    }

    unsafe fn get_unchecked_mut(&mut self, index: P<usize>) -> &mut T {
        self.get_unchecked_mut(index.1).get_unchecked_mut(index.0)
    }
}

impl<T> GridIndex<P<i16>> for Grid<T> {
    type Output = T;
    fn get(&self, index: P<i16>) -> Option<&T> {
        if index.0 < 0 || index.1 < 0 {
            None
        } else {
            Some(self.get(index.1 as usize)?.get(index.0 as usize)?)
        }
    }

    fn get_mut(&mut self, index: P<i16>) -> Option<&mut T> {
        if index.0 < 0 || index.1 < 0 {
            None
        } else {
            Some(self.get_mut(index.1 as usize)?.get_mut(index.0 as usize)?)
        }
    }

    unsafe fn get_unchecked(&self, index: P<i16>) -> &T {
        self.get_unchecked(index.1 as usize).get_unchecked(index.0 as usize)
    }

    unsafe fn get_unchecked_mut(&mut self, index: P<i16>) -> &mut T {
        self.get_unchecked_mut(index.1 as usize).get_unchecked_mut(index.0 as usize)
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
        self.array.chunks_exact(self.stride)
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut [T];
    type IntoIter = ChunksExactMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.array.chunks_exact_mut(self.stride)
    }
}

pub fn run(input: &str) -> impl std::fmt::Display {
    let mut g = Grid::from_ascii(input);

    // let mut h = vec![
    //     Vec::<P<i16>>::with_capacity(384),
    //     Vec::<P<i16>>::with_capacity(384),
    //     Vec::<P<i16>>::with_capacity(384),
    //     Vec::<P<i16>>::with_capacity(384),
    //     Vec::<P<i16>>::with_capacity(384),
    //     Vec::<P<i16>>::with_capacity(384),
    //     Vec::<P<i16>>::with_capacity(384),
    //     Vec::<P<i16>>::with_capacity(384),
    //     Vec::<P<i16>>::with_capacity(384),
    //     Vec::<P<i16>>::with_capacity(384),
    // ];
    let mut h = [[P(0, 0); 512]; 10];
    let mut l = [0; 10];
    let width = g.width;
    let height = g.height;
    for y in 0..g.height {
        for x in 0..g.width {
            unsafe {
                let c = g.get_unchecked_mut(P(x, y));
                let b = (*c - b'0') as usize;
                h.get_unchecked_mut(b)[*l.get_unchecked(b)] = P(x as i16, y as i16);
                *l.get_unchecked_mut(b) += 1;
                *c = (*c == b'0') as u8;
            }
        }
    }

    let mut g2 = Grid::new(vec![0; g.width*g.height], g.width, g.width);
    for (i, w) in h.windows(2).enumerate() {
        let pre = &w[0][..l[i]];
        let v = &w[1][..l[i+1]];
        for &p in v {
            for d in Dir::iter() {
                if let Some(&v) = g.get(p+d) {
                    unsafe { *g2.get_unchecked_mut(p) += v; }
                }
            }
        }
        for &p in pre {
            unsafe { *g.get_unchecked_mut(p) = 0; }
        }
        (g, g2) = (g2, g);
    }
    h[9][..l[9]].iter().map(|p| unsafe { *g.get_unchecked(*p) } as u32).sum::<u32>()
}
