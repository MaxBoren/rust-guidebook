
fn main() {
    let a: u32 = "4294967295".parse::<u32>().unwrap();
    let b: u32 = 1;

    // let sum = a + b;
    let (sum, is_overflow) = a.overflowing_add(b);
    print!("sum = {:?}, is_overflow = {:?}", sum, is_overflow);

}
