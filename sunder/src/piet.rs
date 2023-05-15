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

//! A backend for rendering widgets to a `piet` context.

use super::Backend;
use ui_theme::Theme;

pub use piet;

/// A backend oriented around a [`piet::RenderContext`].
///
/// [`piet::RenderContext`]: https://docs.rs/piet/latest/piet/trait.RenderContext.html
pub struct PietBackend<'a, C: ?Sized> {
    context: &'a mut C,
    theme: &'a Theme,
}

impl<'a, C: piet::RenderContext + ?Sized> PietBackend<'a, C> {
    /// Create a new backend from a `piet` context.
    pub fn new(context: &'a mut C, theme: &'a Theme) -> Self {
        Self { context, theme }
    }

    /// Get the underlying `piet` context.
    pub fn context(&mut self) -> &mut C {
        self.context
    }

    /// Get the underlying theme.
    pub fn theme(&self) -> &Theme {
        self.theme
    }
}

impl<C: piet::RenderContext + ?Sized> Backend for PietBackend<'_, C> {
    type Error = piet::Error;
    type Output = ();
}

/// Utility function for drawing a box based off of a theme class.
pub(crate) fn draw_rectangle(
    rc: &mut (impl piet::RenderContext + ?Sized),
    theme: &ui_theme::WidgetProperties,
    rectangle: crate::Rectangle
) -> Result<(), piet::Error> {
    todo!()
}
