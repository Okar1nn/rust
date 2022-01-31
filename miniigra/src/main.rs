use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Угадай число");
    
    println!("Введедите число:");
    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("Секретное число = {}",secret_number);

    loop{
        let mut number = String::new();

        io::stdin().read_line(&mut number).expect("Ошибка чтения строки");
    
        println!("Вы ввели:{}", number);
        let guess:u8 = number.trim().parse(){
            Ok => num,
            Err(_) => {
                println!("Введите число от 1 до 100");
                continue;
            }
        }
    
        match number.cmp(secret_number){
            Ordering::Less =>println!("Слишком маленькое"),
            Ordering::Equal =>{
                println!("Молодец! Угадал");
                break;
            },
            Ordering::Greater =>println!("Слишком большое")
        }
    }
}
