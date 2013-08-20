/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::libc::types::os::arch::c95::c_double;
use std::cmp::Eq;
use std::ascii::AsciiStr;

macro_rules! define_color(
    ($color:ident, $r:expr, $g:expr, $b:expr) => {
        static $color: Color = Color { red: $r as u8, green: $g as u8, blue: $b as u8, alpha: 1.0 };
    }
)

macro_rules! parse_static_color(
    ($name:expr, $($color:ident),+) => {
        {
            let name = $name.trim().to_owned().into_ascii().to_upper().into_str();
            let mut color = None;
            $(
                if (stringify!($color) == name) {
                    color = Some($color);
                }
            )+
            color
        }
    }
)

#[deriving(Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: float,
}

pub fn rgba(r : u8, g : u8, b : u8, a : float) -> Color {
    Color { red : r, green : g, blue : b, alpha : a}
}

pub fn rgb(r : u8, g : u8, b : u8) -> Color {
    return rgba(r, g, b, 1.0);
}

pub fn hsla(h : float, s : float, l : float, a : float) -> Color {
    // Algorithm for converting hsl to rbg taken from
    // http://www.w3.org/TR/2003/CR-css3-color-20030514/#hsl-color
    let m2 = if l <= 0.5 { l*(s + 1.0) } else { l + s - l*s };
    let m1 = l*2.0 - m2;
    let h = h / 360.0; 
    
    fn hue_to_rgb(m1 : float, m2 : float, h : float) -> float {
        let h = if h < 0.0 { h + 1.0 } else if h > 1.0 { h - 1.0 } else { h };

        // FIXME (Rust #7222) - Auugh. Patterns would be much better here
        if 0.0 <= h && h < 1.0/6.0 {
            m1 + (m2 - m1)*h*6.0
        } else if 1.0/6.0 <= h && h < 1.0/2.0 {
            m2
        } else if 1.0/2.0 <= h && h < 2.0/3.0 {
            m1 + (m2 - m1)*(4.0 - 6.0*h)
        } else if 2.0/3.0 <= h && h <= 1.0 {
            m1
        } else {
          fail!(~"unexpected hue value")
        }
    }

    let r = (255.0*hue_to_rgb(m1, m2, h + 1.0/3.0) as c_double).round();
    let g = (255.0*hue_to_rgb(m1, m2, h) as c_double).round();
    let b = (255.0*hue_to_rgb(m1, m2, h - 1.0/3.0) as c_double).round();

    return rgba(r as u8, g as u8, b as u8, a);
}

pub fn hsl(h : float, s : float, l : float) -> Color {
    return hsla(h, s, l, 1.0);
}

impl Color {
    fn print(&self) -> ~str {
        fmt!("rgba(%u,%u,%u,%f)", self.red as uint, self.green as uint,
             self.blue as uint, self.alpha)
    }
}

pub mod parsing {
    use super::*;
    use super::{Color, rgb, rgba, hsl, hsla};

    /** Parses a color specification in the form rgb(foo,bar,baz) */
    fn parse_rgb(color : &str) -> Option<Color> {
        // Shave off the rgb( and the )
        let only_colors = color.slice(4u, color.len() - 1);

        // split up r, g, and b
        let mut cols = ~[];
        for s in only_colors.split_iter(',') {
            cols.push(s.trim());
        };

        if cols.len() != 3u { return fail_unrecognized(color); }

        match (FromStr::from_str(cols[0]), FromStr::from_str(cols[1]), 
               FromStr::from_str(cols[2])) {
          (Some(r), Some(g), Some(b)) => { Some(rgb(r, g, b)) }
          _ => { fail_unrecognized(color) }
        }
    }

    /** Parses a color specification in the form rgba(foo,bar,baz,qux) */
    fn parse_rgba(color : &str) -> Option<Color> {
        // Shave off the rgba( and the )
        let only_vals = color.slice(5u, color.len() - 1);

        // split up r, g, and b
        let mut cols = ~[];
        for s in only_vals.split_iter(',') {
            cols.push(s);
        };

        if cols.len() != 4u { return fail_unrecognized(color); }

        match (FromStr::from_str(cols[0]), FromStr::from_str(cols[1]), 
               FromStr::from_str(cols[2]), FromStr::from_str(cols[3])) {
          (Some(r), Some(g), Some(b), Some(a)) => { Some(rgba(r, g, b, a)) }
          _ => { fail_unrecognized(color) }
        }
    }

