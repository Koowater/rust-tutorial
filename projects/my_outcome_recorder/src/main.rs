use std::io;
use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug)] // 이게 없으면 println!으로 출력할 수 없다!
struct Outcome {
    name: String,
    price: i32,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid input format".to_string());
        }

        let name = parts[0].to_string();
        let price = parts[1].parse::<i32>().map_err(|e| e.to_string())?;

        Ok(Outcome {
            name, price
        })
    }
}

fn get_outcome_from_cli() {
    let mut input_text = String::new();
    println!("지출을 다음 형식에 맞춰 입력해주세요. (지출명/지출비용)");
    
    io::stdin()
        .read_line(&mut input_text)
        .expect("Fail to read line in 'get_outcone_from_cli'");

    let outcome = Outcome::from_str(input_text.trim());
    println!("{:?}", outcome);

}

fn main() {
    println!("[지출 기록기] 시작");
    let mut job_choice = String::new();
    
    loop{
        println!(" - 실행할 작업을 선택하세요.\n    1) 지출 기록\n    2) 지출 조회");
        job_choice.clear();
        io::stdin()
            .read_line(&mut job_choice)
            .expect("Fail to read line"); // Err를 반환할 경우 expect 내 메세지를 출력
    
        
        if job_choice.trim().to_lowercase() == "exit" {
            println!("[지출 기록기] 중단, 안녕히가세요! 🫡");
            break;
        }
        // trim() : String의 좌우 공백 제거
        // parse() : 문자열을 다른 타입으로 변환
        let job_number: i32 = match job_choice.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("작업을 선택하기 위해 숫자를 입력해주세요. {}, (input: {})", e, job_choice);
                continue;
            }    
        }; 
        
        match Outcome::from_str("커피/4900") {
            Ok(obj) => println!("지출 : {:?}", obj),
            Err(e) => println!("Error: {}", e),
        }
        get_outcome_from_cli();

    }


}
