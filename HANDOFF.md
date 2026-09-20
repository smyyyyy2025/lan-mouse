# Lan Mouse maintained fork handoff

## Current state

- GitHub fork: `https://github.com/smyyyyy2025/lan-mouse`
- Local remotes: `origin` is the fork, `upstream` is `feschber/lan-mouse`
- Maintained branch: `codex/lan-mouse-maintained-main`
- Long-lived baseline branch: `codex/lan-mouse-v0.11-baseline`
- The maintained branch is based on upstream `main` at `b81c595`.
- The default `ssh Windows` route is the router LAN address `192.168.1.195`; explicit fallbacks are `Windows-uu` and `Windows-ts`.

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
- Windows `cargo test -p input-capture --no-default-features` passed: 8 tests.
- Windows `cargo test -p input-emulation --no-default-features` passed: 6 tests.
- Windows `cargo build --release --no-default-features` passed.
- macOS `CARGO_NET_OFFLINE=true cargo build --release --no-default-features` passed.
- Full workspace tests are currently blocked on the Mac host lacking `libadwaita-1` for `libadwaita-sys`.

## Deployment

- Windows installed SHA-256: `20981CA214427FB2E6D0056755F8A5098B73CB6363091B33B143513C49F0C3D9`.
- Windows backup: `lan-mouse.exe.pre-maintained-20260921-003108`.
- Windows scheduled task is running and its process owns UDP `0.0.0.0:4242`.
- macOS installed SHA-256: `57b8d7b9fff05b130548126ade5afcdd84d756d74a826c0561175e8b862cd3ed`.
- macOS backup: `lan-mouse.pre-maintained-20260921-004327`.
- The macOS app was re-signed ad hoc and passes `codesign --verify --deep --strict`.
- Layout is horizontal: Mac right edge targets Windows; Windows left edge targets Mac.
- Both configured clients are active.

## Next step

Perform a physical edge-crossing test in both directions and confirm trackpad/mouse scroll direction. The two local fix commits still need to be pushed to the fork when GitHub connectivity recovers.
