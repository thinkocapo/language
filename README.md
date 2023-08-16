## Run
```cargo run```
## How
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

## Todo
Aug 11th

verbs.txt past participle for past tense? key for displaying past participle
nounts.txt plurality

<pronoun> <verb> <artikel><noun> 

<artikel><noun> <verb>
<artikel><noun> <verb> <noun>
<artikel><noun> <verb> <pronoun>
^ article definite/indefinite is optional.

definite/indefinite article needs declension if Akk/Dat  

linked list of words and print it

LONG-TERM
noun.def_article.aks
noun.indef_article.aks
noun.case1,
noun.case2...
noun.[no article]
struct/enum Tense has values Present,Past,Future
verb.Tense or...
verb.Present (for all pronouns) or...
verb.Present.ich .du
verb.Past (for all pronouns)
// only if you want the conjugation done for you, as an option. 'Enter' is test case, no conjugation, 'Spacebar' is answer
verb. invocation enforcesAkk|Dat or verb.case for the connecting noun .connectVerification in linked list

TODO dummy Test. benchmark it too.
TODO different modules enum:: word:: verb:: pronoun:: in different files?
TODO match my stored vocab '<word> <type> <endings?>' against .dict or Text you pass to it

random generating a Adj, choose it's right casing by selecting a noun, which has a casing (due to the verb?).
'<Adj><Noun>'
enforce a Pronoun Verb Adj-Noun order? decide the noun direct-object first (and have it re-arrange the adj later) or the adjective first?

LinkedList. Graph, with combinations?

## Packages > Crates > Modules > Paths

Packages: A Cargo feature that lets you build, test, and share crates  (Cargo.toml)
Crates: A tree of modules that produces a library or executable  (m)
Modules and use: Let you control the organization, scope, and privacy of paths  
Paths: A way of naming an item, such as a struct, function, or module  

Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.

## Error Handling
Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program.

Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.