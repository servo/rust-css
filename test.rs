/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use extra::url::Url;
use std::FromStr;
use std::cast;
use std::libc;
use std::cell::Cell;
use util::{DataStream, VoidPtrLike};
use values::*;
use types::*;
use units::*;
use select::*;
use color;
use color::rgb;
use stylesheet::Stylesheet;
use computed::ComputedStyle;
use complete::CompleteSelectResults;

fn test_url() -> Url {
    FromStr::from_str("http://foo.com").unwrap()
}

fn style_stream(style: &str) -> DataStream {
    let style = Cell::new(style.to_str());
    let d: DataStream = || {
        if !style.is_empty() {
            let style = style.take();
            Some(style.as_bytes().to_owned())
        } else {
            None
        }
    };
    return d;
}

struct TestNode(@NodeData);

struct NodeData {
    name: ~str,
    id: ~str,
    children: ~[TestNode],
    parent: @mut Option<TestNode>
}

impl VoidPtrLike for TestNode {
    fn from_void_ptr(node: *libc::c_void) -> TestNode {
        assert!(node.is_not_null());
        TestNode(unsafe {
            let box = cast::transmute_copy(&node);
            cast::bump_box_refcount(box);
            box
        })
    }

    fn to_void_ptr(&self) -> *libc::c_void {
        unsafe {
            cast::transmute_copy(&(*self))
        }
    }
}

struct TestHandler {
    bogus: int
}

impl TestHandler {
    fn new() -> TestHandler {
        TestHandler {
            bogus: 0
        }
    }
}

impl SelectHandler<TestNode> for TestHandler {
    fn with_node_name<R>(&self, node: &TestNode, f: &fn(&str) -> R) -> R {
        f((*node).name)
    }
    fn with_node_id<R>(&self, node: &TestNode, f: &fn(Option<&str>) -> R) -> R {
        let s: &str = (*node).id;
        f(Some(s))
    }
    fn named_parent_node(&self, node: &TestNode, name: &str) -> Option<TestNode> {
        match (**node).parent {
            @Some(parent) => {
                if name == (**parent).name {
                    Some(parent)
                } else {
                    None
                }
            }
            @None => None
        }
    }
    fn with_node_classes<R>(&self, _node: &TestNode, f: &fn(Option<&str>) -> R) -> R {
        f(None)
    }
    fn parent_node(&self, node: &TestNode) -> Option<TestNode> { *(**node).parent }
    fn node_has_id(&self, node: &TestNode, name: &str) -> bool { name == node.id }
    fn named_ancestor_node(&self, _node: &TestNode, _name: &str) -> Option<TestNode> { fail!(~"TODO") }
    fn node_is_root(&self, node: &TestNode) -> bool { self.parent_node(node).is_none() }
    fn node_is_link(&self, node: &TestNode) -> bool { "a" == (**node).name }
    fn node_has_class(&self, _node: &TestNode, _s: &str) -> bool { true }
}

fn single_div_test(style: &str, f: &fn(&ComputedStyle)) {
    let sheet = Stylesheet::new(test_url(), style_stream(style));
    let mut select_ctx = SelectCtx::new();
    let handler = TestHandler::new();
    select_ctx.append_sheet(sheet, OriginAuthor);
    let dom = TestNode(@NodeData {
        name: ~"div",
        id: ~"id1",
        children: ~[],
        parent: @mut None
    });
    let style = select_ctx.select_style(&dom, None, &handler);
    let computed = style.computed_style();
    f(&computed);
}

fn single_html_test(style: &str, f: &fn(&ComputedStyle)) {
    let sheet = Stylesheet::new(test_url(), style_stream(style));
    let mut select_ctx = SelectCtx::new();
    let handler = TestHandler::new();
    select_ctx.append_sheet(sheet, OriginAuthor);
    let dom = TestNode(@NodeData {
        name: ~"html",
        id: ~"id1",
        children: ~[],
        parent: @mut None
    });
    let style = select_ctx.select_style(&dom, None, &handler);
    let computed = style.computed_style();
    f(&computed);
}

#[test]
fn test_background_color_simple() {
    let style = "div { background-color: #123456; }";
    do single_div_test(style) |computed| {
        let color = computed.background_color();
        assert!(color == Specified(rgb(0x12, 0x34, 0x56)));
    }
}

