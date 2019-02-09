use num_integer::Integer;
use num_traits::{cast, NumCast};
//use std::cmp::PartialEq;
use std::default::Default;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Mul};
use std::ops::Sub;

//#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Vector2d<T>
{
    pub data: [T; 2]
}

pub trait VecCoordIntegerTrait: Hash + Integer + Display + NumCast + Copy + Mul + Add + AddAssign
{
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


impl<N:  Add<Output=N> + Copy> Add<Vector2d<N>> for Vector2d<N>
{
    type Output = Self;

    fn add(self, rhs: Vector2d<N>) -> Self
    {
        let lhs =  self;

        Vector2d::with_val(
            lhs.data[0] + rhs.data[0],
            lhs.data[1] + rhs.data[1],
        )
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

    //use std::{i64, u64};
    use crate::util::grid::constants::*;

    #[test]
    fn test_add()
    {
        assert_eq!(Vector2d::with_val(0i8, 2),
                   Vector2d::with_val(0i8, 3) + WEST.convert());
    }
}