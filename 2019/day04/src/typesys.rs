use std::marker::PhantomData as Pd;

// numbers are little endian, outside most is least significant

struct N0<T>(Pd<T>);
struct N1<T>(Pd<T>);
struct N2<T>(Pd<T>);
struct N3<T>(Pd<T>);
struct N4<T>(Pd<T>);
struct N5<T>(Pd<T>);
struct N6<T>(Pd<T>);
struct N7<T>(Pd<T>);
struct N8<T>(Pd<T>);
struct N9<T>(Pd<T>);

struct End;

trait Wr<T> {
    type Wr;
}

impl<T> Wr<T> for N0<End> { type Wr = N0<T>; }
impl<T> Wr<T> for N1<End> { type Wr = N1<T>; }
impl<T> Wr<T> for N2<End> { type Wr = N2<T>; }
impl<T> Wr<T> for N3<End> { type Wr = N3<T>; }
impl<T> Wr<T> for N4<End> { type Wr = N4<T>; }
impl<T> Wr<T> for N5<End> { type Wr = N5<T>; }
impl<T> Wr<T> for N6<End> { type Wr = N6<T>; }
impl<T> Wr<T> for N7<End> { type Wr = N7<T>; }
impl<T> Wr<T> for N8<End> { type Wr = N8<T>; }
impl<T> Wr<T> for N9<End> { type Wr = N9<T>; }

trait ContainsDouble {
    const N: usize;
}

trait Max<T> {
    type Max;
}

macro_rules! impl_contains_double {
    ($($match:tt),*
     ===
     $($nonmatch1:tt : [ $($nonmatch2:tt),* ])*
    ) => {
        $(
            impl<T> ContainsDouble for $match<$match<T>> {
                const N: usize = 1;
            }
            impl ContainsDouble for $match<End> {
                const N: usize = 0;
            }
        )*
        $($(
            impl<T> ContainsDouble for $nonmatch1<$nonmatch2<T>>
            where
                $nonmatch2<T>: ContainsDouble,
            {
                const N: usize = <$nonmatch2<T> as ContainsDouble>::N;
            }
        )*)*
    }
}

impl_contains_double! {
    N0, N1, N2, N3, N4, N5, N6, N7, N8, N9
    ===
    N0: [N1, N2, N3, N4, N5, N6, N7, N8, N9]
    N1: [N0, N2, N3, N4, N5, N6, N7, N8, N9]
    N2: [N0, N1, N3, N4, N5, N6, N7, N8, N9]
    N3: [N0, N1, N2, N4, N5, N6, N7, N8, N9]
    N4: [N0, N1, N2, N3, N5, N6, N7, N8, N9]
    N5: [N0, N1, N2, N3, N4, N6, N7, N8, N9]
    N6: [N0, N1, N2, N3, N4, N5, N7, N8, N9]
    N7: [N0, N1, N2, N3, N4, N5, N6, N8, N9]
    N8: [N0, N1, N2, N3, N4, N5, N6, N7, N9]
    N9: [N0, N1, N2, N3, N4, N5, N6, N7, N8]
}

macro_rules! impl_max {
    ($($max:tt > $($other:tt),*;)*) => {
        $($(
            impl Max<$max<End>> for $other<End> {
                type Max = $max<End>;
            }
            impl Max<$other<End>> for $max<End> {
                type Max = $max<End>;
            }
        )*)*
    }
}

impl_max! {
    N9 > N0, N1, N2, N3, N4, N5, N6, N7, N8;
    N8 > N0, N1, N2, N3, N4, N5, N6, N7;
    N7 > N0, N1, N2, N3, N4, N5, N6;
    N6 > N0, N1, N2, N3, N4, N5;
    N5 > N0, N1, N2, N3, N4;
    N4 > N0, N1, N2, N3;
    N3 > N0, N1, N2;
    N2 > N0, N1;
    N1 > N0;
}

impl<T> Max<T> for T { type Max = T; }

trait Repr {
    const REPR: usize;
}

impl Repr for End { const REPR: usize = 0; }
impl<T: Repr> Repr for N0<T> { const REPR: usize = T::REPR * 10; }
impl<T: Repr> Repr for N1<T> { const REPR: usize = T::REPR * 10 + 1; }
impl<T: Repr> Repr for N2<T> { const REPR: usize = T::REPR * 10 + 2; }
impl<T: Repr> Repr for N3<T> { const REPR: usize = T::REPR * 10 + 3; }
impl<T: Repr> Repr for N4<T> { const REPR: usize = T::REPR * 10 + 4; }
impl<T: Repr> Repr for N5<T> { const REPR: usize = T::REPR * 10 + 5; }
impl<T: Repr> Repr for N6<T> { const REPR: usize = T::REPR * 10 + 6; }
impl<T: Repr> Repr for N7<T> { const REPR: usize = T::REPR * 10 + 7; }
impl<T: Repr> Repr for N8<T> { const REPR: usize = T::REPR * 10 + 8; }
impl<T: Repr> Repr for N9<T> { const REPR: usize = T::REPR * 10 + 9; }

trait Incr {
    type Incr: Incr;
}

impl<T: Incr> Incr for N0<T> { type Incr = N1<T>; }
impl<T: Incr> Incr for N1<T> { type Incr = N2<T>; }
impl<T: Incr> Incr for N2<T> { type Incr = N3<T>; }
impl<T: Incr> Incr for N3<T> { type Incr = N4<T>; }
impl<T: Incr> Incr for N4<T> { type Incr = N5<T>; }
impl<T: Incr> Incr for N5<T> { type Incr = N6<T>; }
impl<T: Incr> Incr for N6<T> { type Incr = N7<T>; }
impl<T: Incr> Incr for N7<T> { type Incr = N8<T>; }
impl<T: Incr> Incr for N8<T> { type Incr = N9<T>; }
impl<T: Incr> Incr for N9<T> { type Incr = N0<T::Incr>; }

impl Incr for End { type Incr = N1<End>; }

trait FirstIncreasing { // get the lowest number higher than Self that has increasing digits
    type FirstIncr;
    type MinRq;
}

trait LastIncreasing { // get the highest number lower than Self that has increasing digits
    type LastIncr;
}

impl<T> FirstIncreasing for N0<End> { type FirstIncr = Self; type MinRq = Self; }
impl<T> FirstIncreasing for N1<End> { type FirstIncr = Self; type MinRq = Self; }
impl<T> FirstIncreasing for N2<End> { type FirstIncr = Self; type MinRq = Self; }
impl<T> FirstIncreasing for N3<End> { type FirstIncr = Self; type MinRq = Self; }
impl<T> FirstIncreasing for N4<End> { type FirstIncr = Self; type MinRq = Self; }
impl<T> FirstIncreasing for N5<End> { type FirstIncr = Self; type MinRq = Self; }
impl<T> FirstIncreasing for N6<End> { type FirstIncr = Self; type MinRq = Self; }
impl<T> FirstIncreasing for N7<End> { type FirstIncr = Self; type MinRq = Self; }
impl<T> FirstIncreasing for N8<End> { type FirstIncr = Self; type MinRq = Self; }
impl<T> FirstIncreasing for N9<End> { type FirstIncr = Self; type MinRq = Self; }

impl<T: FirstIncreasing> FirstIncreasing for N0<T> {
    type FirstIncr = <Self::MinRq as Wr<T::FirstIncr>>::Wr;
    type MinRq = <T::MinRq as Max<N0<End>>>::Max;
}

fn main() {

    println!("{}", <N9<N9<N9<End>>> as Incr>::Incr::REPR)

}
