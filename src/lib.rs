/*

`async-gui` is free software: you can redistribute it and/or modify it under the terms of one of
the following licenses:

- The GNU Affero General Public License as published by the Free Software Foundation, either version
  3 of the License, or (at your option) any later version.
- The Patron License at https://github.com/notgull/async-gui/blob/main/LICENSE-PATRON.md, for
  sponsors and contributors, who can ignore the copyleft provisions of the GNU AGPL for this project.

`async-gui` is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General
Public License and the Patron License for more details.

You should have received a copy of the GNU Affero General Public License and the corresponding Patron
License along with `async-gui`. If not, see <https://www.gnu.org/licenses/>.

*/

//! A retained mode GUI framework with emphasis on asynchronous code flow.

use futures_lite::prelude::*;
use sunder::{Backend, Widget as SunWidget};

use std::cell::RefCell;
use std::future::Future;

/// The system to be drawn into.
pub trait System {
    /// The backend used to draw widgets.
    type Backend: Backend;

    /// A type for listening to events.
    type Listener<T>: Listener<T>;

    /// Get the backend for drawing.
    fn with_backend<R>(&self, f: impl FnOnce(&mut Self::Backend, DrawParameters) -> R) -> R;
}

/// Listener for new events.
pub trait Listener<T> {
    type Stream<'a>: Stream<Item = T> + 'a;
    fn events<'a>(&'a mut self) -> Self::Stream<'a>;
}

/// Drawing context.
pub trait DrawContext {
    type Backend: Backend;
    type Notify: Future<Output = ()>;

    fn wait(&self) -> Self::Notify;
    fn draw(
        &self,
        f: impl FnOnce(
            &mut Self::Backend,
            DrawParameters,
        ) -> Result<<Self::Backend>::Output, <Self::Backend>::Error>,
    );
}

pub trait Widget<B: Backend> {
    fn render(&mut self, backend: &mut B, params: DrawParameters) -> B::Output;
}

pub struct DrawParameters {}