    /** Parses a color specification in the form hsl(foo,bar,baz) */
    fn parse_hsl(color : &str) -> Option<Color> {
        // Shave off the hsl( and the )
        let only_vals = color.slice(4u, color.len() - 1);

        // split up h, s, and l
        let mut vals = ~[];
        for s in only_vals.split_iter(',') {
            vals.push(s);
        };

        if vals.len() != 3u { return fail_unrecognized(color); }

        match (FromStr::from_str(vals[0]), FromStr::from_str(vals[1]), 
               FromStr::from_str(vals[2])) {
          (Some(h), Some(s), Some(l)) => { Some(hsl(h, s, l)) }
          _ => { fail_unrecognized(color) }
        }
    }

    /** Parses a color specification in the form hsla(foo,bar,baz,qux) */
    fn parse_hsla(color : &str) -> Option<Color> {
        // Shave off the hsla( and the )
        let only_vals = color.slice(5u, color.len() - 1);

        let mut vals = ~[];
        for s in only_vals.split_iter(',') {
            vals.push(s);
        };

        if vals.len() != 4u { return fail_unrecognized(color); }

        match (FromStr::from_str(vals[0]), FromStr::from_str(vals[1]), 
               FromStr::from_str(vals[2]), FromStr::from_str(vals[3])) {
          (Some(h), Some(s), Some(l), Some(a)) => { Some(hsla(h, s, l, a)) }
          _ => { fail_unrecognized(color) }
        }
    }

    // Currently colors are supported in rgb(a,b,c) form and also by
    // keywords for several common colors.
    // TODO: extend this
    pub fn parse_color(color : &str) -> Option<Color> {
        match color {
          c if c.starts_with("rgb(") => parse_rgb(c),
          c if c.starts_with("rgba(") => parse_rgba(c),
          c if c.starts_with("hsl(") => parse_hsl(c),
          c if c.starts_with("hsla(") => parse_hsla(c),
          c => parse_by_name(c)
        }
    }
}

pub fn fail_unrecognized(col : &str) -> Option<Color> {
    warn!("Unrecognized color %s", col);
    return None;
}

pub fn parse_by_name(name : &str) -> Option<Color> {
    let color = parse_static_color!(name,
            ALICEBLUE, ANTIQUEWHITE, AQUA, AQUAMARINE, AZURE,
            BEIGE, BISQUE, BLACK, BLANCHEDALMOND, BLUE,
            BLUEVIOLET, BROWN, BURLYWOOD, CADETBLUE, CHARTREUSE, 
            CHOCOLATE, CORAL, CORNFLOWERBLUE, CORNSILK, CRIMSON,
            CYAN, DARKBLUE, DARKCYAN, DARKGOLDENROD, DARKGRAY,
            DARKGREEN, DARKGREY, DARKKHAKI, DARKMAGENTA, DARKOLIVEGREEN,
            DARKORANGE, DARKORCHID, DARKRED, DARKSALMON, DARKSEAGREEN,
            DARKSLATEBLUE, DARKSLATEGRAY, DARKSLATEGREY, DARKTURQUOISE, DARKVIOLET,
            DEEPPINK, DEEPSKYBLUE, DIMGRAY, DIMGREY, DODGERBLUE,
            FIREBRICK, FLORALWHITE, FORESTGREEN, FUCHSIA, GAINSBORO,
            GHOSTWHITE, GOLD, GOLDENROD, GRAY, GREY,
            GREEN, GREENYELLOW, HONEYDEW, HOTPINK, INDIANRED,
            INDIGO, IVORY, KHAKI, LAVENDER, LAVENDERBLUSH,
            LAWNGREEN, LEMONCHIFFON, LIGHTBLUE, LIGHTCORAL, LIGHTCYAN,
            LIGHTGOLDENRODYELLOW, LIGHTGRAY, LIGHTGREEN, LIGHTGREY, LIGHTPINK,
            LIGHTSALMON, LIGHTSEAGREEN, LIGHTSKYBLUE, LIGHTSLATEGRAY, LIGHTSLATEGREY,
            LIGHTSTEELBLUE, LIGHTYELLOW, LIME, LIMEGREEN, LINEN,
            MAGENTA, MAROON, MEDIUMAQUAMARINE, MEDIUMBLUE, MEDIUMORCHID,
            MEDIUMPURPLE, MEDIUMSEAGREEN, MEDIUMSLATEBLUE, MEDIUMSPRINGGREEN, MEDIUMTURQUOISE,
            MEDIUMVIOLETRED, MIDNIGHTBLUE, MINTCREAM, MISTYROSE, MOCCASIN,
            NAVAJOWHITE, NAVY, OLDLACE, OLIVE, OLIVEDRAB,
            ORANGE, ORANGERED, ORCHID, PALEGOLDENROD, PALEGREEN,
            PALETURQUOISE, PALEVIOLETRED, PAPAYAWHIP, PEACHPUFF, PERU,
            PINK, PLUM, POWDERBLUE, PURPLE, RED,
            ROSYBROWN, ROYALBLUE, SADDLEBROWN, SALMON, SANDYBROWN,
            SEAGREEN, SEASHELL, SIENNA, SILVER, SKYBLUE,
            SLATEBLUE, SLATEGRAY, SLATEGREY, SNOW, SPRINGGREEN,
            STEELBLUE, TAN, TEAL, THISTLE, TOMATO,
            TURQUOISE, VIOLET, WHEAT, WHITE, WHITESMOKE,
            YELLOW, YELLOWGREEN);

    if color.is_none() {
        return fail_unrecognized(name);
    }else {
        return color;
    }
}

