use std::error::Error; //以后补充
use std::fs; //文件操作模块

//Result泛型第一个参数()，表示不返回任何内容，但是最后得放到Ok里
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //?号代表不会发生panic，它会将错误值返回给调用者
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
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