#[test]
fn test_border_top_style() {
    let style = "div { border-top-style: dotted; }";
    do single_div_test(style) |computed| {
        let style = computed.border_top_style();
        assert!(style == Specified(CSSBorderStyleDotted));
    }
}

#[test]
fn test_border_right_style() {
    let style = "div { border-right-style: solid; }";
    do single_div_test(style) |computed| {
        let style = computed.border_right_style();
        assert!(style == Specified(CSSBorderStyleSolid));
    }
}

#[test]
fn test_border_bottom_style() {
    let style = "div { border-bottom-style: groove; }";
    do single_div_test(style) |computed| {
        let style = computed.border_bottom_style();
        assert!(style == Specified(CSSBorderStyleGroove));
    }
}

#[test]
fn test_border_left_style() {
    let style = "div { border-left-style: inset; }";
    do single_div_test(style) |computed| {
        let style = computed.border_left_style();
        assert!(style == Specified(CSSBorderStyleInset));
    }
}

#[test]
fn test_border_style() {
    let style = "div { border-style: inset; }";
    do single_div_test(style) |computed| {
        let style = computed.border_top_style();
        assert!(style == Specified(CSSBorderStyleInset));
        let style = computed.border_right_style();
        assert!(style == Specified(CSSBorderStyleInset));
        let style = computed.border_left_style();
        assert!(style == Specified(CSSBorderStyleInset));
        let style = computed.border_bottom_style();
        assert!(style == Specified(CSSBorderStyleInset));
   }
}

#[test]
fn test_border_top_width_px() {
    let style = "div { border-top-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_top_width();
        assert!(width == Specified(CSSBorderWidthLength(Px(10.0))));
    }
}

#[test]
fn test_border_right_width_px() {
    let style = "div { border-right-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_right_width();
        assert!(width == Specified(CSSBorderWidthLength(Px(10.0))));
    }
}

#[test]
fn test_border_bottom_width_px() {
    let style = "div { border-bottom-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_bottom_width();
        assert!(width == Specified(CSSBorderWidthLength(Px(10.0))));
    }
}

#[test]
fn test_border_left_width_px() {
    let style = "div { border-left-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_left_width();
        assert!(width == Specified(CSSBorderWidthLength(Px(10.0))));
    }
}

#[test]
fn test_border_width_px() {
    let style = "div { border-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_top_width();
        assert!(width == Specified(CSSBorderWidthLength(Px(10.0))));
        let width = computed.border_right_width();
        assert!(width == Specified(CSSBorderWidthLength(Px(10.0))));
        let width = computed.border_bottom_width();
        assert!(width == Specified(CSSBorderWidthLength(Px(10.0))));
        let width = computed.border_left_width();
        assert!(width == Specified(CSSBorderWidthLength(Px(10.0))));
    }
}

#[test]
fn test_border_color() {
    let style = "div {\
                 border-top-color: red;\
                 border-right-color: green;\
                 border-bottom-color: blue;\
                 border-left-color: yellow;\
                 }";
    do single_div_test(style) |computed| {
        let top_color = computed.border_top_color();
        let right_color = computed.border_right_color();
        let bottom_color = computed.border_bottom_color();
        let left_color = computed.border_left_color();
        assert!(top_color == Specified(rgb(255, 0, 0)));
        assert!(right_color == Specified(rgb(0, 128, 0)));
        assert!(bottom_color == Specified(rgb(0, 0, 255)));
        assert!(left_color == Specified(rgb(255, 255, 0)));
    }
}

#[test]
fn test_border_color_shorthand() {
    let style = "div {\
                 border-color: red;\
                 }";
    do single_div_test(style) |computed| {
        let top_color = computed.border_top_color();
        let right_color = computed.border_right_color();
        let bottom_color = computed.border_bottom_color();
        let left_color = computed.border_left_color();
        assert!(top_color == Specified(rgb(255, 0, 0)));
        assert!(right_color == Specified(rgb(255, 0, 0)));
        assert!(bottom_color == Specified(rgb(255, 0, 0)));
        assert!(left_color == Specified(rgb(255, 0, 0)));
    }
}

