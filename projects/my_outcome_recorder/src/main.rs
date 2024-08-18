use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
// use std::cmp::Ordering;
use std::error::Error;
use serde_derive::{Serialize, Deserialize};
use csv;

#[derive(Serialize, Deserialize, Debug)] // 이게 없으면 println!으로 출력할 수 없다!
struct Outcome {
    name: String,
    cost: i32,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid input format".to_string());
        }

        let name = parts[0].to_string();
        let cost = parts[1].parse::<i32>().map_err(|e| e.to_string())?;

        Ok(Outcome {
            name, cost
        })
    }
}

fn get_prompt_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn print_outcome_list(outcome_list: &Vec<Outcome>) -> Result<(), Box<dyn Error>> {
    println!(" 📜 현재 지출 항목 수 : {}개", outcome_list.len());
    for outcome in outcome_list {
        println!("     * {}: {}", outcome.name, outcome.cost);
    }
    Ok(())
}
 
// Read csv table that recorded outcome history
fn read_record(outcome_list: &mut Vec<Outcome>) -> Result<(), Box<dyn Error>>{
    let csv_path = "./outcome_list.csv";
    let file = File::open(csv_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let iter = rdr.deserialize();
    for result in iter {
        let outcome = result?;
        outcome_list.push(outcome);
    }
    Ok(())
}

fn write_record(outcome_list: &Vec<Outcome>) -> Result<(), Box<dyn Error>> {
    let csv_path = "./outcome_list.csv";
    let file = File::create(csv_path)?;
    let mut wtr = csv::Writer::from_writer(file);

    for outcome in outcome_list {
        wtr.serialize(outcome)?;
    }

    wtr.flush()?;
    println!("지출 내역을 파일로 저장하였습니다. ({})", csv_path);

    Ok(())
}

fn main() {
    println!("[지출 기록기] 시작");
    let mut job_choice = String::new();
    
    let mut outcome_list: Vec<Outcome> = Vec::new();
    
    println!(" 지출 기록기에서는 아래 작업을 수행할 수 있습니다! 😄\n    1) 지출 기록\n    2) 기록 저장\n    3) 기록 불러오기");
    
    loop{
        job_choice.clear();
        
        // 두 가지 방법으로 prompt의 입력을 받아와 변수에 바인딩할 수 있다.
        //   1) expect 예외처리
        // job_choice = get_prompt_input(" 👉 작업 선택 - 1) 지출 기록 2) 기록 불러오기 3) 기록 저장\n >>> ").expect("입력 실패!");
        //   2) match 예외처리
        job_choice = match get_prompt_input(" 👉 작업 선택 - 1) 지출 기록 2) 기록 불러오기 3) 기록 저장\n >>> ") {
            Ok(text) => text,
            Err(e) => {
                println!("입력 실패! {}", e);
                continue;
            }
        };
        
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

        match job_number {
            0 => {
                match print_outcome_list(&outcome_list) {
                    Ok(_) => continue,
                    Err(e) => println!("지출 기록을 출력할 수 없습니다. {}", e),
                }
            }
            1 => {
                let input = match get_prompt_input(" 💲 지출 비용 입력 {지출명/지출금액} \n >>> ") {
                    Ok(text) => text,
                    Err(e) => {
                        println!("입력 실패! {}", e);
                        continue;
                    }
                };
                if input.to_lowercase() == "exit" {
                    println!("[지출 기록기] 중단, 안녕히가세요! 🫡");
                    break;
                } 
                let outcome = Outcome::from_str(&input);
                match outcome {
                    Ok(outcome) => outcome_list.push(outcome),
                    Err(e) => {
                        println!("형식에 맞게 입력해주세요. {{지출명/지출금액}} {}", e);
                        continue;
                    },
                }
            },
            2 => {
                outcome_list.clear();
                match read_record(&mut outcome_list) {
                    Ok(_) => {
                        println!("지출 기록을 성공적으로 불러왔습니다. (불러온 지출 항목 수 : {}개)", outcome_list.len());
                        continue;
                    },
                    Err(e) => println!("지출 기록을 불러올 수 없습니다. {}", e),
                }
            },
            3 => {
                match write_record(&outcome_list) {
                    Ok(_) => continue,
                    Err(e) => println!("지출 기록을 저장할 수 없습니다. {}", e),
                }
            }
            _ => continue,
        }
    }
}
