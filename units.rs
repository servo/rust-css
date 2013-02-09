/*!
Units used by CSS
*/

pub enum Length {
    Em(float), // normalized to 'em'
    Px(float), // normalized to 'px'
    Pt(float)
}

impl Length {
    pure fn rel() -> float {
        match self {
            Em(x) => x,
            _ => fail!(~"attempted to access relative unit of an absolute length")
        }
    }
    pure fn abs() -> float {
        match self {
            Em(x) => x,
            _ => fail!(~"attempted to access relative unit of an absolute length")
        }
    }
}

pub enum BoxSizing { // used by width, height, top, left, etc
    BoxLength(Length),
    BoxPercent(float),
    BoxAuto
}

pub enum AbsoluteSize {
    XXSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge
}

pub enum RelativeSize {
    Larger,
    Smaller
}

#[deriving_eq]
pub enum GenericFontFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
}

