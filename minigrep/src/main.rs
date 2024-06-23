//引入env包，可以使用env::args方法读取并分析传入的命令行参数
//最终通过collect方法输出一个集合类型Vector
use std::env;
use std::process;
//引入操作lib
use minigrep::Config;

fn main() {
    // let args:Vec<String> = env::args().collect();
    // let config = parse_config(&args);
    //将错误信息重定向到stderr，将println!替换为eprintln！
    let config = Config::new(env::args()).unwrap_or_else(
        |err|{
            eprintln!("problem parsing arguments:{err}");
            process::exit(1);
    });
    //尝试打印参数
    //print!("search for {}",config.query);
    //println!(" In file {}",config.file_path);
    //dbg!(args);
    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
// fn parse_config(args:&Vec<String>) -> Config{
//     //存储文件路径和带搜索的字符串
//     //这里索引分别为1和2是因为默认会有一个参数值
//     //例如target\\debug\\minigrep.exe，这是值程序的可执行路径名
//     let query = args[1].to_string();
//     let file_path = args[2].to_string();
//     Config{query,file_path}
// }