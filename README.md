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
| **`sigma_racer_sidearm` library** | M7 safety-bus CAN dictionary, codec, and RPMsg wire format |
| **`sigma-racer-sidearm-bringup`** | UART boot proof (`bringup` feature) |
| **`sigma-racer-sidearm` binary** | Production Embassy firmware (`firmware` feature) |

## Role

The M7 is the deterministic, always-on safety domain, isolated from the A53
Linux side:

| Responsibility | Status |
|----------------|--------|
| Own the **safety CAN bus** to the ECU (`sigma-racer-efi`) on FLEXCAN1 | implemented (`hw/flexcan.rs`) |
| **Fail-operational heartbeat** to the ECU (`0x080` @ 50 Hz) | implemented (`bus/heartbeat.rs`) |
| **RPMsg gateway** — digested state up to Linux | implemented (`hw/rpmsg.rs`, `wire.rs`) |
| **Watchdog + load-shed anchor** | partial (`supervisor.rs` — kick counter; SoC WDOG TBD) |

Linux owns a **second, non-safety bus** on FLEXCAN2 (`can1`) independently.
There is **no telltale/lamp output** on the M7 by design.

## Runtime

Built on **Embassy** with a SysTick-backed `embassy-time` driver (`time.rs`).
Platform init (`hw/`) runs in `pre_init`: cache policy, CCM gates, RDC, UART,
FLEXCAN1, and the OpenAMP resource table for Linux `remoteproc`.

## Build

```bash
cargo test --target x86_64-unknown-linux-gnu          # host — contract + wire codec
cargo build --no-default-features --features bringup  # UART boot proof
cargo build --release --no-default-features --features firmware
```

Linker scripts: `memory-ddr.x` (default), `memory-itcm.x` (`memory-itcm` feature),
`link-rsc.x` (`.resource_table` section). See [`docs/M7_BRINGUP.md`](docs/M7_BRINGUP.md).

## Status

First-silicon ready scaffolding: boot proof, DDR remoteproc layout, FLEXCAN1
driver, virtio RPMsg publisher, and Linux Wingman integration (DT overlay,
firmware recipe, `sigma-racer-vehicle` RPMsg source). Validate on Verdin
hardware before production sign-off.
