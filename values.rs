/*!
The representation of CSS property values

Enums are named after the property. Variants have the same name + the
name of the value used in the spec. This leads to some verbose names,
e.g.:

The property 'background-color' and the specified value called '<color>'
in the spec lead to the variant CSSBackgroundColorColor(Color).

At least it's consistent though.
*/

use cmp::Eq;
use std::net::url::Url;
use netsurfcss::stylesheet::CssStylesheet;
use units::{Length, AbsoluteSize, RelativeSize,
            BoxSizing, BoxLength, BoxPercent, BoxAuto, Px, Em, Pt};
use units::GenericFontFamily;
use color::Color;
use std::cmp::FuzzyEq;

/** A partial CSS value, before inheritance has been resolved */
enum CSSValue<T> {
    Inherit,
    Specified(T),
}


// CSS 2.1, Section 8 - Box model

enum CSSMargin {
    CSSMarginLength(Length),
    CSSMarginPercentage(float),
    CSSMarginAuto
}

enum CSSPadding {
    CSSPaddingLength(Length),
    CSSPaddingPercentage(float)
}

enum CSSBorderWidth {
    CSSBorderWidthThin,
    CSSBorderWidthMedium,
    CSSBorderWidthThick,
    CSSBorderWidthLength(Length)
}

enum CSSBorderColor {
    CSSBorderColorColor(Color),
    CSSBorderColorTransparent
}

enum CSSBorderStyle {
    CSSBorderStyleNone,
    CSSBorderStyleHidden,
    CSSBorderStyleDotted,
    CSSBorderStyleDashed,
    CSSBorderStyleSolid,
    CSSBorderStyleDouble,
    CSSBorderStyleGroove,
    CSSBorderStyleRidge,
    CSSBorderStyleInset,
    CSSBorderStyleOutset,
}

// CSS 2.1, Section 9 - Visual formatting model

enum CSSDisplay {
    CSSDisplayInline,
    CSSDisplayBlock,
    CSSDisplayListItem,
    CSSDisplayInlineBlock,
    CSSDisplayTable,
    CSSDisplayInlineTable,
    CSSDisplayTableRowGroup,
    CSSDisplayTableHeaderGroup,
    CSSDisplayTableFooterGroup,
    CSSDisplayTableRow,
    CSSDisplayTableColumnGroup,
    CSSDisplayTableColumn,
    CSSDisplayTableCell,
    CSSDisplayTableCaption,
    CSSDisplayNone
}

enum CSSPosition {
    CSSPositionStatic,
    CSSPositionRelative,
    CSSPositionAbsolute,
    CSSPositionFixed
}

enum CSSTop {
    CSSTopLength(Length),
    CSSTopPercentage,
    CSSTopAuto
}

enum CSSRight {
    CSSRightLength(Length),
    CSSRightPercentage(float),
    CSSRightAuto
}

enum CSSBottom {
    CSSBottomLength(Length),
    CSSBottomPercentage(float),
    CSSBottomAuto
}

enum CSSLeft {
    CSSLeftLength(Length),
    CSSLeftPercentage(float),
    CSSLeftAuto
}

enum CSSFloat {
    CSSFloatLeft,
    CSSFloatRight,
    CSSFloatNone
}

enum CSSDirection {
    CSSDirectionLtr,
    CSSDirectionRtl
}

// CSS 2.1, Section 10 - Visual formatting model details

enum CSSWidth {
    CSSWidthLength(Length),
    CSSWidthPercentage(float),
    CSSWidthAuto
}

enum CSSHeight {
    CSSHeightLength(Length),
    CSSHeightPercentage(float),
    CSSHeightAuto
}

enum CSSLineHeight {
    CSSLineHeightNormal,
    CSSLineHeightNumber(float),
    CSSLineHeightLength(Length),
    CSSLineHeightPercentage(float),
}

enum CSSVerticalAlign {
    CSSVerticalAlignBaseline,
    CSSVerticalAlignSub,
    CSSVerticalAlignSuper,
    CSSVerticalAlignTop,
    CSSVerticalAlignTextTop,
    CSSVerticalAlignMiddle,
    CSSVerticalAlignBottom,
    CSSVerticalAlignTextBottom,
    CSSVerticalAlignPercentage(float),
    CSSVerticalAlignLength(Length),
}

// CSS 2.1, Section 11 - Visual effects

enum CSSOverflow {
    CSSOverflowVisible,
    CSSOverflowHidden,
    CSSOverflowScroll,
    CSSOverflowAuto
}

