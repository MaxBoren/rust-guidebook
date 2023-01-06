// for range
// a..b 包含a，不包含b，步长为1的迭代器
// a..=b 包含a，包含b，步长为1的迭代器

fn main() {
    // 1
    for i in 0..5 {
        println!("{}", i);
    }
    for i in 0..=5 {
        println!("{}", i);
    }
    // for range
    let myarray = ["a","b","c"];
    for i in myarray.iter() {
        println!("{:?}", i);
    }
    let mut myarray1 = [1,2,3];
    for i in myarray1.iter_mut() {
        *i *= 2
    }
    for i in myarray1.iter() {
        println!("{:?}", i);
    }
}