use std::fmt::Debug;

#[derive(Debug)]
pub struct Food<T>(pub T);

#[derive(Debug)]
pub struct Apple;

pub trait Eatable {
    fn eat(&self);
}

impl<T> Eatable for Food<T> where T: Debug {
    fn eat(&self) {
        println!("Eating {:?}", self);
    }
}

pub fn eat<T>(val: T) where T: Eatable {
    val.eat();
}