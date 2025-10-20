fn main() {
    // ===== 解构赋值 =====
    let (x, mut y): (bool, bool) = (true, false);
    println!("x = {}, y = {}", x, y);

    y = true;

    // 断言 x 和 y 相等, 否则程序崩溃
    assert_eq!(x, y);
    println!("x = {}, y = {}", x, y);

    // ===== 解构式赋值 =====
    let (a, b, c, d, e);
    (a, b) = (1, 2);

    // 数组/切片 模式匹配, 将 c 赋值为 1, d 赋值为 4
    // 中间的 .. 表示忽略中间的值, 所以数组中的 2 和 3 被忽略掉了
    // _ 表示忽略剩下的值
    [c, .., d, _] = [1, 2, 3, 4, 5];

    S { e } = S { e: 5 };

    println!("{:?}", (a, b, c, d, e));

    // ===== 常量 =====
    #[allow(dead_code)]
    const MAX_POINTS: u32 = 100_000;

    // ===== 变量遮蔽 =====
    let x = 5;
    let x = x + 1;

    {
        // 再次遮蔽, 作用域仅限花括号内
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    // 创建了一个新的同名变量, 遮蔽了之前的变量
    let spaces = "     ";
    let spaces = spaces.len();

    // 不能将 usize 类型的值赋值给 &str 类型的变量
    // usize 是无符号整数类型, 用于表示内存地址或集合的大小
    // let mut spaces = "     ";
    // spaces = spaces.len();

    println!("spaces len = {}", spaces);
}

struct S {
    e: i32,
}
