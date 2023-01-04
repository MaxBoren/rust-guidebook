// 切片
// 数组的部分引用,未知长度
// 会修改原数组么? 会,是一个引用
fn main() {
    let mut arr:[i32;5] = [1, 2, 3, 4, 5];
    let slice = &arr[0..3]; // .. 是rust range语言, &是引用符号

    println!("slice[0]={}, length={}", slice[0], slice.len());

    // 取最后两个
    let slice2 = &arr[3..5]; // .. 是rust range语言, &是引用符号

    println!("slice[0]={}, length={}", slice2[0], slice2.len());

    // slice 内置函数
    // slice2.len();
    // slice2.is_empty();

    let slice3 = &mut arr[..];
    slice3[0] = 22;
    println!("slice3[0]={}", slice3[0]);
    println!("arr[0]={}", arr[0]);
}
