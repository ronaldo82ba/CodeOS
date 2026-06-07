//! CodeCore boot sequence — invoked by codeos-init after kernel handoff.

/// Run the early userspace boot sequence.
pub fn boot_sequence() {
    println!("[CodeCore] Boot sequence starting...");
    crate::init_core();
    println!("[CodeCore] Boot sequence complete.");
}
