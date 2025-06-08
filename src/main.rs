use std::env;

fn main() {
    // 벡터 타입 데이터로 변수 선언
    let args: Vec<String> = env::args().collect();
    
    let query =  &args[1];

    let filename = &args[2];

    println!("Searching: {}", query);
    println!("FileName is : {}", filename);
}
