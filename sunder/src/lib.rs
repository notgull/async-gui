/*

`sunder` is free software: you can redistribute it and/or modify it under the terms of one of
the following licenses:

- The GNU Affero General Public License as published by the Free Software Foundation, either version
  3 of the License, or (at your option) any later version.
- The Patron License at https://github.com/notgull/sunder/blob/main/LICENSE-PATRON.md, for
  sponsors and contributors, who can ignore the copyleft provisions of the GNU AGPL for this project.

`sunder` is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General
Public License and the Patron License for more details.

You should have received a copy of the GNU Affero General Public License and the corresponding Patron
License along with `sunder`. If not, see <https://www.gnu.org/licenses/>.

*/

//! Widget rendering and event handling for Rust.
//!
//! GUI interfaces generally have two complex parts: event delivery and incrementalization, and widget
//! rendering and event handling. This crate aims to provide a simple yet powerful interface for the
//! latter parts, in order to allow GUI frameworks to focus on event delivery (which is usually what
//! differentiates the GUI from others).
//!
//! The [`Widget`] trait represents a widget that can be rendered to the screen. It takes two objects:
//! one that represents the persistent state of the widget and one that represents the immediate state.
//! It also has functions that define how the widget reacts to events. The goal is to abstract over
//! the widget drawing itself in a way that can be applied to both retained mode and immediate mode
//! GUI.

#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;

#[cfg(feature = "piet")]
macro_rules! cfg_piet {
    ($($i:item)*) => {
        $($i)*
    };
}

#[cfg(not(feature = "piet"))]
macro_rules! cfg_piet {
    ($($i:item)*) => {};
}

#[cfg(all(target_family = "wasm", feature = "web-sys"))]
macro_rules! cfg_web {
    ($($i:item)*) => {
        $($i)*
    };
}

#[cfg(not(all(target_family = "wasm", feature = "web-sys")))]
macro_rules! cfg_web {
    ($($i:item)*) => {};
}

cfg_piet! {
    extern crate alloc;

    pub mod piet;
}

cfg_web! {
    pub mod web;
}

pub mod widgets;

/// The backend for rendering widgets.
pub trait Backend {
    /// The error type for rendering.
    type Error: fmt::Debug + fmt::Display;

    /// The result of a rendering operation.
    type Output;
}

/// The widget information independent of the backend.
pub trait Widget {
    /// Immediate state of the widget.
    ///
    /// This is expected to change between calls, often in response to user input. This might contain
    /// fields like "is the button pressed".
    ///
    /// This type is intended to be maintained by the GUI framework in response to events.
    type Immediate<'a>: Default + 'a;

    /// Reply to an event in the immediate state.
    ///
    /// Returns true if this change means that the widget needs to be redrawn.
    fn handle_event(&mut self, immediate: &mut Self::Immediate<'_>, event: Event) -> bool;
}

/// The whole point.
pub trait RenderedWidget<B: Backend>: Widget {
    /// Backend-specific state of the widget.
    ///
    /// This can be used in some cases as a cache to avoid recomputing the widget's properties.
    type Cache: Default;

    /// Get the rectangle that this widget is defined by.
    ///
    /// Widgets are drawn at (0, 0).
    fn rectangle(&mut self, cache: &mut Self::Cache, backend: &mut B) -> Result<Size, B::Error>;

    /// Render the widget.
    fn render(
        &self,
        immediate: &Self::Immediate<'_>,
        cache: &mut Self::Cache,
        backend: &mut B,
    ) -> Result<B::Output, B::Error>;
}

/// Events that a widget might care about.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Event {
    /// Where the mouse is, relative to the top left corner of this widget.
    Mouse { x: f64, y: f64 },
}

/// Two dimensional rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rectangle {
    /// The x coordinate of the top left corner.
    pub x: i32,

    /// The y coordinate of the top left corner.
    pub y: i32,

    /// The width of the rectangle.
    pub width: u32,

    /// The height of the rectangle.
    pub height: u32,
}

/// Two dimensional size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    /// The width of the size.
    pub width: u32,

    /// The height of the size.
    pub height: u32,
}
