#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn calc_area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rectangle_compare: &Rectangle) -> bool{
        self.height > rectangle_compare.height && self.width > rectangle_compare.width
    }
}
fn main () {
    let rect1: Rectangle = Rectangle {
        width: 50,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 40,
        height:20,
    };
    println!("area thru the method... method: {}", rect1.calc_area());
    println!("u can fit rect2 in rect1, right? {}. what about the other way around? {}. sure",rect1.can_hold(&rect2), rect2.can_hold(&rect1) );
}
