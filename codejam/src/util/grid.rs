use std::default::Default;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};
use crate::util::vector_2d::Vector2d;
use num_integer::Integer;
use num_traits::NumCast;

#[derive(Clone)]
pub struct Grid<T>
{
    pub data: Vec<T>,
    pub R: usize,
    pub C: usize,
}

pub type GridRowColVec = Vector2d<isize>;
pub type GridCoord = Vector2d<usize>;

//pub struct GridConsts {}

pub mod constants
{
    use super::*;

    pub const NORTH: GridRowColVec = Vector2d { data: [-1, 0] };
    pub const EAST: GridRowColVec = Vector2d { data: [0,1] };
    pub const SOUTH: GridRowColVec = Vector2d { data: [1, 0] };
    pub const WEST: GridRowColVec = Vector2d { data: [0, -1] };

    pub const DIRECTIONS: [Vector2d<isize>; 4] = [NORTH, EAST, SOUTH, WEST];


}

impl<T> Grid<T>
{
    pub fn new(r: usize, c: usize) -> Grid<T>
    where
        T: Default,
    {
        let mut g = Grid {
            R: r,
            C: c,
            data: Vec::new(),
        };
        for _ in 0..r * c {
            g.data.push(Default::default());
        }
        g
    }

    pub fn get_val_usize<'a>(&'a self, row: usize, col: usize)
        -> Option<&'a T>
    {
        if row >= self.R || col >= self.C  {
            return None;
        }

        Some(&self.data[row * self.C + col])
    }

    pub fn get_val<'a>(&'a self, row: isize, col: isize)
        -> Option<&'a T>
    {
        if row < 0 || col < 0 || row as usize >= self.R || col as usize >= self.C  {
            return None;
        }

        Some(&self.data[ row as usize * self.C + col as usize])
    }

    pub fn get_value<'a, N: Integer+Copy+NumCast>(&'a self, row_col_index: &Vector2d<N>)
        -> Option<&'a T>
    {
        if row_col_index.data[0] < N::zero() || row_col_index.data[1] < N::zero() {
            return None;
        }
        let row_col_index: Vector2d<usize> = row_col_index.convert();

        if row_col_index.data[0] >= self.R || row_col_index.data[1] >= self.C {
            return None;
        }

        Some(&self.data[row_col_index.data[0] * self.C + row_col_index.data[1]])
    }

    pub fn filter_by_val<'a>(&'a self, val: &'a T) -> impl Iterator<Item = GridCoord> + 'a
    where
        //I: 'a,
        T: PartialEq,
    {
        self.data
            .iter()
            .enumerate()
            .filter(move |(_index, value)| *value == val)
            .map(move |(index, _value)| Vector2d::with_val(index / self.C, index % self.C))
    }

    pub fn filter_by_pred<'a, P>(&'a self, predicate: P) -> impl Iterator<Item = GridCoord> + 'a
    where
        P: Fn(&T) -> bool + 'a,
        T: PartialEq,
    {
        self.data
            .iter()
            .enumerate()
            .filter(move |(_index, value)| predicate(*value))
            .map(move |(index, _value)| Vector2d::with_val(index / self.C, index % self.C))
    }

    pub fn iter_loc<'a>(&'a self) -> impl Iterator<Item = (GridCoord, &T)> + 'a
    where
        T: PartialEq,
    {
        self.data
            .iter()
            .enumerate()
            .map(move |(index, value)| (Vector2d::with_val(index / self.C, index % self.C), value))
    }

    pub fn transform<'a, P>(&'a mut self, transformer: P)
    where
        P: Fn((GridCoord, &'a mut T)) -> () + 'a,
        T: 'a,
    {
        //GridMutIterator{grid:self, cur_index:0}

        let C = self.C;

        for (index, value) in self.data.iter_mut().enumerate() {
            transformer((Vector2d::with_val(index / C, index % C), value));
        }
    }
}

//https://www.reddit.com/r/rust/comments/6ffrbs/implementing_a_safe_mutable_iterator/

/*
struct GridMutIterator<'b, T>
{
     grid: &'b mut Grid<T>,
    cur_index: usize
}

impl <'a, T> Iterator for GridMutIterator<'a, T> {
    type Item = (GridCoord, &'a mut T);

    fn next(&mut self) -> Option<(GridCoord, &'a mut T)>

    {

        if self.cur_index >= self.grid.data.len() {
            return None;
        }

        let index = self.cur_index;
        self.cur_index += 1;
        let coord = Vector2d(index / self.grid.C, index % self.grid.C);
        let v: &'a mut T = self.grid.data.get_mut(index).unwrap();
        Some( (coord, v ) )


    }
}*/

//get a cell
impl<T> Index<usize> for Grid<T>
{
    type Output = T;

    fn index(&'_ self, index: usize) -> &'_ T
    {
        &self.data[index]
    }
}
//get a cell
impl<T> Index<&Vector2d<usize>> for Grid<T>
{
    type Output = T;

    fn index<'a>(&'a self, row_col_index: &Vector2d<usize>) -> &'a T
    {
        /*if row_col_index.0 >= self.R || row_col_index.1 >= self.C {
            panic!(
                "RowCol {:?} invalid for grid {}, {}",
                row_col_index, self.R, self.C
            );
        }*/

        &self.data[ row_col_index.data[0] * self.C + row_col_index.data[1]  ]
    }
}
impl<T> Index<&Vector2d<i64>> for Grid<T>
{
    type Output = T;

    fn index<'a>(&'a self, row_col_index: &Vector2d<i64>) -> &'a T
    {
        /*if row_col_index.0 >= self.R || row_col_index.1 >= self.C {
            panic!(
                "RowCol {:?} invalid for grid {}, {}",
                row_col_index, self.R, self.C
            );
        }*/

        &self.data[ (row_col_index.data[0] * self.C as i64 + row_col_index.data[1] ) as usize ]
    }
}

