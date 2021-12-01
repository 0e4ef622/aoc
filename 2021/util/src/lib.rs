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
        F: FnOnce(&mut Self) -> (),
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
    fn collectv(self) -> Vec<Self::Item> {
        self.collect()
    }
}

impl<I: Iterator> CollectVec for I {}
