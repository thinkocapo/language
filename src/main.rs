use rand::Rng;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

// TODO warnings for derive(Debug) 
// calling to_string()
// modularizing file reader and line parsing. copy/ownership of who is using line reader, access to it.
fn main() -> std::io::Result<()> {

    let pronouns = ["Ich", "Du", "er/es/sie", "Wir", "Ihr", "Sie"];
    let random = rand::thread_rng().gen_range(0..=pronouns.len()-1);
    let pronoun = pronouns[random];
    
    // NOUN
    // casing (nominative, accusative, dative) and article()
    #[derive(Debug)]
    pub struct Noun{
        singular: String,
        plural: String,
        gender: String
    }

    impl Noun {
        fn quantity(&self) -> String {
            let random = rand::thread_rng().gen_range(0..=1);
            if random%2==0 {
                return self.singular.to_string();
            } else {
                return self.plural.to_string();
            }
        }
    }

    #[derive(Debug)]
    pub struct Verb{
        present: String,
        past: String
    }

    impl Verb {
        fn tense(&self) -> String {
            let random = rand::thread_rng().gen_range(0..=1);
            if random%2==0 {
                return self.present.to_string();
            } else {
                return self.past.to_string();
            }
        }
    }

    // NOUNS
    let nouns = Path::new("./src/nouns.txt");
    let file_nouns_1 = File::open(nouns)?;
    let file_nouns_2 = File::open(nouns)?;
    let file_nouns_reader_1 = BufReader::new(file_nouns_1);
    let file_nouns_reader_2 = BufReader::new(file_nouns_2);
    let noun_count = file_nouns_reader_1.lines().count();
    let mut noun_line_number = 0;
    let random_noun_number = rand::thread_rng().gen_range(0..=noun_count-1);

    let mut noun = Noun{singular:"".to_string(), plural:"".to_string(),gender:"".to_string()};
    
    for line in file_nouns_reader_2.lines() {
        let line = line?;
        let parts: Vec<&str> = line.trim().split(' ').collect();
        // for word in line.trim().split(' ') {
            // println!("{}", word);
        // }
        if random_noun_number == noun_line_number {
            noun = Noun{singular:parts[0].to_string(), plural: parts[1].to_string(), gender:String::from(parts[2])};
        }
        noun_line_number = noun_line_number+1;
    }

    // VERBS
    let verbs = Path::new("./src/verbs.txt");
    let file = File::open(verbs)?;
    let file2 = File::open(verbs)?;
    let reader = BufReader::new(file);
    let reader2 = BufReader::new(file2);

    let verb_count = reader2.lines().count();
    let mut verb_line_number = 0;
    let random_verb_number = rand::thread_rng().gen_range(0..=verb_count-1);
    
    let mut verb = Verb{present:"".to_string(),past:"".to_string()};

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.trim().split(' ').collect();
        if random_verb_number == verb_line_number {
            verb = Verb{present: parts[0].to_string(), past: parts[1].to_string()};
        }   
        verb_line_number = verb_line_number+1;
    }

    // noun() or even 'word()' could decide which to apply, and print...?
    // println!("\n pronoun {0} present: {1} past: {2} singular: {3} plural: {4} gender: {5}\n", pronoun, verb.present, verb.past, noun.singular, noun.plural, noun.gender);
    println!("\n {0} {1} {2}\n", pronoun, verb.tense(), noun.quantity());

    // struct Word<T> {
    //     class : T
    //     // value, data, next: Word
    // }
    // impl<T> Word<T> {
        // pub fn new(class: T) -> Self {
        //     Word { class }
        // }
        // pub fn next() -> Self {}
    // }
    
    // let word = Word::new(Noun{s:"verkehr".to_string(), p:"verkehr".to_string(), g:"m".to_string()});
    // let word1 = Word::new(noun);
    // println!("word1 is: {:?}", word1.class);
    // let word2 = Word::new(pronoun);
    // println!("word2 is: {:?}", word2.class);
    // let word3 = Word::new(verb);
    // println!("word3 is: {:?}", word3.class);

    Ok(())
}
