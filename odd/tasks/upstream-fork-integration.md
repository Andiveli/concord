# Upstream fork integration
Status: active. Base: `origin/main` at `9292170`; target: `upstream/main` at `5bac7a2` (verify before merge).
Branch: `integration/fork-current`. Do not overwrite user-generated untracked directories.

## Specs
S1. "Okay, ahora vamos a la integración" — integrate current upstream into the fork-based branch.
S2. "Podemos actualizar a upstream, pero conservando nuestros cambios?" — retain fork changes during integration.
S3. "no quiero hacerlo con cargo porque no tengo espacio ahora" — avoid a local Cargo install or space-heavy build.

## Tasks
T1 [S1] Fetch and inspect upstream merge candidates; route: inline; commit: none (not explicitly requested). DONE: upstream/main `5bac7a2`; merge-tree reports seven conflicts (media/avatar, runtime/media_runtime, runtime/placement, runtime/redraw, state.rs, ui.rs, hit_test.rs modify/delete).
T2 [S1-S2] Integrate and resolve conflicts without dropping fork features; route: inline + bounded media/UI workers; commit: pending explicit user authorization. Merge staged with eight conflicts resolved; source review retained fork icons/redraw and upstream mouse/KLIPY, but not compiled.
T3 [S1-S3] Verify source integration with proportionate checks and independent read-only review; route: verify; commit: pending explicit user authorization. Structural checks passed; functional Cargo checks blocked by user's disk constraint and CI requires authorized commit/push.

## Log
L1. Okay, ahora vamos a la integración
L2. Context: current branch `integration/fork-current` has clean tracked files and untracked `.codegraph/`, `.playwright-mcp/`; upstream is 55 commits ahead of its old common base. User previously required preserving fork changes and avoiding local Cargo installation due disk space.
L3. Evidence: fetched upstream/main `5bac7a2`; read-only `git merge-tree --write-tree origin/main upstream/main` reports seven conflicts spanning media, redraw, placement, state and UI input. Incident: preview command unintentionally wrote `/tmp/concord-merge-preview-55445.txt` outside project; user authorized deletion and it was removed.
L4. User decisions: approved incremental multi-area integration and inspection/removal of only the named `/tmp` diagnostic file. Actual no-commit merge had eight conflicts; resolved media.rs/avatar.rs, runtime media_runtime.rs/placement.rs/redraw.rs, state.rs/ui.rs, removed retired hit_test.rs and ported two-row guild hit mapping into interaction.rs. Added guild icon source/retry lifecycle and regression test. `cargo fmt --all --check`, `git diff --cached --check`, and unmerged-index check passed. Independent verifier first identified three source blockers; they were fixed and focused recheck found no definite blocker in that scope. No local Cargo build/test, no commit/push; MERGE_HEAD remains `5bac7a2`.
L5. User authorized a Conventional merge commit and push of a new fork branch for CI. Fork `.github/workflows/ci.yml` triggers only for pushes to `main` and PRs targeting `main`; pushing the integration branch alone will not run CI. Ask separately before creating a PR; never treat formatting/static readback as compilation proof.
