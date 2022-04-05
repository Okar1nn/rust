use std::{io, result};
fn summa(chislo1:u32,chislo2:u32){
    let result = chislo1 + chislo2;
    println!("{} + {} = {}",chislo1,chislo2,result);
}
fn minus(chislo1:u32,chislo2:u32) {
    let result = chislo1 - chislo2;
    println!("{} - {} = {}",chislo1,chislo2,result);
}
fn umnojenie(chislo1:u32,chislo2:u32) {
    let result = chislo1 * chislo2;
    println!("{} * {} = {}",chislo1,chislo2,result);
}
fn delenie(chislo1:f64,chislo2:f64){
    let result = chislo1 / chislo2;
    println!("{} / {} = {}", chislo1,chislo2,result);
}
fn main(){
    summa(3, 4);
    println!("Введите два числа, которые нужно прибавить:");
    let mut n1 = String::new();
    io::stdin().read_line(&mut n1).expect("Ошибка");
    println!("Первое число: {}",n1);
    let numbern1:u32 = n1.trim().parse().expect("Ошибка");
    let mut n2 = String::new();
    io::stdin().read_line(&mut n2).expect("Ошибка");
    println!("Второе число: {}", n2);
    let numbern2:u32 = n2.trim().parse().expect("Ошибка");
    summa(numbern1, numbern2);
    minus(numbern1, numbern2);
    umnojenie(numbern1, numbern2);
    let numbern1:f64 = n1.trim().parse().expect("Ошибка");
    let numbern2:f64 = n2.trim().parse().expect("Ошибка");
    delenie(numbern1, numbern2);
}