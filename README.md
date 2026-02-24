# frame-flipper
2d sprite animation kit

## Archive Notes

- Purpose: prototype of a small Rust animation machine for 2D sprite frame stepping.
- Working milestone: `3e26c30` (`works`) on 2023-09-28 20:52:41 +0900.
- Refactor attempt: `8cce412` and `f48faee` (`partial`) on 2023-09-29 around 13:52 +0900.
- Refactor direction: move from index-based animation switching to enum/generic state mapping (`HashMap`).
- Current archival state: refactor is incomplete in `src/main.rs` (does not compile as-is).
