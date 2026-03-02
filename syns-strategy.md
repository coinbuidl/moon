# SYNS Strategy: Processed Intelligence & Structural Anchoring

## Status: ACTIVE
## Date: 2026-03-02
## Target: Layer 2 Wisdom Synthesis (moon distill --mode syns)

---

## 🎯 Strategic Core
The **SYNS pipeline** is the mechanism that transforms high-velocity daily logs into durable, high-signal structural intelligence. It operates as the bridge between "What happened today" and "What must be remembered forever."

## 🏗️ Structural Anchoring (The Memory Header)
To ensure the synthesis model (Gemini 3.1 Pro) maintains its role and preserves the integrity of the long-term memory, **`MEMORY.md`** must always lead with the following directive:

> **#This file is your current memory. The moon daemon should synthesize it with the memory/YYYY-MM-DD.md files to update this file as requested.#**

### Why this works:
*   **Semantic Priming**: Forces the LLM to recognize its role as a "Librarian" before processing the data.
*   **Instructional Persistence**: Prevents the model from accidentally summarizing away the memory system's own operational rules.
*   **Trigger Agnostic**: Covers both the automatic midnight daemon cycles and manual agent-driven updates.

## 🌪️ Execution Modes

### 1. Automatic Synthesis (The Midnight Watch)
*   **Trigger**: M.O.O.N. Daemon.
*   **Schedule**: First cycle after residential midnight.
*   **Default Sources**: Yesterday's daily log (`memory/YYYY-MM-DD.md`) + Current `MEMORY.md`.
*   **Role**: Routine house-cleaning and context compaction.

### 2. Manual/Strategic Synthesis (On-Demand)
*   **Trigger**: Agent command: `moon distill --mode syns [--file <paths>]`.
*   **Selection Logic**: Use the `--file` flag to surgically include specific architectural documents (e.g., MIPs) or multi-day logs when a major milestone is reached.
*   **Role**: Locking in critical breakthroughs or correcting "shallow" memory.

## ⚖️ Weighting & Quality Control
*   **Recency Bias**: Prioritize facts from the current daily log while maintaining the "Durable Decisions" established in the history.
*   **Noise Suppression**: Explicitly filter out tool-use chatter, PTY echoes, and repetitive status updates.
*   **Signal Density**: `MEMORY.md` must remain under **4,000 tokens**. If size limits are reached, the model is instructed to move older "Lessons Learned" into the **Cold Archive** (Librarian/LanceDB).

## ✅ Success Metrics
*   **Zero Loss**: No core architectural decisions are lost during the merge.
*   **High Retrieval**: Information in `MEMORY.md` is immediately "top-of-mind" for the agent at session start.
*   **Structural Stability**: The markdown format (Headings, Bullets, Bolded Terms) remains consistent across every synthesis run.
