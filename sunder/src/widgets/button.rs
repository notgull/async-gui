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

//! A push button with text inside.

use super::text_view::TextView;
use crate::{RenderedWidget, Size, Widget};

cfg_piet! {
  use crate::piet::PietBackend;
  use super::text_view::PietCache as TextPietCache;
  use piet::RenderContext;
}

/// A push button with text inside.
pub struct PushButton<'a> {
    text: TextView<'a>,
    size: Size,
}

#[derive(Default)]
pub struct ImmediateState {
    /// Whether the button is currently pressed.
    pressed: bool,
}

cfg_piet! {
  pub struct PietCache<R: RenderContext + ?Sized> {
    text: TextPietCache<R>,
  }

  impl<R: RenderContext + ?Sized> Default for PietCache<R> {
    fn default() -> Self {
      Self {
        text: Default::default(),
      }
    }
  }
}

impl Widget for PushButton<'_> {
    type Immediate<'a> = ImmediateState;

    fn handle_event(&mut self, immediate: &mut Self::Immediate<'_>, event: crate::Event) -> bool {
        todo!()
    }
}

#[cfg(feature = "piet")]
impl<R: RenderContext + ?Sized> RenderedWidget<PietBackend<'_, R>> for PushButton<'_> {
    type Cache = PietCache<R>;

    fn rectangle(
        &mut self,
        cache: &mut Self::Cache,
        backend: &mut PietBackend<'_, R>,
    ) -> Result<Size, <PietBackend<'_, R> as crate::Backend>::Error> {
        todo!()
    }

    fn render(
        &self,
        immediate: &Self::Immediate<'_>,
        cache: &mut Self::Cache,
        backend: &mut PietBackend<'_, R>,
    ) -> Result<(), piet::Error> {
        todo!()
    }
}