enum CSSVisibility {
    CSSVisibilityVisible,
    CSSVisibilityHidden,
    CSSVisibilityCollapse
}

// CSS 2.1, Section 12 - Generated content, automatic numbering, and lists

// CSS 2.1, Section 13 - Paged media

// CSS 2.1, Section 14 - Colors and Backgrounds

enum CSSColor {
    CSSColorColor(Color)
}

enum CSSBackgroundColor {
    CSSBackgroundColorColor(Color),
    CSSBackgroundColorTransparent
}

enum CSSBackgroundImage {
    CSSBackgroundUri(Url),
    CSSBackgroundImageNone
}

enum CSSBackgroundRepeat {
    CSSBackgroundRepeatRepeat,
    CSSBackgroundRepeatRepeatX,
    CSSBackgroundRepeatRepeatY,
    CSSBackgroundRepeatNoRepeat
}

enum CSSBackgroundAttachment {
    CSSBackgroundAttachmentScroll,
    CSSBackgroundAttachmentFixed
}

enum CSSBackgroundPosition {
    CSSBackgroundPositionPercentage(float),
    CSSBackgroundPositionLength(Length),
    CSSBackgroundPositionLeft,
    CSSBackgroundPositionCenter,
    CSSBackgroundPositionRight,
    CSSBackgroundPositionTop,
    CSSBackgroundPositionBottom
}

// CSS 2.1, Section 15 - Fonts

enum CSSFontFamily {
    CSSFontFamilyFamilyName(~str),
    CSSFontFamilyGenericFamily(GenericFontFamily)
}

enum CSSFontStyle {
    CSSFontStyleNormal,
    CSSFontStyleItalic,
    CSSFontStyleOblique
}

enum CSSFontWeight {
    CSSFontWeightNormal,
    CSSFontWeightBold,
    CSSFontWeightBolder,
    CSSFontWeightLighter,
    CSSFontWeight100,
    CSSFontWeight200,
    CSSFontWeight300,
    CSSFontWeight400,
    CSSFontWeight500,
    CSSFontWeight600,
    CSSFontWeight700,
    CSSFontWeight800,
    CSSFontWeight900
}

enum CSSFontSize {
    CSSFontSizeAbsoluteSize(AbsoluteSize),
    CSSFontSizeRelativeSize(RelativeSize),
    CSSFontSizeLength(Length),
    CSSFontSizePercentage(float)
}

// CSS 2.1, Section 16 - Text

enum CSSTextAlign {
    CSSTextAlignLeft,
    CSSTextAlignRight,
    CSSTextAlignCenter,
    CSSTextAlignJustify
}

enum CSSTextDecoration {
    CSSTextDecorationNone,
    CSSTextDecorationUnderline,
    CSSTextDecorationOverline,
    CSSTextDecorationLineThrough,
    CSSTextDecorationBlink
}

enum CSSTextTransform {
    CSSTextTransformCapitalize,
    CSSTextTransformUppercase,
    CSSTextTransformLowercase,
    CSSTextTransformNone
}

// CSS 2.1, Section 17 - Tables

// CSS 2.1, Section 18 - User interface


// Implementations of Eq

impl Length: cmp::Eq {
    pure fn eq(other: &Length) -> bool {
        match (self, *other) {
          (Em(a), Em(b)) => a == b,
          (Px(a), Px(b)) => a == b,
          (Pt(a), Pt(b)) => a == b,
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
            (CSSBackgroundColorColor(a), CSSBackgroundColorColor(b)) => a == b,
            (CSSBackgroundColorTransparent, CSSBackgroundColorTransparent) => true,
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
            (CSSColorColor(a), CSSColorColor(b)) => a == b
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
            (CSSFontSizeAbsoluteSize(a), CSSFontSizeAbsoluteSize(b)) => a == b,
            (CSSFontSizeRelativeSize(a), CSSFontSizeRelativeSize(b)) => a == b,
            (CSSFontSizeLength(a), CSSFontSizeLength(b)) => a == b,
            (CSSFontSizePercentage(a), CSSFontSizePercentage(b))  => a == b,
            (_, _) => false
        }
    }
    pure fn ne(other: &CSSFontSize) -> bool {
        return !self.eq(other);
    }
}

impl<T: Eq> CSSValue<T> : Eq {
    pure fn eq(other: &CSSValue<T>) -> bool {
        match (&self, other) {
            (&Inherit, &Inherit) => true,
            (&Specified(a), &Specified(b)) => a == b,
            _ => false
        }
    }
    pure fn ne(other: &CSSValue<T>) -> bool {
        return !self.eq(other);
    }
}

