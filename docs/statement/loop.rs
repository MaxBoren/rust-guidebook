// loop
// while
// break 退出循环， continue, 跳过其余的迭代开始新的循环

fn main() {
    //  1+2+3...+100
    let mut sum = 0;
    let mut n = 1;
    loop {
        sum += n;
        n += 1;
        if n > 100 {
            break
        }
    }
    println!("sum = {}", sum);

    // 可用于重试某个任务
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };
    println!("result = {}", result);
}