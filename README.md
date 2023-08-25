# Overview
This is a tool for learning any foreign language. It's tested for A1/A2 level language.  
It’s a command-line script that generates random word combinations that you need to connect and speak in grammatically correct sentences.  
I chose Rust because it’s a new programming language I wanted to try.  
https://github.com/thinkocapo/language  

## Run
1. ```cargo run```
2. Read the output
3. Figure out how to conjugate the verb with the given pronoun
4. Figure out how to use a article, preposition or possessive pronoun in the right Casing/Declension given the verb+noun.
5. Speak it all together as one sentence. See examples below:

If it prints, "We clap nieces"
> Wir klappen Nichten  

How about, "We clap FOR OUR nieces"
> Wir klappen fur unsure Nichten.  

If it prints, "You all open keys"
> Ihr aufmachen Schlüssel

How about, "You all open IT WITH THE keys"
> Ihr macht ES MIT DEN Schlüssel

If it prints, "I argued Uncle"
> Ich gestritten Onkel

How about, "I argued WITH YOUR uncle"
> Ich habe mit deinem Onkel gestritten

## Todo  
Print different combos of these. Construct it into a linkedList and enforce syntax (e.g. only noun can come after a verb)
```
<pronoun> <verb> <artikel><noun>  
<pronoun> <verb> <def_artikel/indef_article><noun>  
<pronoun> <verb> <def_artikel/indef_article><noun>  
<artikel><noun> <verb> <pronoun>  
<artikel><noun> <verb> <noun>  
         <noun> <verb> <pronoun>    
      <pronoun> <verb> <pronoun>  
```

Mode for displaying the noun's gender in parenthesis

## Working with Rust & Cargo
```
 ~/thinkocapo/german   rustc main.rs
 ~/thinkocapo/german   ls
main    main.rs
 ~/thinkocapo/german   ./main

```

```
rustup doc
rustup docs --book
```

```
cargo check
#
cargo build
./target/debug/language
# or
cargo run

#
cargo build --release
./target/release/language
```

updating crates  
```
Cargo provides the command update, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. Cargo will then write those versions to the Cargo.lock file. Otherwise, by default, Cargo will only look for versions greater than 0.8.5 and less than 0.9.0.
```

## Packages > Crates > Modules > Paths

Packages: A Cargo feature that lets you build, test, and share crates  (Cargo.toml)
Crates: A tree of modules that produces a library or executable  (m)
Modules and use: Let you control the organization, scope, and privacy of paths  
Paths: A way of naming an item, such as a struct, function, or module  

Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.

## Error Handling
Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program.

Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.
