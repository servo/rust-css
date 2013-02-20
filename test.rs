use std::net::url::Url;
use url_from_str = std::net::url::from_str;
use std::cell::Cell;
use util::{DataStream, VoidPtrLike};
use values::*;
use types::*;
use units::*;
use select::*;
use color;
use color::{Color, rgb};
use stylesheet::Stylesheet;
use computed::ComputedStyle;
use complete::CompleteSelectResults;

fn test_url() -> Url {
    result::unwrap(url_from_str("http://foo.com"))
}

fn style_stream(style: &str) -> DataStream {
    let style = Cell(style.to_str());
    let d: DataStream = || {
        if !style.is_empty() {
            Some(str::to_bytes(style.take()))
        } else {
            None
        }
    };
    return d;
}

enum TestNode = @NodeData;

struct NodeData {
    name: ~str,
    id: ~str,
    children: ~[TestNode],
    mut parent: Option<TestNode>
}

impl VoidPtrLike for TestNode {
    static fn from_void_ptr(node: *libc::c_void) -> TestNode {
        assert node.is_not_null();
        TestNode(unsafe {
            let box = cast::reinterpret_cast(&node);
            cast::bump_box_refcount(box);
            box
        })
    }

    fn to_void_ptr(&self) -> *libc::c_void {
        unsafe { cast::reinterpret_cast(&(*self)) }
    }
}

struct TestHandler {
    bogus: int
}

impl TestHandler {
    static fn new() -> TestHandler {
        TestHandler {
            bogus: 0
        }
    }
}

impl SelectHandler<TestNode> for TestHandler {
    fn with_node_name<R>(node: &TestNode, f: &fn(&str) -> R) -> R {
        f((*node).name)
    }
    fn with_node_id<R>(node: &TestNode, f: &fn(Option<&str>) -> R) -> R {
        let s: &str = (*node).id;
        f(Some(s))
    }
    fn named_parent_node(node: &TestNode, name: &str) -> Option<TestNode> {
        match (**node).parent {
            Some(parent) => {
                if name == (**parent).name {
                    Some(parent)
                } else {
                    None
                }
            }
            None => None
        }
    }
    fn parent_node(node: &TestNode) -> Option<TestNode> { (**node).parent }
    fn node_has_id(node: &TestNode, name: &str) -> bool { name == node.id }
    fn named_ancestor_node(node: &TestNode, name: &str) -> Option<TestNode> { fail!(~"TODO") }
    fn node_is_root(node: &TestNode) -> bool { self.parent_node(node).is_none() }
}

fn single_div_test(style: &str, f: &fn(&ComputedStyle)) {
    let sheet = Stylesheet::new(test_url(), style_stream(style));
    let mut select_ctx = SelectCtx::new();
    let handler = &TestHandler::new();
    select_ctx.append_sheet(sheet, OriginAuthor);
    let dom = &TestNode(@NodeData {
        name: ~"div",
        id: ~"id1",
        children: ~[],
        parent: None
    });
    let style = select_ctx.select_style(dom, handler);
    let computed = style.computed_style();
    f(&computed);
}

#[test]
fn test_background_color_simple() {
    let style = "div { background-color: #123456; }";
    do single_div_test(style) |computed| {
        let color = computed.background_color();
        assert color == Specified(rgb(0x12, 0x34, 0x56));
    }
}

