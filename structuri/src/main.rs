use std::mem;
// struct User {
//     username: String,
//     email: String,
//     age: u64,
//     active: bool,
// }
// fn main()
// {
//     let mut user1 = User{
//         email: String::from("user1@mail.com"),
//         username: String::from("ping_machine"),
//         active: true,
//         age: 42,
//     };
//     user1.age = 20;
//     let user2 = User{
//         email: String::from("another_email@mail.com"),
//         username: String::from("another_name"),
//         ..user1
//     };
// }
// struct PingMachine{
//     times_to_ping: usize,
// }

// impl PingMachine{
//     fn get_num_times_to_ping(&self) ->usize {
//        self.times_to_ping
//     }
//     fn ping(&mut self) {
//         if self.times_to_ping > 0 {
//             println!("ping");
//             self.times_to_ping -=1;
//         }else {
//             println!("No more pings!");
//         }
//     }
// }
// fn main(){
//     let mut ping_machine = PingMachine{
//         times_to_ping: 11,
//     };
//     ping_machine.ping();
//     ping_machine.ping();
//     ping_machine.ping();
//     ping_machine.ping();
//     ping_machine.ping();
//     ping_machine.ping();
//     ping_machine.ping();
//     ping_machine.ping();
//     ping_machine.ping();

//     ping_machine.ping();
//     ping_machine.ping();
// }
// struct Lel{
//     width: u32,
//     heigth: u32,
// }
// fn main(){
//     let rect = Lel{
//         width: 30,
//         heigth: 50,
//     };
//     println!("ploshad {}",ploshad(&rect));
// }
// fn ploshad(lol:&Lel) -> u32{
//     lol.heigth * lol.width
// }


// -----------KORTEJI + STRUCTURI-------------

// #[derive(Debug)]
// struct Matrix(f32, f32, f32, f32);
// fn reverse(pair: (i32, bool)) -> (bool, i32) {
//     // `let` можно использовать для создания связи между кортежем и переменной
//     let (integer, boolean) = pair;

//     (boolean, integer)
// }
// fn main(){
// let pair = (1, true);
//     println!("pair хранит в себе {:?}", pair);
//     println!("перевёрнутая pair будет {:?}", reverse(pair));
//     println!("кортеж из одного элемента: {:?}", (5u32,));
//     println!("просто целочисленное значение: {:?}", (5u32));
//     let tuple = (1, "привет", 4.5, true);
//     let (a, b, c, d) = tuple;
//     println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
//     let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
//     println!("{:?}", matrix);

// }

//--------------MASSIVI + SREZI--------------------
// fn analyze_slice(slice: &[i32]){
//     println!("srez soderjit {}", slice[0]);
//     println!("v sreze {} elementov", slice.len());
// }
// fn main(){
//     let xs:[i32;5] = [1,2,3,4,5];
//     let ys:[i32;500] = [0;500];
//     println!("perviy element massiva: {}", xs[0]);
//     println!("dlina massiva: {}", xs.len());
//     println!("massiv zanimaet {} byte",mem::size_of_val(&xs));
//     println!("заимствуем весь массив, используя срез");
//     analyze_slice(&xs);
//     analyze_slice(&ys[1 .. 4]);
//     println!("{}", xs[4]);
// }
// ------------ STRUKTURI TESTING------------
// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
//     email: String,
//     region: String,
// }
// struct Point {
//     x: f32,
//     y: f32,
// }
// struct Velichina {
//     visota: u32,
//     shirina: u32,
// }
// #[allow(dead_code)]
// struct Rectangle{
//     top_left: Point,
//     bottom_left: Point,
// }

// #[derive(Debug)]
// struct Lumerin{
//     name: String,
//     price: u32,
//     x: u32,
//     gem: bool,
//     rect: bool,
// }
// #[derive(Debug)]
// struct Umee{
//     name: String,
//     price:u32,
//     x: u32,
//     gem: bool,
//     rect: bool,
// }
// fn main(){
//     let peter = Person{
//     name: String::from("Peter"),
//     age: 15,
//     email: String::from("aboba@mail.ru"),
//     region:String::from("Russia"),
//     };
//     //let peter = Person{name, age, email, region};
//     println!("{:?}", peter);
//     let point: Point = Point{x: 10.4, y: 5.3};
//     let vel: Velichina = Velichina{visota: 155, shirina: 55};
//     println!("visota - {}, shirina {}", vel.visota,vel.shirina);
//     println!("koordinati tochki {}, {}", point.x, point.y);
//     let ico = Lumerin{
//         name: String::from("Lumerin"),
//         price:2500,
//         x: 30,
//         gem: true,
//         rect: false,
//     };
//     let umee = Lumerin{
//         name: String::from("Umee"),
//         ..ico
//     };
//     let 
//     println!("{:?}", ico);
//     println!("{:?}", umee);
// }
//-----------STRUKTURI ZADANIE------------
#[derive(Debug)]
struct Point{
    x: f32,
    y: f32,
}
#[allow(dead_code)]
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}
fn rect_area(ploshad:&Rectangle) -> f32 {
    ploshad.top_left.x   * ploshad.bottom_right.y
}
fn main(){
    let point:Point = Point{
        x: 17.8,
        y: 3.2,
    };
    println!("koordinaty tochek x = {:?}, y = {:?}",point.x,point.y);
    let bottom_right = Point{
        x: 5.3,
        ..point
    };
    println!("vtoraya tochka x = {:?}, y = {:?}", bottom_right.x,bottom_right.y);
    let Point{
        x: left_edge,
        y: top_edge,
    } = point;
    let _rectangle = Rectangle{
        top_left: Point{x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };
    let plosh = Point{
        x: left_edge,
        y: point.y
    };
    println!("{:?}, {:?}", _rectangle.top_left, _rectangle.bottom_right);
    println!("{:?}", rect_area(&_rectangle));
}