impl CSSBorderWidth: Eq {
    pure fn eq(other: &CSSBorderWidth) -> bool {
        match (self, *other) {
            (CSSBorderWidthThin, CSSBorderWidthThin) => true,
            (CSSBorderWidthMedium, CSSBorderWidthMedium) => true,
            (CSSBorderWidthThick, CSSBorderWidthThick) => true,
            (CSSBorderWidthLength(l1), CSSBorderWidthLength(l2)) => l1 == l2,
            (_, _) => false
        }
    }
    pure fn ne(other: &CSSBorderWidth) -> bool { !self.eq(other) }
}

impl CSSMargin: Eq {
    pure fn eq(other: &CSSMargin) -> bool {
        match (self, *other) {
            (CSSMarginLength(l1), CSSMarginLength(l2)) => l1 == l2,
            (CSSMarginPercentage(p1), CSSMarginPercentage(p2)) => p1.fuzzy_eq(&p2),
            (CSSMarginAuto, CSSMarginAuto) => true,
            (_, _) => false
        }
    }

    pure fn ne(other: &CSSMargin) -> bool { !self.eq(other) }
}

impl CSSFloat: Eq {
    pure fn eq(other: &CSSFloat) -> bool {
        match (self, *other) {
            (CSSFloatLeft, CSSFloatLeft) => true,
            (CSSFloatRight, CSSFloatRight) => true,
            (CSSFloatNone, CSSFloatNone) => true,
            (_, _) => false
        }
    }

    pure fn ne(other: &CSSFloat) -> bool { !self.eq(other) }
}

impl CSSPosition: Eq {
    pure fn eq(other: &CSSPosition) -> bool {
        match (self, *other) {
            (CSSPositionStatic, CSSPositionStatic) => true,
            (CSSPositionRelative, CSSPositionRelative) => true,
            (CSSPositionAbsolute, CSSPositionAbsolute) => true,
            (CSSPositionFixed, CSSPositionFixed) => true,
            (_, _) => false
        }
    }

    pure fn ne(other: &CSSPosition) -> bool { !self.eq(other) }
}

impl CSSWidth: Eq {
    pure fn eq(other: &CSSWidth) -> bool {
        match (self, *other) {
            (CSSWidthLength(l1), CSSWidthLength(l2)) => l1 == l2,
            (CSSWidthPercentage(p1), CSSWidthPercentage(p2)) => p1.fuzzy_eq(&p2),
            (CSSWidthAuto, CSSWidthAuto) => true,
            (_, _) => false
        }
    }

    pure fn ne(other: &CSSWidth) -> bool { !self.eq(other) }
}

impl CSSHeight: Eq {
    pure fn eq(other: &CSSHeight) -> bool {
        match (self, *other) {
            (CSSHeightLength(l1), CSSHeightLength(l2)) => l1 == l2,
            (CSSHeightPercentage(p1), CSSHeightPercentage(p2)) => p1.fuzzy_eq(&p2),
            (CSSHeightAuto, CSSHeightAuto) => true,
            (_, _) => false
        }
    }

    pure fn ne(other: &CSSHeight) -> bool { !self.eq(other) }
}

impl CSSFontFamily: Eq {
    pure fn eq(other: &CSSFontFamily) -> bool {
        match (&self, other) {
            (&CSSFontFamilyFamilyName(ref f1), &CSSFontFamilyFamilyName(ref f2)) => f1 == f2,
            (&CSSFontFamilyGenericFamily(g1), &CSSFontFamilyGenericFamily(g2)) => g1 == g2,
            (_, _) => false
        }
    }

    pure fn ne(other: &CSSFontFamily) -> bool { !self.eq(other) }
}

impl CSSFontStyle: Eq {
    pure fn eq(other: &CSSFontStyle) -> bool {
        self as uint == *other as uint
    }

    pure fn ne(other: &CSSFontStyle) -> bool { !self.eq(other) }
}

impl CSSFontWeight: Eq {
    pure fn eq(other: &CSSFontWeight) -> bool {
        self as uint == *other as uint
    }

    pure fn ne(other: &CSSFontWeight) -> bool { !self.eq(other) }
}

impl CSSTextAlign: Eq {
    pure fn eq(other: &CSSTextAlign) -> bool {
        self as uint == *other as uint
    }

    pure fn ne(other: &CSSTextAlign) -> bool { !self.eq(other) }
}