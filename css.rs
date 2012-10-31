use netsurfcss::stylesheet::CssStylesheet;
use netsurfcss::select::{CssSelectCtx, css_select_ctx_create, CssSelectResults, CssSelectHandler};
use netsurfcss::types::CssQName;
use netsurfcss::properties::CssProperty;
use netsurfcss::ll::types::{CSS_ORIGIN_AUTHOR, CSS_MEDIA_SCREEN};
use netsurfcss::hint::{CssHint, CssHintDefault};
use wapcaplet::from_rust_string;
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

    fn select_style<N, H: SelectHandler<N>>(&self, node: &N, handler: &H) -> SelectResults {
        let inner_handler = InnerHandler {
            inner: ptr::to_unsafe_ptr(handler)
        };
        SelectResults {
            inner: self.inner.select_style::<N, InnerHandler<N, H>>(node, CSS_MEDIA_SCREEN, None, &inner_handler)
        }
    }
}

pub struct SelectResults {
    inner: CssSelectResults
}

pub trait SelectHandler<N> {
    fn node_name(node: &N) -> ~str;
}

struct InnerHandler<N, H: SelectHandler<N>> {
    // FIXME: Can't encode region variables
    inner: *H
}

priv impl<N, H: SelectHandler<N>> InnerHandler<N, H> {
    priv fn inner_ref() -> &self/H {
        unsafe { &*self.inner }
    }
}

impl<N, H: SelectHandler<N>> InnerHandler<N, H>: CssSelectHandler<N> {
    fn node_name(node: &N) -> CssQName {
        CssQName {
            ns: None,
            name: from_rust_string(self.inner_ref().node_name(node))
        }
    }
    fn ua_default_for_property(property: CssProperty) -> CssHint {
        error!("not specifiying ua default for property %?", property);
        CssHintDefault
    }
}
