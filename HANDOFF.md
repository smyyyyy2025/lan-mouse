# Lan Mouse maintained fork handoff

## Current state

- GitHub fork: `https://github.com/smyyyyy2025/lan-mouse`
- Local remotes: `origin` is the fork, `upstream` is `feschber/lan-mouse`
- Maintained branch: `codex/lan-mouse-maintained-main`
- Long-lived baseline branch: `codex/lan-mouse-v0.11-baseline`
- The maintained branch is based on upstream `main` at `b81c595`.

## Included changes

- Windows clamped outer-edge detection and out-of-bounds cursor fallback.
- Corrected remote vertical scroll sign on Windows.
- macOS bidirectional capture-loop prevention with physical/emulated provenance.
- macOS Natural Scrolling handling for emulated scroll input.
- High-resolution scroll accumulation.
- Cross-axis cursor position synchronization with protocol capability negotiation.
- Exposed-edge geometry handling for non-rectangular multi-display layouts, with a Windows clamped-edge fallback.

## Verification

- `cargo fmt --all -- --check` passed.
- `cargo test -p input-capture --no-default-features` passed: 5 tests.
- `cargo test -p input-emulation --no-default-features` passed: 5 tests.
- `cargo test -p input-event` passed: 25 tests.
- `cargo test -p lan-mouse-proto` passed: 3 tests.
- Full workspace tests are currently blocked on the Mac host lacking `libadwaita-1` for `libadwaita-sys`.
- Windows build and deployment are still pending because `ssh Windows` timed out on `100.116.204.102:22` on 2026-09-20.

## Next step

When Windows SSH is reachable, fetch `origin/codex/lan-mouse-maintained-main` in the Windows source checkout, run the Windows release build and focused tests, then deploy only after the service health check passes.
