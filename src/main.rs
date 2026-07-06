//! # cafe-racer-sidearm
//!
//! All-Rust **M7 safety-core** firmware for the Sigma Racer Wingman instrument
//! cluster, targeting the i.MX8M Plus **Cortex-M7** real-time core.
//!
//! Per the M7 architecture decisions:
//! 1. **The M7 owns the safety CAN bus** to the ECU (`sigma-racer-efi`); Linux
//!    owns a second bus independently.
//! 2. **No telltale output** — the M7 is a real-time CAN gateway + supervisor,
//!    not a lamp driver.
//! 3. **All Rust** — reusing the shared [`sigma_racer_wingman_m7_can`] contract
//!    so this core and the Linux `vehicle-service` can never disagree on the
//!    message IDs or signal scaling.
//!
//! This is scaffolding: the control flow and module seams are in place, but the
//! hardware drivers (FlexCAN-FD, RPMsg/OpenAMP, watchdog) are stubs marked with
//! `TODO`. It builds for `thumbv7em-none-eabihf` and does not touch hardware.

#![no_std]
#![no_main]
#![forbid(unsafe_code)]

use panic_halt as _;

mod heartbeat;
mod rpmsg;
mod safety_bus;
mod supervisor;

use cortex_m_rt::entry;
use heartbeat::Heartbeat;
use rpmsg::RpmsgLink;
use safety_bus::SafetyBus;
use sigma_racer_wingman_m7_can as m7;

#[entry]
fn main() -> ! {
    // Parse the shared M7 dictionary once at boot. With the `heapless` backend
    // this builds bounded tables with no heap; the reactive loop below never
    // allocates. A parse failure means the embedded `.dbc` is broken (a build
    // bug), so hold in a safe state rather than run with no contract.
    let dbc = match m7::parse() {
        Ok(dbc) => dbc,
        Err(_) => supervisor::fault_loop(),
    };

    let mut state = m7::M7Signals::default();
    let mut bus = SafetyBus::new();
    let mut heartbeat = Heartbeat::new();
    let mut link = RpmsgLink::new();

    loop {
        // 1. Drain the safety bus and decode ECU frames into the shared state.
        while let Some(frame) = bus.poll() {
            m7::decode_into(&dbc, frame.id, frame.payload(), &mut state);
        }

        // 2. Keep the fail-operational heartbeat to the ECU alive.
        heartbeat.tick(&mut bus);

        // 3. Hand the digested state up to Linux over RPMsg.
        link.publish(&state);

        // 4. Feed the watchdog / evaluate load-shedding.
        supervisor::service();
    }
}
