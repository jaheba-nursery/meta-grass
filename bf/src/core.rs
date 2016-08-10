

use grass;

use std::cmp::PartialEq;

use std::ops::{Deref, DerefMut};

pub const TAPESIZE: usize = 2048;

pub struct Tape {
    pub tape: [usize; TAPESIZE],
    pub position: usize,
}



pub struct MyMergePointData<'core> {
    pub program: &'core Vec<usize>,
}

create_bind!(MyMergePoint for MyMergePointData<'core> where pc: usize, tape: Tape);


impl<'a, 'core> MyMergePoint<'a, 'core> {
    pub fn hash(&self) {
        let tape_address = self.tape as *const Tape;
    }
}

impl<'a, 'core> PartialEq for MyMergePoint<'a, 'core> {
    fn eq(&self, other: &Self) -> bool {
        self.pc == other.pc && self.program == other.program
    }
}

impl<'a, 'core> grass::MergePoint for MyMergePoint<'a, 'core> {

}