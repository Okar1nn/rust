fn main(){
    let ctr = String::from("Hello World!");
    println!("{}", ctr);
    let ptr = imp_str(ctr);
    println!("{}", ptr);
    let rptr = return_str(ptr);
    println!("{}",rptr);
}

fn imp_str(stroka:String) -> String{
    let plus_stroka = stroka + "How are you?";
    plus_stroka
}

fn return_str(strochka:String) -> String {
    strochka
}