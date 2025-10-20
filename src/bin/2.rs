fn main() {
    // let 声明的变量不可变
    // 这里的 a 没有制定类型, 编译器会自动推断为 i32, 即有符号 32 位整数
    let a = 10;

    // 显示指定类型为 i32
    let b: i32 = 20;

    // 数值为 30, 类型为 i32
    // mut 是可变的, 此处会显示警告, 因为 c 没有被修改过
    #[allow(unused_mut)] // 此处忽略了警告
    let mut c = 30i32;

    // 或者在数值和类型之间加一个下划线
    let d = 30_i32;

    let e = add(add(a, b), add(c, d));
    println!("{}", e);
}

fn add(i: i32, j: i32) -> i32 {
    // 省略 return
    i + j
}
