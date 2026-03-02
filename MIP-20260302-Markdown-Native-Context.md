# MIP-20260302: Markdown-Native Active Context Architecture

## Status: Draft
## Author: Lilac Livint (per Master's Vision)
## Target: OpenClaw Gateway Memory Engine

---

### 1. Abstract
The current OpenClaw memory model relies on `sessions/*.jsonl` for active context, which contains significant "structural noise" (raw tool JSON, internal IDs, and verbose metadata). This proposal outlines a shift toward a **Markdown-Native Context Window**, where the primary active memory is sourced from M.O.O.N. Projection files (`mlib/*.md`). This shift aims to reduce token consumption by ~70-90% while maintaining high-signal semantic continuity.

---

### 2. The Problem: The "JSONL Bloat"
- **Redundancy**: JSONL files store tool results, internal signatures, and parent IDs that are rarely needed for logical reasoning but consume massive amounts of the 200k token window.
- **Cost**: High-frequency compactions (triggered by noise) lead to excessive token costs.
- **Signal-to-Noise**: During deep troubleshooting, the model's focus can be distracted by malformed or overly large raw JSON blobs.

---

### 3. Proposed Architecture: "Projection-First" Boot

#### Tier A: The Core Bootstrap (Static)
Remains as is: Identity, User Profile, Core Skills (loaded from `openclaw.json`).

#### Tier B: The Active Window (Markdown-Native)
Instead of loading `session.jsonl`, the Gateway boots the session using the latest **M.O.O.N. Projection (`mlib/*.md`)**.
- **Signal**: The projection uses the "Timeline Table" and "Thematic Summaries" already optimized by the SYNS pipeline.
- **Persistence**: Tool state (pending callbacks) is handled via a dedicated `state.json` rather than within the conversation context.

#### Tier C: The "Librarian" Bridge (On-Demand)
If the model needs the *exact* raw JSON of a past tool result (e.g., for debugging a specific API failure):
- It uses `moon recall` or a new `raw-fetch` tool to pull the specific JSONL line from the archive.
- This keeps the raw data out of the "Head" (RAM) until explicitly needed.

---

### 4. Implementation Steps
1.  **Refine Projection Format**: Update `moon watch` to include "Tool State Markers" in the `mlib/*.md` files so the model knows a task is in progress.
2.  **Gateway Loader Update**: Implement a `context_source: "projection"` toggle in `openclaw.json`.
3.  **Hot-Reload Logic**: Enable the Gateway to watch for new `.md` updates from the MOON daemon and update the active RAM context window in real-time.

---

### 5. Expected Metrics
- **Context Capacity**: 200k tokens of "Markdown wisdom" can hold approximately 10x more narrative history than "JSONL raw history."
- **Compaction Frequency**: Reduction from ~20+ compactions per day to <3 per day.
- **Economic Yield**: Projected 80% reduction in daily token expenditure.

---

### 6. Master's Strategic Goal
*“Move from a Raw History context to a Processed Wisdom context.”*

---
**Lilac's Note**: *Master, I have drafted this based on our discussion about the 90% size reduction. I am ready to begin the feasibility study on the Gateway Loader if you approve.*
