// use std::io;
// use rand::{Rng, thread_rng};
// use std::cmp::Ordering;

// fn main(){
//     println!("Угадайте число");
//     //Создаем строку в которой будем хранить значение из ввода
//     //делаем из строки - число
//     //thread_rng - генератор случайных чисел
//     //gen_range - метод для задания границ между рандомными числами
//     let secret_number = thread_rng().gen_range(1..101);
//     loop {
//         println!("Введите свое число:");
//     let mut chislo = String::new();
//     //в read_line указываем изменяемую ссылку на эту строку из функции stdin
//     //.expect нужно обязательно для обработки ошибки, в данном случае мы проссто завершаем программу с сообщением
//     io::stdin().read_line(&mut chislo).expect("Возникла ошибка, завершаю программу");
//     //Меняем тип со строки на число. trim - удаляет все пробелы, parse - меняет тип ранее указанный перед переменной, expect - сообщение ошибки
//     //добавляем match для проверки числа, если все ок - возвращаем число, если нет =- продолжаем.
//     let chislo:u32 = match chislo.trim().parse(){
//         Ok(num) => num,
//         Err(_) => continue,
//     };
//     println!("Вы ввели: {}", chislo);
//     //cmp - сравнивает два значения chislo и secret number
//     //match перебирает каждое значение и выводит как написано в сравнении
//     match chislo.cmp(&secret_number){
//         Ordering::Less => println!("Слишком маленькое"),
//         Ordering::Greater => println!("Слишком большое"),
//         Ordering::Equal => {
//         println!("Ты угадал!");
//         break;
//         }
//        }
//     }
// }
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Секретное число: {}", secret_number);
    loop {
    println!("Введите ваше число:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Я не могу читать это число");
    println!("Ваше число: {}", guess);
    let guess:u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Слишком маленькое число"),
        Ordering::Greater => println!("Слишком большое число"),
        Ordering::Equal => {
            println!("Красавчик!");
            break;
        }
    }
}
}