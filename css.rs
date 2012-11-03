use netsurfcss::stylesheet::CssStylesheet;
use netsurfcss::select::{CssSelectCtx, css_select_ctx_create, CssSelectResults, CssSelectHandler, CssPseudoElementNone};
use netsurfcss::types::{CssQName, CssColor};
use netsurfcss::types::{CssUnit, CssUnitPx, CssUnitEm};
use netsurfcss::properties::CssProperty;
use netsurfcss::values::{CssColorValue, CssColorInherit, CssColorColor};
use netsurfcss::values::{CssBorderWidthValue, CssBorderWidthInherit, CssBorderWidthThin, CssBorderWidthMedium, CssBorderWidthThick, CssBorderWidthWidth};
use netsurfcss::ll::types::{CSS_ORIGIN_AUTHOR, CSS_MEDIA_SCREEN};
use netsurfcss::hint::{CssHint, CssHintDefault};
use netsurfcss::computed::CssComputedStyle;
use netsurfcss::util::css_fixed_to_float;
use lwcstr_from_rust_str = wapcaplet::from_rust_string;
use util::DataStream;
use std::net::url::Url;
use values::{CSSValue, Inherit, Specified, Length, Em, Px};
use values::{CSSBorderWidth, BdrWidthThin, BdrWidthMedium, BdrWidthThick, BdrWidthLength};
use color::{Color, rgba};

pub use netsurfcss::util::VoidPtrLike;

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

    fn select_style<N: VoidPtrLike, H: SelectHandler<N>>(&self, node: &N, handler: &H) -> SelectResults {
        let inner_handler = SelectHandlerWrapper {
            inner: ptr::to_unsafe_ptr(handler)
        };
        SelectResults {
            inner: self.inner.select_style::<N, SelectHandlerWrapper<N, H>>(node, CSS_MEDIA_SCREEN, None, &inner_handler)
        }
    }
}

pub struct SelectResults {
    inner: CssSelectResults
}

impl SelectResults {
    fn computed_style(&self) -> ComputedStyle/&self {
        ComputedStyle {
            inner: self.inner.computed_style(CssPseudoElementNone)
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

fn rust_str_to_net_qname(s: &str) -> CssQName {
    CssQName {
        ns: None,
        name: lwcstr_from_rust_str(s)
    }
}

fn net_qname_to_rust_str(qname: &CssQName) -> ~str {
    qname.name.to_str()
}

impl<N, H: SelectHandler<N>> SelectHandlerWrapper<N, H>: CssSelectHandler<N> {
    fn node_name(node: &N) -> CssQName {
        rust_str_to_net_qname(self.inner_ref().node_name(node))
    }

    fn named_parent_node(node: &N, qname: &CssQName) -> Option<N> {
        self.inner_ref().named_parent_node(node, net_qname_to_rust_str(qname))
    }

    fn parent_node(node: &N) -> Option<N> {
        self.inner_ref().parent_node(node)
    }

    fn ua_default_for_property(property: CssProperty) -> CssHint {
        warn!("not specifiying ua default for property %?", property);
        CssHintDefault
    }
}

pub struct ComputedStyle {
    inner: CssComputedStyle
}

impl ComputedStyle {
    pub fn background_color() -> CSSValue<Color> {
        convert_net_color_value(self.inner.background_color())
    }

    pub fn border_top_width() -> CSSValue<CSSBorderWidth> {
        convert_net_border_width(self.inner.border_top_width())
    }

    pub fn border_right_width() -> CSSValue<CSSBorderWidth> {
        convert_net_border_width(self.inner.border_right_width())
    }

    pub fn border_bottom_width() -> CSSValue<CSSBorderWidth> {
        convert_net_border_width(self.inner.border_bottom_width())
    }

    pub fn border_left_width() -> CSSValue<CSSBorderWidth> {
        convert_net_border_width(self.inner.border_left_width())
    }
}

fn convert_net_color(color: CssColor) -> Color {
    rgba(color.r, color.g, color.b, (color.a as float) / 255.0)
}

fn convert_net_color_value(color: CssColorValue) -> CSSValue<Color> {
    match color {
        CssColorInherit => Inherit,
        CssColorColor(v) => Specified(convert_net_color(v))
    }
}

fn convert_net_border_width(width: CssBorderWidthValue) -> CSSValue<CSSBorderWidth> {
    match width {
        CssBorderWidthInherit => Inherit,
        CssBorderWidthThin => Specified(BdrWidthThin),
        CssBorderWidthMedium => Specified(BdrWidthMedium),
        CssBorderWidthThick => Specified(BdrWidthThick),
        CssBorderWidthWidth(width) => Specified(BdrWidthLength(convert_net_unit_to_length(width))),
    }
}

fn convert_net_unit_to_length(unit: CssUnit) -> Length {
    match unit {
        CssUnitPx(l) => Px(css_fixed_to_float(l)),
        CssUnitEm(l) => Em(css_fixed_to_float(l)),
        _ => unimpl("unit")
    }
}

fn unimpl(what: &str) -> ! {
    fail fmt!("css unimplemented %?", what)
}