#[test]
fn test_margin() {
    let style = "div {\
                 margin-top: 10px;\
                 margin-right: 20px;\
                 margin-bottom: 30px;\
                 margin-left: auto;\
                 }";
    do single_div_test(style) |computed| {
        assert!(computed.margin_top() == Specified(CSSMarginLength(Px(10.0))));
        assert!(computed.margin_right() == Specified(CSSMarginLength(Px(20.0))));
        assert!(computed.margin_bottom() == Specified(CSSMarginLength(Px(30.0))));
        assert!(computed.margin_left() == Specified(CSSMarginAuto));
    }
}

#[test]
fn test_display() {
    let style = "div { display: none; }";
    do single_div_test(style) |computed| {
        assert!(computed.display(false) == Specified(CSSDisplayNone));
    }
}

#[test]
fn test_float() {
    let style = "div { float: right; }";
    do single_div_test(style) |computed| {
        assert!(computed.float() == Specified(CSSFloatRight));
    }
}

#[test]
fn test_clear() {
    let style = "div { clear: both; }";
    do single_div_test(style) |computed| {
        assert!(computed.clear() == Specified(CSSClearBoth));
    }
}

#[test]
fn test_position() {
    let style = "div { position: static; }";
    do single_div_test(style) |computed| {
        assert!(computed.position() == Specified(CSSPositionStatic));
    }
    let style = "div { position: relative; }";
    do single_div_test(style) |computed| {
        assert!(computed.position() == Specified(CSSPositionRelative));
    }
    let style = "div { position: absolute; }";
    do single_div_test(style) |computed| {
        assert!(computed.position() == Specified(CSSPositionAbsolute));
    }
    let style = "div { position: fixed; }";
    do single_div_test(style) |computed| {
        assert!(computed.position() == Specified(CSSPositionFixed));
    }
}

#[test]
fn test_width() {
    let style = "div { width: 10px; }";
    do single_div_test(style) |computed| {
        assert!(computed.width() == Specified(CSSWidthLength(Px(10.0))));
    }
}

#[test]
fn test_height() {
    let style = "div { height: 10px; }";
    do single_div_test(style) |computed| {
        assert!(computed.height() == Specified(CSSHeightLength(Px(10.0))));
    }
}

#[test]
fn test_font_family_generic() {
    use units::Fantasy;

    let style = "div { font-family: fantasy; }";
    do single_div_test(style) |computed| {
        let fam = computed.font_family();
        let spec = Specified(~[CSSFontFamilyGenericFamily(Fantasy)]);
        assert!(fam.eq(&spec));
    }
}

#[test]
fn test_font_family_specific() {
    let style = "div { font-family: Wombat, Jones; }";
    do single_div_test(style) |computed| {
        assert!(computed.font_family() == Specified(~[
            CSSFontFamilyFamilyName(~"Wombat"),
            CSSFontFamilyFamilyName(~"Jones")
        ]));
    }
}

#[test]
fn test_font_size() {
    let style = "span { font-size: 10px; }";
    do child_test(style) |computed| {
        assert!(computed.font_size() == Specified(CSSFontSizeLength(Px(10.0))));
    }
    let style = "span { font-size: 10%; }";
    do child_test(style) |computed| {
        assert!(computed.font_size() == Specified(CSSFontSizePercentage(10.0)));
    }
    let style = "span { font-size: small; }";
    do child_test(style) |computed| {
        assert!(computed.font_size() == Specified(CSSFontSizeAbsoluteSize(Small)));
    }
    let style = "span { font-size: smaller; }";
    do child_test(style) |computed| {
        assert!(computed.font_size() == Specified(CSSFontSizeRelativeSize(Smaller)));
    }
}

#[test]
fn test_font_style() {
    let style = "div { font-style: oblique; }";
    do single_div_test(style) |computed| {
        assert!(computed.font_style() == Specified(CSSFontStyleOblique));
    }
}

#[test]
fn test_font_weight() {
    let style = "div { font-weight: bold; }";
    do single_div_test(style) |computed| {
        assert!(computed.font_weight() == Specified(CSSFontWeightBold));
    }
}

