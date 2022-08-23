use std::io;


fn main(){
	println!("He-hey! More rust trickery! This time, the program recieves a sentence from YOU, and gives you an output with a lot of cool stuff!");
	loop {
		println!("Enter a sentence: ");
		// creates a mutatable string (it has to be this wayy)
		let mut sentence: String = String::new();
		io::stdin()
			.read_line(&mut sentence)
			.expect("what did you do???");
		// its a pain to work with an untrimmed string
		let trimmed_string = sentence.trim().to_string();
		// this kind of assignment of values is convenient asf
		let (amount_of_words, index_last) = count_and_disect_words(&trimmed_string);
		// print everything outte
		println!("The original string: {}", trimmed_string);
		println!("the goofy stuff - words: {}, last character num: i dunno, this variable is redundant, last index: {}",amount_of_words, index_last) 
	}
}
// it HAS to take a reference, not a chance at the original
fn count_and_disect_words (string_sentence: &String) -> (i32, i32){
	let mut amount_of_words = 0;
	let mut index_last = 0;
	// made to prevent 2 spaces count as a separate word
	let mut last_character_index = 0;
	// for fun :)
	if string_sentence == "" {
		println!("why?")
	}
	// prints ALL characters
	for character in string_sentence.chars(){
		println!("{}", character)
	}
	let mut detected_word: String = String::new();
	// word detection, that's it
	for (i, character) in string_sentence.chars().into_iter().enumerate() {
		if (character != ' ') && (character != '\n') {
			detected_word.push(character);
		}
		last_character_index = if (character != ' ') && (character != '\n') && (character != '-') {
			i
		} else {
			last_character_index
		};
        if (character == ' ') && (i <= last_character_index + 1) {
			amount_of_words = amount_of_words + 1;
			println!("{}. {}",amount_of_words, detected_word);
			detected_word = String::new();
		}
		index_last = i;
    }
	// print what cant return, return what can
	println!("{}. {}",&amount_of_words + 1, &detected_word);
	(amount_of_words+1, index_last.try_into().unwrap()) 
	}
