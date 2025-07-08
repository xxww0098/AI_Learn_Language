// 07 Rust结构体 - 定义和使用结构体
// 本章介绍如何定义和使用结构体来组织相关数据

// 定义基本结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 单元结构体（没有字段）
struct AlwaysEqual;

fn main() {
    // 创建结构体实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
    
    // 结构体更新语法
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // 使用user1的其他字段
    };
    
    println!("用户2的用户名: {}", user2.username);
    
    // 使用构造函数
    let user3 = build_user(String::from("user3@example.com"), String::from("user3"));
    println!("用户3的邮箱: {}", user3.email);
    
    // 元组结构体示例
    tuple_struct_example();
    
    // 矩形面积计算示例
    rectangle_example();
}

// 案例1：构造函数和方法
fn build_user(email: String, username: String) -> User {
    User {
        email,      // 字段初始化简写
        username,   // 字段初始化简写
        active: true,
        sign_in_count: 1,
    }
}

fn tuple_struct_example() {
    println!("\n=== 元组结构体示例 ===");
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("黑色: ({}, {}, {})", black.0, black.1, black.2);
    println!("原点: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // 单元结构体
    let subject = AlwaysEqual;
}

// 案例2：矩形面积计算
#[derive(Debug)]  // 派生Debug trait，允许打印
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数（构造函数）
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // 方法：计算面积
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 方法：判断是否为正方形
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    // 方法：判断是否能包含另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 可变方法：调整大小
    fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }
}

fn rectangle_example() {
    println!("\n=== 矩形示例 ===");
    
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    
    println!("rect1: {:?}", rect1);
    println!("rect1的面积: {}", rect1.area());
    println!("rect1是正方形吗？{}", rect1.is_square());
    
    println!("rect1能包含rect2吗？{}", rect1.can_hold(&rect2));
    println!("rect1能包含rect3吗？{}", rect1.can_hold(&rect3));
    
    // 可变矩形
    let mut rect4 = Rectangle::new(20, 20);
    println!("修改前: {:?}", rect4);
    rect4.resize(25, 30);
    println!("修改后: {:?}", rect4);
}

// 人员信息结构体
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

impl Person {
    // 构造函数
    fn new(name: String, age: u32, email: String) -> Person {
        Person { name, age, email }
    }
    
    // 获取年龄组
    fn age_group(&self) -> &str {
        match self.age {
            0..=12 => "儿童",
            13..=19 => "青少年",
            20..=59 => "成人",
            60.. => "老年人",
        }
    }
    
    // 生日（增加年龄）
    fn birthday(&mut self) {
        self.age += 1;
    }
    
    // 获取问候语
    fn greeting(&self) -> String {
        format!("你好，我是{}，今年{}岁", self.name, self.age)
    }
}

// 银行账户结构体
#[derive(Debug)]
struct BankAccount {
    account_number: String,
    balance: f64,
    owner: String,
}

impl BankAccount {
    // 创建新账户
    fn new(account_number: String, owner: String) -> BankAccount {
        BankAccount {
            account_number,
            balance: 0.0,
            owner,
        }
    }
    
    // 存款
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("存款 {:.2} 元，余额: {:.2} 元", amount, self.balance);
        }
    }
    
    // 取款
    fn withdraw(&mut self, amount: f64) -> bool {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!("取款 {:.2} 元，余额: {:.2} 元", amount, self.balance);
            true
        } else {
            println!("取款失败：余额不足或金额无效");
            false
        }
    }
    
    // 查询余额
    fn get_balance(&self) -> f64 {
        self.balance
    }
}

fn person_example() {
    println!("\n=== 人员信息示例 ===");
    
    let mut person = Person::new(
        String::from("张三"),
        25,
        String::from("zhangsan@example.com"),
    );
    
    println!("{:?}", person);
    println!("{}", person.greeting());
    println!("年龄组: {}", person.age_group());
    
    person.birthday();
    println!("生日后: {:?}", person);
}

fn bank_account_example() {
    println!("\n=== 银行账户示例 ===");
    
    let mut account = BankAccount::new(
        String::from("123456789"),
        String::from("李四"),
    );
    
    println!("账户信息: {:?}", account);
    
    account.deposit(1000.0);
    account.withdraw(300.0);
    account.withdraw(800.0);  // 余额不足
    
    println!("当前余额: {:.2} 元", account.get_balance());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = build_user(
            String::from("test@example.com"),
            String::from("testuser"),
        );
        
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "testuser");
        assert_eq!(user.active, true);
        assert_eq!(user.sign_in_count, 1);
    }

    #[test]
    fn test_rectangle() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.area(), 200);
        assert_eq!(rect.is_square(), false);
        
        let square = Rectangle::new(5, 5);
        assert_eq!(square.is_square(), true);
    }

    #[test]
    fn test_person() {
        let mut person = Person::new(
            String::from("测试"),
            30,
            String::from("test@test.com"),
        );
        
        assert_eq!(person.age_group(), "成人");
        person.birthday();
        assert_eq!(person.age, 31);
    }

    #[test]
    fn test_bank_account() {
        let mut account = BankAccount::new(
            String::from("123"),
            String::from("测试用户"),
        );
        
        account.deposit(100.0);
        assert_eq!(account.get_balance(), 100.0);
        
        assert_eq!(account.withdraw(50.0), true);
        assert_eq!(account.get_balance(), 50.0);
        
        assert_eq!(account.withdraw(100.0), false);
        assert_eq!(account.get_balance(), 50.0);
    }

    #[test]
    fn test_examples() {
        tuple_struct_example();
        rectangle_example();
        person_example();
        bank_account_example();
    }
}

// 结构体要点总结：
// 1. 结构体用于组织相关数据
// 2. 使用struct关键字定义结构体
// 3. 可以定义方法和关联函数
// 4. 使用#[derive(Debug)]可以打印结构体
// 5. 方法的第一个参数是&self、&mut self或self
// 6. 关联函数不接受self参数，类似构造函数
// 7. 使用impl块为结构体实现方法