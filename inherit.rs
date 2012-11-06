use select::SelectResults;
use computed::ComputedStyle;
use n::u::float_to_css_fixed;
use values::*;

pub struct CompleteSelectResults {
    inner: SelectResults
}

impl CompleteSelectResults {
    static fn new_root(root: SelectResults) -> CompleteSelectResults {
        CompleteSelectResults {
            inner: move root
        }
    }

    static fn new_from_parent(parent: &CompleteSelectResults, child: SelectResults) -> CompleteSelectResults {
        let mut child = move child;

        // New lifetime
        {
            let parent_computed = parent.computed_style();
            let mut child_computed = child.computed_style();
            let net_parent_computed = &parent_computed.inner.inner;
            let net_child_computed = &mut child_computed.inner;
            // FIXME: Need to get real font sizes
            let cb: n::c::ComputeFontSizeCb = fn@(_parent: &Option<n::h::CssHint>) -> n::h::CssHint {
                n::h::CssHintLength(n::t::CssUnitPx(float_to_css_fixed(10.0)))
            };
            n::c::compose(net_parent_computed, net_child_computed, cb, net_child_computed);
        }

        CompleteSelectResults {
            inner: move child
        }
    }

    fn computed_style(&self) -> CompleteComputedStyle/&self {
        CompleteComputedStyle {
            inner: self.inner.computed_style()
        }
    }
}

pub struct CompleteComputedStyle {
    inner: ComputedStyle
}

impl CompleteComputedStyle {

    // CSS 2.1, Section 8 - Box model

    pub fn margin_top() -> CSSMargin {
        strip(self.inner.margin_top())
    }

    pub fn margin_right() -> CSSMargin {
        strip(self.inner.margin_right())
    }

    pub fn margin_bottom() -> CSSMargin {
        strip(self.inner.margin_bottom())
    }

    pub fn margin_left() -> CSSMargin {
        strip(self.inner.margin_left())
    }

    pub fn border_top_width() -> CSSBorderWidth {
        strip(self.inner.border_top_width())
    }

    pub fn border_right_width() -> CSSBorderWidth {
        strip(self.inner.border_right_width())
    }

    pub fn border_bottom_width() -> CSSBorderWidth {
        strip(self.inner.border_bottom_width())
    }

    pub fn border_left_width() -> CSSBorderWidth {
        strip(self.inner.border_left_width())
    }

    pub fn border_top_color() -> Color {
        strip(self.inner.border_top_color())
    }

    pub fn border_top_color() -> Color {
        strip(self.inner.border_top_color())
    }

    pub fn border_right_color() -> Color {
        strip(self.inner.border_right_color())
    }

    pub fn border_bottom_color() -> Color {
        strip(self.inner.border_bottom_color())
    }

    pub fn border_left_color() -> Color {
        strip(self.inner.border_left_color())
    }

    // CSS 2.1, Section 9 - Visual formatting model

    pub fn display(root: bool) -> CSSDisplay {
        strip(self.inner.display(root))
    }

    pub fn position() -> CSSPosition {
        strip(self.inner.position())
    }

    pub fn float() -> CSSFloat {
        strip(self.inner.float())
    }

    // CSS 2.1, Section 10 - Visual formatting model details

    pub fn width() -> CSSWidth {
        strip(self.inner.width())
    }

    pub fn height() -> CSSHeight {
        strip(self.inner.height())
    }

    // CSS 2.1, Section 11 - Visual effects

    // CSS 2.1, Section 12 - Generated content, automatic numbering, and lists

    // CSS 2.1, Section 13 - Paged media

    // CSS 2.1, Section 14 - Colors and Backgrounds

    pub fn background_color() -> Color {
        strip(self.inner.background_color())
    }

    // CSS 2.1, Section 15 - Fonts

    // CSS 2.1, Section 16 - Text

    // CSS 2.1, Section 17 - Tables

    // CSS 2.1, Section 18 - User interface

}

fn strip<T>(value: CSSValue<T>) -> T {
    match move value {
        Inherit => fail ~"unexpected 'inherit' value in complete style",
        Specified(move v) => move v
    }
}

