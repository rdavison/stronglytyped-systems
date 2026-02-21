---
title: "Building Coself: One Brain, Three Transports, Zero Hallucinations"
date: "2026-02-20"
description: "How I built an AI agent system with persistent memory, formal verification, and a single brain serving Discord, WhatsApp, and HTTP."
tags:
  - "rust"
  - "ai-agents"
  - "systems"
  - "formal-verification"
---

Most AI agent systems are chatbots with a database bolted on. Coself is not that.

Coself is a system where one brain — a single invocation pathway — serves three transports (Discord, WhatsApp, HTTP) while maintaining persistent memory across every interaction. Every invocation loads the same identity files, runs the same reasoning engines, and writes to the same audit log. There is no "Discord version" and "WhatsApp version." There is one mind.

This post is an overview of how the system works — the architecture and the decisions that shaped it.

## The Problem

I wanted an AI system that could:

1. **Remember everything.** Not "retrieve relevant context" — actually remember, across sessions, across days, with belief confidence scores that update as evidence accumulates.
2. **Reason formally.** Not just generate plausible text, but prove things. Verify behavioral invariants before acting.
3. **Serve multiple interfaces.** A Discord DM, a WhatsApp message, and an HTTP request should all reach the same brain with the same memory.

None of the existing agent frameworks solved all three. LangChain gives you chains but not identity. AutoGPT gives you loops but not formal reasoning. I needed something built from scratch.

## Architecture

![System architecture](/images/coself-architecture.svg "Coself system architecture")

The system has three layers:

**Gateways** handle transport. Each gateway (Discord, WhatsApp, HTTP) manages its own connection lifecycle — WebSocket heartbeats, webhook verification, Axum routes — and converts incoming messages into a common format. Then it calls `brain::invoke()`.

**The Brain** is the core. Every invocation:

- Loads three identity files (SOUL.md, CONTEXT.md, BELIEFS.md)
- Runs four reasoning engines in parallel
- Constructs a system prompt with all of this context
- Invokes Claude with stream-json output
- Parses the response, writes an audit log, returns the result

The brain doesn't know or care which gateway called it. It doesn't know if you're on Discord or WhatsApp. It just thinks.

**The Event Runtime** dispatches invocations to scoped programs. Not every message needs the full reasoning stack. A simple acknowledgment doesn't need formal proofs. The runtime examines the incoming event, matches it against program triggers, and routes to the appropriate handler — with the option to fall back to the full brain for anything unrecognized.

## The Logic Engine

Before every brain invocation, four reasoning engines run in parallel:

- **Deductive reasoning** — Facts and rules about identity, relationships, patterns. "What blocks goal X?" is a query against the knowledge base.
- **Formal verification** — A theorem prover. The deductive engine *searches* for answers; this one *proves* them. If it compiles, it's true. I use this to verify behavioral invariants about the agent's own architecture.
- **Parallel computation** — Belief propagation through influence networks, Monte Carlo simulation of action outcomes, priority-weighted impact scoring.
- **Constraint optimization** — Given effort costs, dependencies, and a time budget, what's the optimal schedule?

The engines don't talk to each other. They each receive the current state, produce their output, and the brain synthesizes everything.

## Memory Model

Three files, always loaded:

- **SOUL.md** — Immutable identity. Values, cognitive signature, core principles. Rarely changes.
- **CONTEXT.md** — Session log. Every meaningful interaction gets recorded here. People, conversations, decisions, open questions.
- **BELIEFS.md** — Bayesian belief state. Every belief has a confidence score (0.0–1.0) with cited evidence. Beliefs update as evidence accumulates. Contradictory evidence decreases confidence. Nothing is silently dropped.

This isn't RAG. It's not "retrieve the top 5 relevant chunks." The brain reads *all* of these files, *every time*. The context window carries the full state. This is deliberate: the system should have the same context regardless of what the current message is about.

## The Desugaring Pass

Every idea that enters the development pipeline passes through a 3-way tournament:

1. **The Formalist** defines simplification rules and a type system for the idea
2. **The UX Designer** ensures the simplified version actually serves the user
3. **The Adversary** attacks both complexity theater and oversimplification

The output must be shorter than the input. If simplification takes more words than the original idea, the pass failed. This prevents the system (and me) from over-engineering — which, as a builder, is my primary failure mode.

## Sprint Methodology

Development follows a formalized loop: research → desugaring pass → scope → DSL design → implementation → docs → UX feedback → QA → report → recurse. Maximum 10 sprints per cycle. The cycle terminates when QA passes or the budget is exhausted.

The DSL designed during the sprint becomes the contract. Any two systems that want to interoperate must implement and communicate via that spec. No ad-hoc coupling.

## What I Didn't Build

I didn't build a multi-agent system. There is one mind, many modes. The personas (Formalist, Engineer, Child, Adversary) are lenses, not separate agents with separate contexts. They share the same memory, the same beliefs, the same identity. This was a deliberate architectural decision: splitting into separate agents creates coordination overhead and identity fragmentation.

I also didn't build a RAG pipeline. The context window carries everything. This trades cost for coherence. It's more expensive per invocation, but the system never "forgets" something because it wasn't in the retrieval set.

## What's Next

The system is running. The portal renders live state as a spatial field on a GPU-rendered canvas. The sprint methodology is producing shipped artifacts. The memory model works.

The question now is whether this architecture scales — not in the "millions of users" sense, but in the "can one person's digital identity keep pace with their life" sense. That's the experiment.

---

*Built with Rust, Axum, and Tokio. The system runs on a MacBook and talks to Claude.*
