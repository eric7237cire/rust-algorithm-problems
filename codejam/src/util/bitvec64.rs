//use std::ops::Index;
use std::iter::FromIterator;

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

    pub fn getb(&self, index: usize) -> bool
    {
        (self.data >> index) & 1 > 0
    }

    fn new() -> BitVec64
    {
        BitVec64 { data: 0 }
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