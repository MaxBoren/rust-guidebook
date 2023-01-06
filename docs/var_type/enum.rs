// 枚举
// 无参数的枚举, 带枚举值的枚举 和 带参数的枚举 
// 模式匹配, 通常和match一起使用

enum IPAddr {
    IPv4(u8, u8, u8, u8),
    IPv6(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
}
fn main() {
    let localhost: IPAddr = IPAddr::IPv4(127, 0, 0, 1);

    match localhost {
        IPAddr::IPv4((a), (b), (c), (d)) => {
            println!("{} {} {} {}", a, b, c, d);
        }
        // 不是ipv4,匹配不到走这个分支
        _ => {}
    }
}
