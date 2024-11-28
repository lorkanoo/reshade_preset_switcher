mod addon;
mod api;
pub mod config;
pub mod context;
mod render;
mod thread;

use crate::addon::Addon;
use nexus::{AddonFlags, UpdateProvider};

nexus::export! {
    name: "Reshade Preset Switcher",
    signature: -0xc347f83,
    flags: AddonFlags::None,
    load: Addon::load,
    unload: Addon::unload,
    provider: UpdateProvider::GitHub,
    update_link: env!("CARGO_PKG_REPOSITORY"),
}
