//! Load and disply simple SVG files in Bevy.
//!
//! This crate provides a Bevy `Plugin` to easily load and display a simple SVG file.
//!
//! ## Usage
//! Simply add the crate in your `Cargo.toml` and add the plugin to your app
//!
//! ```rust
//! fn main() {
//!     App::new()
//!         .add_plugin(bevy_svg::prelude::SvgPlugin)
//!         .run();
//! }
//! ```

// rustc
#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, unused)]
#![allow(elided_lifetimes_in_paths)]
// clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod loader;
#[cfg(any(feature = "2d", feature = "3d"))]
mod origin;
#[cfg(any(feature = "2d", feature = "3d"))]
mod plugin;
mod render;
mod resources;
mod svg;
mod util;

/// Import this module as `use bevy_svg::prelude::*` to get convenient imports.
pub mod prelude {
    pub use super::{SvgPlugin, SvgSet};
    #[cfg(any(feature = "2d", feature = "3d"))]
    pub use crate::origin::Origin;
    #[cfg(feature = "2d")]
    pub use crate::render::{Svg2d, Svg2dBundle};
    #[cfg(feature = "3d")]
    pub use crate::render::{Svg3d, Svg3dBundle};
    pub use crate::svg::Svg;
    pub use lyon_tessellation::{
        FillOptions, FillRule, LineCap, LineJoin, Orientation, StrokeOptions,
    };
}

pub use plugin::SvgSet;

#[cfg(any(feature = "2d", feature = "3d"))]
use crate::plugin::SvgRenderPlugin;
use crate::{loader::SvgAssetLoader, svg::Svg};
use bevy::{
    app::{App, Plugin},
    asset::AssetApp,
};

/// A plugin that provides resources and a system to draw [`Svg`]s.
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Svg>()
            .init_asset_loader::<SvgAssetLoader>();
        #[cfg(any(feature = "2d", feature = "3d"))]
        app.add_plugins(SvgRenderPlugin);
    }
}

/// A locally defined [`std::convert::Into`] surrogate to overcome orphan rules.
pub trait Convert<T>: Sized {
    /// Converts the value to `T`.
    fn convert(self) -> T;
}
