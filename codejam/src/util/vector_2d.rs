use num_traits::{cast, NumCast};
use std::default::Default;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub, Rem, Div,Index, IndexMut};
use std::ops::Neg;

//#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct Vector2d<T>
{
    pub data: [T; 2]
}

impl<T> Vector2d<T>
{
    pub fn new() -> Vector2d < T >
    where
    T: Default,
    {
        Vector2d { data: [Default::default(), Default::default()] }
    }

    pub fn with_val(x: T, y: T) -> Vector2d < T >

    {
        Vector2d { data: [x, y] }
    }


}


impl<N: Display> Debug for Vector2d<N>
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "({}, {})", self.data[0], self.data[1])
    }
}

impl<T> Index<usize> for Vector2d<T>
{
    type Output = T;

    fn index(&self, index: usize) -> &T
    {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for Vector2d<T>
{
    fn index_mut(&'_ mut self, index: usize) -> &'_ mut T
    {
        &mut self.data[index]
    }
}

impl<T> Vector2d<T> where T: Copy
{
    pub fn r(&self) -> T
    {
        self.data[0]
    }
    pub fn c(&self) -> T
    {
        self.data[1]
    }
    pub fn x(&self) -> T
    {
        self.data[0]
    }
    pub fn y(&self) -> T
    {
        self.data[1]
    }
}

impl<T> Vector2d<T> where T: Neg<Output=T> + Copy
{
    pub fn rotate_rc_right(&self) -> Vector2d<T>
    {
        Vector2d{ data: [self.data[1], -self.data[0] ] }
    }
    pub fn rotate_rc_left(&self) -> Vector2d<T>
    {
        Vector2d{ data: [-self.data[1], self.data[0] ] }
    }
    pub fn rotate_rc_reverse(&self) -> Vector2d<T>
    {
        Vector2d{ data: [-self.data[0], -self.data[1] ] }
    }

    pub fn rotate_rc_right_mut(&mut self)
    {
        let r = self.data[0];
        let c = self.data[1];
        self.data[0] = c;
        self.data[1] = -r;


    }
    pub fn rotate_rc_left_mut(&mut self)
    {
        let r = self.data[0];
        let c = self.data[1];
        self.data[0] = -c;
        self.data[1] = r;
    }
    pub fn rotate_rc_reverse_mut(&mut self)
    {
        self.data[0] = -self.data[0];
        self.data[1] = -self.data[1];
    }
}


impl<T> Vector2d<T> where T: Rem<Output=T> + Div<Output=T> + Copy
{
    pub fn from_rowcol_index(row_col_index: T, C: T) -> Vector2d<T>
    {
        //row / col
        Vector2d { data: [row_col_index / C, row_col_index % C] }
    }
}

impl<T> Vector2d<T> where T: PartialOrd + Sub<Output=T> + Add<Output=T> + Copy
{
    pub fn manhat_distance(&self, rhs: &Self) -> T
    {
        let r = if self.data[0] > rhs.data[0] {
            self.data[0] - rhs.data[0]
        } else {
            rhs.data[0] - self.data[0]
        };
        let c = if self.data[1] > rhs.data[1] {
            self.data[1] - rhs.data[1]
        } else {
            rhs.data[1] - self.data[1]
        };
        r + c
    }
}

impl<T> Vector2d<T> where T: num_traits::cast::NumCast + Copy
{
    pub fn convert<M: NumCast>(&self) -> Vector2d<M>
    {
        Vector2d::with_val( cast::<T, M>(self.data[0]).unwrap(),
                            cast::<T, M>(self.data[1]).unwrap())
    }
}


impl<N: Add<Output=N> + Copy> Add<&Vector2d<N>> for Vector2d<N>
{
    type Output = Self;

    fn add(self, rhs: &Vector2d<N>) -> Self
    {
        let lhs =  self;

        Vector2d::with_val(
            lhs.data[0] + rhs.data[0],
            lhs.data[1] + rhs.data[1],
        )
    }
}

/*
impl<N: Copy + AddAssign> AddAssign<&Vector2d<N>> for &mut Vector2d<N>
{
    fn add_assign(&mut self, other: &Vector2d<N>)
    {
        self.data[0] += other.data[0];
        self.data[1] += other.data[1];
    }
}*/

impl<N: Copy + AddAssign> AddAssign<Vector2d<N>> for Vector2d<N>
{
    fn add_assign(&mut self, other: Vector2d<N>)
    {
        self.data[0] += other.data[0];
        self.data[1] += other.data[1];
    }
}


impl<N: Copy + AddAssign> AddAssign<&Vector2d<N>> for Vector2d<N>
{
    fn add_assign(&mut self, other: &Vector2d<N>)
    {
        self.data[0] += other.data[0];
        self.data[1] += other.data[1];
    }
}


impl<N: Copy + Mul<Output=N>> Mul<N> for Vector2d<N>
{
    type Output = Self;

    fn mul(self, rhs: N) -> Self
    {
        Vector2d::with_val(self.data[0] * rhs, self.data[1] * rhs)
    }
}



#[cfg(test)]
mod test_vector2d
{
    use self::super::*;
    use crate::util::grid::constants::*;

    #[test]
    fn test_add()
    {
        assert_eq!(Vector2d::with_val(0i8, 2),
                   Vector2d::with_val(0i8, 3) + &WEST.convert());
    }

    #[test]
    fn test_rotate()
    {
        assert_eq!(NORTH.rotate_rc_right(), EAST);
        assert_eq!(EAST.rotate_rc_right(), SOUTH);
        assert_eq!(SOUTH.rotate_rc_right(), WEST);
        assert_eq!(WEST.rotate_rc_right(), NORTH);

        assert_eq!(NORTH.rotate_rc_left(), WEST);
        assert_eq!(WEST.rotate_rc_left(), SOUTH);
        assert_eq!(SOUTH.rotate_rc_left(), EAST);
        assert_eq!(EAST.rotate_rc_left(), NORTH);


        assert_eq!(NORTH.rotate_rc_reverse(), SOUTH);
        assert_eq!(WEST.rotate_rc_reverse(), EAST);
        assert_eq!(EAST.rotate_rc_reverse(), WEST);
        assert_eq!(SOUTH.rotate_rc_reverse(), NORTH);

    }
}