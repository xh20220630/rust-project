/*
 * @Author: xh20220630 haox84961@gmail.com
 * @Date: 2024-11-21 22:40:34
 * @LastEditors: xh20220630 haox84961@gmail.com
 * @LastEditTime: 2024-11-21 23:28:46
 * @FilePath: \rust-project\world_hello\src\ownership\mod.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// rust 当中的所有权

pub fn ownership() {
    // 具有所有权的 str
    let s1: String = String::from("hello");
    let x: &str = "hello, world";
    let y = x;
    println!("{},{},{}", s1, x, y);

    // false
    println!("{}", x == s1);

    // true
    println!("{}",x == y);

    let mut clone_s1 = s1.clone();
    println!("{}",clone_s1 == s1);

    // 值传递 前 拷贝 一份
    let clone_s2 = clone_s1.clone();

    println!("{}",clone_s1 == clone_s2);
    // 值传递
    let new_str = append_str(clone_s1);

    // 后面 就无法使用 clone_s1 了
    // clone_s1.push_str("world2");
    // println!("{}",clone_s1);


    println!("{}",new_str == clone_s2);


    // 引用
    let mut str = String::from("引用的值");
    let mut_str = append_reference_str(&mut str);
    // 不可以 继续 访问
    // let new_mut_str = append_reference_str(&mut str);

    // 块作用域 可以 继续 访问
    {
        let new_mut_str = append_reference_str(&mut str);
    }
    // 不可以 继续 访问
    // append_str(str);
    // 可以 继续 访问
    println!("{}",str);

}

fn append_str(mut str: String) -> String {
    str.push_str("world");
    println!("{}",str);

    return str;
}



fn append_reference_str(str: &mut String) -> &String {
    str.push_str("world");
    println!("{}",str);

    return str;
}
