---
title: "Dark Factories and the Case for Building Your Own AI"
date: "2026-02-12"
description: "Everyone's talking about autonomous software factories. I built a personal one — an AI assistant that remembers everything, reasons before acting, and runs across every device I use."
tags:
  - "ai-agents"
  - "systems"
  - "rust"
---

There's a video making the rounds about "dark factories" — fully autonomous software teams where no human writes or reviews code. Specs go in, working software comes out. Three engineers running an entire product. Lights off.

It's real. A handful of teams are genuinely operating there. But most of the industry is stuck at what one framework calls "level two" — using AI as a slightly faster junior developer, getting measurably slower while believing they're speeding up.

I've been thinking about this gap from a different angle. Instead of trying to automate a team, I asked: what if I automated *myself*?

## What I Built

Coself is a personal AI assistant I built from scratch. It runs on my phone, my laptop, and my desktop. I can talk to it over Discord, WhatsApp, or a web browser. It doesn't matter which — they all reach the same brain.

That last part is the key architectural decision. There's one brain, not three chatbots. Every message, regardless of where it comes from, goes through the same invocation path. The system loads the same memory, runs the same reasoning, writes to the same log. There is no "Discord version" that forgot what I said on WhatsApp.

![System architecture](/images/coself-architecture.svg "Coself system architecture")

## It Remembers Everything

Not "retrieves relevant context." Actually remembers. The full identity state — conversations, decisions, beliefs, open questions — loads on every single invocation. The failure mode of most AI assistants is amnesia. You have the same conversation three times because the system forgot the first two. Mine doesn't forget.

## It Reasons Before Acting

Before every response, multiple reasoning engines run in parallel. The system doesn't just *sound* right — it has actually checked its work against formal logic, numerical analysis, and constraint optimization before the language model ever generates a response.

## It Does Real Work

This isn't a toy. Here's what it handles for me on a regular basis:

- **Trip planning.** It researches logistics, compares itineraries, and coordinates multi-step travel planning including bookings and appointments.
- **Email triage.** It connects to my Gmail, archives junk, and surfaces the messages that actually need my attention.
- **Sprint management.** It runs a formalized development workflow — research, scope, implement, test, document — with structured QA feedback loops.
- **Writing.** You're reading a draft it helped produce right now. Having an AI that knows my voice and can get words on the page when I'm stuck is a genuine quality-of-life improvement.
- **Scheduling and coordination.** It tracks what I need to do, when, and what depends on what.

## Why I Built It From Scratch

There are plenty of AI assistant products out there. I could have used one. But they all share the same limitation: they don't know me.

A generic assistant starts every conversation from zero. It doesn't know my preferences, my relationships, my ongoing projects, my belief system. It can't reason about my life because it has no model of my life.

Building from scratch meant I could design the memory model, the reasoning pipeline, and the identity system exactly the way I needed them. The system is built in Rust. It's fast, it's typed, and it doesn't crash.

## The Dark Factory Connection

The video about dark factories talks about a future where specs go in and software comes out. That's interesting for teams. But I think there's a version of this for individuals that's underexplored.

Most people don't need a software factory. They need an assistant that actually works — one that remembers what they said last week, that can reason about tradeoffs instead of just generating text, that can handle real tasks across real systems.

The gap between "chatbot with a database" and "assistant that knows you" is enormous. It's the same gap the video describes between level two and level five. The difference isn't the model — it's the architecture around the model. Memory, reasoning, identity, multi-transport. That's what makes the system useful instead of just impressive.

I built Coself because I wanted an AI that could keep up with my life. It does. And every time the underlying models improve, the system gets better without me changing the architecture. The brain is the constant. The intelligence is the variable.

---

*Built with Rust, Axum, and Tokio. Runs on a MacBook. Talks to Claude.*

*If this kind of systems thinking is what your team needs, [hire me](/hire-me).*
