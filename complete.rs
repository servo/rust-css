/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cast;
use color::Color;
use select::SelectResults;
use computed::ComputedStyle;
use n::h::CssHintLength;
use n::u::float_to_css_fixed;
use values::*;
use n;

pub struct CompleteSelectResults {
    inner: SelectResults
}

impl<'self> CompleteSelectResults {
    pub fn new_root(root: SelectResults) -> CompleteSelectResults {
        CompleteSelectResults {
            inner: root
        }
    }

    pub fn new_from_parent(parent: &CompleteSelectResults,
                           child: SelectResults) -> CompleteSelectResults {
        // New lifetime
        {
            let parent_computed = parent.computed_style();
            let child_computed = child.computed_style();
            //let net_parent_computed = &parent_computed.inner.inner;
            let net_child_computed = &/*mut*/ child_computed.inner;
            // FIXME: Need to get real font sizes
            let cb: n::c::ComputeFontSizeCb =
                |parent: &Option<n::h::CssHint>, child: &n::h::CssHint| -> n::h::CssHint {
                match *child {
                    CssHintLength(n::t::CssUnitEm(child_em)) => {
                        match *parent {
                            Some(CssHintLength(parent_unit)) => {
                                // CSS3 Values 5.1.1: Multiply parent unit by child unit.
                                let mut new_value =
                                    n::u::css_fixed_to_float(parent_unit.to_css_fixed());
                                new_value *= n::u::css_fixed_to_float(child_em);
                                let unit = parent_unit.modify(n::u::float_to_css_fixed(
                                    new_value));
                                CssHintLength(unit)
                            }
                            _ => n::h::CssHintLength(n::t::CssUnitEm(child_em)),
                        }
                    }
                    CssHintLength(unit) => CssHintLength(unit),
                    _ => {
                        n::h::CssHintLength(n::t::CssUnitPx(float_to_css_fixed(16.0)))
                    }
                }
            };
            // XXX: Need an aliasable &mut here
            let net_result_computed: &mut n::c::CssComputedStyle = unsafe { cast::transmute(net_child_computed) };
            let net_child_computed: &mut n::c::CssComputedStyle = unsafe { cast::transmute(&child_computed.inner) };
            let net_parent_computed = &parent_computed.inner.inner;
            n::c::compose(net_parent_computed, net_child_computed, cb, net_result_computed);
        }

        CompleteSelectResults {
            inner: child
        }
    }

    pub fn computed_style(&'self self) -> CompleteStyle<'self> {
        CompleteStyle {
            inner: self.inner.computed_style()
        }
    }
}

pub struct CompleteStyle<'self> {
    inner: ComputedStyle<'self>
}

impl<'self> CompleteStyle<'self> {

    // CSS 2.1, Section 8 - Box model

    pub fn margin_top(&self) -> CSSMargin {
        strip(self.inner.margin_top())
    }

    pub fn margin_right(&self) -> CSSMargin {
        strip(self.inner.margin_right())
    }

    pub fn margin_bottom(&self) -> CSSMargin {
        strip(self.inner.margin_bottom())
    }

    pub fn margin_left(&self) -> CSSMargin {
        strip(self.inner.margin_left())
    }

    pub fn padding_top(&self) -> CSSPadding {
        strip(self.inner.padding_top())
    }

    pub fn padding_right(&self) -> CSSPadding {
        strip(self.inner.padding_right())
    }

    pub fn padding_bottom(&self) -> CSSPadding {
        strip(self.inner.padding_bottom())
    }

    pub fn padding_left(&self) -> CSSPadding {
        strip(self.inner.padding_left())
    }

    pub fn border_top_width(&self) -> CSSBorderWidth {
        strip(self.inner.border_top_width())
    }

    pub fn border_right_width(&self) -> CSSBorderWidth {
        strip(self.inner.border_right_width())
    }

    pub fn border_bottom_width(&self) -> CSSBorderWidth {
        strip(self.inner.border_bottom_width())
    }

    pub fn border_left_width(&self) -> CSSBorderWidth {
        strip(self.inner.border_left_width())
    }

    pub fn border_top_color(&self) -> Color {
        strip(self.inner.border_top_color())
    }

    pub fn border_right_color(&self) -> Color {
        strip(self.inner.border_right_color())
    }

    pub fn border_bottom_color(&self) -> Color {
        strip(self.inner.border_bottom_color())
    }

    pub fn border_left_color(&self) -> Color {
        strip(self.inner.border_left_color())
    }

    // CSS 2.1, Section 9 - Visual formatting model

    pub fn display(&self, root: bool) -> CSSDisplay {
        strip(self.inner.display(root))
    }

    pub fn position(&self) -> CSSPosition {
        strip(self.inner.position())
    }

    pub fn float(&self) -> CSSFloat {
        strip(self.inner.float())
    }

    // CSS 2.1, Section 10 - Visual formatting model details

    pub fn width(&self) -> CSSWidth {
        strip(self.inner.width())
    }

    pub fn height(&self) -> CSSHeight {
        strip(self.inner.height())
    }

    pub fn line_height(&self) -> CSSLineHeight {
        strip(self.inner.line_height())
    }

    // CSS 2.1, Section 11 - Visual effects

    // CSS 2.1, Section 12 - Generated content, automatic numbering, and lists

    // CSS 2.1, Section 13 - Paged media

    // CSS 2.1, Section 14 - Colors and Backgrounds

    pub fn background_color(&self) -> Color {
        strip(self.inner.background_color())
    }

    pub fn color(&self) -> Color {
        strip(self.inner.color())
    }

    // CSS 2.1, Section 15 - Fonts

    pub fn font_family(&self) -> ~[CSSFontFamily] {
        strip(self.inner.font_family())
    }

    pub fn font_style(&self) -> CSSFontStyle {
        strip(self.inner.font_style())
    }

    pub fn font_weight(&self) -> CSSFontWeight {
        strip(self.inner.font_weight())
    }

    pub fn font_size(&self) -> CSSFontSize {
        strip(self.inner.font_size())
    }

    pub fn text_decoration(&self) -> CSSTextDecoration{
        strip(self.inner.text_decoration())
    }

    // CSS 2.1, Section 16 - Text

    pub fn text_align(&self) -> CSSTextAlign {
        strip(self.inner.text_align())
    }

    // CSS 2.1, Section 17 - Tables

    // CSS 2.1, Section 18 - User interface

}

fn strip<T>(value: CSSValue<T>) -> T {
    match value {
        Inherit => fail!(~"unexpected 'inherit' value in complete style"),
        Specified(v) => v
    }
}

