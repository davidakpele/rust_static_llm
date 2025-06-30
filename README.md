# 🧠 Terminal Copilot Assistant (JSON Edition)

A simple terminal-based question-answering tool built in Rust.  
This version uses a static JSON file (`qa.json`) to answer user questions by matching them to predefined entries with fuzzy similarity.

---

## 📁 File Overview


---

## 🧩 What It Does

- Prompts users in the terminal with:
  - `1. Ask a question`
  - `2. Exit`
- Once a user selects "Ask a question", they stay logged in to ask repeatedly until they type `exit`.
- On each question, it searches a local JSON file (`data/qa.json`) for similar entries using fuzzy matching (`jaro_winkler`).
- It returns the most relevant answers from that file (max 2).

---

## 🧠 JSON Structure

The assistant looks for a file at: `data/qa.json`  
The file should be an array of objects with the following format:

```json
[
  {
    "question": "How do I create a new git branch?",
    "answer": "Run: git checkout -b branch-name"
  },
  {
    "question": "How do I pull from remote?",
    "answer": "Use: git pull origin main"
  }
]
```
## 🔍 How Matching Works (static_qa.rs)
- The input question is normalized (case-insensitive, punctuation removed).

- Each stored question in the JSON file is compared using jaro_winkler similarity scoring.

- Only results with similarity score > 0.7 are considered.

- The best 1-2 matches are returned and displayed with a typing effect.

## 🧵 Typing Effect
> The answers are printed character-by-character with a slight delay to simulate natural assistant-like output.

## ▶️ How to Run
- Make sure you have Rust installed.

```
=== Terminal Copilot Assistant ===
1. Ask a question
2. Exit
Enter choice (1 or 2): 1

Type your question (or type 'exit' to logout): How do I create a new git branch?

💡 Answer:
Run: git checkout -b branch-name
```

## 💡 Future Ideas
- Add document (TXT or PDF) search alongside JSON
- Show answer sources (e.g., [From JSON])
- Enable editing or saving answers interactively

