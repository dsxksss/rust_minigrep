/*实现简单的一个命令行程序
1、接受命令行参数(完成)
2、读取文件(完成)
3、重构:改进模块和错误处理
4、使用TDD(测试驱动开发)开放Text库功能
5、使用环境变量
6、将错误消息写入标准错误而不是标准输出
*/

use std::{env, fs}; //导入 读取命令行参数的模块/文件操作模块

use std::process; //导入中止程序的exit函数模块

struct Config {
    query: String,
    filename: String,
}

impl Config {
    //取得一段输入数据的参数值 query、filename
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        //返回一个Config结构体实例
        Ok(Config { query, filename })
    }
}

fn main() {
    //返回读取到的命令行参数,是一个vec的string数组
    //第一个内容是这个文件的目录，后面的内容是用户输入的参数
    //参数间用空格分隔
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argunments: {}", err);
        process::exit(1); //这里的1表示程序退出的状态码，这个可以自定义的
    }); //这种闭包方法类似于javaScript里的箭头匿名函数一样

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file!");
    println!("With text:\n{}", contents);
}
