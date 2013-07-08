/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*!
Selector matching

Select matching is performed on generic node types. Client-specific details
about the DOM are encapsulated in the `SelectHandler` type which the `SelectCtx`
uses to query various DOM and UA properties.
*/

use stylesheet::Stylesheet;
use computed::ComputedStyle;
use util::VoidPtrLike;
use wapcaplet::LwcString;
use lwcstr_from_rust_str = wapcaplet::from_rust_string;
use n::u::{rust_str_to_net_qname, net_qname_to_rust_str};
use types::StylesheetOrigin;
use n;

pub struct SelectCtx {
    inner: n::s::CssSelectCtx
}

/**
The SelectCtx, used for performing selector matching.

The `SelectCtx` takes ownership of any number of `Stylesheet` objects,
encapsulates the cascade. Individual node styles can be requested with
the `select_style` method.
*/
impl SelectCtx {
    pub fn new() -> SelectCtx {
        SelectCtx {
            inner: n::s::css_select_ctx_create()
        }
    }

    /**
    Add `Stylesheet`s to the selection context, where they will participate in the cascade
    during future selector matching
    */
    pub fn append_sheet(&mut self, sheet: Stylesheet, origin: StylesheetOrigin) {
        let sheet = match sheet {
            Stylesheet { inner: inner } => inner
        };

        self.inner.append_sheet(sheet, origin.to_net(), n::ll::t::CSS_MEDIA_SCREEN)
    }

    /**
    Select the style for a single node. `handler` is used to query the client for
    a wide range of client-specific details like node relationships, names, and UA
    defaults.
    */
    pub fn select_style<N: VoidPtrLike, H: SelectHandler<N>>(&self, node: &N, handler: &H) -> SelectResults {
        let inner_handler = SelectHandlerWrapper {
            inner: handler
        };
        SelectResults {
            inner: self.inner.select_style::<N, SelectHandlerWrapper<N, H>>(node, n::ll::t::CSS_MEDIA_SCREEN, None, &inner_handler)
        }
    }
}

/**
Represents the 'style' of a single node, including it's pseudo-elements.
*/
pub struct SelectResults {
    inner: n::s::CssSelectResults
}

impl<'self> SelectResults {
    /** Retrieve the computed style of a single pseudo-element */
    pub fn computed_style(&'self self) -> ComputedStyle<'self> {
        ComputedStyle {
            inner: self.inner.computed_style(n::s::CssPseudoElementNone)
        }
    }
}

/**
Callbacks used to query the implementation-specific DOM
*/
pub trait SelectHandler<N> {
    fn with_node_name<R>(&self, node: &N, f: &fn(&str) -> R) -> R;
    fn with_node_classes<R>(&self, node: &N, f: &fn(Option<&str>) -> R) -> R;
    fn with_node_id<R>(&self, node: &N, f: &fn(Option<&str>) -> R) -> R;
    fn named_parent_node(&self, node: &N, name: &str) -> Option<N>;
    fn parent_node(&self, node: &N) -> Option<N>;
    fn node_has_class(&self, node: &N, &str) -> bool;
    fn node_has_id(&self, node: &N, &str) -> bool;
    fn named_ancestor_node(&self, node: &N, name: &str) -> Option<N>;
    fn node_is_root(&self, node: &N) -> bool;
    fn node_is_link(&self, node: &N) -> bool;
}

/** Used to convert the netsurfcss CssSelectHandler callbacks to out SelectHandler callbacks */
struct SelectHandlerWrapper<N, H> {
    // FIXME: Can't encode region variables
    inner: *H
}

impl<'self, N, H: SelectHandler<N>> SelectHandlerWrapper<N, H> {
    priv fn inner_ref(&self) -> &'self H {
        unsafe { &*self.inner }
    }
}

impl<N, H: SelectHandler<N>> n::s::CssSelectHandler<N> for SelectHandlerWrapper<N, H> {
    fn node_name(&self, node: &N) -> n::t::CssQName {
        do self.inner_ref().with_node_name(node) |name| {
            rust_str_to_net_qname(name)
        }
    }

    fn node_classes(&self, node: &N) -> Option<~[LwcString]> {
        do self.inner_ref().with_node_classes(node) |node_classes_opt| {
           do node_classes_opt.map |s| {
               debug!("SelectHandlerWrapper::node_classes - classes: %?", *s);
               let mut v = ~[];
               for s.split_iter(' ').advance |s| { 
                   debug!("SelectHandlerWrapper::node_classes - class: %?", s);
                   if s != "" { v.push(lwcstr_from_rust_str(s)) }
               }
               debug!("SelectHandlerWrapper::classes: %?", v);
               v
           }
        }
    }

    fn node_id(&self, node: &N) -> Option<LwcString> {
        do self.inner_ref().with_node_id(node) |node_id_opt| {
            node_id_opt.map(|s| lwcstr_from_rust_str(*s))
        }
    }

    fn named_parent_node(&self, node: &N, qname: &n::t::CssQName) -> Option<N> {
        self.inner_ref().named_parent_node(node, net_qname_to_rust_str(qname))
    }

    fn parent_node(&self, node: &N) -> Option<N> {
        self.inner_ref().parent_node(node)
    }

    fn node_has_class(&self, node: &N, name: LwcString) -> bool {
        self.inner_ref().node_has_class(node, name.to_str_slice())
    }

    fn node_has_id(&self, node: &N, name: LwcString) -> bool {
        self.inner_ref().node_has_id(node, name.to_str_slice())
    }

    fn named_ancestor_node(&self, node: &N, qname: &n::t::CssQName) -> Option<N> {
        self.inner_ref().named_ancestor_node(node, net_qname_to_rust_str(qname))
    }

    fn node_is_root(&self, node: &N) -> bool {
        self.inner_ref().node_is_root(node)
    }

    fn node_is_link(&self, node: &N) -> bool {
        self.inner_ref().node_is_link(node)
    }

    fn node_is_visited(&self, _node: &N) -> bool {
        // FIXME
        warn_unimpl("node_is_visited");
        false
    }

    fn ua_default_for_property(&self, property: n::p::CssProperty) -> n::h::CssHint {
        warn!("not specifiying ua default for property %?", property);
        n::h::CssHintDefault
    }
}

fn warn_unimpl(what: &str) {
    warn!("unimplemented select handler: %?", what);
}
