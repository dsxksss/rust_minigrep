/*实现简单的一个命令行程序
1、接受命令行参数(完成)
2、读取文件(完成)
3、重构:改进模块和错误处理
4、使用TDD(测试驱动开发)开放Text库功能
5、使用环境变量
6、将错误消息写入标准错误而不是标准输出
*/

use std::{env, fs}; //导入 读取命令行参数的模块/文件操作模块
fn main() {
    //返回读取到的命令行参数,是一个vec的string数组
    //第一个内容是这个文件的目录，后面的内容是用户输入的参数
    //参数间用空格分隔
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Search for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");
    println!("With text:\n{}", contents);
}
