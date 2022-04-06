fn main(){
    //инициализируем строку
    let ctr = String::from("Hello World!");
    println!("{}", ctr);
    //передаем ее во владение переменной ptr
    let ptr = imp_str(ctr);
    println!("{}", ptr);
    //передаем ptr во владение rptr
    let rptr = return_str(ptr);
    println!("{}",rptr);
    //создаем кортеж, в котором создаем 2 пееременные, одна из которых это строка, вторая длина и передаем туда нашу строку 
    let(rrptr,len) = calculate_length(rptr);
    println!("stroka = {}, length = {}",rrptr,len);
//ТОТ ЖЕ ПРИМЕР ТОЛЬКО СО ССЫЛКАМИ
    //здесь идем по ссылке на значение
    let stroka = String::from("Novaya Stroka svobodnaya i bez returnov");
    let len = calculate_length_ssila(&stroka);
    println!("stroka so ssilkoi = {}, dlina ee = {}", stroka,len);
}

fn imp_str(stroka:String) -> String{
    let plus_stroka = stroka + "How are you?";
    plus_stroka
}

fn return_str(strochka:String) -> String {
    strochka
}
fn calculate_length(s:String) -> (String,usize){
    // .len - высчитать длину строки
    let length = s.len();
    //возвращаеем кортеж из длины строки и строки 
    (s,length)
}
//------------------------ССЫЛКИ-----------------------------
// Обозначаем в параметрах функции, что мы берем ССЫЛКУ на строку, а не строку. Тем самым не берем ее во владение и на не нужно ее возвращать.
fn calculate_length_ssila(s:&String) -> usize{
    let length = s.len();
    length
}

// --------------ОТРАБОТКА------------
// fn main(){
//     let s1 = String::from("Hello");
//     let s2 = s_vladeniem_imp(s1);
//     println!("s2 = {}", s2);
//     let mut s3 = String::from("Ssilka");
//     let s4 = bez_vladeniya_ssilkoy(&s3);
//     println!("stroka = {}, length = {}", s3,s4);
//     popitka_izmenit_znachenie_po_ssilke(&mut s3);
// }
// fn s_vladeniem_imp(s:String) -> String{
//     let ss1 = s + " World";
//     ss1
// }
// fn bez_vladeniya_ssilkoy(s:&String) -> usize {
//     let len = s.len();
//     len
// }
// fn popitka_izmenit_znachenie_po_ssilke(s:&mut String){
//     s.push_str(" s izmeneniem");
//     println!("{}",s);
// }