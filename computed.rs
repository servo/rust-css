use values::*;
use color::{Color, rgba};
use units::{Length, Px, Em};
use netsurfcss::util::css_fixed_to_float;

pub struct ComputedStyle {
    inner: n::c::CssComputedStyle
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
