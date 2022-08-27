use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
struct Country {
    official_name: String,
    shorthand_name: String,
    current_leader: String,
    type_of_govern: String,
    last_three_leaders: String,
    area: u64,
    population: u64,
    gdp: u64,
    inner_debt: i64,
    outer_debt: i64,
}
fn main() {
    // Create a path to the desired file
    let path = Path::new("/home/ruslan/allMyProjects/rust_trickery_learning/src/index.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("file parsed successfully"),
    }
    let html = String::from("<body>smth</body>");
    let parsed_info = parse_html(&s);
    println!("info: {}", parsed_info.replace("\n", " "));
    // `file` goes out of scope, and the "hello.txt" file gets closed
}
fn parse_html(html: &String) -> String{
    let mut start_iter:bool=false;
    let mut tags:String = String::new();
    let mut string_of_text = String::new();
    for character in html.chars(){
        if character == '<'{
                start_iter = true;
        }
        if start_iter {
            tags.push(character);
        } else {
            string_of_text.push(character);
        }
        if character == '>'{
            start_iter = false;
            if tags == "</tr>"{
                return string_of_text   
            } else {
                tags = String::new();
            }
        }
    }
    String::from("sorreh, found nothin")
}
//fn search_thru_tags(string_html: &String) -> String {
//    for character in string_html.chars(){
//    
//    }
//}
