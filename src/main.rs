use rand::Rng;
use std::fs::File;
use std::io::{BufReader, BufRead};

use std::path::Path;


fn main() -> std::io::Result<()> {
    // PRONOUNS
    // Immutable and no methods needed yet. Maybe struct ones Word is evaluated for matching.
    // pub struct Pronoun;
    // let pronouns = ["Ich", "Du", "er/es/sie", "wir", "ihr", "sie/Sie"];
    let pronouns = ["Ich", "Du", "Wir", "Ihr", "Sie"];
    let pronoun_number = rand::thread_rng().gen_range(0..=pronouns.len()-1);
    // let pronoun_pick = pronouns[pronoun_number];
    let pronoun = pronouns[pronoun_number];
    
    // NOUN
    // singular, plural, gender. casing (nominative, accusative, dative) "noun Apfel Apfel m/f/n";
    #[derive(Debug)]
    pub struct Noun{
        singular: String,
        plural: String,
        gender: String // or takes def+indef articles (der, ein. das, ein. eine, die. and die for plural)
    }
    impl Noun {
        // fn singular(&self) -> String {
        //     return self.singular.to_string();
        // }
        
        //fn article(&self) -> String {
            // if g == 'm' then "der" or "ein" [or none] + self.s.to_string();
            // if g == 'n' then "das" or "ein" [or none]  + self.s.to_string();
            // if g == 'f' then "die" or "eine" [or none]  + self.s.to_string();
            // or the prompt could show, "Du verkaufen puppe 'singular' 'definite" singular/plural and def/indef as part of promt
            // pick if want singular or plural?
        //}
        // assigns the articles needed for use later? but defer this part, if loading millions of words?
        // ? assume pre-construction of der/das/der ein/ein/eine, only select 'definite' or 'indefinite'. and assumes Nominative case

        // noun.express(DEFINITE) nah...
        // noun.definite_article() nah...
        // noun() yaa, and it picks for you :)
        // fn new(&self) -> Self {
            // casing Nominative (then definite, indefinite article) and a casing has articles in it?
                // nominative.definite = "der" nominative.definite()
                // nominative.indefinite = "der" nominative.definite()
            // casing Accusative (then definite, indefinite article) and a casing has articles in it?
                // accusative.definite = "der" nominative.definite()
        //}
        //fn to_string(&self) -> String {
            // Plurality.pick() article = nominative.gender().plurality() ? .gender() returns 'M' or 'F' and then plurality uses that?
            // 1 pick if want sing or plural (those are properties already) .plurality() .quantity()
            // 2 pick Nominative case for now .case() .casing()
            // 3 pick nominative.article() accusative.article() nominative.article(singular) or nominative.singular(article)
            // or 
            // der or ein...
        //}
    }

    // VERBS
    #[derive(Debug)]
    pub struct Verb{
        present: String,
        past: String
    }

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
    println!("\n pronoun {0} present: {1} past: {2} singular: {3} plural: {4} gender: {5}\n", pronoun, verb.present, verb.past, noun.singular, noun.plural, noun.gender);

    // let word = Word::new(Noun{s:"verkehr".to_string(), p:"verkehr".to_string(), g:"m".to_string()});
    // let word1 = Word::new(noun);
    // println!("word1 is: {:?}", word1.class);
    // let word2 = Word::new(pronoun);
    // println!("word2 is: {:?}", word2.class);
    // let word3 = Word::new(verb);
    // println!("word3 is: {:?}", word3.class);

    Ok(())
}
