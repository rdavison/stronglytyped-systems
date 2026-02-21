---
title: "Debate as Error Correction"
date: "2026-02-20"
description: "Hallucinations aren't a model problem. They're a single-sample problem. Run the same question through multiple perspectives and the noise cancels out."
tags:
  - "ai-agents"
  - "llm"
  - "systems"
---

Here's something I've noticed while building Coself that I don't see discussed enough: most hallucinations disappear if you just ask twice from different angles.

A single LLM response is a sample from a distribution. Sometimes that sample lands on a confident-sounding falsehood. We call this a hallucination, and the industry treats it like a fundamental flaw — something to be solved with better training data, retrieval augmentation, or chain-of-thought prompting.

But there's a simpler framing. A hallucination is an outlier. And outliers get filtered by averaging.

## The Single-Sample Trap

When you ask a model one question and get one answer, you're drawing a single point from a probability distribution. If that point happens to be in the tail — plausible-sounding but wrong — you have no signal telling you it's wrong. The answer reads the same whether it's accurate or fabricated. You can't distinguish signal from noise with n=1.

This is the fundamental problem with how most people use language models. One prompt, one response, done. It's like measuring a noisy sensor once and treating the reading as ground truth. No engineer would do that with physical sensors. We average. We filter. We take multiple readings and look for convergence. But with LLMs, we accept the first response as gospel.

## What Happens When You Debate

In Coself, significant decisions go through a parliament — multiple personas examining the same question from different angles. The formalist checks logical consistency. The adversary tries to break the argument. The engineer asks whether it's buildable. The philosopher asks whether it's even the right question.

What I've found is that hallucinations almost never survive this process. Not because any single persona is better at catching them, but because fabricated claims tend to be fragile. They only hold up from one angle. The moment you rotate the question — ask "is this logically consistent?" then "what evidence contradicts this?" then "what would this actually look like in practice?" — the hallucination collapses under its own weight.

The real answers, the accurate ones, hold up from every angle. They survive cross-examination. That's the signal.

## Smoothing, Not Voting

This isn't majority voting, where you generate three responses and pick the most common one. Voting treats each response as independent and discrete. What I'm describing is closer to signal averaging — each perspective adds information, and the ensemble converges on something more accurate than any individual sample.

The distinction matters. Voting can amplify systematic biases if the model consistently gets something wrong the same way. Debate from genuinely different perspectives catches those systematic errors because each perspective has different blind spots. The formalist misses practical concerns. The engineer misses conceptual flaws. The adversary misses when something is actually fine. But together, the coverage is nearly complete.

It's the same principle behind ensemble methods in machine learning. A random forest outperforms a single decision tree not because any individual tree is better, but because the errors are uncorrelated across trees. When you average uncorrelated errors, they cancel. What remains is signal.

## What This Looks Like in Practice

A concrete example. I asked the system to evaluate whether a particular API design would handle concurrent writes correctly. A single-pass response gave me a confident, detailed analysis — and it was subtly wrong. It described a race condition that couldn't actually occur given the locking semantics, and missed a real one hiding in a different code path.

When I ran the same question through multiple angles — one pass focused on formal correctness, another on practical failure modes, a third adversarially challenging the first response's claims — the phantom race condition collapsed immediately (the adversary pointed out the lock made it impossible) and the real one surfaced (the engineer asked "what happens if this callback fires before the lock is acquired?").

The corrected output wasn't just more accurate. It was more honest about its own uncertainty. Debate doesn't just filter errors — it calibrates confidence. Claims that survive cross-examination get stated with conviction. Claims that wobble get flagged as uncertain. The system learns to distinguish what it knows from what it's guessing.

## The Cost Objection

The obvious pushback: this is expensive. Multiple passes means multiple API calls. Three perspectives on one question costs three times as much as one answer.

True. But consider what you're comparing against. The cost of a confident hallucination that makes it into a decision isn't measured in tokens. It's measured in wasted effort, wrong directions, and lost trust. If you're using an LLM for anything that matters — anything where being wrong has consequences — the multi-pass cost is cheap insurance.

And you don't need to run the full parliament on everything. Most questions are low-stakes. "Summarize this document" doesn't need adversarial cross-examination. But "is this concurrent data structure correct" absolutely does. The system should know when to invest in accuracy and when a single pass is fine. That judgment about when to debate is itself a valuable architectural decision.

## Why This Isn't Just "Ask Again"

Simply re-running the same prompt and comparing outputs catches some hallucinations, but it's weak. The model tends to land in the same region of its distribution for the same prompt. You get minor variations on the same answer, not genuinely independent perspectives.

The key is perspective diversity. Each pass needs a structurally different angle on the question. Not "answer this again" but "now try to break this answer," "now check this against first principles," "now tell me what's missing." The diversity of the perspectives is what makes the errors uncorrelated. And uncorrelated errors are the ones that cancel when you average.

This is why the parliament has seven personas, not seven copies of the same one. A formalist and an adversary have genuinely different failure modes. Their errors don't correlate. Averaging across them produces something better than either could alone.

## The Takeaway

Hallucination isn't a model problem that will be solved by the next training run. It's a sampling problem that's solved by taking more samples from different angles. Debate, cross-examination, adversarial challenge — these aren't just useful for decision-making. They're error correction mechanisms. They turn a noisy channel into a reliable one.

The models are already good enough. The architecture around them is what determines whether you get noise or signal.

---

*This is the same principle that makes peer review work in science, cross-examination work in law, and ensemble methods work in machine learning. The underlying math hasn't changed. We're just applying it to a new kind of noisy oracle.*

*If this kind of systems thinking is what your team needs, [hire me](/hire-me).*
