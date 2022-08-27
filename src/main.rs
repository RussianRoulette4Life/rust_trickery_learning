use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
mod word_detection;
use crate::word_detection::count_and_disect_words;
struct Country {
    official_name: String,
    rank_by_population: String,
    continent: String,
    population: String,
    percent_pop_of_world: String,
    last_pop_count_date: String,
    means_of_pop_count: String,
}
fn main() {
    // Create a path to the desired file
    let path = Path::new("/home/ruslan/allMyProjects/rust_trickery_learning/src/index_full.txt");
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
        Ok(_) => print!(""),
    }
    let amount_of_iterations = count_tr_tags(&s);
    println!("{amount_of_iterations}");
    let html_to_parse: String = return_html_up_to_a_tag(&s, 2);
    let vector: Vec<String> = parse_html(&html_to_parse);
    println!("{:#?}", vector)
    // let parsed_info = (parse_html(&s, false).replace("\n", " ")).0;
    // let parsed_info_finalised = String::from(parsed_info).trim();
    // let vector_of_parsed_words = (count_and_disect_words(&parsed_info_finalised)).2;
}
fn parse_html(html: &String) -> Vec<String>{
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
                let string_final:String = String::from((string_of_text.replace('\n', " ")).trim());
                return count_and_disect_words(&string_final).2  
            } else {
                tags = String::new();
            }
        }
    }
    vec![]
}

fn count_tr_tags (html: &String) -> u32 {
    let mut start_iter:bool = false;
    let mut tags = String::new();
    let mut num_of_tags:u32 = 0; 
    for character in html.chars(){
        if character == '<'{
                start_iter = true;
        }
        if start_iter {
            tags.push(character);
        }
        if (character == '>') {
            if tags == "</tr>" {
                num_of_tags += 1;
            }
            tags = String::new()
        } 
    }
    return num_of_tags
}
fn return_html_up_to_a_tag(html:&String, tag_number:u32) -> String {
    let mut new_html = String::new();
    let mut start_iter:bool=false;
    let mut tags:String = String::new();
    let mut string_of_text = String::new();
    let mut num_of_tags: u32 = 0;
    for character in html.chars(){
        if character == '<'{
                start_iter = true;
        }
        if start_iter {
            tags.push(character);
        }
        string_of_text.push(character);
        if character == '>'{
            start_iter = false;
            if tags == "</tr>"{
                num_of_tags+=1;
                if tag_number == num_of_tags {
                    return string_of_text
                } else {
                    string_of_text = String::new();
                }
            } else {
                tags = String::new();
            }
        }
    }
    String::from("must've been an error")
}