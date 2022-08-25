#[derive(Debug)]
enum testEnum {
    test1((i32,i32,i32)),
    test2(String),
}
fn main(){
    let tuple:(i32,i32,i32) = (1,7,2);
    let tesst = testEnum::test1(tuple);
    println!("{:?}", tesst);
}
