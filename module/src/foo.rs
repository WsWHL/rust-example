/**
    定义模块: 创建公有结构体及关联方法
    pub: 标记为公有元素，默认私有
 */

#[derive(Debug)]
pub struct Bar {
    pub id:u8,
    pub name:String,
}

// 私有结构体，无法被外部调用
struct Private;

impl Bar {
    pub fn init() {
        println!("Bar type initialized");
    }

    pub fn display(self) {
        println!("id: {}, name: {}", self.id, self.name)
    }
}