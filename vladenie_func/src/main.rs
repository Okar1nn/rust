fn main(){
    let s1 = imp_str();
    println!("s1 iz imp_str = {}",s1);
    let s2 = String::from("world!");
    println!("s2 = {}", s2);
    let s3 = return_str(s2);
    println!("s3 iz return_str s2 - {}", s3);
    let s4 = return_das(s3);
    println!("{}",s4);
}

fn imp_str() -> String {
    let imp = String::from("hello");
    imp
}
fn return_str(stroka1:String) -> String {
    stroka1 + "vsem privet"
}
fn return_das(stroka2:String) -> String {
    let ab = String::from("Yellow");
    let abc = ab + &stroka2;
    abc
}