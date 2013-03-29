/*!
CSS stylesheets, owned types, immutable after creation
*/

use std::net::url::Url;
use util::DataStream;
use netsurfcss::stylesheet::CssStylesheet;
use parser::parse_stylesheet;

pub struct Stylesheet {
    inner: CssStylesheet
}

pub impl Stylesheet {
    fn new(url: Url, input: DataStream) -> Stylesheet {
        Stylesheet {
            inner: parse_stylesheet(url, input)
        }
    }
}
