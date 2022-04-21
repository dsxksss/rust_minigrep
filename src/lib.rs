use std::env;
use std::error::Error; //以后补充
use std::fs; //文件操作模块 //环境变量操作模块

//Result泛型第一个参数()，表示不返回任何内容，但是最后得放到Ok里
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //?号代表不会发生panic，它会将错误值返回给调用者
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}
pub struct Config {
    pub query: String,        //要搜索的值
    pub filename: String,     //要搜索内容的文件名
    pub case_sensitive: bool, //用于是否启用区分大小写搜索功能
}

impl Config {
    //取得一段输入数据的参数值 query、filename
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        //读取环境变量,var里的参数是环境变量的名称,
        //var函数返回的是一个Result，is_err函数是判断是否发生了错误，
        //如果发生了错误就返回false，没有发生错误则返回true
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        //返回一个Config结构体实例
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

//查找内容相同的文字片段并返回一个字符串集合(区分大小写)
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

//查找内容相同的文字片段并返回一个字符串集合(不区分大小写)
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    //转换成小写内容再去搜索,这里新创建的query是拥有所有权的
    let query = query.to_lowercase();

    //检查每行是否存在query这段内容
    //如果有的话就存放到results这个数组里
    //最后返回这个results这个集合就可以了
    for line in contents.lines() {
        //相应的也要先转换成小写再去判断,这里不修改原有内容
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        //query是测试用的搜索值
        //contents是要进行搜索的内容
        let query = "rust";
        let contents = "\
Rust:
rust高性能 它能够胜任对性能要求特别高的服务
Rust还有其他的高性能功能可供开发者使用";
        //对比搜索到的一段或多段内容是否相等
        assert_eq!(
            vec!["rust高性能 它能够胜任对性能要求特别高的服务",],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        //query是测试用的搜索值
        //contents是要进行搜索的内容
        let query = "rUsT";
        let contents = "\
RuSt:
Rust高性能 它能够胜任对性能要求特别高的服务
还有其他的高性能功能可供rust开发者使用";
        //对比搜索到的一段或多段内容是否相等
        assert_eq!(
            vec![
                "RuSt:",
                "Rust高性能 它能够胜任对性能要求特别高的服务",
                "还有其他的高性能功能可供rust开发者使用"
            ],
            search_case_insensitive(query, contents)
        )
    }
}