// Define the colors specified by css
define_color!(ALICEBLUE, 240, 248, 255)
define_color!(ANTIQUEWHITE, 250, 235, 215)
define_color!(AQUA, 0, 255, 255)
define_color!(AQUAMARINE, 127, 255, 212)
define_color!(AZURE, 240, 255, 255)
define_color!(BEIGE, 245, 245, 220)
define_color!(BISQUE, 255, 228, 196)
define_color!(BLACK, 0, 0, 0)
define_color!(BLANCHEDALMOND, 255, 235, 205)
define_color!(BLUE, 0, 0, 255)
define_color!(BLUEVIOLET, 138, 43, 226)
define_color!(BROWN, 165, 42, 42)
define_color!(BURLYWOOD, 222, 184, 135)
define_color!(CADETBLUE, 95, 158, 160)
define_color!(CHARTREUSE, 127, 255, 0)
define_color!(CHOCOLATE, 210, 105, 30)
define_color!(CORAL, 255, 127, 80)
define_color!(CORNFLOWERBLUE, 100, 149, 237)
define_color!(CORNSILK, 255, 248, 220)
define_color!(CRIMSON, 220, 20, 60)
define_color!(CYAN, 0, 255, 255)
define_color!(DARKBLUE, 0, 0, 139)
define_color!(DARKCYAN, 0, 139, 139)
define_color!(DARKGOLDENROD, 184, 134, 11)
define_color!(DARKGRAY, 169, 169, 169)
define_color!(DARKGREEN, 0, 100, 0)
define_color!(DARKGREY, 169, 169, 169)
define_color!(DARKKHAKI, 189, 183, 107)
define_color!(DARKMAGENTA, 139, 0, 139)
define_color!(DARKOLIVEGREEN, 85, 107, 47)
define_color!(DARKORANGE, 255, 140, 0)
define_color!(DARKORCHID, 153, 50, 204)
define_color!(DARKRED, 139, 0, 0)
define_color!(DARKSALMON, 233, 150, 122)
define_color!(DARKSEAGREEN, 143, 188, 143)
define_color!(DARKSLATEBLUE, 72, 61, 139)
define_color!(DARKSLATEGRAY, 47, 79, 79)
define_color!(DARKSLATEGREY, 47, 79, 79)
define_color!(DARKTURQUOISE, 0, 206, 209)
define_color!(DARKVIOLET, 148, 0, 211)
define_color!(DEEPPINK, 255, 20, 147)
define_color!(DEEPSKYBLUE, 0, 191, 255)
define_color!(DIMGRAY, 105, 105, 105)
define_color!(DIMGREY, 105, 105, 105)
define_color!(DODGERBLUE, 30, 144, 255)
define_color!(FIREBRICK, 178, 34, 34)
define_color!(FLORALWHITE, 255, 250, 240)
define_color!(FORESTGREEN, 34, 139, 34)
define_color!(FUCHSIA, 255, 0, 255)
define_color!(GAINSBORO, 220, 220, 220)
define_color!(GHOSTWHITE, 248, 248, 255)
define_color!(GOLD, 255, 215, 0)
define_color!(GOLDENROD, 218, 165, 32)
define_color!(GRAY, 128, 128, 128)
define_color!(GREY, 128, 128, 128)
define_color!(GREEN, 0, 128, 0)
define_color!(GREENYELLOW, 173, 255, 47)
define_color!(HONEYDEW, 240, 255, 240)
define_color!(HOTPINK, 255, 105, 180)
define_color!(INDIANRED, 205, 92, 92)
define_color!(INDIGO, 75, 0, 130)
define_color!(IVORY, 255, 255, 240)
define_color!(KHAKI, 240, 230, 140)
define_color!(LAVENDER, 230, 230, 250)
define_color!(LAVENDERBLUSH, 255, 240, 245)
define_color!(LAWNGREEN, 124, 252, 0)
define_color!(LEMONCHIFFON, 255, 250, 205)
define_color!(LIGHTBLUE, 173, 216, 230)
define_color!(LIGHTCORAL, 240, 128, 128)
define_color!(LIGHTCYAN, 224, 255, 255)
define_color!(LIGHTGOLDENRODYELLOW, 250, 250, 210)
define_color!(LIGHTGRAY, 211, 211, 211)
define_color!(LIGHTGREEN, 144, 238, 144)
define_color!(LIGHTGREY, 211, 211, 211)
define_color!(LIGHTPINK, 255, 182, 193)
define_color!(LIGHTSALMON, 255, 160, 122)
define_color!(LIGHTSEAGREEN, 32, 178, 170)
define_color!(LIGHTSKYBLUE, 135, 206, 250)
define_color!(LIGHTSLATEGRAY, 119, 136, 153)
define_color!(LIGHTSLATEGREY, 119, 136, 153)
define_color!(LIGHTSTEELBLUE, 176, 196, 222)
define_color!(LIGHTYELLOW, 255, 255, 224)
define_color!(LIME, 0, 255, 0)
define_color!(LIMEGREEN, 50, 205, 50)
define_color!(LINEN, 250, 240, 230)
define_color!(MAGENTA, 255, 0, 255)
define_color!(MAROON, 128, 0, 0)
define_color!(MEDIUMAQUAMARINE, 102, 205, 170)
define_color!(MEDIUMBLUE, 0, 0, 205)
define_color!(MEDIUMORCHID, 186, 85, 211)
define_color!(MEDIUMPURPLE, 147, 112, 219)
define_color!(MEDIUMSEAGREEN, 60, 179, 113)
define_color!(MEDIUMSLATEBLUE, 123, 104, 238)
define_color!(MEDIUMSPRINGGREEN, 0, 250, 154)
define_color!(MEDIUMTURQUOISE, 72, 209, 204)
define_color!(MEDIUMVIOLETRED, 199, 21, 133)
define_color!(MIDNIGHTBLUE, 25, 25, 112)
define_color!(MINTCREAM, 245, 255, 250)
define_color!(MISTYROSE, 255, 228, 225)
define_color!(MOCCASIN, 255, 228, 181)
define_color!(NAVAJOWHITE, 255, 222, 173)
define_color!(NAVY, 0, 0, 128)
define_color!(OLDLACE, 253, 245, 230)
define_color!(OLIVE, 128, 128, 0)
define_color!(OLIVEDRAB, 107, 142, 35)
define_color!(ORANGE, 255, 165, 0)
define_color!(ORANGERED, 255, 69, 0)
define_color!(ORCHID, 218, 112, 214)
define_color!(PALEGOLDENROD, 238, 232, 170)
define_color!(PALEGREEN, 152, 251, 152)
define_color!(PALETURQUOISE, 175, 238, 238)
define_color!(PALEVIOLETRED, 219, 112, 147)
define_color!(PAPAYAWHIP, 255, 239, 213)
define_color!(PEACHPUFF, 255, 218, 185)
define_color!(PERU, 205, 133, 63)
define_color!(PINK, 255, 192, 203)
define_color!(PLUM, 221, 160, 221)
define_color!(POWDERBLUE, 176, 224, 230)
define_color!(PURPLE, 128, 0, 128)
define_color!(RED, 255, 0, 0)
define_color!(ROSYBROWN, 188, 143, 143)
define_color!(ROYALBLUE, 65, 105, 225)
define_color!(SADDLEBROWN, 139, 69, 19)
define_color!(SALMON, 250, 128, 114)
define_color!(SANDYBROWN, 244, 164, 96)
define_color!(SEAGREEN, 46, 139, 87)
define_color!(SEASHELL, 255, 245, 238)
define_color!(SIENNA, 160, 82, 45)
define_color!(SILVER, 192, 192, 192)
define_color!(SKYBLUE, 135, 206, 235)
define_color!(SLATEBLUE, 106, 90, 205)
define_color!(SLATEGRAY, 112, 128, 144)
define_color!(SLATEGREY, 112, 128, 144)
define_color!(SNOW, 255, 250, 250)
define_color!(SPRINGGREEN, 0, 255, 127)
define_color!(STEELBLUE, 70, 130, 180)
define_color!(TAN, 210, 180, 140)
define_color!(TEAL, 0, 128, 128)
define_color!(THISTLE, 216, 191, 216)
define_color!(TOMATO, 255, 99, 71)
define_color!(TURQUOISE, 64, 224, 208)
define_color!(VIOLET, 238, 130, 238)
define_color!(WHEAT, 245, 222, 179)
define_color!(WHITE, 255, 255, 255)
define_color!(WHITESMOKE, 245, 245, 245)
define_color!(YELLOW, 255, 255, 0)
define_color!(YELLOWGREEN, 154, 205, 50)

