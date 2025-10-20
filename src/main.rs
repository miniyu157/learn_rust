fn main() {
    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";

    let records = penguin_data.lines(); // 返回迭代器, 迭代器可以被 for 消耗

    for (i, record) in records.enumerate() { // .enumerate() 将迭代器变成带有索引的元祖
        if i == 0 || record.trim().is_empty() {
            continue;
        }
        // Vec 可伸缩集合, <_> 自动推断元素类型
        // split 依然返回迭代器
        // map 迭代器适配器, 对每个元素操作, 返回操作后的新迭代器 (类似 lambda??)
        // collect 消耗迭代器, 返回一个具体的集合类型
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        // 仅在 debug 模式下执行
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];

        // .parse() 返回 Result 枚举, 在这里使用 if...let 只匹配 ok
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
        // 如果想要匹配其他情况, 使用 match
        // match fields[1].parse::<f32>() {
        //     Ok(length) => println!("{}, {}cm", name, length),
        //     Err(_) => println!("{}, {}cm", name, record.trim()),
        // }
    }
}
