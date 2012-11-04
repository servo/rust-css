/*!
CSS stylesheets, owned types, immutable after creation
*/

use std::net::url::Url;
use util::DataStream;

pub struct Stylesheet {
    inner: n::s::CssStylesheet
}

impl Stylesheet {
    static fn new(url: Url, input: DataStream) -> Stylesheet {
        Stylesheet {
            inner: parser::parse_stylesheet(move url, input)
        }
    }
}
