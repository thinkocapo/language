# Plan

## Overview

Rewriting this app. Fine to lose all Rust code and src .txt files.

German will be the initial language. It doesn't have to be generalizable to other languages yet.

## Core Behavior

- Open the app and it generates 10 random sentences using a1/a2 vocab.
- User speaks a sentence using all this. There is nothing to edit in the UI. Speak it grammatically correct.
- Compare your spoken answer to the answer Key
- User clicks "ANSWER" to reveal the correct answer. Put a non-functional answer button for now.
- User need to use creativity for adjust AO/DO to make it sound more realistic.

## Vocab & Sentences

- A1 + A2 level nouns, verbs, adjectives. Can hard-code it into frontend assets for now, but eventually serve it via db perhaps (FastAPI).
- Ask Claude to find and preview a list

## Sentence Syntax
When this app generates the 10 sentences, the following is displayed.

Personal pronoun, a random one.

Infinitive Verb, with a label above it indicating what tense it needs to be in.
- present, praterite, perfect, future

Object is a noun, it ends up being a Accusative or Direct object, I can decide that myself, for what may make the most sense.

Here's the syntax:
`pronoun | verb_infinitive | object` a third of the time  
`pronoun | verb_infinitive | pronoun` a third of the time  
`pronoun | verb_infinitive | object | pronoun` a third of the time  

definite_article before the object, 1/3 of the time
indefinite_article before the object, 1/3 of the time
no article before the object, 1/3 of the time

so example would be like

           indefinite
sie laufen tische

see how 'indefinite' it should be a tag label displayed above the noun, so that I have to figure out if it's ein vs eine  or der vs die etc. as well as den vs dem.

## Running the Plan
1. put a1/a2 infinitive verb stems + nouns + adjectives in a .txt file, or just store in javascript somewhere. maybe needs be written into the html? But doesn't need to be shown at all times. 
2. ask for a plan, how this will be stored (in JS, in a variable of memory?)
3. it's single page, ask if nextjs is overkill or is it faster to just test this in a html app

"give me a plan"

## Upcoming Features

- Click on any word to regenerate just that word with a new random one
- "Add Preposition" button that inserts a preposition before the noun

## Architecture (deciding)

- Plain HTML page — unclear where all vocab would be stored (in-memory?)
- Web app may be better for storage. TODO research it.
- Next.js app is a strong candidate
- Use ChatGPT or a language translation API to provide answers
- Goal: eventually host on personal website as a page, if it works well


