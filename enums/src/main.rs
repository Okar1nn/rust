// enum IpAddrKind{
//     V4,
//     V6,
// }
// struct IpAdd{
//     Kind: IpAddrKind,
//     address: String,
// }
// let home = IpAdd{
//     Kind: IpAddrKind::V4,
//     address: String::from("198.0.0.1"),
// }
// let nohome = IpAdd{
//     Kind: IpAddrKind::V6,
//     address: String::from("::1"),
// }
// //------------------------
// // или
// //------------------------
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// let home = IpAddr::V4(String::from("127.0.0.1"));

// let loopback = IpAddr::V6(String::from("::1"));

// // четыре значения в перечисление

// enum Ipaddr {
//     V4(u32,u32,u32,u32),
//     V6(String),
// }
// let home = Ipaddr::V4(127.0.1.1);

// let loopback = Ipaddr::V6("..0");

// // встроенный айпиадр
// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// // шаблон match для сравнения в перечислении
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// //------

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
// #[derive(Debug)]

// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }
// enum Operation{
//     Substract,
//     Add,
//     Divide,
//     Multiply
// }
// fn main(){
//     let op = Operation::Add;
//     let result = calc(op, 5, 7);
//     println!("{}",result);
// }
// fn calc(op: Operation, x:u32, y:u32) -> u32{
//     match op{
//         Operation::Add => x + y,
//         Operation::Multiply => x * y,
//         Operation::Divide => x / y,
//         _ => 0
//     }
// }
// }
// fn main(){
//     let number:u32 = 7;
//     let fnumber = number as f64;
//     match   number{
//         1..=4 => println!("one"),
//         5..=9 => println!("two"),
//         10..=15 => println!("three"),
//         _ => println!("aboba")
//     }    
// }