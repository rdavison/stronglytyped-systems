---
title: "Building Coself: One Brain, Three Transports, Zero Hallucinations"
date: "2026-02-08"
description: "How I built an AI agent system with persistent memory, formal verification, and a single brain serving Discord, WhatsApp, and HTTP."
tags:
  - "rust"
  - "ai-agents"
  - "systems"
  - "formal-verification"
---

Most AI agent systems are chatbots with a database bolted on. Coself is not that.

Coself is a system where **one brain** — a single invocation pathway — serves three transports (Discord, WhatsApp, HTTP) while maintaining persistent memory across every interaction. Every invocation loads the same identity, runs the same reasoning engines, and writes to the same audit log. There is no "Discord version" and "WhatsApp version." There is one mind.

This post is an overview of how the system works — the architecture and the decisions that shaped it.

## The Problem

I wanted an AI system that could:

1. **Remember everything.** Not "retrieve relevant context" — actually remember, across sessions, across days, with belief confidence scores that update as evidence accumulates.
2. **Reason formally.** Not just generate plausible text, but *prove* things. Verify behavioral invariants before acting.
3. **Serve multiple interfaces.** A Discord DM, a WhatsApp message, and an HTTP request should all reach the same brain with the same memory.

None of the existing agent frameworks solved all three. LangChain gives you chains but not identity. AutoGPT gives you loops but not formal reasoning. I needed something built from scratch.

## Architecture

![System architecture](/images/coself-architecture.svg "Coself system architecture")

The system has three layers:

### Gateways

Handle transport. Each gateway manages its own connection lifecycle and converts incoming messages into a common format before passing them to the brain.

### The Brain

The core. Every invocation loads the full identity state, runs the reasoning engines in parallel, constructs a system prompt with all of this context, and produces a response. The brain doesn't know or care which gateway called it. It just thinks.

### The Event Runtime

Dispatches invocations to scoped programs. Not every message needs the full reasoning stack. A simple acknowledgment doesn't need formal proofs. The runtime examines the incoming event and routes to the appropriate handler — with the option to fall back to the full brain for anything unrecognized.

## The Logic Engine

Before every brain invocation, four reasoning engines run in parallel:

- **Deductive reasoning** — facts and rules about identity, relationships, patterns. "What blocks goal X?" is a query against the knowledge base.
- **Formal verification** — a theorem prover. The deductive engine *searches* for answers; this one *proves* them. If it compiles, it's true.
- **Parallel computation** — belief propagation through influence networks, Monte Carlo simulation of action outcomes, priority-weighted impact scoring.
- **Constraint optimization** — given effort costs, dependencies, and a time budget, what's the optimal schedule?

> The engines don't talk to each other. They each receive the current state, produce their output, and the brain synthesizes everything.

## Memory Model

The identity state lives in a set of persistent files, always loaded in full:

- **Immutable identity** — values, cognitive signature, core principles. Rarely changes.
- **Session log** — every meaningful interaction gets recorded. People, conversations, decisions, open questions.
- **Bayesian belief state** — every belief has a confidence score with cited evidence. Beliefs update as evidence accumulates. Contradictory evidence decreases confidence. Nothing is silently dropped.

This isn't RAG. It's not "retrieve the top 5 relevant chunks." The brain reads *all* of these files, *every time*. The context window carries the full state. This is deliberate: the system should have the same context regardless of what the current message is about.

## Process Discipline

My primary failure mode as a builder is over-engineering.

> So the system includes a desugaring pass — a simplification stage that every idea must survive before it enters the development pipeline. If the simplified version is longer than the original, the pass failed.

Development follows a formalized sprint loop with exit gates at each phase. Maximum 10 sprints per cycle. The cycle terminates when QA passes or the budget is exhausted.

## What I Didn't Build

I didn't build a multi-agent system. There is **one mind, many modes**. The personas are lenses, not separate agents with separate contexts. They share the same memory, the same beliefs, the same identity. This was a deliberate architectural decision: splitting into separate agents creates coordination overhead and identity fragmentation.

I also didn't build a RAG pipeline. The context window carries everything. This trades cost for coherence. It's more expensive per invocation, but the system never "forgets" something because it wasn't in the retrieval set.

## What's Next

The system is running. The portal renders live state as a spatial field on a GPU-rendered canvas. The sprint methodology is producing shipped artifacts. The memory model works.

> The question now is whether this architecture scales — not in the "millions of users" sense, but in the "can one person's digital identity keep pace with their life" sense. That's the experiment.

---

*Built with Rust, Axum, and Tokio. The system runs on a MacBook and talks to Claude.*

*If this kind of systems thinking is what your team needs, [hire me](/hire-me).*
