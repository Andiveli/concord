# Fork binary update
Status: active. Source: `integration/fork-current` at `ef4c44c`.
Destination: `Andiveli/concord:main`; executable: current user-local Concord binary.

## Specs
S1. "Primero deja todo en mi fork remoto, deja todo limpio acá en la pc, y luego instala el binario de mi fork".
S2. "No usemos ci, ni nada super formal, es un fork así que solo hagamoslo básico".
S3. "no quiero hacerlo con cargo porque no tengo espacio ahora".

## Tasks
T1 [S1-S3] Publish integrated fork main and prepare one manual remote binary-only build without full CI; route: inline; commit: pending.
T2 [S1] Remove local Playwright captures, keep active CodeGraph data and hide it only locally; route: inline; commit: not applicable.
T3 [S1-S3] Build Linux x86_64 fork executable remotely, download, verify identity, and replace user-local executable; route: inline; commit: not applicable.
T4 [S1] Confirm remote branch and installed binary identity, report unavailable checks; route: independent verify; commit: not applicable.

## Log
L1. Primero deja todo en mi fork remoto, deja todo limpio acá en la pc, y luego instala el binario de mi fork
L2. User chose: update fork main, remote binary-only GitHub Actions build instead of local Cargo, delete `.playwright-mcp/`, keep active `.codegraph/` hidden from Git. Existing three stashes and sibling worktrees are not cleanup targets.
L3. Evidence: `origin/main` at `9292170` is ancestor of integrated `ef4c44c`; fork has no releases, Actions artifacts, or binary. Local disk has ~5 GB free; no local Cargo build. GitHub manual dispatch requires workflow on default branch; `[skip ci]` in the new commit avoids triggering the existing full CI on the main push.
