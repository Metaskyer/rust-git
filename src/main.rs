use std::env;

fn main() {
    // 벡터 타입 데이터로 변수 선언
    let args: Vec<String> = env::args().collect();
    
    let query =  &args[1];

    let filename = &args[2];

    // 한글로 변경시도
    println!("검색: {}", query);
    println!("파일의 이름은 : {}", filename);
}
