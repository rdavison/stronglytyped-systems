---
title: "Agent Swarms Are Not What You Think"
date: "2026-02-16"
description: "I built a system where seven personas debate every decision and a sprint factory turns the output into shipped code. Here's what I learned about agent coordination."
tags:
  - "ai-agents"
  - "systems"
  - "methodology"
---

When people hear "agent swarm," they picture a hundred AI agents all doing different things at once. One writes code, one writes tests, one deploys, one monitors. A distributed workforce of digital interns.

That's not what I built. What I built is closer to a parliament.

## Seven Voices, One Mind

Coself runs a deliberation system with seven personas. They're not separate agents with separate contexts — they're lenses on the same identity, sharing the same memory, the same beliefs, the same values. The distinction matters. Separate agents create coordination overhead. Personas sharing a substrate create depth.

The seven: a formalist who thinks in type systems, an engineer who ships under constraints, a musician who understands rhythm and embodied knowledge, a philosopher who names things precisely, a child who asks "what if," a poet who makes the invisible visible, and an adversary whose only job is to attack every position including their own.

When a significant decision needs to be made, I convene a parliament. Each persona writes a position paper. Then they cross-examine each other. Over two or three rounds, positions evolve, merge, or get destroyed. What survives is sharper than anything a single perspective produces.

## What Actually Happens in a Parliament

Let me give a concrete example. I wanted to build a creativity engine — a system that could generate novel ideas by combining concepts from different domains. The first round of parliament produced an ambitious design: constraint solvers finding optimal combinations, theorem provers verifying coherence, Monte Carlo simulations exploring the possibility space.

It sounded impressive. It was also completely wrong.

By round two, the adversary had torn the elaborate machinery apart. The core insight: the system isn't *producing* creativity. It's *provoking* it. In me. The difference between those two framings killed about 80% of the design. You don't need a constraint solver to generate provocations. You need random juxtaposition and a way to rank the output.

By round three, the honest version emerged: pick two concepts from different domains, mash them together, let a language model evaluate whether the collision is interesting. If it is, present it to me. If not, try again. That's it. The three-round parliament took a multi-engine formalization and reduced it to something I could actually ship.

The adversary's parting observation stuck with me: *the gap between the elaborate pitch and the honest simple version is where the engineering addiction lives.* I've been carrying that around since.

## The Desugaring Pass

That parliament debate produced something I didn't expect: a permanent stage in my development pipeline.

Every idea that enters the system now passes through what I call a desugaring pass — borrowed from compiler terminology, where syntactic sugar gets reduced to its core form. Multiple perspectives evaluate the idea, attacking it from both directions: overcomplexity ("this is engineering theater") and dangerous oversimplification ("you removed the load-bearing wall").

The rule: the output must be shorter than the input. If your simplification takes more words than the original idea, the pass failed. You made it more complicated, not less.

This has saved me from myself more times than I'd like to admit. My failure mode as a builder is over-engineering. I reach for formal machinery the way some people reach for their phone — reflexively, before checking whether the situation actually calls for it. The desugaring pass catches that reflex and asks: do you actually need this, or does it just feel good to build?

## Sprint Factories

The other half of the system is a formalized sprint methodology. Not "agile" in the corporate sense — more like a compiler that takes a domain and produces shipped artifacts through a defined sequence of phases.

Each phase has exit gates — predicates that must hold before advancing. If the gates don't pass, you don't advance. It's the same principle as a type checker: if it doesn't compile, it doesn't ship. Maximum ten sprints per cycle.

This sounds rigid, and it is — deliberately. The rigidity prevents the thing that kills most personal projects: scope creep disguised as "making it better."

I shipped an event runtime in four sprints this way. Kernel types and routing, brain refactor and gateway wiring, dispatch switch, then performance tuning. Each sprint had clear gates, clear deliverables, and a clear exit. The whole thing took less time than I would have spent debating the architecture in my head.

## What I've Learned

Three things, mostly.

**Agent swarms are a coordination problem, not a scale problem.** Adding more agents doesn't help if they can't synthesize. Seven personas sharing context produces better output than seventy agents with separate memories. Depth beats breadth.

**Formalism is a tool, not a destination.** The desugaring pass exists because I kept confusing "building formal machinery" with "making progress." Sometimes the formal version is right. Often, the naive version works and the formal version is procrastination wearing a lab coat.

**The hardest part is letting go of the code.** The dark factory framing from the previous post applies here too. The sprint factory works best when I stop trying to control the implementation and focus on writing clear specs. The spec is the creative work. The implementation is execution. If I can describe what I want precisely enough, the system handles the rest.

That last point is the one I'm still learning. I'm a builder. My instinct is to reach for the keyboard. The sprint factory asks me to reach for the specification instead. It's a different muscle, and it's harder than it sounds.

---

*All of this runs on a single MacBook. The parliament debates happen in a conversation. The sprint factory is a methodology enforced by the system itself. No cloud orchestration, no Kubernetes, no fleet of VMs. Just structured thinking applied consistently.*

*If this kind of systems thinking is what your team needs, [hire me](/hire-me).*