impl<T> Index<&Vector2d<isize>> for Grid<T>
{
    type Output = T;

    fn index<'a>(&'a self, row_col_index: &Vector2d<isize>) -> &'a T
    {

        &self.data[ (row_col_index.data[0] * self.C as isize + row_col_index.data[1] ) as usize ]
    }
}
//set a cell
impl<T> IndexMut<&Vector2d<i64>> for Grid<T>
{
    fn index_mut<'a>(&'a mut self, row_col_index: &Vector2d<i64>) -> &'a mut T
    {
        &mut self.data[ (row_col_index.data[0] * self.C as i64 + row_col_index.data[1] ) as usize ]
    }
}
impl<T> IndexMut<&Vector2d<isize>> for Grid<T>
{
    fn index_mut<'a>(&'a mut self, row_col_index: &Vector2d<isize>) -> &'a mut T
    {
        &mut self.data[ (row_col_index.data[0] * self.C as isize + row_col_index.data[1] ) as usize ]
    }
}
impl<T> IndexMut<&Vector2d<usize>> for Grid<T>
{
    fn index_mut<'a>(&'a mut self, row_col_index: &Vector2d<usize>) -> &'a mut T
    {
        &mut self.data[ row_col_index.data[0] * self.C + row_col_index.data[1]  ]
    }
}
impl<T> Index<(usize, usize)> for Grid<T>
{
    type Output = T;

    fn index(&'_ self, row_col_index: (usize, usize)) -> &'_ T
    {
        &self.data[row_col_index.0 * self.C + row_col_index.1]
    }
}
//set a cell
impl<T> IndexMut<(usize, usize)> for Grid<T>
{
    fn index_mut(&'_ mut self, row_col_index: (usize, usize)) -> &'_ mut T
    {
        &mut self.data[row_col_index.0 * self.C + row_col_index.1]
    }
}
impl<T> IndexMut<usize> for Grid<T>
{
    fn index_mut(&'_ mut self, index: usize) -> &'_ mut T
    {
        &mut self.data[index]
    }
}

impl<T> Debug for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        //if formatter.alternate() {

        let spacing = match f.precision() {
            Some(precision) => precision,
            _ => 4,
        };

        let row_label_width = 5;

        //headers
        write!(f, "{:>width$} ", "", width = row_label_width).unwrap();

        for c in 0..self.C {
            write!(f, "{:>width$} |", format!("C{}", c), width = spacing - 1).unwrap();
        }
        writeln!(f,).unwrap();

        write!(f, "{:width$}+", "", width = row_label_width).unwrap();

        for _ in 0..self.C {
            write!(f, "{}+", "-".repeat(spacing),).unwrap();
        }
        writeln!(f).unwrap();

        for r in 0..self.R {
            write!(
                f,
                "{:>width$} |",
                format!("R{}", r),
                width = row_label_width - 1
            )
            .unwrap();

            for c in 0..self.C {
                write!(f, "{:>width$} |", self[(r, c)], width = spacing - 1).unwrap();
            }
            writeln!(f).unwrap();

            write!(f, "{:width$}+", "", width = row_label_width).unwrap();

            for _ in 0..self.C {
                write!(f, "{}+", "-".repeat(spacing),).unwrap();
            }
            writeln!(f).unwrap();
        }
        write!(f, "")
    }
}

/////////////////////////////
/// Grid coordinate methods & trait implementations

/*
impl<N: VecCoordIntegerTrait> Vector2d<N>
{

}*/


/*

impl<N: VecCoordIntegerTrait> Display for Vector2d<N>
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "(R{}, C{})", self.0, self.1)
    }
}*/

#[cfg(test)]
mod test_grid
{

    use self::super::*;

    use std::{i64, u64};


/*
    #[test]
    fn test_get_value()
    {
        let mut grid: Grid<char> = Grid::new(2, 2);
        grid[(0, 0)] = 'a';
        grid[(1, 0)] = 'b';
        grid[(1, 1)] = 'd';

        assert_eq!(Some(&'d'), grid.get_value(Vector2d::with_val(1i16, 1)));
    }
*/
    #[test]
    fn test_get_dist()
    {
        assert_eq!(
            Vector2d::with_val(u64::MAX, 1).manhat_distance(&Vector2d::with_val(u64::MAX - 3, 10)),
            12
        );
        assert_eq!(
            Vector2d::with_val(u64::MAX - 4, 5).manhat_distance(&Vector2d::with_val(u64::MAX, 4)),
            5
        );

        assert_eq!(
            Vector2d::with_val(i64::MAX - 4, i64::MIN)
                .manhat_distance(&Vector2d::with_val(i64::MAX, i64::MIN + 5)),
            9
        );
    }
}
