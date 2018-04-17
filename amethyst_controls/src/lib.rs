//! Amethyst control crate.
extern crate amethyst_core;
extern crate amethyst_input;
extern crate amethyst_renderer;
#[macro_use]
extern crate log;
extern crate winit;

#[macro_use]
#[cfg(feature = "profiler")]
extern crate thread_profiler;

pub mod components;
pub mod bundles;
pub mod systems;
