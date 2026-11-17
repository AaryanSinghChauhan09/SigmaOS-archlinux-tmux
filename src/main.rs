//! SigmaOS-archlinux-tmux — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_archlinux_tmux_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-archlinux-tmux: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-archlinux-tmux: ok");
}
