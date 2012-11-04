/*!
Defines how css rules, both selectors and style specifications, are
stored.  CSS selector-matching rules, as presented by 
http://www.w3.org/TR/CSS2/selector.html are represented by nested types.
*/

use SharedColor = color::Color;
use cmp::Eq;
use std::net::url::Url;
use netsurfcss::stylesheet::CssStylesheet;

/** A partial CSS value, before inheritance has been resolved */
enum CSSValue<T> {
    Inherit,
    Specified(T),
}

impl<T: Eq Copy> CSSValue<T> : Eq {
    pure fn eq(other: &CSSValue<T>) -> bool {
        match (self, *other) {
            (Inherit, Inherit) => true,
            (Specified(a), Specified(b)) => a == b,
            _ => false
        }
    }
    pure fn ne(other: &CSSValue<T>) -> bool {
        return !self.eq(other);
    }
}

pub enum Length {
    Em(float), // normalized to 'em'
    Px(float) // normalized to 'px'
}

impl Length {
    pure fn rel() -> float {
        match self {
            Em(x) => x,
            _ => fail ~"attempted to access relative unit of an absolute length"
        }
    }
    pure fn abs() -> float {
        match self {
            Em(x) => x,
            _ => fail ~"attempted to access relative unit of an absolute length"
        }
    }
}

pub enum BoxSizing { // used by width, height, top, left, etc
    BoxLength(Length),
    BoxPercent(float),
    BoxAuto
}

enum AbsoluteSize {
    XXSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge
}

enum RelativeSize {
    Larger,
    Smaller
}

// CSS property values

enum CSSBackgroundAttachment {
    BgAttachScroll,
    BgAttachFixed
}

enum CSSBackgroundColor {
    BgColor(SharedColor),
    BgColorTransparent
}

enum CSSBackgroundRepeat {
    BgRepeat,
    BgRepeatX,
    BgRepeatY,
    BgNoRepeat
}

enum CSSBackgroundImage {
    BgImage(Url),
    BgImageNone,
}

enum CSSBorderColor {
    BdrColor(SharedColor),
    BdrColorTransparent
}

enum CSSBorderStyle {
    BdrStyleNone,
    BdrStyleHidden,
    BdrStyleDotted,
    BdrStyleDashed,
    BdrStyleSolid,
    BdrStyleDouble,
    BdrStyleGroove,
    BdrStyleRidge,
    BdrStyleInset,
    BdrStyleOutset,
}

enum CSSBorderWidth {
    BdrWidthThin,
    BdrWidthMedium,
    BdrWidthThick,
    BdrWidthLength(Length)
}

impl CSSBorderWidth: Eq {
    pure fn eq(other: &CSSBorderWidth) -> bool {
        match (self, *other) {
            (BdrWidthThin, BdrWidthThin) => true,
            (BdrWidthMedium, BdrWidthMedium) => true,
            (BdrWidthThick, BdrWidthThick) => true,
            (BdrWidthLength(l1), BdrWidthLength(l2)) => l1 == l2,
            (_, _) => false
        }
    }
    pure fn ne(other: &CSSBorderWidth) -> bool { !self.eq(other) }
}

enum CSSColor {
    TextColor(SharedColor)
}

enum CSSDirection {
    DirectionLtr,
    DirectionRtl
}

enum CSSDisplay {
    DisplayInline,
    DisplayBlock,
    DisplayListItem,
    DisplayInlineBlock,
    DisplayTable,
    DisplayInlineTable,
    DisplayTableRowGroup,
    DisplayTableHeaderGroup,
    DisplayTableFooterGroup,
    DisplayTableRow,
    DisplayTableColumnGroup,
    DisplayTableColumn,
    DisplayTableCell,
    DisplayTableCaption,
    DisplayNone
}

enum CSSFloat {
    FloatLeft,
    FloatRight,
    FloatNone
}

enum CSSFontSize {
    AbsoluteSize(AbsoluteSize),
    RelativeSize(RelativeSize),
    LengthSize(Length),
    PercentSize(float)
}

enum CSSPosition {
    PosStatic,
    PosRelative,
    PosAbsolute,
    PosFixed
}

impl Length: cmp::Eq {
    pure fn eq(other: &Length) -> bool {
        match (self, *other) {
          (Em(a), Em(b)) => a == b,
          (Px(a), Px(b)) => a == b,
          (_, _) => false
        }
    }
    pure fn ne(other: &Length) -> bool {
        return !self.eq(other);
    }
}

impl BoxSizing: cmp::Eq {
    pure fn eq(other: &BoxSizing) -> bool {
        match (self, *other) {
          (BoxLength(a), BoxLength(b)) => a == b,
          (BoxPercent(a), BoxPercent(b)) => a == b,
          (BoxAuto, BoxAuto) => true,
          (_, _) => false
        }
    }
    pure fn ne(other: &BoxSizing) -> bool {
        return !self.eq(other);
    }
}

impl AbsoluteSize: cmp::Eq {
    pure fn eq(other: &AbsoluteSize) -> bool {
        self as uint == (*other) as uint
    }
    pure fn ne(other: &AbsoluteSize) -> bool {
        return !self.eq(other);
    }
}

impl RelativeSize: cmp::Eq {
    pure fn eq(other: &RelativeSize) -> bool {
        self as uint == (*other) as uint
    }
    pure fn ne(other: &RelativeSize) -> bool {
        return !self.eq(other);
    }
}

impl CSSBackgroundColor: cmp::Eq {
    pure fn eq(other: &CSSBackgroundColor) -> bool {
        match (self, *other) {
            (BgColor(a), BgColor(b)) => a == b,
            (BgColorTransparent, BgColorTransparent) => true,
            (_, _) => false
        }
    }
    pure fn ne(other: &CSSBackgroundColor) -> bool {
        return !self.eq(other);
    }
}


impl CSSColor: cmp::Eq {
    pure fn eq(other: &CSSColor) -> bool {
        match (self, *other) {
            (TextColor(a), TextColor(b)) => a == b
        }
    }
    pure fn ne(other: &CSSColor) -> bool {
        return !self.eq(other);
    }
}

impl CSSDisplay: cmp::Eq {
    pure fn eq(other: &CSSDisplay) -> bool {
        self as uint == (*other) as uint
    }
    pure fn ne(other: &CSSDisplay) -> bool {
        return !self.eq(other);
    }
}


impl CSSFontSize: cmp::Eq {
    pure fn eq(other: &CSSFontSize) -> bool {
        match (self, *other) {
            (AbsoluteSize(a), AbsoluteSize(b)) => a == b,
            (RelativeSize(a), RelativeSize(b)) => a == b,
            (LengthSize(a),   LengthSize(b))   => a == b,
            (PercentSize(a),  PercentSize(b))  => a == b,
            (_, _) => false
        }
    }
    pure fn ne(other: &CSSFontSize) -> bool {
        return !self.eq(other);
    }
}
