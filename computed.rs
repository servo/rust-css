use values::*;
use color::{Color, rgba};
use units::{Length, Px, Em};
use netsurfcss::util::css_fixed_to_float;

pub struct ComputedStyle {
    inner: n::c::CssComputedStyle
}

impl ComputedStyle {

    // CSS 2.1, Section 8 - Box model

    pub fn margin_top() -> CSSValue<CSSMargin> {
        convert_net_margin(self.inner.margin_top())
    }

    pub fn margin_right() -> CSSValue<CSSMargin> {
        convert_net_margin(self.inner.margin_right())
    }

    pub fn margin_bottom() -> CSSValue<CSSMargin> {
        convert_net_margin(self.inner.margin_bottom())
    }

    pub fn margin_left() -> CSSValue<CSSMargin> {
        convert_net_margin(self.inner.margin_left())
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

    pub fn border_top_color() -> CSSValue<Color> {
        convert_net_color_value(self.inner.border_top_color())
    }

    pub fn border_top_color() -> CSSValue<Color> {
        convert_net_color_value(self.inner.border_top_color())
    }

    pub fn border_right_color() -> CSSValue<Color> {
        convert_net_color_value(self.inner.border_right_color())
    }

    pub fn border_bottom_color() -> CSSValue<Color> {
        convert_net_color_value(self.inner.border_bottom_color())
    }

    pub fn border_left_color() -> CSSValue<Color> {
        convert_net_color_value(self.inner.border_left_color())
    }

    // CSS 2.1, Section 9 - Visual formatting model

    pub fn display(root: bool) -> CSSValue<CSSDisplay> {
        convert_net_display_value(self.inner.display(root))
    }

    pub fn position() -> CSSValue<CSSPosition> {
        convert_net_position_value(self.inner.position())
    }

    pub fn float() -> CSSValue<CSSFloat> {
        convert_net_float_value(self.inner.float())
    }

    // CSS 2.1, Section 10 - Visual formatting model details

    pub fn width() -> CSSValue<CSSWidth> {
        convert_net_width_value(self.inner.width())
    }

    pub fn height() -> CSSValue<CSSHeight> {
        convert_net_height_value(self.inner.height())
    }

    // CSS 2.1, Section 11 - Visual effects

    // CSS 2.1, Section 12 - Generated content, automatic numbering, and lists

    // CSS 2.1, Section 13 - Paged media

    // CSS 2.1, Section 14 - Colors and Backgrounds

    pub fn background_color() -> CSSValue<Color> {
        convert_net_color_value(self.inner.background_color())
    }

    // CSS 2.1, Section 15 - Fonts

    // CSS 2.1, Section 16 - Text

    // CSS 2.1, Section 17 - Tables

    // CSS 2.1, Section 18 - User interface

}

fn convert_net_color(color: n::t::CssColor) -> Color {
    rgba(color.r, color.g, color.b, (color.a as float) / 255.0)
}

fn convert_net_color_value(color: n::v::CssColorValue) -> CSSValue<Color> {
    match color {
        n::v::CssColorInherit => Inherit,
        n::v::CssColorColor(v) => Specified(convert_net_color(v))
    }
}

fn convert_net_border_width(width: n::v::CssBorderWidthValue) -> CSSValue<CSSBorderWidth> {
    match width {
        n::v::CssBorderWidthInherit => Inherit,
        n::v::CssBorderWidthThin => Specified(CSSBorderWidthThin),
        n::v::CssBorderWidthMedium => Specified(CSSBorderWidthMedium),
        n::v::CssBorderWidthThick => Specified(CSSBorderWidthThick),
        n::v::CssBorderWidthWidth(width) => Specified(CSSBorderWidthLength(convert_net_unit_to_length(width))),
    }
}

fn convert_net_margin(margin: n::v::CssMarginValue) -> CSSValue<CSSMargin> {
    match margin {
        n::v::CssMarginInherit => Inherit,
        n::v::CssMarginSet(length) => Specified(CSSMarginLength(convert_net_unit_to_length(length))),
        n::v::CssMarginAuto => Specified(CSSMarginAuto)
    }
}

fn convert_net_width_value(value: n::v::CssWidthValue) -> CSSValue<CSSWidth> {
    match value {
        n::v::CssWidthInherit => Inherit,
        n::v::CssWidthSet(length) => Specified(CSSWidthLength(convert_net_unit_to_length(length))),
        n::v::CssWidthAuto => Specified(CSSWidthAuto)
    }
}

fn convert_net_height_value(value: n::v::CssHeightValue) -> CSSValue<CSSHeight> {
    match value {
        n::v::CssHeightInherit => Inherit,
        n::v::CssHeightSet(length) => Specified(CSSHeightLength(convert_net_unit_to_length(length))),
        n::v::CssHeightAuto => Specified(CSSHeightAuto)
    }
}

fn convert_net_display_value(value: n::v::CssDisplayValue) -> CSSValue<CSSDisplay> {
    match value {
        n::v::CssDisplayInherit => Inherit,
        n::v::CssDisplayInline => Specified(CSSDisplayInline),
        n::v::CssDisplayBlock => Specified(CSSDisplayBlock),
        n::v::CssDisplayListItem => Specified(CSSDisplayListItem),
        n::v::CssDisplayRunIn => fail unimpl("display: run-in"), // FIXME: Not in CSS 2.1
        n::v::CssDisplayInlineBlock => Specified(CSSDisplayInlineBlock),
        n::v::CssDisplayTable => Specified(CSSDisplayTable),
        n::v::CssDisplayInlineTable => Specified(CSSDisplayInlineTable),
        n::v::CssDisplayTableRowGroup => Specified(CSSDisplayTableRowGroup),
        n::v::CssDisplayTableHeaderGroup => Specified(CSSDisplayTableHeaderGroup),
        n::v::CssDisplayTableFooterGroup => Specified(CSSDisplayTableFooterGroup),
        n::v::CssDisplayTableRow => Specified(CSSDisplayTableRow),
        n::v::CssDisplayTableColumnGroup => Specified(CSSDisplayTableColumnGroup),
        n::v::CssDisplayTableColumn => Specified(CSSDisplayTableColumn),
        n::v::CssDisplayTableCell => Specified(CSSDisplayTableCell),
        n::v::CssDisplayTableCaption => Specified(CSSDisplayTableCaption),
        n::v::CssDisplayNone => Specified(CSSDisplayNone)
    }
}

fn convert_net_float_value(value: n::v::CssFloatValue) -> CSSValue<CSSFloat> {
    match value {
        n::v::CssFloatInherit => Inherit,
        n::v::CssFloatLeft => Specified(CSSFloatLeft),
        n::v::CssFloatRight => Specified(CSSFloatRight),
        n::v::CssFloatNone => Specified(CSSFloatNone)
    }
}

fn convert_net_position_value(value: n::v::CssPositionValue) -> CSSValue<CSSPosition> {
    match value {
        n::v::CssPositionInherit => Inherit,
        n::v::CssPositionStatic => Specified(CSSPositionStatic),
        n::v::CssPositionRelative => Specified(CSSPositionRelative),
        n::v::CssPositionAbsolute => Specified(CSSPositionAbsolute),
        n::v::CssPositionFixed => Specified(CSSPositionFixed)
    }
}

fn convert_net_unit_to_length(unit: n::t::CssUnit) -> Length {
    match unit {
        n::t::CssUnitPx(l) => Px(css_fixed_to_float(l)),
        n::t::CssUnitEm(l) => Em(css_fixed_to_float(l)),
        _ => unimpl("unit")
    }
}

fn unimpl(what: &str) -> ! {
    fail fmt!("css unimplemented %?", what)
}
