pub fn main() {
    let x = (500, 6.4, 1);
    printTuple();
}

// 打印元组
fn printTuple() {
    let my_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);

    // 最多 打印 12 个 元素
    println!("Tuple elements: {:#?}", my_tuple);
}