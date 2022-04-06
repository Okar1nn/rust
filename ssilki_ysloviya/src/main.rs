fn main() {
    //инициализируем строку
    let s = String::from("Hello");
    //записываем в переменную l результат calculate_length со ссылкой на строку. То есть мы взаимствуем значение из s, а 
    //не берем его во владение
    let l = calculate_length(&s);
    println!("stroka - {}, length - {}",s,l);
    //инициализируем новую ИЗМЕНЯЕМУЮ ПЕРЕМЕННУЮ
    let mut s2 = String::from("Hello");
    //вызываем функцию и вставляем в параметр ИЗМЕНЯЕМУЮ ССЫЛКУ
    change(&mut s2);
}
//в функции обозначаем именно, что получаем ссылку на String, а не String
fn calculate_length(s:&String) -> usize{
    let len = s.len();
    len
}
//создаем функцию для изменения со ссылкой на изменяемое значение
fn change(s:&mut String){
    s.push_str(" World!");
}