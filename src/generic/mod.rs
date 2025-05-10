use std::{fmt::Display, ops::Add};

fn create_and_print<T>()
where
    T: From<i32> + Display,
{
    let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
    println!("a is: {}", a);
}

struct A; //单元结构体（Unit Struct）
struct S(A); // 元组结构体（Tuple Struct） 自带一个 pub A的属性
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn sum<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}

struct Point<T>{
    x:T,
    y:T
}

pub fn main() {
    create_and_print::<f64>();
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(32));
    generic(SGen::<i32>(32));

    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
