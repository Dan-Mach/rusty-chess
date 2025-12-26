# Logic Errors Report for rusty-chess

**Date**: 2025-12-26
**Repository**: Dan-Mach/rusty-chess
**Analysis Type**: Comprehensive Logic Error Review

## Executive Summary

This report documents logic errors discovered in the rusty-chess repository. A total of **2 critical logic errors** and **1 medium-severity issue** were identified that could cause incorrect chess game behavior.

---

## Critical Logic Errors

### 1. En Passant Detection Bug (CRITICAL)
**Location**: `engine/src/board.rs`, line 211  
**Severity**: Critical  
**Impact**: En passant target square is never set correctly, breaking en passant captures

**Description**:
The code incorrectly compares the destination rank with the source file when checking for pawn double-pushes:

```rust
// INCORRECT CODE (line 211)
if (to_r_enum.to_index() as i8 - from_f_enum.to_index() as i8).abs() == 2 {
```

This compares:
- `to_r_enum` (destination **rank**)
- `from_f_enum` (source **file**)

**Expected Behavior**:
Should compare destination rank with source rank:

```rust
// CORRECT CODE
if (to_r_enum.to_index() as i8 - from_r_enum.to_index() as i8).abs() == 2 {
```

**Example**:
- Move e2 to e4 (white pawn double push)
  - from_rank = 1 (Second rank)
  - to_rank = 3 (Fourth rank)
  - from_file = 4 (E file)
  - Current check: abs(3 - 4) = 1 ❌ (fails, no en passant set)
  - Correct check: abs(3 - 1) = 2 ✓ (should succeed)

**Consequences**:
- En passant target squares are never properly set after pawn double-pushes
- En passant captures become impossible or behave incorrectly
- Game rules are violated

---

### 2. Move Constructor Ignores Promotion Parameter (CRITICAL)
**Location**: `engine/src/genmove.rs`, line 13-14  
**Severity**: Critical  
**Impact**: Promotion moves cannot be created using the standard constructor

**Description**:
The `Move::new()` constructor accepts a `promotion` parameter but ignores it:

```rust
// INCORRECT CODE (lines 13-14)
pub fn new(from: Square, to: Square, promotion:Option<Piece>) -> Self {
    Move { from, to, promotion: None }  // Always sets promotion to None!
}
```

**Expected Behavior**:
Should use the provided promotion parameter:

```rust
// CORRECT CODE
pub fn new(from: Square, to: Square, promotion: Option<Piece>) -> Self {
    Move { from, to, promotion }
}
```

**Consequences**:
- Callers using `Move::new()` with a promotion piece will have it silently ignored
- Only `Move::new_promotion()` works correctly for promotions
- Inconsistent API behavior
- Potential bugs if code relies on `Move::new()` for promotions

**Note**: The codebase appears to use `Move::new_promotion()` in practice, but this is still a critical bug waiting to happen.

---

### 3. Coordinate System Inconsistency (MEDIUM)
**Location**: `engine/src/coordinates.rs`, line 21  
**Severity**: Medium  
**Impact**: Potential confusion and errors in array indexing

**Description**:
The coordinate conversion has an inconsistency in how it maps squares to array indices:

```rust
// Line 21 in square_to_array_indices
let array_rank_idx = 7usize.saturating_sub(rank_val);
```

This flips the rank coordinate (rank 0 becomes array index 7, rank 7 becomes array index 0), which means:
- Array index 0 represents the 8th rank (from white's perspective)
- Array index 7 represents the 1st rank

While this is a valid design choice (FEN notation uses this convention), it requires careful attention throughout the codebase.

**Potential Issues**:
- The comment on line 18 says "0 for 1st rank (Rank::First), 7 for 8th rank (Rank::Eighth)" but this refers to the `rank_val` variable, not the array index
- This could be confusing and lead to off-by-one errors
- The array coordinate system is inverted from the square coordinate system

**Current Impact**: 
The code appears to handle this correctly in most places, but the mixing of coordinate systems increases cognitive load and error potential.

---

## Minor Issues

### 4. Unused Function in main.rs (LOW)
**Location**: `engine/src/main.rs`, line 7-10  
**Severity**: Low  
**Impact**: Dead code, no functional impact

**Description**:
The `undo_move()` function is defined but only prints a message without actually undoing moves:

```rust
pub fn undo_move(board: &mut Board, game_move: &Move) {
    println!("Undoing move from {} to {}", game_move.from, game_move.to);
    // No actual undo logic
}
```

The `Board` struct has its own `undo_move()` method that works correctly. This function appears to be leftover test/debug code.

---

## Additional Observations

### Compiler Warnings
The project generates 27 warnings during compilation, including:
- Unused imports (22 instances)
- Unused variables (3 instances)
- Dead code (1 instance)
- Non-local impl definitions (4 instances from the `failure` crate)

While these are not logic errors, they indicate code quality issues.

### Test Coverage
The existing tests pass successfully:
- 7 tests all passing
- Tests cover checkmate, stalemate, and basic move generation
- However, tests do not catch the en passant bug or the Move::new() bug

**Recommendation**: Add specific tests for:
1. En passant captures after pawn double-pushes
2. Promotion moves using Move::new()
3. Edge cases in coordinate conversions

---

## Recommendations

### Immediate Actions Required

1. **Fix En Passant Bug** (Priority: CRITICAL)
   - Change line 211 in board.rs from `from_f_enum` to `from_r_enum`
   - Add test cases for en passant scenarios
   - Verify en passant works correctly after fix

2. **Fix Move Constructor** (Priority: CRITICAL)
   - Update Move::new() to use the promotion parameter
   - Add tests to verify promotion parameter is respected
   - Consider deprecating Move::new() in favor of Move::new_quiet() and Move::new_promotion()

3. **Improve Coordinate System Documentation** (Priority: MEDIUM)
   - Add comprehensive documentation explaining the coordinate systems
   - Include diagrams showing array layout vs. square numbering
   - Add validation tests for coordinate conversions

4. **Clean Up Code** (Priority: LOW)
   - Run `cargo fix` to address compiler warnings
   - Remove unused undo_move() function from main.rs
   - Consider updating the `failure` crate dependency

### Testing Strategy

Add comprehensive test cases for:
- All en passant scenarios (white/black, different files)
- Promotion using both Move::new() and Move::new_promotion()
- Boundary conditions in coordinate conversions
- Edge cases for pawn double-pushes

---

## Conclusion

The rusty-chess repository contains **2 critical logic errors** that affect core chess functionality:
1. En passant detection is broken due to comparing rank with file
2. Move constructor ignores the promotion parameter

Both issues should be fixed immediately to ensure correct chess game behavior. The codebase shows good structure and comprehensive FEN parsing, but would benefit from additional test coverage, especially for special moves like en passant and pawn promotion.

---

**Prepared by**: GitHub Copilot Code Analysis
**Review Status**: Ready for Developer Review
