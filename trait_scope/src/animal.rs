pub trait Eat {
    fn eat(&self) {
        println!("eating...");
    }
}

pub trait Code {
    fn code(&self) {
        println!("coding...");
    }
}

pub trait Sleep {
    fn sleep(&self) {
        println!("sleeping...");
    }
}

// 使用"+"组合实现多种特征
pub trait Programmer: Eat + Sleep + Code {
    fn animate(&self) {
        self.eat();
        self.code();
        self.sleep();
        println!("repeat!");
    }
}

pub struct Bob;

impl Programmer for Bob {}

impl Eat for Bob {}

impl Code for Bob {}

impl Sleep for Bob {}

