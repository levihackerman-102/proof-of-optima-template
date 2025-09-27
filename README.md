# Proof-of-Optima Template Repository

A template repository for contest organizers to create zero-knowledge proof-based optimization competitions where contestants can prove they've found optimal solutions without revealing their actual solutions or methods.

## Overview

This template enables contest organizers to quickly set up competitions where participants submit zero-knowledge proofs of optimization results rather than the solutions themselves. Contestants prove properties like "my solution achieves error ≤ X" or "my algorithm found a path with cost ≤ Y" while keeping their implementation and results private.

## Key Features

- Privacy-preserving competitions: Contestants prove solution quality without revealing methods

- Verifiable results: All submissions are cryptographically verifiable on-chain or off-chain

- Flexible problem types: Supports various optimization problems (ML model training, pathfinding, scheduling, etc.)

- RISC Zero zkVM integration: Built on proven zero-knowledge infrastructure

- Automated verification: Contest organizers can verify all submissions programmatically

## Proof generation

Run the example with:

```bash
cargo run --release
```

Congratulations! You just constructed a zero-knowledge proof that you know an optimal path for a given TSP problem.

### What gets proven?

The [receipt] proves that the [guest program] was executed correctly, and that
the contents of `receipt.journal` match what was written by `env::commit()`
during the execution of the guest program.

By running the demo, Alice demonstrates that she knows an optimal path to take for the TSP according to the graph given in `receipt.journal`. Thus, Alice proves that she knows a path of her claimed length written in `receipt.journal` without revealing any further information.