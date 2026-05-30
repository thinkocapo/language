# Plan

## Overview

Rewriting this app. Fine to lose all Rust code and src .txt files.

## Core Behavior

- Generates 10 random sentences.
- User edits each word to make it grammatically correct, and speak each out loud.
- Compare your spoken answer to the answer Key
- User clicks "ANSWER" to reveal the correct answer

## Vocab & Sentences

- A1 + A2 level nouns, verbs, adjectives. Can hard-code it into frontend assets for now, but eventually serve it via db perhaps (FastAPI).
- Ask Claude to find and preview a list

## Architecture (deciding)

- Plain HTML page — unclear where all vocab would be stored (in-memory?)
- Web app may be better for storage. TODO research it.
- Next.js app is a strong candidate
- Use ChatGPT or a language translation API to provide answers
- Goal: eventually host on personal website as a page, if it works well


