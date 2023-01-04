// 结构体 struct
// rust 多编程范式, 面向对象,函数shi,过程
// 结构体不是对象,结构体仅仅是数据的集合.而没有算法
// 三种类型的结构 1. 元组结构 2. 经典的C结构 3. 无字段的单元结构
// 结构体使用驼峰命名

// 派生属性 编译时的语法,会在编译时工作
#[derive(Debug)] 
// 元组结构
struct Pair(i32, f32);
// 经典的C结构
#[derive(Debug)]
struct Person {
    name: String,
    age: u32, //不会是负数
}
// 无字段的单元结构, 在泛型中经常使用
#[derive(Debug)]
struct Unit;
fn main() {
    // 初始化
    let pair = Pair(10, 4.2);
    // struct.member
    println!("{}", pair.0);
    
    let jack = Person {
        name: String::from("jack"),
        age: 7,
    };
    println!("name={} age={}", jack.name, jack.age);

    let unit = Unit;
    // 打印所有结构 需要派生
    println!("jack={:?}", jack);
    println!("unit={:?}", unit);
    // X print 是一个宏,不能直接实现

    
}
