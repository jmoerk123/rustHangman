use std::io;
use rand::Rng;
// use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};
fn main() //-> Result<String, io::Error>
{
    let word_list = ["test", "hej", "glat", "dum", "drøj"];
    println!("What is the word?");
    let mut given_word_mut: String = String::new();
    io::stdin().read_line(&mut given_word_mut).expect("Failed");
    if given_word_mut.trim().is_empty(){
        let index = rand::thread_rng().gen_range(0..=4);
        given_word_mut = word_list[index].to_string();
    }
    let giver_word: String = given_word_mut.trim().to_string();
    let giver_word_len: usize = giver_word.chars().count();
    let hidden_word: Vec<char> = giver_word.chars().collect();
    // println!("{:?}", hidden_word.iter().map(|x| x.to_string() + " ").collect::<String>());
    let mut guessed_word: Vec<char> = Vec::with_capacity(giver_word_len);
    for _i in 0..giver_word_len{
        guessed_word.push('_');
    }
    
    // std::process::Command::new("clear").status().unwrap();
    println!("Guess a letter");
    println!("{:?}", guessed_word.iter().map(|x| x.to_string() + " ").collect::<String>());
    loop {
        // println!("{:?}", guessed_word.iter().map(|x| x.to_string() + " ").collect::<String>());
        let mut guessed_letter: String = String::new();
        io::stdin().read_line(&mut guessed_letter).expect("Failed");
        std::process::Command::new("clear").status().unwrap();
        let guessed_char = guessed_letter.chars().next().expect("string is empty");
    
        if hidden_word.contains(&guessed_char) { 
            for i in 0..giver_word_len {
                if hidden_word[i] == guessed_char {
                    let _ = std::mem::replace(&mut guessed_word[i], guessed_char);
                }
            }
        }
        println!("Guess a letter");
        println!("{:?}", guessed_word.iter().map(|x| x.to_string() + " ").collect::<String>());
        // println!("{:?}", hidden_word.iter().map(|x| x.to_string() + " ").collect::<String>());

        // io::stdout().queue(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
        if !guessed_word.contains(&'_') { 
            break;
        }
    }
}