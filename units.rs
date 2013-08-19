/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*!
Units used by CSS
*/

#[deriving(Eq)]
pub enum Length {
    Em(float), // normalized to 'em'
    Px(float), // normalized to 'px'
}

impl Length {
    fn rel(self) -> float {
        match self {
            Em(x) => x,
            _ => fail!(~"attempted to access relative unit of an absolute length")
        }
    }
    fn abs(self) -> float {
        match self {
            Em(x) => x,
            _ => fail!(~"attempted to access relative unit of an absolute length")
        }
    }
}

#[deriving(Eq)]
pub enum BoxSizing { // used by width, height, top, left, etc
    BoxLength(Length),
    BoxPercent(float),
    BoxAuto
}

#[deriving(Eq)]
pub enum AbsoluteSize {
    XXSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge
}

#[deriving(Eq)]
pub enum RelativeSize {
    Larger,
    Smaller
}

#[deriving(Eq)]
pub enum GenericFontFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
}

