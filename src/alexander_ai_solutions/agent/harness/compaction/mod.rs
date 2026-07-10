//! System-prompt cache alignment (the content-compaction engine has moved).
//!
//! The content-aware tool-output compaction that used to live here — content
//! routing, the JSON/diff/log compressors, and the CCR store — has been
//! consolidated into the unified [`crate::alexander_ai_solutions::tokenjuice`] content
//! router (TokenJuice 2.0). The single entry points are
//! [`crate::alexander_ai_solutions::tokenjuice::compact_tool_output`] /
//! [`crate::alexander_ai_solutions::tokenjuice::compact_output`], and recovery is via the
//! `tokenjuice_retrieve` tool (with `retrieve_tool_output` kept as an alias).
//!
//! Only the system-prompt cache-aligner remains here, as it is concerned with
//! KV-cache prefix stability rather than content compression.

pub mod cache_align;
