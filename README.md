# sigma-racer-sidearm

All-Rust **M7 safety-core** firmware for the **Sigma Racer Wingman** instrument
cluster, running on the i.MX8M Plus **Cortex-M7** real-time core alongside the
Linux (A53) cockpit.

This repository is also the **single source of truth** for the M7 safety-bus CAN
contract: message IDs, the embedded `.dbc`, and the frame⇄signal codec. The M7
firmware, Linux stack (`sigma-racer-vehicle`), and ECU (`sigma-racer-efi`) all
depend on this crate so the compute domains can never disagree on message IDs or
signal scaling.

## Crate layout

| Target | Role |
|--------|------|
| **`sigma_racer_sidearm` library** | M7 safety-bus CAN dictionary and codec (`std` / `alloc` / `heapless`) |
| **`sigma-racer-sidearm` binary** | M7 Embassy firmware (`firmware` feature, `thumbv7em-none-eabihf`) |

## Role

The M7 is the deterministic, always-on safety domain, isolated from the A53
Linux side:

| Responsibility | Status |
|----------------|--------|
| Own the **safety CAN-FD bus** to the ECU (`sigma-racer-efi`) | stub (`bus/safety_bus.rs`) |
| **Fail-operational heartbeat** to the ECU | stub (`bus/heartbeat.rs`) |
| **RPMsg/OpenAMP gateway** — digested state up to Linux | stub (`rpmsg.rs`) |
| **Watchdog + load-shed anchor** | stub (`supervisor.rs`) |

Linux owns a **second, non-safety bus** independently (already handled by
`sigma-racer-vehicle`'s SocketCAN path). There is **no telltale/lamp output** on the
M7 by design — it is a real-time CAN gateway and supervisor, not a display.

## Runtime

Built on **Embassy** — the chip-agnostic thread-mode `embassy-executor` plus
`embassy-time`. The i.MX8M Plus M7 has no vendor Embassy HAL, so `time.rs`
registers a **SysTick-backed time driver** (via `systick-timer`) to supply the
global time base that `embassy-time` needs. `SYSTICK_FREQ_HZ` there is a
bring-up placeholder and **must be set to the real M7 SysTick clock** before
timeouts can be trusted.

## Build

```bash
cargo test                                    # host — CAN contract round-trip
cargo build --no-default-features --features firmware
cargo build --release --no-default-features --features firmware
```

The default target, DBC table capacities, and linker script are configured in
`.cargo/config.toml`, `rust-toolchain.toml`, and `memory.x`.

> **Hardware note:** `memory.x`, `SYSTICK_FREQ_HZ` (in `src/bin/sidearm/time.rs`), and the
> CAN/RPMsg/watchdog drivers are placeholders. Set the memory origins to match
> your U-Boot `bootaux` load address, calibrate the SysTick clock, and implement
> the peripheral drivers before running on real hardware.

## Status

Scaffolding: the boot flow and module seams compile and are wired to the shared
CAN contract; the hardware drivers are `TODO` stubs. It does not touch hardware.
