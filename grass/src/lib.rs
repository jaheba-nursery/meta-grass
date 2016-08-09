
use std::cmp::PartialEq;

pub trait LoopIdentifier: PartialEq {
}

pub trait MergePoint<Id: LoopIdentifier> {
    fn loop_identifier(&self) -> &Id;
}


pub fn merge_point<Id>(mp: &mut MergePoint<Id>) {

}