#[test]
fn test_text_align() {
    let style = "div { text-align: center; }";
    do single_div_test(style) |computed| {
        assert!(computed.text_align() == Specified(CSSTextAlignCenter));
    }
}

#[test]
fn test_text_decoration(){
    let style = "div { text-decoration: none; }";
    do single_html_test(style) |computed| {
        assert!(computed.text_decoration() == Specified(CSSTextDecorationNone));
    }
    let style = "html { text-decoration: underline; }";
    do single_html_test(style) |computed| {
        assert!(computed.text_decoration() == Specified(CSSTextDecorationUnderline));
    }
    let style = "";
    do single_html_test(style) |computed| {
        assert!(computed.text_decoration() == Specified(CSSTextDecorationNone));
    }
}

#[test]
fn test_id_selector() {
    let style = "#id1 { text-align: center; }";
    do single_div_test(style) |computed| {
        assert!(computed.text_align() == Specified(CSSTextAlignCenter));
    }
}

#[test]
fn test_line_height() {
    let style = "div { line-height: 2; }";
    do single_div_test(style) |computed| {
        assert!(computed.line_height() == Specified(CSSLineHeightNumber(2.0)));
    }
}

#[test]
fn test_vertical_align() {
    let style = "div { vertical-align: 20%; }";
    do single_div_test(style) |computed| {
        assert!(computed.vertical_align() == Specified(CSSVerticalAlignPercentage(20.0)));
    }
    let style = "div { vertical-align: text-top; }";
    do single_div_test(style) |computed| {
        assert!(computed.vertical_align() == Specified(CSSVerticalAlignTextTop));
    }
}

fn child_test(style: &str, f: &fn(&ComputedStyle)) {
    let sheet = Stylesheet::new(test_url(), style_stream(style));
    let mut select_ctx = SelectCtx::new();
    let handler = &TestHandler::new();
    select_ctx.append_sheet(sheet, OriginAuthor);
    let child = TestNode(@NodeData {
        name: ~"span",
        id: ~"id1",
        children: ~[],
        parent: @mut None
    });
    let parent = TestNode(@NodeData {
        name: ~"div",
        id: ~"id2",
        children: ~[child],
        parent: @mut None
    });
    *child.parent = Some(parent);
    let style = select_ctx.select_style(&child, None, handler);
    let computed = style.computed_style();
    f(&computed);
}

#[test]
fn test_child() {
    let style = "div > span { border-left-width: 10px; }";
    do child_test(style) |computed| {
        let width = computed.border_left_width();
        assert!(width == Specified(CSSBorderWidthLength(Px(10.0))));
    }
}

#[test]
fn test_not_child() {
    let style = "div > not_span { border-left-width: 10px; }";
    do child_test(style) |computed| {
        let width = computed.border_left_width();
        assert!(width != Specified(CSSBorderWidthLength(Px(10.0))));
    }
}

#[test]
#[ignore]
fn test_descendant() {
    let style = "div span { border-left-width: 10px; }";
    do child_test(style) |computed| {
        let width = computed.border_left_width();
        assert!(width == Specified(CSSBorderWidthLength(Px(10.0))));
    }
}




#[test]
fn test_compose() {
    let style = "div { background-color: blue; }\
                 span { background-color: inherit; }";

    let sheet = Stylesheet::new(test_url(), style_stream(style));
    let mut select_ctx = SelectCtx::new();
    let handler = &TestHandler::new();
    select_ctx.append_sheet(sheet, OriginAuthor);
    let child = TestNode(@NodeData {
        name: ~"span",
        id: ~"id1",
        children: ~[],
        parent: @mut None
    });
    let parent = TestNode(@NodeData {
        name: ~"div",
        id: ~"id2",
        children: ~[child],
        parent: @mut None
    });
    *child.parent = Some(parent);
    let parent_results = select_ctx.select_style(&parent, None, handler);
    let child_results = select_ctx.select_style(&child, None, handler);

    let complete_parent_results = CompleteSelectResults::new_root(parent_results);
    let complete_child_results = CompleteSelectResults::new_from_parent(&complete_parent_results,
                                                                        child_results);

    let computed = complete_child_results.computed_style();

    assert!(computed.background_color() == color::parsing::parse_color("blue").unwrap());
}