#[test]
fn test_border_top_width_px() {
    let style = "div { border-top-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_top_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
fn test_border_right_width_px() {
    let style = "div { border-right-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_right_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
fn test_border_bottom_width_px() {
    let style = "div { border-bottom-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_bottom_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
fn test_border_left_width_px() {
    let style = "div { border-left-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_left_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
fn test_border_width_px() {
    let style = "div { border-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_top_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
        let width = computed.border_right_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
        let width = computed.border_bottom_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
        let width = computed.border_left_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
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
        assert top_color == Specified(rgb(255, 0, 0));
        assert right_color == Specified(rgb(0, 128, 0));
        assert bottom_color == Specified(rgb(0, 0, 255));
        assert left_color == Specified(rgb(255, 255, 0));
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
        assert top_color == Specified(rgb(255, 0, 0));
        assert right_color == Specified(rgb(255, 0, 0));
        assert bottom_color == Specified(rgb(255, 0, 0));
        assert left_color == Specified(rgb(255, 0, 0));
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
        assert computed.margin_top() == Specified(CSSMarginLength(Px(10.0)));
        assert computed.margin_right() == Specified(CSSMarginLength(Px(20.0)));
        assert computed.margin_bottom() == Specified(CSSMarginLength(Px(30.0)));
        assert computed.margin_left() == Specified(CSSMarginAuto);
    }
}

#[test]
fn test_display() {
    let style = "div { display: none; }";
    do single_div_test(style) |computed| {
        assert computed.display(false) == Specified(CSSDisplayNone);
    }
}

#[test]
fn test_float() {
    let style = "div { float: right; }";
    do single_div_test(style) |computed| {
        assert computed.float() == Specified(CSSFloatRight);
    }
}

#[test]
fn test_position() {
    let style = "div { position: static; }";
    do single_div_test(style) |computed| {
        assert computed.position() == Specified(CSSPositionStatic);
    }
    let style = "div { position: relative; }";
    do single_div_test(style) |computed| {
        assert computed.position() == Specified(CSSPositionRelative);
    }
    let style = "div { position: absolute; }";
    do single_div_test(style) |computed| {
        assert computed.position() == Specified(CSSPositionAbsolute);
    }
    let style = "div { position: fixed; }";
    do single_div_test(style) |computed| {
        assert computed.position() == Specified(CSSPositionFixed);
    }
}

#[test]
fn test_width() {
    let style = "div { width: 10px; }";
    do single_div_test(style) |computed| {
        assert computed.width() == Specified(CSSWidthLength(Px(10.0)));
    }
}

#[test]
fn test_height() {
    let style = "div { height: 10px; }";
    do single_div_test(style) |computed| {
        assert computed.height() == Specified(CSSHeightLength(Px(10.0)));
    }
}

#[test]
fn test_font_family_generic() {
    use units::Fantasy;

    let style = "div { font-family: fantasy; }";
    do single_div_test(style) |computed| {
        let fam = computed.font_family();
        let spec = Specified(~[CSSFontFamilyGenericFamily(Fantasy)]);
        assert fam.eq(&spec);
    }
}

#[test]
fn test_font_family_specific() {
    let style = "div { font-family: Wombat, Jones; }";
    do single_div_test(style) |computed| {
        assert computed.font_family() == Specified(~[
            CSSFontFamilyFamilyName(~"Wombat"),
            CSSFontFamilyFamilyName(~"Jones")
        ]);
    }
}

#[test]
#[ignore]
fn test_font_size() {
    let style = "div { font-size: 10pt; }";
    do single_div_test(style) |computed| {
        assert computed.font_size() == Specified(CSSFontSizeLength(Pt(10.0)));
    }
    let style = "div { font-size: 10%; }";
    do single_div_test(style) |computed| {
        assert computed.font_size() == Specified(CSSFontSizePercentage(10.0));
    }
    let style = "div { font-size: small; }";
    do single_div_test(style) |computed| {
        assert computed.font_size() == Specified(CSSFontSizeAbsoluteSize(Small));
    }
    let style = "div { font-size: smaller; }";
    do single_div_test(style) |computed| {
        assert computed.font_size() == Specified(CSSFontSizeRelativeSize(Smaller));
    }
}

#[test]
fn test_font_style() {
    let style = "div { font-style: oblique; }";
    do single_div_test(style) |computed| {
        assert computed.font_style() == Specified(CSSFontStyleOblique);
    }
}

#[test]
fn test_font_weight() {
    let style = "div { font-weight: bold; }";
    do single_div_test(style) |computed| {
        assert computed.font_weight() == Specified(CSSFontWeightBold);
    }
}

#[test]
fn test_text_align() {
    let style = "div { text-align: center; }";
    do single_div_test(style) |computed| {
        assert computed.text_align() == Specified(CSSTextAlignCenter);
    }
}

#[test]
fn test_id_selector() {
    let style = "#id1 { text-align: center; }";
    do single_div_test(style) |computed| {
        assert computed.text_align() == Specified(CSSTextAlignCenter);
    }
}

#[test]
fn test_line_height() {
    let style = "div { line-height: 2; }";
    do single_div_test(style) |computed| {
        assert computed.line_height() == Specified(CSSLineHeightNumber(2.0));
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
        parent: None
    });
    let parent = TestNode(@NodeData {
        name: ~"div",
        id: ~"id2",
        children: ~[child],
        parent: None
    });
    child.parent = Some(parent);
    let style = select_ctx.select_style(&child, handler);
    let computed = style.computed_style();
    f(&computed);
}

#[test]
fn test_child() {
    let style = "div > span { border-left-width: 10px; }";
    do child_test(style) |computed| {
        let width = computed.border_left_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
fn test_not_child() {
    let style = "div > not_span { border-left-width: 10px; }";
    do child_test(style) |computed| {
        let width = computed.border_left_width();
        assert width != Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
#[ignore]
fn test_descendant() {
    let style = "div span { border-left-width: 10px; }";
    do child_test(style) |computed| {
        let width = computed.border_left_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
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
        parent: None
    });
    let parent = TestNode(@NodeData {
        name: ~"div",
        id: ~"id2",
        children: ~[child],
        parent: None
    });
    child.parent = Some(parent);
    let parent_results = select_ctx.select_style(&parent, handler);
    let child_results = select_ctx.select_style(&child, handler);

    let complete_parent_results = CompleteSelectResults::new_root(parent_results);
    let complete_child_results = CompleteSelectResults::new_from_parent(&complete_parent_results,
                                                                        child_results);

    let computed = complete_child_results.computed_style();

    assert computed.background_color() == color::css_colors::blue();
}
