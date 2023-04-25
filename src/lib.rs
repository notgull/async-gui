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
use sunder::{Backend, RenderedWidget, Widget as SunWidget};

use std::cell::RefCell;
use std::future::Future;

type BackResult<B> = Result<<B as Backend>::Output, <B as Backend>::Error>;

/// The system to be drawn into.
pub trait System {
    /// The backend used to draw widgets.
    type Backend: Backend;

    /// A type for listening to events.
    type Listener<T>: Listener<T>;

    /// The future for waiting until a redraw is requested of a component.
    type RedrawRequested<'a>: Future<Output = ()> + 'a
    where
        Self: 'a;

    /// Draw a widget.
    fn draw(
        &self,
        f: impl FnOnce(&mut Self::Backend, DrawParameters) -> BackResult<Self::Backend>,
    ) -> (BackResult<Self::Backend>, Self::RedrawRequested<'_>);
}

impl<'x, Sys: System + ?Sized> System for &'x Sys {
    type Backend = Sys::Backend;
    type Listener<T> = Sys::Listener<T>;
    type RedrawRequested<'a> = Sys::RedrawRequested<'a> where 'x: 'a;

    fn draw(
        &self,
        f: impl FnOnce(&mut Self::Backend, DrawParameters) -> BackResult<Self::Backend>,
    ) -> (BackResult<Self::Backend>, Self::RedrawRequested<'_>) {
        (**self).draw(f)
    }
}

/// Listener for new events.
pub trait Listener<T> {
    type Stream<'a>: Stream<Item = T> + 'a
    where
        Self: 'a;
    fn events<'a>(&'a mut self) -> Self::Stream<'a>;
}

pub struct DrawParameters {}

/// Wraps a `sunder` widget.
pub struct Widget<'a, Sys: System + ?Sized, S: RenderedWidget<Sys::Backend>> {
    /// The underlying widget.
    widget: S,

    /// The widget's immediate state.
    state: S::Immediate<'a>,

    /// Cache of widget-specific data.
    cache: RefCell<S::Cache>,

    /// The system to be drawn into.
    system: Sys,
}

impl<'a, Sys: System + ?Sized, S: RenderedWidget<Sys::Backend>> Widget<'a, Sys, S> {
    /// Create a new widget.
    pub fn new(system: Sys, widget: S) -> Self
    where
        Sys: Sized,
    {
        Self {
            state: <S::Immediate<'a>>::default(),
            cache: RefCell::new(<S::Cache>::default()),
            widget,
            system,
        }
    }

    pub async fn draw<'x>(&'x self) -> ! {
        loop {
            // Draw the widget.
            let (res, wait) = self.system.draw(|backend, _| {
                self.widget
                    .render(&self.state, &mut self.cache.borrow_mut(), backend)
            });

            // TODO: Handle error
            let _ = res;

            // Wait for the next redraw.
            wait.await;
        }
    }
}
