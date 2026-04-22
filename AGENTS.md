# AI Usage Documentation (AGENTS.md)

## Summary of AI Collaboration
In this project, I collaborated with Gemini (an AI model) and the AI agent within VS code to build a Personal Finance Tracker in Rust. AI was used for architectural guidance, debugging, and setting up the CI/CD pipeline.

## Log of AI Interactions

### 1. Project Setup & CI/CD
* **How AI helped:** Provided the specific YAML configuration for GitHub Actions and explained the folder structure required for `.github/workflows`.
* **What I learned:** I learned that GitHub Actions looks for specific hidden directories to trigger automated tests.

### 2. Core Logic Design
* **How AI helped:** Suggested using `structs` and `enums` to represent transactions and helped define the `FinanceTracker` implementation.
* **What I learned:** I learned how Rust's ownership model works when adding items to a vector inside a struct.

### 3. Building the actual Tracker
* **How AI helped:** Suggested a way to implement the main code to track transactions, income, etc.

### Making Program Interactive
* **How AI helped:** Gemini provided the loop structure and a helper function to handle stdin and stdout flushing.