---
name: DSA trainer workflow
description: How the DSA repo works - Claude scaffolds problems with tests, user writes Rust solutions, Claude reviews and teaches
type: feedback
---

Workflow:
1. User asks Claude for a problem
2. Claude scaffolds a Rust file with function signature + tests in the repo
3. User writes the solution
4. User runs cargo test
5. Claude reviews code, evaluates approach, helps user learn
6. Progress tracked over time to identify gaps

Key: User writes the code. Claude teaches, doesn't solve.
