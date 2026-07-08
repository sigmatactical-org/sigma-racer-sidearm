# sigma-racer-sidearm

All-Rust **M7 safety-core** firmware for the **Sigma Racer Wingman** instrument
cluster, running on the i.MX8M Plus **Cortex-M7** real-time core alongside the
Linux (A53) cockpit.

## Workspace

| Crate | Role |
|-------|------|
| **`sigma-racer-sidearm`** (root) | M7 Embassy firmware binary |
| **`sigma-racer-wingman-m7-can`** | Shared M7 safety-bus CAN dictionary and codec |

The CAN contract is co-located here so the M7 firmware and Linux stack
(`sigma-racer-vehicle`, `sigma-racer-efi`) depend on one repo for message IDs,
`.dbc`, and frame⇄signal encoding.

## Role

The M7 is the deterministic, always-on safety domain, isolated from the A53
Linux side:

| Responsibility | Status |
|----------------|--------|
| Own the **safety CAN-FD bus** to the ECU (`sigma-racer-efi`) | stub (`safety_bus.rs`) |
| **Fail-operational heartbeat** to the ECU | stub (`heartbeat.rs`) |
| **RPMsg/OpenAMP gateway** — digested state up to Linux | stub (`rpmsg.rs`) |
| **Watchdog + load-shed anchor** | stub (`supervisor.rs`) |

Linux owns a **second, non-safety bus** independently (already handled by
`sigma-racer-vehicle`'s SocketCAN path). There is **no telltale/lamp output** on the
M7 by design — it is a real-time CAN gateway and supervisor, not a display.

## Runtime

Built on **Embassy** — the chip-agnostic thread-mode `embassy-executor` plus
`embassy-time`. The i.MX8M Plus M7 has no vendor Embassy HAL, so `src/time.rs`
registers a **SysTick-backed time driver** (via `systick-timer`) to supply the
global time base that `embassy-time` needs. `SYSTICK_FREQ_HZ` there is a
bring-up placeholder and **must be set to the real M7 SysTick clock** before
timeouts can be trusted.

## Build

```bash
cargo build            # thumbv7em-none-eabihf (default target)
cargo build --release
cargo test -p sigma-racer-wingman-m7-can
```

The default target, DBC table capacities, and linker script are configured in
`.cargo/config.toml`, `rust-toolchain.toml`, and `memory.x`.

> **Hardware note:** `memory.x`, `SYSTICK_FREQ_HZ` (in `src/time.rs`), and the
> CAN/RPMsg/watchdog drivers are placeholders. Set the memory origins to match
> your U-Boot `bootaux` load address, calibrate the SysTick clock, and implement
> the peripheral drivers before running on real hardware.

## Status

Scaffolding: the boot flow and module seams compile and are wired to the shared
CAN contract; the hardware drivers are `TODO` stubs. It does not touch hardware.
