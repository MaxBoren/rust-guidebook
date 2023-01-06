// if-else
// 如果要返回一个值，那么需要返回相同的类型

fn main() {
    let n = 0;

    if n > 0 {
        println!("{} is positive", n)
    } else if n < 0 {
        println!("{} is negative", n)
    } else {
        println!("{} is zero", n)
    }

    // 作为表达式，返回相同类型
    // 表达式需要有赋值
    let m = if n < 0 {
        2.0
    } else {
        3.0
    };
    println!("{}", m)
}
