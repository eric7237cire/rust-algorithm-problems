//use std::ops::Index;
use std::iter::FromIterator;
use std::fmt;
use std::ops::BitOrAssign;

#[derive(Default, Copy, Clone)]
pub struct BitVec64
{
    pub data: usize
}

impl BitVec64
{
    pub fn set(&mut self, index: usize, val: bool)
    {
        if val {
            self.data |= 1 << index;
        } else {
            self.data &= !(1 << index);
        }
    }

    pub fn get(self, index: usize) -> bool
    {
        (self.data >> index) & 1 > 0
    }

    pub fn new() -> BitVec64
    {
        BitVec64 { data: 0 }
    }

    pub fn with_val( data: usize ) -> BitVec64
    {
        BitVec64 { data }
    }

    pub fn pop_count(self) -> u32
    {
        self.data.count_ones()
    }
}


impl FromIterator<char> for BitVec64 {
    fn from_iter<I: IntoIterator<Item=char>>(iter: I) -> Self {
        let mut c = BitVec64::new();

        let mut idx = 0;
        for i in iter {
            if i == '1' {
                c.set(idx, true);
            }
            idx += 1;
        }

        c
    }
}

impl FromIterator<bool> for BitVec64 {
    fn from_iter<I: IntoIterator<Item=bool>>(iter: I) -> Self {
        let mut c = BitVec64::new();

        let mut idx = 0;
        for i in iter {
            if i {
                c.set(idx, true);
            }
            idx += 1;
        }

        c
    }
}


impl fmt::Binary for BitVec64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //todo pass on other width parameters, fill char etc.
        write!(f, "{:b}", self.data)
    }
}


impl BitOrAssign for BitVec64 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data |= rhs.data;
    }
}
