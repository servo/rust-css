/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*!
CSS stylesheets, owned types, immutable after creation
*/

use extra::url::Url;
use util::DataStream;
use netsurfcss::stylesheet::CssStylesheet;
use parser::{parse_stylesheet, parse_style_attribute};

pub struct Stylesheet {
    inner: CssStylesheet
}

impl Stylesheet {
    pub fn new(url: Url, input: DataStream) -> Stylesheet {
        Stylesheet {
            inner: parse_stylesheet(url, input)
        }
    }

    pub fn from_attribute(url: Url, data: &str) -> Stylesheet {
        Stylesheet {
            inner: parse_style_attribute(url, data)
        }
    }
}
