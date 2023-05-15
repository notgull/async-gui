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

//! A backend for rendering widgets to HTML elements.

use super::Backend;

use web_sys::Document;

use core::fmt;

/// The HTML backend.
pub struct HtmlBackend {
    document: Document,
}

/// An error for the HTML backend.
#[derive(Debug)]
pub struct Error(Repr);

#[derive(Debug)]
enum Repr {
    /// A static error message.
    Msg(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Repr::Msg(msg) => write!(f, "{}", msg),
        }
    }
}

impl HtmlBackend {
    /// Create a new HTML backend.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            document: web_sys::window()
                .ok_or(Error(Repr::Msg("no window")))?
                .document()
                .ok_or(Error(Repr::Msg("no document")))?,
        })
    }
}

impl Backend for HtmlBackend {
    type Error = Error;
    type Output = web_sys::Element;
}
