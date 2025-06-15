use std::env;

fn main() {
    // 벡터 타입 데이터로 변수 선언
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    
    // let query =  &args[1];

    // let filename = &args[2];

    // 한글로 변경시도
    println!("검색: {}", config.query);
    println!("대상 파일 : {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("파일을 읽지 못했습니다.");
}

struct Config {
    query: String,
    filename: String
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {query, filename}
}