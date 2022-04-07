/**
    mod: 声明模块
    use: 引入要使用的模块结构体信息
*/
mod foo;
mod user;

use foo::Bar;
// 目录模块结构体导入
use crate::user::student::Student;

fn main() {
    // 调用模块中结构体的方法
    foo::Bar::init();
    Bar::init();

    // 创建模块中结构体实例
    let bar = Bar{id:1, name:"test".to_string()};
    bar.display();

    let student = Student::new_with("hello", 28, -1);
    student.display()
}
