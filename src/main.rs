use rand::Rng;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() -> std::io::Result<()> {

    // TODO constants/struct enumeration, function 
    let pronoun: String = pick_pronoun();
    let noun: Noun = pick_noun("./src/nouns.txt").unwrap();
    let verb: Verb = pick_verb("./src/verbs.txt").unwrap();

    println!("\n------------------------");
    println!("\n {0} {1} {2} ({3}) \n", pronoun, verb.tense(), noun.quantity(), noun.gender);
    println!("------------------------");
    
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
        // noun.gender;
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
