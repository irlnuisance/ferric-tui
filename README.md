# ferric

A small, safe, TUI-first CLI utility for burning ISO images to USB drives — think of it as a simpler, terminal-based alternative to [Rufus](https://rufus.ie/). Built with [Ratatui](https://ratatui.rs) and implemented in a **functional style** inspired by OCaml and Elm.

## Features

- **Safety-First Design**: Multiple layers of protection to avoid accidentally overwriting system disks
  - Automatic exclusion of mounted filesystems and root partitions
  - Explicit confirmation step requiring typing "YES"
  - Clear device information display (model, size, transport)
- **Interactive TUI**: Smooth keyboard-driven interface with live filtering and progress feedback
- **Minimal Dependencies**: Standard library implementations where possible; small dependency footprint
- **Pure Functional Architecture**: Predictable state management using The Elm Architecture pattern

## Current Status

**Implemented:**
- ISO search with live filtering across common directories (`$PWD`, `~/Downloads`, `~`)
- Safe device enumeration (filters out system disks, loop devices, read-only drives)
- Explicit confirmation screen
- Chunked writing with real-time progress (bytes, percentage, speed, ETA)
- Context-aware keyboard navigation (Tab cycles between screens based on state)

**Roadmap:**
- Automatic unmounting of target device partitions
- Optional verification (byte-compare or hash-based)
- Cancellation during write
- CLI flags for non-interactive mode (`--source`, `--target`, `--yes`, `--verify`)
- Root privilege detection and warnings

## Quick Start

```bash
# Build
cargo build --release

# Run (requires root for writing)
sudo cargo run
```

**Requirements:**
- Linux (uses `lsblk` for device enumeration)
- A real TTY (Ratatui requires interactive terminal, I rate Ghostty)

## Why OCaml-Flavored Rust?

ferric is built using **The Elm Architecture** (TEA) pattern, bringing the clarity and predictability of functional programming to Rust TUI development. This approach, inspired by OCaml and Elm, structures the entire application as a pure state machine.

### The Core Pattern

```rust
// 1. Model — Immutable state container
pub struct Model {
    pub screen: Screen,
    pub iso: Option<IsoMeta>,
    pub device: Option<Device>,
    pub verifying: bool,
}

pub enum Screen {
    IsoSearch(IsoSearchState),
    DeviceSelect(DeviceSelectState),
    Confirm(ConfirmState),
    Writing(WritingState),
    Done(Result<Outcome, WriteError>),
}

// 2. Msg — Events driving state transitions
pub enum Msg {
    Tick,
    Key(KeyEvent),
    // ISO search
    IsoQueryChanged(String),
    IsoResults(Vec<IsoMeta>),
    IsoSelected(IsoMeta),
    // Device selection
    DevicesRefreshed(Vec<Device>),
    DeviceSelected(Device),
    // Confirmation
    ConfirmInputChanged(String),
    ConfirmAccepted,
    // Write lifecycle
    WriteStarted,
    WriteProgress(Progress),
    WriteFinished(Result<Outcome, WriteError>),
}

// 3. Pure update function
pub fn update(model: Model, msg: Msg) -> (Model, Vec<Cmd>) {
    match (model.screen, msg) {
        (Screen::IsoSearch(state), Msg::IsoQueryChanged(query)) => {
            let new_state = state.with_query(query.clone());
            let cmd = Cmd::ScanIso {
                roots: vec![/* ... */],
                query
            };
            (Model { screen: Screen::IsoSearch(new_state), ..model }, vec![cmd])
        }
        // ... pattern match all transitions
    }
}

// 4. Cmd — Effect descriptions (no execution here!)
pub enum Cmd {
    ScanIso { roots: Vec<PathBuf>, query: String },
    RefreshDevices,
    Write { iso: IsoPath, device: DevicePath },
    Verify { iso: IsoPath, device: DevicePath },
}

// 5. Pure view function
impl Widget for &Model {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Deterministic rendering based solely on model state
        match &self.screen {
            Screen::IsoSearch(state) => render_iso_search(state, area, buf),
            Screen::Writing(state) => render_progress(state, area, buf),
            // ...
        }
    }
}
```

### How It Works in Practice

The runtime loop is beautifully simple:

1. **Event** arrives (keypress, tick, I/O completion) → converted to a `Msg`
2. **Update** is called: `(new_model, cmds) = update(old_model, msg)`
3. **Commands** are interpreted asynchronously; their results send new `Msg` back
4. **View** renders `&new_model` to the terminal
5. Repeat

Every state transition is a pure function call. Effects are data (`Cmd`), not side effects.

### Key Benefits

**1. Predictability**
```rust
// This is ALWAYS true:
let (model2, _) = update(model1.clone(), msg.clone());
let (model3, _) = update(model1.clone(), msg.clone());
assert_eq!(model2, model3);
```
Same input = same output. No hidden state, no surprises.

**2. Testability**
```rust
#[test]
fn test_iso_selection() {
    let model = Model::new();
    let iso = IsoMeta { path: "test.iso".into(), size: 1_000_000 };

    let (new_model, cmds) = update(model, Msg::IsoSelected(iso.clone()));

    assert_eq!(new_model.iso, Some(iso));
    assert_eq!(new_model.screen, Screen::DeviceSelect(_));
    assert!(cmds.iter().any(|c| matches!(c, Cmd::RefreshDevices)));
}
```
No mocks, no async, no test harness — just call `update()`.

**3. Explicit Effects**
```rust
// Effects are just data until the runtime interprets them
let cmd = Cmd::Write {
    iso: self.iso.unwrap(),
    device: self.device.unwrap()
};
// Nothing happened yet! The runtime will execute this asynchronously
// and send back Msg::WriteProgress or Msg::WriteFinished
```
Side effects are visible in function signatures. I/O never happens inside `update()`.

**4. Strong Types as Documentation**
```rust
// Newtypes prevent mistakes
pub struct IsoPath(PathBuf);    // Can't pass a device path by accident
pub struct DevicePath(PathBuf); // Can't pass an ISO path by accident
pub struct ByteSize(u64);       // Can't confuse bytes with other integers

// Algebraic data types make impossible states impossible
pub enum Screen {
    Writing(WritingState),  // Has progress
    Done(Result<_, _>),     // Has final status
    // Can't be "writing but also done" — the type system prevents it
}
```

**5. Time-Travel Debugging (Future)**
Because `update()` is pure, you can record every `Msg` and replay them:
```rust
let history = vec![msg1, msg2, msg3, /* ... */];
let final_model = history.into_iter().fold(Model::new(), |m, msg| {
    let (new_m, _) = update(m, msg);
    new_m
});
```

### Comparison with Traditional Imperative Style

**Imperative approach:**
```rust
impl App {
    fn handle_key(&mut self, key: KeyEvent) {
        if key == 'r' {
            self.is_loading = true;
            let devices = self.refresh_devices(); // Side effect!
            self.devices = devices;
            self.is_loading = false;
            self.selected_idx = 0;
        }
    }
}
```
Problems: mutation everywhere, side effects hidden in methods, hard to test, race conditions.

**Functional approach (ferric):**
```rust
pub fn update(model: Model, msg: Msg) -> (Model, Vec<Cmd>) {
    match msg {
        Msg::Key(key) if key.code == KeyCode::Char('r') => {
            (model, vec![Cmd::RefreshDevices])
        }
        Msg::DevicesRefreshed(devices) => {
            let new_screen = Screen::DeviceSelect(DeviceSelectState {
                devices,
                selected_idx: 0,
            });
            (Model { screen: new_screen, ..model }, vec![])
        }
        // ...
    }
}
```
Benefits: no mutation, effects explicit, trivial to test, impossible to have race conditions in pure code.

## Implementation Philosophy

### Std-First Approach

We favor standard library implementations unless a crate provides substantial value:

- **ISO Discovery**: Custom recursive walker with `std::fs::read_dir` (depth limits, size filters)
- **Device Enumeration**: Parse `lsblk -P` output with a hand-rolled tokenizer (no `serde`)
- **Progress Formatting**: Custom helpers for bytes, throughput, ETA (no `humansize` crate)

**Result**: Tiny dependency tree, fast compile times, full control over behavior.

### Strong Domain Types

Newtypes and enums encode invariants at compile time:

```rust
pub struct IsoMeta {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
}

pub struct Device {
    pub path: DevicePath,
    pub name: String,
    pub size: u64,
    pub model: Option<String>,
    pub removable: bool,
    pub hotplug: bool,
}

pub enum WriteError {
    PermissionDenied,
    DeviceDisconnected,
    IsoReadError(std::io::Error),
    VerificationFailed { expected: u64, actual: u64 },
}
```

## Safety Model

ferric goes to great lengths to prevent accidents:

1. **Device Filtering**: Automatically excludes
   - Loop devices (`/dev/loop*`)
   - Read-only devices
   - Devices with mounted partitions
   - The root disk (`/` mount point)

2. **Explicit Confirmation**: User must type `YES` (configurable)

3. **Clear Information Display**: Shows device model, size, and path before writing

4. **Future**: Will unmount partitions and detect root privileges

## Architecture

```
src/
├── app/
│   ├── mod.rs      # Runtime: event loop, command interpreter
│   ├── state.rs    # Model, Screen, domain state types
│   ├── msg.rs      # Msg and Cmd enums
│   ├── update.rs   # Pure update function (pattern matching)
│   └── cmd.rs      # Async command interpreter
├── event.rs        # Event types and sender
└── ui.rs           # Widget implementation for &Model
```

Key files:
- [src/app/state.rs](src/app/state.rs) — Core types
- [src/app/update.rs](src/app/update.rs) — State machine logic
- [src/app/cmd.rs](src/app/cmd.rs) — Effect interpreter
- [src/ui.rs](src/ui.rs) — Rendering

See [CLAUDE.md](CLAUDE.md) for detailed architecture documentation.

## Dependencies

Minimal and purposeful:

```toml
[dependencies]
ratatui = "0.29"      # TUI framework
crossterm = "0.28"    # Terminal backend
tokio = "1"           # Async runtime
color-eyre = "0.6"    # Error handling
futures = "0.3"       # Async utilities
```

No `serde`, `walkdir`, `clap`, or heavy crates — std library handles most needs.

## Keyboard Navigation

- **Global**: `q`/`Ctrl-C` quit, `Esc` go back
- **Tab/Shift-Tab**: Cycle between screens (context-aware)
- **ISO Search**: Type to filter, `↑`/`↓` to navigate, `Enter` to select
- **Device Select**: `↑`/`↓` to navigate, `r` to refresh, `Enter` to select
- **Confirm**: Type `YES` then `Enter` to start writing

## Non-Goals

- Advanced partitioning or persistence creation
- Windows/macOS support (initially)
- Filesystem-level operations beyond raw writes
- GUI interface

## Inspiration

This project draws inspiration from:
- **Elm** — The Elm Architecture for predictable state management
- **OCaml** — Strong typing, algebraic data types, explicit effects
- **Redux** — Unidirectional data flow
- **Rufus** — Feature set and UX goals

## License

Copyright (c) simon carucci

This project is licensed under the MIT license ([LICENSE](./LICENSE) or <http://opensource.org/licenses/MIT>)

## Contributing

Contributions welcome! When adding features:
- Maintain the functional architecture: extend `Msg`/`Cmd` enums, add cases to `update()`
- Keep effects in `cmd.rs`, never in `update()` or rendering code
- Add tests for pure functions (they're easy to test!)
- Prefer std library implementations

See [CLAUDE.md](CLAUDE.md) for detailed contributor guidelines.
