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
    fn cs(self) -> BTreeSet<Self::Item> {
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
        if i == j { return; }
        if self.s[j] > self.s[i] {
            self.p[i] = j;
            self.s[j] += self.s[i];
        } else {
            self.p[j] = i;
            self.s[i] += self.s[j];
        }
    }
}
