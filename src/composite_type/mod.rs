use std::ops::Add;

/**
 * @Author: xh20220630 haox84961@gmail.com
 * @Date: 2022-11-21 22:40:34
 * @LastEditors: xh20220630
 * @LastEditTime: 2022-11-21 23:28:46
 * @FilePath: /rust-project/world_hello/src/composite_type/mod.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
pub fn main() {
    /* let s1 = String::from("hello");
    // error 单独访问 会报错
    // let h = s1[0];
    let h = &s1[0..1]; */

    // append_str();

    // insert_str();

    /*   let string_replace: &str = "I like rust. Learning rust is my favorite! rust";
    // replace 全局 替换
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);

    // replacen 局部替换， 第三个 参数 为 替换多少次， 如果 不传和 replace 一样
    let new_string_replacen = string_replace.replace("rust", "RUST");
    dbg!(new_string_replacen);

    let hello = "hello".to_string();

    // let helloWorld =  String::add(hello, "world");

    let world = "world".to_string();
    let helloWorld = hello + &world;

    dbg!(helloWorld);


    let format_str_ame = "xionghao";
    let format_str_age = 22;
    let format_res_str = format!("{} is {} years old", format_str_ame, format_str_age);
    dbg!(format_res_str);

    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str); */

    // 字符串练习题
    string_test();

}

/* 追加字符串 */
fn append_str() {
    let mut s = String::from("Hello ");

    // 字符 类型
    let mut c = char::from('1');

    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);

    s.push('!');
    println!("追加字符 push() -> {}", s);
}

/* 插入字符串 */
fn insert_str() {
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);
}

fn string_test() {

    string_test_one();


    string_test_two();

    string_test_three();
}
    // 第一题
fn string_test_one() {
    let s: &str = "hello, world";
}
    // 第二题
fn string_test_two() {
    let s: Box<str> = "hello, world".into();

    // 第一种
    greetings(&s);

    // 第二种
    greetings(&s);
}

// 第三题
fn string_test_three(){
    let mut  s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    let s1:Box<str> = "ces".into(); 
    s += "!";

    dbg!(s1);

    println!("{}", s);
}


fn greetings(s: &str) {
    println!("{}", s)
}
