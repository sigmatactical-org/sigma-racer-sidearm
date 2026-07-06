# cafe-racer-sidearm

All-Rust **M7 safety-core** firmware for the **Sigma Racer Wingman** instrument
cluster, running on the i.MX8M Plus **Cortex-M7** real-time core alongside the
Linux (A53) cockpit.

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
`vehicle-service`'s SocketCAN path). There is **no telltale/lamp output** on the
M7 by design — it is a real-time CAN gateway and supervisor, not a display.

## Single source of truth

The CAN contract (message IDs, `.dbc`, and the frame⇄signal codec) is **not**
defined here — it comes from the shared
[`sigma-racer-wingman-m7-can`](../sigma-instrumentation/sigma-racer-wingman-m7-can)
crate, which the Linux `vehicle-service` also uses. This firmware builds it with
the `heapless` backend so the reactive loop is allocation-free.

## Build

```bash
cargo build            # thumbv7em-none-eabihf (default target)
cargo build --release
```

The default target, DBC table capacities, and linker script are configured in
`.cargo/config.toml`, `rust-toolchain.toml`, and `memory.x`.

> **Hardware note:** `memory.x` and the CAN/RPMsg/watchdog drivers are
> placeholders. Set the TCM origins to match your U-Boot `bootaux` load address
> and implement the peripheral drivers before running on real hardware.

## Status

Scaffolding: the boot flow and module seams compile and are wired to the shared
CAN contract; the hardware drivers are `TODO` stubs. It does not touch hardware.
