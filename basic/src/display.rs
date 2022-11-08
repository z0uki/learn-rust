use std::fmt::{Display, Formatter};

// 添加Debug宏 自动实现Display trait
// #[derive(Debug)]
enum Sex {
    Man,
    Woman,
}

// #[derive(Debug)]
struct User {
    name: &'static str,
    age: u32,
    sex: Sex,
}

// User的关联函数
impl User {
    fn new(name: &'static str, age: u32, sex: Sex) -> Self {
        User{name,age,sex}
    }
}

// 手动实现Display trait
impl Display for Sex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Sex::Man => write!(f, "男"),
            Sex::Woman => write!(f, "女"),
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "user_name = {} user_age = {} user_sex = {}",
            self.name,
            self.age,
            self.sex,
        )
    }
}

pub fn run() {
    let s = Sex::Man;
    println!("{}", s);

    let u = User::new("sics",22, s);
    println!("{}", u);
}

#[test]
fn test() {
    let s = Sex::Man;
    println!("{}", s);

    let u = User::new("晚风",22, s);
    println!("{}", u);
}