use rand::Rng;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

// TODO
// copy/ownership of who is using line reader, access to it.

// needs Akk/Dat if used as direct/indirect object, verb may need to specify that
    // sentence/word connects verb to noun, verb has akk/dat property or preference,
    // noun.call() checks reference to its noun for preferred property type? no.
    // or just print 'def' or 'indef'. could have also done print 'sing plural' and then you have to decide how noun written. Nah
// Start presentation, or explaining

// TODO modularize er es sie picker
// TODO warnings for derive(Debug) 
// calling to_string()
// how to make noun() word() itself execute as a function without calling noun.method() - Constructor method? but on an instance?
fn main() -> std::io::Result<()> {

    let pronoun: String = pick_pronoun();
    let noun: Noun = pick_noun("./src/nouns.txt").unwrap();
    let verb: Verb = pick_verb("./src/verbs.txt").unwrap();

    // println!("\n pronoun {0} present: {1} past: {2} singular: {3} plural: {4} gender: {5}\n", pronoun, verb.present, verb.past, noun.singular, noun.plural, noun.gender);
    println!("\n {0} {1} {2}\n", pronoun, verb.tense(), noun.quantity());

    
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

    fn pick_pronoun() -> String {
        let pronouns = ["Ich", "Du", "Er Es Sie", "Wir", "Ihr", "Sie"];
        let random = rand::thread_rng().gen_range(0..=pronouns.len()-1);
        let mut pronoun = pronouns[random];
        if pronoun == "Er Es Sie" {
            let random_pronoun = rand::thread_rng().gen_range(0..=2);
            let er_sie_es: Vec<&str> = pronoun.split(' ').collect();
            pronoun = er_sie_es[random_pronoun];
        }
        return pronoun.to_string();
    }

    fn pick_noun(file_path: &str) -> Result<Noun, Box<dyn std::error::Error>> {

        let mut noun = Noun{singular:"".to_string(), plural:"".to_string(),gender:"".to_string()};

        let nouns = Path::new(file_path);
        let file_nouns_1 = File::open(nouns)?;
        let file_nouns_reader_1 = BufReader::new(file_nouns_1);
        let noun_count = file_nouns_reader_1.lines().count();
        let random_noun_number = rand::thread_rng().gen_range(0..=noun_count-1);
    
        let file_nouns_2 = File::open(nouns)?;
        let file_nouns_reader_2 = BufReader::new(file_nouns_2);
    
        // Pick a random Noun upon iterating through the lines from the file. this would be easier if I could convert 'lines' into an Array and access by index Id
        let mut noun_line_number = 0;
        for line in file_nouns_reader_2.lines() {
            let line = line?;
            let parts: Vec<&str> = line.trim().split(' ').collect();
    
            if random_noun_number == noun_line_number {
                noun = Noun{singular:parts[0].to_string(), plural: parts[1].to_string(), gender:String::from(parts[2])};
            }
            noun_line_number = noun_line_number+1;
        }

        Ok(noun)
    }

    fn pick_verb(file_path: &str) -> Result<Verb, Box<dyn std::error::Error>> {
        let verbs = Path::new(file_path);
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
        Ok(verb)
    }

    Ok(())
}
