#[derive(Debug)]
pub struct Student {
    id: u8,
    name: String,
    age: u8,
    sex: i8
}

impl Student {
    // 创建一个学生实例
    pub fn new_with(name:&str, age:u8, sex:i8) -> Student {
        let student = Student {
            id: 1,
            name: name.to_string(),
            age,
            sex
        };
        student
    }

    pub fn display(self) {
        println!("student(id={}, name={}, age={}, sex={})", self.id, self.name, self.age, self.sex)
    }
}