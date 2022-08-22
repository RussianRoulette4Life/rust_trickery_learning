use std::io;

fn main(){
	println!("He-hey! More rust trickery! This time, the program recieves a sentence from YOU, and gives you an output with a lot of cool stuff!");
	loop {
		println!("Enter a sentence: ");
		let mut sentence: String = String::new();
		io::stdin()
			.read_line(&mut sentence)
			.expect("what did you do???");
		let (amount_of_words, last_character_index, index_last) = count_words(&sentence);
		println!("the goofy stuff - words: {}, last character num: {}, last index: {}",amount_of_words, last_character_index, index_last) 
	}
}
fn count_words (string_sentence: &String) -> (i32, i32, i32){
	let mut amount_of_words = 1;
	let mut last_character_index = 9999999;
	let mut index_last = 0;
	let bytes = string_sentence.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if (item != 32) && (item != 10){
			last_character_index = i;
		}
        if (item == 32) && (i <= last_character_index + 1) && (last_character_index != 9999999) {
			amount_of_words = amount_of_words + 1;
		}
		index_last = i;
    }
	let slice_of_string: &str = &string_sentence[last_character_index+1..index_last];
	if slice_of_string != ""{
		amount_of_words = amount_of_words-1;
	}
	(amount_of_words, last_character_index.try_into().unwrap(), index_last.try_into().unwrap()) 
}