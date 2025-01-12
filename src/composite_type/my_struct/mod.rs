/*
 * @Author: xh20220630 haox84961@gmail.com
 * @Date: 2025-01-12 11:38:11
 * @LastEditors: xh20220630 haox84961@gmail.com
 * @LastEditTime: 2025-01-12 12:26:30
 * @FilePath: \rust-project\src\composite_type\my_struct\mod.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct File {
    name: String,
    data: Vec<u8>,
}
#[derive(Debug)]
struct Color(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// 需要实现 生命 周期
// struct LeftUser {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

pub fn main() {
    // 申明 一个 可变的 user 用户
    /*  let mut user = User {
         active: true,
         username: String::from("haox84961"),
         email: String::from("haox84961@gmail"),
         sign_in_count: 0,
     };

    user.username = String::from("xh20220630"); */

    /* let mut user1 = build_user(String::from("haox84961@gmail"), String::from("haox84961"));

    let mut user2 = build_user(String::from("2222@qq.com"), String::from("2222"));

    user1.active = false;

    let user3: User = User {
        email: String::from("another@example.com"),
        ..user2 // 后面 不用 跟 ,必须在结构体的尾部使用。 user 2 所有权 发生了 偏移，无法使用。
    }; // right

    let user4: User = User {
        email: String::from("another@example.com"),
        ..user1
    }; */

    /*  let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user1.active);

    println!("{}", user1.username);
    // 下面这行会报错
    println!("{:?}", user1.email); */

   /*  let mut f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let newStr = String::from("222222");

    let mut f1_name = &f1.name;
    let f1_length = &f1.data.len();

    f1_name = &newStr;

    println!("{:?}", f1.name);
    println!("{} is {} bytes long", f1_name, f1_length); */

    let color = Color(0, 0, 0);

    // println!("color is {}", color); // 实现 display 方法

    println!("color is {:?}", color); // 实现 debug 需要 添加 #[derive(Debug)]

    println!("color is {:#?}", color); // # 代表里面的属性换行输出 实现 debug 需要 添加 #[derive(Debug)]

    dbg!(color); // 添加了代码的 行号 属性 也 可以添加
}
