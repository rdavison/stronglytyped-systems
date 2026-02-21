---
title: "Why I Built My Own AI Instead of Using OpenClaw"
date: "2026-02-05"
description: "OpenClaw is impressive. It scared me a little. So I built my own from scratch — and ended up somewhere architecturally different."
tags:
  - "ai-agents"
  - "architecture"
  - "rust"
---

I need to start with honesty: OpenClaw scared me.

Not in the "AI is going to take over" sense. In the "this is clearly the future and I'm watching someone else build it" sense. When I saw what Peter Steinberger had done — a personal AI assistant that runs locally, talks to you on WhatsApp, controls your browser, manages your calendar, sends your emails — my first reaction was envy. My second was fear. My third was: I need to build this myself.

Not because OpenClaw is bad. It's genuinely impressive. But because the engineer in me couldn't accept using someone else's architecture for something this personal. An AI assistant that knows everything about your life, that reads your messages and manages your schedule — that's not a tool you download. That's infrastructure you need to understand at the deepest level. And the only way I understand something is by building it.

So I did. And what came out is architecturally different from OpenClaw in ways I didn't expect.

## The Fork in the Road: Multi-Agent vs. One Mind

The biggest architectural difference is philosophical. OpenClaw uses a multi-agent model. You can spin up isolated agents — a work agent on Slack with access to project management tools, a personal agent on WhatsApp for everyday tasks. Each agent gets its own workspace, its own session history, its own tool access. They're "fully scoped brains" that don't share context.

Coself does the opposite. There is one brain. Every message, regardless of transport — Discord, WhatsApp, HTTP — goes through the same invocation path, loads the same memory, runs the same reasoning. There are no isolated agents. There are perspectives within a single identity.

This isn't a matter of one being right and the other wrong. It's a tradeoff between isolation and coherence.

OpenClaw's isolation is a feature when multiple people share a server, when you want strict boundaries between work and personal contexts, when security demands that your Slack agent can't access your WhatsApp history. It's clean, it's well-architected, and it scales to teams.

Coself's coherence is a feature when you're building for one person. I don't want my work context isolated from my personal context — they inform each other. The visa appointment I'm booking affects my calendar which affects my work schedule which affects what I tell my wife. Splitting those into separate agents with separate memories means each one is reasoning with incomplete information.

One mind means every invocation has the full picture. It's more expensive per call — the context window carries everything — but nothing is ever missing because it was in a different agent's workspace.

## Memory: Retrieval vs. Full Load

OpenClaw stores conversations, long-term memory, and skills as plain Markdown and YAML files in your workspace. When context is needed, the relevant pieces get retrieved and injected into the prompt. It's the standard RAG-adjacent pattern: store everything, retrieve what's relevant.

Coself doesn't retrieve. It loads everything. Every invocation reads three complete files — identity, conversation history, and belief state — in their entirety. The context window carries the full state every time.

This is deliberately wasteful. It costs more tokens. It uses more of the context window. But it means the system never "forgets" something because it wasn't in the retrieval set. There's no relevance scoring to get wrong. There's no query that misses a critical piece of context because the embedding didn't capture the right semantic relationship.

The tradeoff is obvious: this doesn't scale to years of conversation history. At some point, the context window fills up and you need a different strategy. But for now, for one person's life over weeks and months, the full-load approach means the AI has the same context I have. It knows what I know. Nothing falls through the cracks.

## Reasoning: LLM-Only vs. Multi-Engine

This is where the architectures diverge most sharply.

OpenClaw's workflow is: message arrives, intent gets parsed, relevant context retrieved, appropriate tools selected, actions executed, response delivered. The LLM handles the reasoning. The system around it handles the plumbing.

Coself adds a layer that OpenClaw doesn't have: before the LLM ever sees the prompt, four reasoning engines run in parallel. Deductive logic queries a knowledge base of facts and relationships. A formal verifier checks structural invariants. A parallel computation engine runs numerical analysis — belief propagation, Monte Carlo simulations, impact scoring. A constraint solver finds optimal schedules given effort costs and dependencies.

