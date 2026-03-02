# MIP-MEMORY-ARCHI: Recency-Weighted Semantic Architecture

## Status: PROPOSED
## Date: 2026-03-01
## Author: Lilac Livint (Assistant) / Brian (Master)

---

## 🎯 Objective
Establish a formal "Recency-First" weighting strategy for the M.O.O.N. memory system to optimize for high-velocity development context while maintaining long-tail architectural integrity.

## 🏗️ The Multi-Tier Weighting Stack

### 1. Tier 0: The Active Context (Highest Weight)
*   **Source**: Current OpenClaw session (`moon watch` managed).
*   **Window**: Fixed at **200,000 tokens** with a **0.50 (50%)** compaction ratio.
*   **Weight**: 1.0 (Absolute Priority).
*   **Role**: Real-time logic and immediate instruction flow.

### 2. Tier 1: The SYNS Pipeline (Processed Intelligence)
*   **Source**: `MEMORY.md` and `memory/YYYY-MM-DD.md`.
*   **Weight**: 0.8.
*   **Role**: High-signal summaries, durable decisions, and "Wisdom" synthesized by Stage 2 models (e.g., Gemini 1.5 Pro).

### 3. Tier 2: Warm Recall (Recency-Biased Search)
*   **Source**: LanceDB `history` collection (last 72 hours).
*   **Weight**: 0.6.
*   **Policy**: `moon recall` should prioritize the top 3 hits from the most recent session projections (`archives/mlib/`).
*   **Implementation**: Agents must check the `projection_date` metadata before assuming a hit is the "current" truth.

### 4. Tier 3: Cold Recall (Long-Tail Retrieval)
*   **Source**: Deep archives (> 72 hours).
*   **Weight**: 0.3.
*   **Role**: Historical "Why" behind old decisions. Used only when Tier 0-2 return `HISTORY_NOT_FOUND` or contradictory signals.

---

## 🛠️ Implementation Requirements

1.  **Metadata Tagging**: All `moon index` projections must include a deterministic `ISO-8601` timestamp in the frontmatter.
2.  **Recall Logic**: When calling `moon recall`, the agent should append temporal hints (e.g., "recent decisions on X" vs "original design of X").
3.  **Librarian Boost**: Future `moon` updates should explore `order by timestamp desc` boosting in the LanceDB vector search query.

---

## ✅ Success Metrics
*   **Zero Regression**: No "hallucinated" old ratios (like the 0.78 incident) when a 0.50 exists in recent context.
*   **Noise Reduction**: Reduced token usage in recall-heavy sessions.
*   **Latency**: Faster response times by avoiding unnecessary deep-archive reads.
