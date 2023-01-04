// 数组
// 固定长度,一致类型
fn main() {
    let myarray = [1, 2, 3, 4, 5];

    // unwrap parse失败,程序会奔溃
    let index = "4".parse::<usize>().unwrap();
    println!("myarray[1]= {}", myarray[1]);
    println!("myarray[5]= {}", myarray[index]);

    // 多个相同数组初始化
    let mut mybuffers:[u32; 32*1024] = [0; 32*1024];
    println!("mybuffers[1024]{{}}= {:?}", mybuffers[1024]);

    // 修改其中的元素
    mybuffers[1024] = 13;
    println!("mybuffers[1024]= {}", mybuffers[1024]);
}
