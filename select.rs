use stylesheet::Stylesheet;
use computed::ComputedStyle;
use util::VoidPtrLike;
use lwcstr_from_rust_str = wapcaplet::from_rust_string;

pub struct SelectCtx {
    inner: n::s::CssSelectCtx
}

impl SelectCtx {
    static fn new() -> SelectCtx {
        SelectCtx {
            inner: n::s::css_select_ctx_create()
        }
    }

    fn append_sheet(&mut self, sheet: Stylesheet) {
        let sheet = match move sheet {
            Stylesheet { inner: move inner } => move inner
        };

        self.inner.append_sheet(move sheet, n::ll::t::CSS_ORIGIN_AUTHOR, n::ll::t::CSS_MEDIA_SCREEN)
    }

    fn select_style<N: VoidPtrLike, H: SelectHandler<N>>(&self, node: &N, handler: &H) -> SelectResults {
        let inner_handler = SelectHandlerWrapper {
            inner: ptr::to_unsafe_ptr(handler)
        };
        SelectResults {
            inner: self.inner.select_style::<N, SelectHandlerWrapper<N, H>>(node, n::ll::t::CSS_MEDIA_SCREEN, None, &inner_handler)
        }
    }
}

pub struct SelectResults {
    inner: n::s::CssSelectResults
}

impl SelectResults {
    fn computed_style(&self) -> ComputedStyle/&self {
        ComputedStyle {
            inner: self.inner.computed_style(n::s::CssPseudoElementNone)
        }
    }
}

pub trait SelectHandler<N> {
    fn node_name(node: &N) -> ~str;
    fn named_parent_node(node: &N, name: &str) -> Option<N>;
    fn parent_node(node: &N) -> Option<N>;
}

struct SelectHandlerWrapper<N, H: SelectHandler<N>> {
    // FIXME: Can't encode region variables
    inner: *H
}

priv impl<N, H: SelectHandler<N>> SelectHandlerWrapper<N, H> {
    priv fn inner_ref() -> &self/H {
        unsafe { &*self.inner }
    }
}

fn rust_str_to_net_qname(s: &str) -> n::t::CssQName {
    n::t::CssQName {
        ns: None,
        name: lwcstr_from_rust_str(s)
    }
}

fn net_qname_to_rust_str(qname: &n::t::CssQName) -> ~str {
    qname.name.to_str()
}

impl<N, H: SelectHandler<N>> SelectHandlerWrapper<N, H>: n::s::CssSelectHandler<N> {
    fn node_name(node: &N) -> n::t::CssQName {
        rust_str_to_net_qname(self.inner_ref().node_name(node))
    }

    fn named_parent_node(node: &N, qname: &n::t::CssQName) -> Option<N> {
        self.inner_ref().named_parent_node(node, net_qname_to_rust_str(qname))
    }

    fn parent_node(node: &N) -> Option<N> {
        self.inner_ref().parent_node(node)
    }

    fn ua_default_for_property(property: n::p::CssProperty) -> n::h::CssHint {
        warn!("not specifiying ua default for property %?", property);
        n::h::CssHintDefault
    }
}
