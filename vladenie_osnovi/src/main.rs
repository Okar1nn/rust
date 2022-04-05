fn main() {
    let s = String::from("Test string");
    string_outer(s);
    // s вышла из области видимости  и нельзя ее больше здесь использовать
    //println!("{}",s); - не сработае, т.к. s вышла из области видимости
    let x = 5;
    chislo_outer(x);
    //с числами все ок, т.к. работает принцип copy, что со сторокой невозможно по дефолту
}
// s вошла в область видимости string_outer и перенесла свое значение
fn string_outer(stroka:String) {
    println!("s = {}",stroka);
}
fn chislo_outer(chislo:u32){
    println!("x = {}", chislo);
}
