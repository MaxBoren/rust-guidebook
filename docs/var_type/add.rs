fn avg(a: u32, b: u32)-> u32 {
    // 求平均值
    (a & b) + ((a ^ b) >> 1)
}

fn main() {
    assert_eq!(avg(322, 324), 323);
    assert_eq!(avg(0, 0), 0);
    assert_eq!(avg(10, 20), 15);
    print!("passed")
}
