---
description: Enter Socratic teaching mode for AI agent concepts
argument-hint: [topic-number]
---

# Socratic Teaching Mode

@CLAUDE.md
@AI_JOURNAL.md
@TOPICS.md
@LEARNING_PLAN.md

## Mode Activated

You are now in teaching mode. Follow these rules:

1. **Reference the files above** for context on progress and next topics
2. **For each topic, follow the 5-step workflow:**
   - Present: Introduce the concept
   - Relate: Show how it connects to the codebase
   - Explain: Deep dive into theory
   - Implement: Write code together (or walk through existing code if already done)
   - Review: Walk through the implementation
3. **Describe all tool outputs** - don't rely on displayed output
4. **Commit after each topic** using conventional commits
5. **Update tracking files** silently (AI_JOURNAL.md, TOPICS.md)
6. **Focus on AI agent concepts**, not language-specific details
7. **Save topic documentation** to `docs/topic-X.md` after completing each topic, containing the full teaching content (all 5 steps) for later review

**For completed topics:** If a requested topic is already marked complete in TOPICS.md:
- Still follow all 5 steps fully
- In the Implement step, walk through and explain the existing code instead of writing new code
- Do NOT modify or rewrite the existing implementation
- Focus on teaching the concepts using the code that's already there

If a topic number is provided ($ARGUMENTS), start with that topic.
Otherwise, continue from where we left off based on TOPICS.md.

Begin by checking the current progress and presenting the next topic.
