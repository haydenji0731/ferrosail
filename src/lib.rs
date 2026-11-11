//! ferrosail: parasail-compatible pairwise sequence alignment in Rust.
//!
//! Step 1 is a port of Liftoff's NumPy `sg_dx_trace`
//! (`Liftoff_at/src/liftoff/align/semiglobal.py`), which reproduces
//! `parasail.sg_dx_trace_scan_sat`:
//!
//! * `s1` is the *query* (rows `i`), `s2` the *database* (columns `j`).
//! * `sg_dx`: the query aligns end to end; leading and trailing database
//!   bases are free (a "fitting" alignment).
//! * A gap of length `L` costs `open + (L - 1) * extend`.
//!
//! Every function body below is `todo!()`: it compiles, and panics if called.
//! Fill them in one at a time.

// While everything is a stub, the compiler warns that nothing is used yet.
// Delete this line once the functions are implemented.
#![allow(dead_code, unused_variables)]

// ---------------------------------------------------------------------------
// Trace flags, bit-compatible with parasail's PARASAIL_* constants.
// `const` items are inlined at compile time; `u8` is an unsigned byte.
// ---------------------------------------------------------------------------
const INS: u8 = 1; // H came from E
const DEL: u8 = 2; // H came from F
const DIAG: u8 = 4; // H came from the diagonal
const DIAG_E: u8 = 8; // E was opened from H
const INS_E: u8 = 16; // E was extended from E
const DIAG_F: u8 = 32; // F was opened from H
const DEL_F: u8 = 64; // F was extended from F

/// Far below any reachable score, but safely away from integer overflow.
const NEG_INF: i64 = -(1 << 40);

// ---------------------------------------------------------------------------
// Scoring matrix
// ---------------------------------------------------------------------------

/// A substitution matrix over a byte alphabet.
///
/// Mirrors `parasail.matrix_create`: one row/column per alphabet character,
/// plus a final catch-all row/column (scored 0) for unknown characters.
///
/// `#[derive(...)]` asks the compiler to write common trait impls for us:
/// `Debug` (printable with `{:?}`) and `Clone` (explicit `.clone()` copies).
#[derive(Debug, Clone)]
pub struct ScoringMatrix {
    /// Characters with explicit scores.
    alphabet: Vec<u8>,
    /// Number of rows/columns: `alphabet.len() + 1`.
    size: usize,
    /// Row-major `size * size` score table (one flat Vec, not Vec<Vec<_>>).
    scores: Vec<i64>,
    /// Maps every possible byte value to a matrix index.
    mapper: [u8; 256],
}

impl ScoringMatrix {
    /// Build a simple match/mismatch matrix.
    ///
    /// Equivalent to Python's `ScoringMatrix.create(alphabet, match, mismatch,
    /// case_sensitive=...)`. `Self` is shorthand for `ScoringMatrix`.
    pub fn new(alphabet: &[u8], match_score: i64, mismatch: i64, case_sensitive: bool) -> Self {
        todo!()
    }

    /// Overwrite one entry. `&mut self` = we borrow the matrix mutably.
    pub fn set(&mut self, row: usize, col: usize, score: i64) {
        todo!()
    }

    /// Read one entry. `&self` = a shared, read-only borrow.
    pub fn score(&self, row: usize, col: usize) -> i64 {
        todo!()
    }

    /// Map a sequence to matrix indices.
    pub fn encode(&self, seq: &[u8]) -> Vec<u8> {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Result and error types
// ---------------------------------------------------------------------------

/// Result of [`sg_dx_trace`].
///
/// `PartialEq` lets tests compare two alignments with `assert_eq!`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alignment {
    /// Optimal alignment score.
    pub score: i64,
    /// Zero-based query index of the alignment end.
    pub end_query: usize,
    /// Zero-based database index of the alignment end.
    pub end_ref: usize,
    /// Gapped query, covering the whole query and database.
    pub query_traceback: String,
    /// Gapped database, aligned column-for-column with `query_traceback`.
    pub ref_traceback: String,
}

/// What can go wrong. Rust returns errors as values (`Result`), not exceptions.
///
/// Note there is no "negative penalty" variant: penalties are `u32`, so the
/// type system already rules that out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlignError {
    /// The query or database sequence was empty.
    EmptySequence,
}

// ---------------------------------------------------------------------------
// Alignment
// ---------------------------------------------------------------------------

/// Semi-global alignment with free end gaps in `s2`, with traceback.
///
/// Equivalent to `parasail.sg_dx_trace_scan_sat(s1, s2, open, extend, matrix)`.
/// Sequences are byte slices (`&[u8]`), e.g. `b"ACGT"`, not `&str`.
pub fn sg_dx_trace(
    s1: &[u8],
    s2: &[u8],
    open: u32,
    extend: u32,
    matrix: &ScoringMatrix,
) -> Result<Alignment, AlignError> {
    todo!()
}

/// Fill the DP matrices and record trace flags.
///
/// Returns `(trace, best_score, end_ref)`, where `trace` is a row-major
/// `s1.len() * s2.len()` table of flag bytes. Start with a plain cell-by-cell
/// double loop; worry about speed later.
fn fill(
    s1: &[u8],
    s2: &[u8],
    open: i64,
    extend: i64,
    matrix: &ScoringMatrix,
) -> (Vec<u8>, i64, usize) {
    todo!()
}

/// Walk trace flags back from `(i, j)` and build the gapped strings.
///
/// Hint: Python lets `i` and `j` reach -1; `usize` cannot go below 0, so
/// decide how to represent "exhausted" (e.g. loop on `i + 1` counts instead).
fn traceback(s1: &[u8], s2: &[u8], trace: &[u8], i: usize, j: usize) -> (String, String) {
    todo!()
}

// ---------------------------------------------------------------------------
// Tests: run with `cargo test`. Remove `#[ignore]` as you implement things.
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "ScoringMatrix::new not implemented yet"]
    fn matrix_has_catch_all_row_and_column() {
        let m = ScoringMatrix::new(b"ACGTacgt*", 2, -4, true);
        assert_eq!(m.size, 10);
        assert_eq!(m.score(0, 0), 2); // A-A match
        assert_eq!(m.score(0, 1), -4); // A-C mismatch
        assert_eq!(m.score(9, 3), 0); // unknown scores zero
    }

    #[test]
    #[ignore = "sg_dx_trace not implemented yet"]
    fn empty_sequence_is_an_error() {
        let m = ScoringMatrix::new(b"ACGT", 2, -4, false);
        assert_eq!(
            sg_dx_trace(b"", b"ACGT", 10, 1, &m),
            Err(AlignError::EmptySequence)
        );
    }
}