The LLM synthesizes all of this into a response. But it's not reasoning from scratch. It's reasoning from a foundation of verified facts, proven invariants, and computed optima. The difference is subtle but compounding: over time, the system accumulates formal knowledge that the LLM can build on, not just conversation transcripts.

Is this overkill for "remind me to buy groceries"? Absolutely. But for "what's the optimal order to tackle these five life priorities given their dependencies and my available time this week" — that's a constraint optimization problem, not a vibes problem. The formal machinery earns its keep on the hard questions.

## Identity: Configurable vs. Persistent

OpenClaw agents are configurable. You define a personality in a SOUL.md file, set up tool access, bind it to channels. You can create, modify, or delete agents as needed. The agent is a configuration artifact.

Coself's identity isn't configurable — it's persistent and evolving. The system maintains a Bayesian belief state where every belief has a confidence score with cited evidence. Beliefs update as evidence accumulates. They never get silently dropped. If something contradicts a prior belief, the confidence adjusts and the contradiction is recorded.

This means the system has opinions. Not hardcoded opinions — evidence-weighted probabilistic assessments that shift over time. It knows what it thinks, why it thinks it, and how confident it is. When new information arrives, it doesn't just store it — it integrates it into an existing model of the world.

OpenClaw's approach is more flexible. You can reconfigure, reset, start fresh. Coself's approach is more opinionated. The system accumulates understanding over time, and that understanding shapes every future interaction.

## Skills vs. Programs

OpenClaw has a rich skills ecosystem. Hundreds of community-built skills for email processing, data analysis, media control. The AI can even generate and install new skills autonomously. It's an extensible platform with a thriving community.

Coself has typed event dispatch. Incoming events match against program triggers and route to scoped handlers — lightweight programs for common patterns, with a fallback to the full reasoning stack for anything unrecognized. Programs are defined in Rust, not YAML. They're compiled, typed, and verified at build time.

This is the builder vs. platform tradeoff. OpenClaw is a platform designed for a community of users. Coself is a system designed for one user. OpenClaw needs the flexibility of runtime-configurable skills because it can't know what its users will need. Coself can compile its capabilities because I know exactly what I need — and if I need something new, I add it to the codebase and rebuild.

## The Gateway: Broad vs. Deep

OpenClaw's Gateway is genuinely impressive. WhatsApp, Telegram, Slack, Discord, Google Chat, Signal, iMessage, Teams, Matrix — over a dozen channels, with sophisticated routing rules, per-agent bindings, compound matching conditions. It's a complete orchestration layer.

Coself supports three transports: Discord, WhatsApp, and HTTP. That's it. But each one feeds into a brain that runs four reasoning engines, loads a full identity model, and writes to an immutable audit log. The Gateway is simpler; the brain is deeper.

This reflects different design priorities. OpenClaw optimizes for reach — meet users wherever they already are. Coself optimizes for depth — make every interaction count, regardless of which transport delivered it.

## What I Actually Learned

Building from scratch forced me to make architectural decisions that I wouldn't have made if I'd started from OpenClaw's codebase. The one-mind model, the full-load memory, the multi-engine reasoning — these aren't features I would have added to an existing platform. They're consequences of starting from a different premise.

OpenClaw asks: how do we build a personal AI assistant that works for everyone?

Coself asks: how do we build a personal AI assistant that works for me?

Those questions lead to different architectures. OpenClaw's is better for most people. It's more flexible, more extensible, more community-supported. If you want a personal AI assistant and you're not an engineer who needs to understand every line of code in the stack, use OpenClaw. Seriously.

But if you're the kind of person who can't use a system they didn't build — if you need to know why every architectural decision was made and whether it was the right one — then building from scratch teaches you things that no amount of configuration can. I learned more about AI system design in a month of building Coself than I did in a year of using other people's tools.

The fear I felt watching OpenClaw wasn't about being left behind. It was about watching someone else define the architecture of something deeply personal. The only cure for that kind of fear is to build your own.

---

*OpenClaw is open source and actively maintained. You should check it out. Coself is what happened when I couldn't stop myself from building it anyway.*

*If this kind of systems thinking is what your team needs, [hire me](/hire-me).*
