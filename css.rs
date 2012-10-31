use netsurfcss::stylesheet::CssStylesheet;
use netsurfcss::select::{CssSelectCtx, css_select_ctx_create};
use netsurfcss::ll::types::{CSS_ORIGIN_AUTHOR, CSS_MEDIA_SCREEN};
use util::DataStream;
use std::net::url::Url;

pub struct Stylesheet {
    inner: CssStylesheet
}

impl Stylesheet {
    static fn new(url: Url, input: DataStream) -> Stylesheet {
        Stylesheet {
            inner: parser::parse_stylesheet(move url, input)
        }
    }
}

pub struct SelectCtx {
    inner: CssSelectCtx
}

impl SelectCtx {
    static fn new() -> SelectCtx {
        SelectCtx {
            inner: css_select_ctx_create()
        }
    }

    fn append_sheet(&mut self, sheet: Stylesheet) {
        let sheet = match move sheet {
            Stylesheet { inner: move inner } => move inner
        };

        self.inner.append_sheet(move sheet, CSS_ORIGIN_AUTHOR, CSS_MEDIA_SCREEN)
    }
}