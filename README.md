# German Practice

A single-page German sentence drill. No framework, no build step — open `index.html` in a browser and go.

## What it does

Generates 10 random German sentences using A1/A2 vocabulary. Each sentence shows:

- **Pronoun** — random subject (ich, du, er, sie…)
- **Verb** — infinitive form, with a label above it indicating the tense to use (present, präterite, perfekt, futur)
- **Object** — a noun with a label indicating whether the article should be definite, indefinite, or omitted — *you* figure out the correct form (der/die/das, den/dem, ein/eine…)
- **Object pronoun** — some sentences include a second pronoun (mich, dich, ihn…)

Your job: speak the sentence out loud, grammatically correct. Click **ANSWER** to reveal the correct answer and compare.

## Sentence patterns

```
pronoun | verb_infinitive | noun             (1/3 of sentences)
pronoun | verb_infinitive | object pronoun   (1/3 of sentences)
pronoun | verb_infinitive | noun | pronoun   (1/3 of sentences)
```

Article type (definite / indefinite / none) is randomized per sentence.

## Running locally

Open directly in the browser:
```
open index.html
```

For hot reload while editing, use [browser-sync](https://browsersync.io/):
```bash
npx browser-sync start --server --files "index.html"
```

Or install the **Live Server** extension in VS Code, right-click `index.html` → *Open with Live Server*.

## Stack

Plain HTML + vanilla JS. Vocab is hardcoded in JS arrays in `index.html`.
Future: FastAPI backend to serve vocab from a database, hosted as a page on personal site.
