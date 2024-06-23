//引入添加环境变量包，在env包中存在
use std::env;
use std::error::Error;
//引入文件操作包
use std::fs;

/**
 * 用于存放配置
 */
pub struct Config{
    pub query:String,
    pub file_path:String,
    //是否开启大小写敏感
    pub ignore_case:bool,
}
/**
 * 修改为使用迭代器
 */
impl Config{
    pub fn new(mut args:impl Iterator<Item = String>) -> Result<Config, &'static str>{
        //第一个参数不需要，空调用一次
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path= match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = match env::var("IGNORE_CASE"){
            Err(_) => matches!(args.next().as_deref(), Some("ignore_case") | Some("ignore")),
            Ok(arg) => arg == "1".to_string(), 
        };
        Ok(Config{query,file_path,ignore_case})
    }
}

//使用构造函数初始化一个Config实例,返回一个Result
// impl Config{
//     pub fn new(args:&Vec<String>) -> Result<Config, &'static str>{
//         //判断处理异常信息
//         if args.len() < 3{
//             //panic!("not enough arguments");
//             return Err("not enough arguments");
//         }
//         let query = args[1].to_string();
//         let file_path = args[2].to_string();
//         let ignore_case = match env::var("IGNORE_CASE"){
//             Err(_) => matches!(args.get(3).map(String::as_str), Some("ignore_case") | Some("ignore")),
//             Ok(arg) => arg == "1".to_string(), 
//         };
//         Ok(Config{query,file_path,ignore_case})
//     }
// }
/**
 * 获取文件中内容
 */
pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    // .expect("should have been able to read the file");
    //println!("text:\n{contents}");
    let results = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)  

    }else{
        search(&config.query, &contents)  
    };
    
    for line in results{
        println!("{line}");
    }
    Ok(())
}

// pub fn search<'a>(query:&str,contents:&'a str) -> Vec<&'a str>{
//     let mut result = Vec::new();
//     //lines方法将字符串按行分割，lines（）返回一个迭代器
//     for line in contents.lines(){
//         if line.contains(query){
//             result.push(line);
//         }
//     }
//     result
// }
// pub fn search_case_insensitive<'a>(query:&str,contents:&'a str) -> Vec<&'a str>{
//     let mut result:Vec<&str> = Vec::new();
//     //把query参数全部转换成小写
//     let query = query.to_lowercase();
//     for line in contents.lines(){
//         //同理转换为小写后再匹配是否存在
//         if line.to_lowercase().contains(&query){
//             result.push(line);
//         }
//     }
//     result
// }

/**
 * 迭代器版本
 */
pub fn search<'a>(query:&str,contents:&'a str) -> Vec<&'a str>{
    contents.lines().filter(|line| line.contains(query)).collect()
}
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    contents.lines().filter(|line|line.to_lowercase().contains(&query)).collect()
}


//添加测试
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    //大小写是否敏感测试用例
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

