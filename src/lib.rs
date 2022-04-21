use std::error::Error; //以后补充
use std::fs; //文件操作模块

//Result泛型第一个参数()，表示不返回任何内容，但是最后得放到Ok里
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //?号代表不会发生panic，它会将错误值返回给调用者
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    //取得一段输入数据的参数值 query、filename
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        //返回一个Config结构体实例
        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    //检查每行是否存在query这段内容
    //如果有的话就存放到results这个数组里
    //最后返回这个results这个集合就可以了
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        //query是测试用的搜索值
        //contents是要进行搜索的内容
        let query = "高性能";
        let contents = "\
Rust:
高性能 - Rust 速度惊人且内存利用率极高。由于没有运行时和垃圾回收，它能够胜任对性能要求特别高的服务，可以在嵌入式设备上运行，还能轻松和其他语言集成。";
        assert_eq!(vec!["高性能 - Rust 速度惊人且内存利用率极高。由于没有运行时和垃圾回收，它能够胜任对性能要求特别高的服务，可以在嵌入式设备上运行，还能轻松和其他语言集成。"], search(query, contents))
    }
}