#[cfg(test)]
mod test {
    use super::{rgb, rgba};
    use super::parsing::parse_color;

    #[test]
    fn test_parsing_rgb() {
        assert!(parse_color("red").unwrap().eq(&parse_color("rgb(255,0,0)").unwrap()));
        assert!(parse_color("red").unwrap().eq(&parse_color("rgba(255,0,0,1.0)").unwrap()));
        assert!(parse_color("red").unwrap().eq(&parse_color("rgba(255,0,0,1)").unwrap()));
        assert!(parse_color("lime").unwrap().eq(&parse_color("rgba(0,255,0,1.00)").unwrap()));
        assert!(rgb(1u8,2u8,3u8).eq(&parse_color("rgb(1,2,03)").unwrap()));
        assert!(rgba(15u8,250u8,3u8,0.5).eq(&parse_color("rgba(15,250,3,.5)").unwrap()));
        assert!(rgba(15u8,250u8,3u8,0.5).eq(&parse_color("rgba(15,250,3,0.5)").unwrap()));
        assert!(None == parse_color("rbga(1,2,3)"));
    }

    #[test]
    fn test_parsing_hsl() {
        assert!(parse_color("red").unwrap().eq(&parse_color("hsl(0,1,.5)").unwrap()));
        assert!(parse_color("lime").unwrap().eq(&parse_color("hsl(120.0,1.0,.5)").unwrap()));
        assert!(parse_color("blue").unwrap().eq(&parse_color("hsl(240.0,1.0,.5)").unwrap()));
        assert!(parse_color("green").unwrap().eq(&parse_color("hsl(120.0,1.0,.25)").unwrap()));
        assert!(parse_color("white").unwrap().eq(&parse_color("hsl(1.0,1.,1.0)").unwrap()));
        assert!(parse_color("white").unwrap().eq(&parse_color("hsl(129.0,0.3,1.0)").unwrap()));
        assert!(parse_color("black").unwrap().eq(&parse_color("hsl(231.2,0.75,0.0)").unwrap()));
        assert!(parse_color("black").unwrap().eq(&parse_color("hsl(11.2,0.0,0.0)").unwrap()));
        assert!(parse_color("gray").unwrap().eq(&parse_color("hsl(0.0,0.0,0.5)").unwrap()));
        assert!(parse_color("maroon").unwrap().eq(&parse_color("hsl(0.0,1.0,0.25)").unwrap()));
        assert!(parse_color("purple").unwrap().eq(&parse_color("hsl(300.0,1.0,0.25)").unwrap()));
        assert!(parse_color("fuchsia").unwrap().eq(&parse_color("hsl(300,1.0,0.5)").unwrap()));
        assert!(parse_color("olive").unwrap().eq(&parse_color("hsl(60.,1.0,0.25)").unwrap()));
        assert!(parse_color("yellow").unwrap().eq(&parse_color("hsl(60.,1.0,0.5)").unwrap()));
        assert!(parse_color("navy").unwrap().eq(&parse_color("hsl(240.0,1.0,.25)").unwrap()));
        assert!(parse_color("teal").unwrap().eq(&parse_color("hsl(180.0,1.0,.25)").unwrap()));
        assert!(parse_color("aqua").unwrap().eq(&parse_color("hsl(180.0,1.0,.5)").unwrap()));
        assert!(None == parse_color("hsl(1,2,3,.4)"));
    }
}
