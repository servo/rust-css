/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use n;

pub enum StylesheetOrigin {
    OriginUA,
    OriginUser,
    OriginAuthor
}

impl StylesheetOrigin {
    pub fn to_net(&self) -> n::ll::t::css_origin {
        match *self {
            OriginUA => n::ll::t::CSS_ORIGIN_UA,
            OriginUser => n::ll::t::CSS_ORIGIN_USER,
            OriginAuthor => n::ll::t::CSS_ORIGIN_AUTHOR
        }
    }
}
