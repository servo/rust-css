/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::libc::types::os::arch::c95::c_double;
use std::cmp::Eq;

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
    use super::{Color, rgb, rgba, hsl, hsla};
    use super::css_colors::{black, silver, gray, white, maroon, red, purple, fuchsia, green, lime, olive, yellow, navy, blue, teal, aqua};

    fn fail_unrecognized(col : &str) -> Option<Color> {
        warn!("Unrecognized color %s", col);
        return None;
    }

    /** Match an exact color keyword. */
    fn parse_by_name(color : &str) -> Option<Color> {
        let col = match color.to_ascii().to_lower().to_str_ascii() {
            ~"black" => black(),
            ~"silver" => silver(),
            ~"gray" => gray(),
            ~"grey" => gray(),
            ~"white" => white(),
            ~"maroon" => maroon(),
            ~"red" => red(),
            ~"purple" => purple(),
            ~"fuchsia" => fuchsia(),
            ~"green" => green(),
            ~"lime" => lime(),
            ~"olive" => olive(),
            ~"yellow" => yellow(),
            ~"navy" => navy(),
            ~"blue" => blue(),
            ~"teal" => teal(),
            ~"aqua" => aqua(),
            _ => return fail_unrecognized(color)
        };

        return Some(col);
    }
    
    /** Parses a color specification in the form rgb(foo,bar,baz) */
    fn parse_rgb(color : &str) -> Option<Color> {
        // Shave off the rgb( and the )
        let only_colors = color.slice(4u, color.len() - 1);

        // split up r, g, and b
        let mut cols = ~[];
        for s in only_colors.split_iter(',') {
            cols.push(s);
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

#[cfg(test)]
mod test {
    use super::{rgb, rgba};
    use super::css_colors::*;
    use super::parsing::parse_color;

    #[test]
    fn test_parse_by_name() {
        assert!(red().eq(&parse_color("red").unwrap()));
        assert!(lime().eq(&parse_color("Lime").unwrap()));
        assert!(blue().eq(&parse_color("BLUE").unwrap()));
        assert!(green().eq(&parse_color("GreEN").unwrap()));
        assert!(white().eq(&parse_color("white").unwrap()));
        assert!(black().eq(&parse_color("Black").unwrap()));
        assert!(gray().eq(&parse_color("Gray").unwrap()));
        println("silver");
        assert!(silver().eq(&parse_color("SiLvEr").unwrap()));
        assert!(maroon().eq(&parse_color("maroon").unwrap()));
        assert!(purple().eq(&parse_color("PURPLE").unwrap()));
        assert!(fuchsia().eq(&parse_color("FUCHSIA").unwrap()));
        assert!(olive().eq(&parse_color("oLiVe").unwrap()));
        assert!(yellow().eq(&parse_color("yellow").unwrap()));
        assert!(navy().eq(&parse_color("NAVY").unwrap()));
        assert!(teal().eq(&parse_color("Teal").unwrap()));
        assert!(aqua().eq(&parse_color("Aqua").unwrap()));
        assert!(None == parse_color("foobarbaz"));
    }

    #[test]
    fn test_parsing_rgb() {
        assert!(red().eq(&parse_color("rgb(255,0,0)").unwrap()));
        assert!(red().eq(&parse_color("rgba(255,0,0,1.0)").unwrap()));
        assert!(red().eq(&parse_color("rgba(255,0,0,1)").unwrap()));
        assert!(lime().eq(&parse_color("rgba(0,255,0,1.00)").unwrap()));
        assert!(rgb(1u8,2u8,3u8).eq(&parse_color("rgb(1,2,03)").unwrap()));
        assert!(rgba(15u8,250u8,3u8,0.5).eq(&parse_color("rgba(15,250,3,.5)").unwrap()));
        assert!(rgba(15u8,250u8,3u8,0.5).eq(&parse_color("rgba(15,250,3,0.5)").unwrap()));
        assert!(None == parse_color("rbga(1,2,3)"));
    }

    #[test]
    fn test_parsing_hsl() {
        assert!(red().eq(&parse_color("hsl(0,1,.5)").unwrap()));
        assert!(lime().eq(&parse_color("hsl(120.0,1.0,.5)").unwrap()));
        assert!(blue().eq(&parse_color("hsl(240.0,1.0,.5)").unwrap()));
        assert!(green().eq(&parse_color("hsl(120.0,1.0,.25)").unwrap()));
        assert!(white().eq(&parse_color("hsl(1.0,1.,1.0)").unwrap()));
        assert!(white().eq(&parse_color("hsl(129.0,0.3,1.0)").unwrap()));
        assert!(black().eq(&parse_color("hsl(231.2,0.75,0.0)").unwrap()));
        assert!(black().eq(&parse_color("hsl(11.2,0.0,0.0)").unwrap()));
        assert!(gray().eq(&parse_color("hsl(0.0,0.0,0.5)").unwrap()));
        assert!(maroon().eq(&parse_color("hsl(0.0,1.0,0.25)").unwrap()));
        assert!(purple().eq(&parse_color("hsl(300.0,1.0,0.25)").unwrap()));
        assert!(fuchsia().eq(&parse_color("hsl(300,1.0,0.5)").unwrap()));
        assert!(olive().eq(&parse_color("hsl(60.,1.0,0.25)").unwrap()));
        assert!(yellow().eq(&parse_color("hsl(60.,1.0,0.5)").unwrap()));
        assert!(navy().eq(&parse_color("hsl(240.0,1.0,.25)").unwrap()));
        assert!(teal().eq(&parse_color("hsl(180.0,1.0,.25)").unwrap()));
        assert!(aqua().eq(&parse_color("hsl(180.0,1.0,.5)").unwrap()));
        assert!(None == parse_color("hsl(1,2,3,.4)"));
    }
}


/** Define the colors specified by css */
pub mod css_colors {
    use super::Color;

    pub fn aliceblue() -> Color {
        Color {red : 240u8, green : 248u8, blue : 255u8, alpha : 1.0}
    }
    pub fn antiquewhite() -> Color {
        Color {red : 250u8, green : 235u8, blue : 215u8, alpha : 1.0}
    }
    pub fn aqua() -> Color {
        Color {red : 0u8, green : 255u8, blue : 255u8, alpha : 1.0}
    }
    pub fn aquamarine() -> Color {
        Color {red : 127u8, green : 255u8, blue : 212u8, alpha : 1.0}
    }
    pub fn azure() -> Color {
        Color {red : 240u8, green : 255u8, blue : 255u8, alpha : 1.0}
    }
    pub fn beige() -> Color {
        Color {red : 245u8, green : 245u8, blue : 220u8, alpha : 1.0}
    }
    pub fn bisque() -> Color {
        Color {red : 255u8, green : 228u8, blue : 196u8, alpha : 1.0}
    }
    pub fn black() -> Color {
        Color {red : 0u8, green : 0u8, blue : 0u8, alpha : 1.0}
    }
    pub fn blanchedalmond() -> Color {
        Color {red : 255u8, green : 235u8, blue : 205u8, alpha : 1.0}
    }
    pub fn blue() -> Color {
        Color {red : 0u8, green : 0u8, blue : 255u8, alpha : 1.0}
    }
    pub fn blueviolet() -> Color {
        Color {red : 138u8, green : 43u8, blue : 226u8, alpha : 1.0}
    }
    pub fn brown() -> Color {
        Color {red : 165u8, green : 42u8, blue : 42u8, alpha : 1.0}
    }
    pub fn burlywood() -> Color {
        Color {red : 222u8, green : 184u8, blue : 135u8, alpha : 1.0}
    }
    pub fn cadetblue() -> Color {
        Color {red : 95u8, green : 158u8, blue : 160u8, alpha : 1.0}
    }
    pub fn chartreuse() -> Color {
        Color {red : 127u8, green : 255u8, blue : 0u8, alpha : 1.0}
    }
    pub fn chocolate() -> Color {
        Color {red : 210u8, green : 105u8, blue : 30u8, alpha : 1.0}
    }
    pub fn coral() -> Color {
        Color {red : 255u8, green : 127u8, blue : 80u8, alpha : 1.0}
    }
    pub fn cornflowerblue() -> Color {
        Color {red : 100u8, green : 149u8, blue : 237u8, alpha : 1.0}
    }
    pub fn cornsilk() -> Color {
        Color {red : 255u8, green : 248u8, blue : 220u8, alpha : 1.0}
    }
    pub fn crimson() -> Color {
        Color {red : 220u8, green : 20u8, blue : 60u8, alpha : 1.0}
    }
    pub fn cyan() -> Color {
        Color {red : 0u8, green : 255u8, blue : 255u8, alpha : 1.0}
    }
    pub fn darkblue() -> Color {
        Color {red : 0u8, green : 0u8, blue : 139u8, alpha : 1.0}
    }
    pub fn darkcyan() -> Color {
        Color {red : 0u8, green : 139u8, blue : 139u8, alpha : 1.0}
    }
    pub fn darkgoldenrod() -> Color {
        Color {red : 184u8, green : 134u8, blue : 11u8, alpha : 1.0}
    }
    pub fn darkgray() -> Color {
        Color {red : 169u8, green : 169u8, blue : 169u8, alpha : 1.0}
    }
    pub fn darkgreen() -> Color {
        Color {red : 0u8, green : 100u8, blue : 0u8, alpha : 1.0}
    }
    pub fn darkgrey() -> Color {
        Color {red : 169u8, green : 169u8, blue : 169u8, alpha : 1.0}
    }
    pub fn darkkhaki() -> Color {
        Color {red : 189u8, green : 183u8, blue : 107u8, alpha : 1.0}
    }
    pub fn darkmagenta() -> Color {
        Color {red : 139u8, green : 0u8, blue : 139u8, alpha : 1.0}
    }
    pub fn darkolivegreen() -> Color {
        Color {red : 85u8, green : 107u8, blue : 47u8, alpha : 1.0}
    }
    pub fn darkorange() -> Color {
        Color {red : 255u8, green : 140u8, blue : 0u8, alpha : 1.0}
    }
    pub fn darkorchid() -> Color {
        Color {red : 153u8, green : 50u8, blue : 204u8, alpha : 1.0}
    }
    pub fn darkred() -> Color {
        Color {red : 139u8, green : 0u8, blue : 0u8, alpha : 1.0}
    }
    pub fn darksalmon() -> Color {
        Color {red : 233u8, green : 150u8, blue : 122u8, alpha : 1.0}
    }
    pub fn darkseagreen() -> Color {
        Color {red : 143u8, green : 188u8, blue : 143u8, alpha : 1.0}
    }
    pub fn darkslateblue() -> Color {
        Color {red : 72u8, green : 61u8, blue : 139u8, alpha : 1.0}
    }
    pub fn darkslategray() -> Color {
        Color {red : 47u8, green : 79u8, blue : 79u8, alpha : 1.0}
    }
    pub fn darkslategrey() -> Color {
        Color {red : 47u8, green : 79u8, blue : 79u8, alpha : 1.0}
    }
    pub fn darkturquoise() -> Color {
        Color {red : 0u8, green : 206u8, blue : 209u8, alpha : 1.0}
    }
    pub fn darkviolet() -> Color {
        Color {red : 148u8, green : 0u8, blue : 211u8, alpha : 1.0}
    }
    pub fn deeppink() -> Color {
        Color {red : 255u8, green : 20u8, blue : 147u8, alpha : 1.0}
    }
    pub fn deepskyblue() -> Color {
        Color {red : 0u8, green : 191u8, blue : 255u8, alpha : 1.0}
    }
    pub fn dimgray() -> Color {
        Color {red : 105u8, green : 105u8, blue : 105u8, alpha : 1.0}
    }
    pub fn dimgrey() -> Color {
        Color {red : 105u8, green : 105u8, blue : 105u8, alpha : 1.0}
    }
    pub fn dodgerblue() -> Color {
        Color {red : 30u8, green : 144u8, blue : 255u8, alpha : 1.0}
    }
    pub fn firebrick() -> Color {
        Color {red : 178u8, green : 34u8, blue : 34u8, alpha : 1.0}
    }
    pub fn floralwhite() -> Color {
        Color {red : 255u8, green : 250u8, blue : 240u8, alpha : 1.0}
    }
    pub fn forestgreen() -> Color {
        Color {red : 34u8, green : 139u8, blue : 34u8, alpha : 1.0}
    }
    pub fn fuchsia() -> Color {
        Color {red : 255u8, green : 0u8, blue : 255u8, alpha : 1.0}
    }
    pub fn gainsboro() -> Color {
        Color {red : 220u8, green : 220u8, blue : 220u8, alpha : 1.0}
    }
    pub fn ghostwhite() -> Color {
        Color {red : 248u8, green : 248u8, blue : 255u8, alpha : 1.0}
    }
    pub fn gold() -> Color {
        Color {red : 255u8, green : 215u8, blue : 0u8, alpha : 1.0}
    }
    pub fn goldenrod() -> Color {
        Color {red : 218u8, green : 165u8, blue : 32u8, alpha : 1.0}
    }
    pub fn gray() -> Color {
        Color {red : 128u8, green : 128u8, blue : 128u8, alpha : 1.0}
    }
    pub fn green() -> Color {
        Color {red : 0u8, green : 128u8, blue : 0u8, alpha : 1.0}
    }
    pub fn greenyellow() -> Color {
        Color {red : 173u8, green : 255u8, blue : 47u8, alpha : 1.0}
    }
    pub fn grey() -> Color {
        Color {red : 128u8, green : 128u8, blue : 128u8, alpha : 1.0}
    }
    pub fn honeydew() -> Color {
        Color {red : 240u8, green : 255u8, blue : 240u8, alpha : 1.0}
    }
    pub fn hotpink() -> Color {
        Color {red : 255u8, green : 105u8, blue : 180u8, alpha : 1.0}
    }
    pub fn indianred() -> Color {
        Color {red : 205u8, green : 92u8, blue : 92u8, alpha : 1.0}
    }
    pub fn indigo() -> Color {
        Color {red : 75u8, green : 0u8, blue : 130u8, alpha : 1.0}
    }
    pub fn ivory() -> Color {
        Color {red : 255u8, green : 255u8, blue : 240u8, alpha : 1.0}
    }
    pub fn khaki() -> Color {
        Color {red : 240u8, green : 230u8, blue : 140u8, alpha : 1.0}
    }
    pub fn lavender() -> Color {
        Color {red : 230u8, green : 230u8, blue : 250u8, alpha : 1.0}
    }
    pub fn lavenderblush() -> Color {
        Color {red : 255u8, green : 240u8, blue : 245u8, alpha : 1.0}
    }
    pub fn lawngreen() -> Color {
        Color {red : 124u8, green : 252u8, blue : 0u8, alpha : 1.0}
    }
    pub fn lemonchiffon() -> Color {
        Color {red : 255u8, green : 250u8, blue : 205u8, alpha : 1.0}
    }
    pub fn lightblue() -> Color {
        Color {red : 173u8, green : 216u8, blue : 230u8, alpha : 1.0}
    }
    pub fn lightcoral() -> Color {
        Color {red : 240u8, green : 128u8, blue : 128u8, alpha : 1.0}
    }
    pub fn lightcyan() -> Color {
        Color {red : 224u8, green : 255u8, blue : 255u8, alpha : 1.0}
    }
    pub fn lightgoldenrodyellow() -> Color {
        Color {red : 250u8, green : 250u8, blue : 210u8, alpha : 1.0}
    }
    pub fn lightgray() -> Color {
        Color {red : 211u8, green : 211u8, blue : 211u8, alpha : 1.0}
    }
    pub fn lightgreen() -> Color {
        Color {red : 144u8, green : 238u8, blue : 144u8, alpha : 1.0}
    }
    pub fn lightgrey() -> Color {
        Color {red : 211u8, green : 211u8, blue : 211u8, alpha : 1.0}
    }
    pub fn lightpink() -> Color {
        Color {red : 255u8, green : 182u8, blue : 193u8, alpha : 1.0}
    }
    pub fn lightsalmon() -> Color {
        Color {red : 255u8, green : 160u8, blue : 122u8, alpha : 1.0}
    }
    pub fn lightseagreen() -> Color {
        Color {red : 32u8, green : 178u8, blue : 170u8, alpha : 1.0}
    }
    pub fn lightskyblue() -> Color {
        Color {red : 135u8, green : 206u8, blue : 250u8, alpha : 1.0}
    }
    pub fn lightslategray() -> Color {
        Color {red : 119u8, green : 136u8, blue : 153u8, alpha : 1.0}
    }
    pub fn lightslategrey() -> Color {
        Color {red : 119u8, green : 136u8, blue : 153u8, alpha : 1.0}
    }
    pub fn lightsteelblue() -> Color {
        Color {red : 176u8, green : 196u8, blue : 222u8, alpha : 1.0}
    }
    pub fn lightyellow() -> Color {
        Color {red : 255u8, green : 255u8, blue : 224u8, alpha : 1.0}
    }
    pub fn lime() -> Color {
        Color {red : 0u8, green : 255u8, blue : 0u8, alpha : 1.0}
    }
    pub fn limegreen() -> Color {
        Color {red : 50u8, green : 205u8, blue : 50u8, alpha : 1.0}
    }
    pub fn linen() -> Color {
        Color {red : 250u8, green : 240u8, blue : 230u8, alpha : 1.0}
    }
    pub fn magenta() -> Color {
        Color {red : 255u8, green : 0u8, blue : 255u8, alpha : 1.0}
    }
    pub fn maroon() -> Color {
        Color {red : 128u8, green : 0u8, blue : 0u8, alpha : 1.0}
    }
    pub fn mediumaquamarine() -> Color {
        Color {red : 102u8, green : 205u8, blue : 170u8, alpha : 1.0}
    }
    pub fn mediumblue() -> Color {
        Color {red : 0u8, green : 0u8, blue : 205u8, alpha : 1.0}
    }
    pub fn mediumorchid() -> Color {
        Color {red : 186u8, green : 85u8, blue : 211u8, alpha : 1.0}
    }
    pub fn mediumpurple() -> Color {
        Color {red : 147u8, green : 112u8, blue : 219u8, alpha : 1.0}
    }
    pub fn mediumseagreen() -> Color {
        Color {red : 60u8, green : 179u8, blue : 113u8, alpha : 1.0}
    }
    pub fn mediumslateblue() -> Color {
        Color {red : 123u8, green : 104u8, blue : 238u8, alpha : 1.0}
    }
    pub fn mediumspringgreen() -> Color {
        Color {red : 0u8, green : 250u8, blue : 154u8, alpha : 1.0}
    }
    pub fn mediumturquoise() -> Color {
        Color {red : 72u8, green : 209u8, blue : 204u8, alpha : 1.0}
    }
    pub fn mediumvioletred() -> Color {
        Color {red : 199u8, green : 21u8, blue : 133u8, alpha : 1.0}
    }
    pub fn midnightblue() -> Color {
        Color {red : 25u8, green : 25u8, blue : 112u8, alpha : 1.0}
    }
    pub fn mintcream() -> Color {
        Color {red : 245u8, green : 255u8, blue : 250u8, alpha : 1.0}
    }
    pub fn mistyrose() -> Color {
        Color {red : 255u8, green : 228u8, blue : 225u8, alpha : 1.0}
    }
    pub fn moccasin() -> Color {
        Color {red : 255u8, green : 228u8, blue : 181u8, alpha : 1.0}
    }
    pub fn navajowhite() -> Color {
        Color {red : 255u8, green : 222u8, blue : 173u8, alpha : 1.0}
    }
    pub fn navy() -> Color {
        Color {red : 0u8, green : 0u8, blue : 128u8, alpha : 1.0}
    }
    pub fn oldlace() -> Color {
        Color {red : 253u8, green : 245u8, blue : 230u8, alpha : 1.0}
    }
    pub fn olive() -> Color {
        Color {red : 128u8, green : 128u8, blue : 0u8, alpha : 1.0}
    }
    pub fn olivedrab() -> Color {
        Color {red : 107u8, green : 142u8, blue : 35u8, alpha : 1.0}
    }
    pub fn orange() -> Color {
        Color {red : 255u8, green : 165u8, blue : 0u8, alpha : 1.0}
    }
    pub fn orangered() -> Color {
        Color {red : 255u8, green : 69u8, blue : 0u8, alpha : 1.0}
    }
    pub fn orchid() -> Color {
        Color {red : 218u8, green : 112u8, blue : 214u8, alpha : 1.0}
    }
    pub fn palegoldenrod() -> Color {
        Color {red : 238u8, green : 232u8, blue : 170u8, alpha : 1.0}
    }
    pub fn palegreen() -> Color {
        Color {red : 152u8, green : 251u8, blue : 152u8, alpha : 1.0}
    }
    pub fn paleturquoise() -> Color {
        Color {red : 175u8, green : 238u8, blue : 238u8, alpha : 1.0}
    }
    pub fn palevioletred() -> Color {
        Color {red : 219u8, green : 112u8, blue : 147u8, alpha : 1.0}
    }
    pub fn papayawhip() -> Color {
        Color {red : 255u8, green : 239u8, blue : 213u8, alpha : 1.0}
    }
    pub fn peachpuff() -> Color {
        Color {red : 255u8, green : 218u8, blue : 185u8, alpha : 1.0}
    }
    pub fn peru() -> Color {
        Color {red : 205u8, green : 133u8, blue : 63u8, alpha : 1.0}
    }
    pub fn pink() -> Color {
        Color {red : 255u8, green : 192u8, blue : 203u8, alpha : 1.0}
    }
    pub fn plum() -> Color {
        Color {red : 221u8, green : 160u8, blue : 221u8, alpha : 1.0}
    }
    pub fn powderblue() -> Color {
        Color {red : 176u8, green : 224u8, blue : 230u8, alpha : 1.0}
    }
    pub fn purple() -> Color {
        Color {red : 128u8, green : 0u8, blue : 128u8, alpha : 1.0}
    }
    pub fn red() -> Color {
        Color {red : 255u8, green : 0u8, blue : 0u8, alpha : 1.0}
    }
    pub fn rosybrown() -> Color {
        Color {red : 188u8, green : 143u8, blue : 143u8, alpha : 1.0}
    }
    pub fn royalblue() -> Color {
        Color {red : 65u8, green : 105u8, blue : 225u8, alpha : 1.0}
    }
    pub fn saddlebrown() -> Color {
        Color {red : 139u8, green : 69u8, blue : 19u8, alpha : 1.0}
    }
    pub fn salmon() -> Color {
        Color {red : 250u8, green : 128u8, blue : 114u8, alpha : 1.0}
    }
    pub fn sandybrown() -> Color {
        Color {red : 244u8, green : 164u8, blue : 96u8, alpha : 1.0}
    }
    pub fn seagreen() -> Color {
        Color {red : 46u8, green : 139u8, blue : 87u8, alpha : 1.0}
    }
    pub fn seashell() -> Color {
        Color {red : 255u8, green : 245u8, blue : 238u8, alpha : 1.0}
    }
    pub fn sienna() -> Color {
        Color {red : 160u8, green : 82u8, blue : 45u8, alpha : 1.0}
    }
    pub fn silver() -> Color {
        Color {red : 192u8, green : 192u8, blue : 192u8, alpha : 1.0}
    }
    pub fn skyblue() -> Color {
        Color {red : 135u8, green : 206u8, blue : 235u8, alpha : 1.0}
    }
    pub fn slateblue() -> Color {
        Color {red : 106u8, green : 90u8, blue : 205u8, alpha : 1.0}
    }
    pub fn slategray() -> Color {
        Color {red : 112u8, green : 128u8, blue : 144u8, alpha : 1.0}
    }
    pub fn slategrey() -> Color {
        Color {red : 112u8, green : 128u8, blue : 144u8, alpha : 1.0}
    }
    pub fn snow() -> Color {
        Color {red : 255u8, green : 250u8, blue : 250u8, alpha : 1.0}
    }
    pub fn springgreen() -> Color {
        Color {red : 0u8, green : 255u8, blue : 127u8, alpha : 1.0}
    }
    pub fn steelblue() -> Color {
        Color {red : 70u8, green : 130u8, blue : 180u8, alpha : 1.0}
    }
    pub fn tan() -> Color {
        Color {red : 210u8, green : 180u8, blue : 140u8, alpha : 1.0}
    }
    pub fn teal() -> Color {
        Color {red : 0u8, green : 128u8, blue : 128u8, alpha : 1.0}
    }
    pub fn thistle() -> Color {
        Color {red : 216u8, green : 191u8, blue : 216u8, alpha : 1.0}
    }
    pub fn tomato() -> Color {
        Color {red : 255u8, green : 99u8, blue : 71u8, alpha : 1.0}
    }
    pub fn turquoise() -> Color {
        Color {red : 64u8, green : 224u8, blue : 208u8, alpha : 1.0}
    }
    pub fn violet() -> Color {
        Color {red : 238u8, green : 130u8, blue : 238u8, alpha : 1.0}
    }
    pub fn wheat() -> Color {
        Color {red : 245u8, green : 222u8, blue : 179u8, alpha : 1.0}
    }
    pub fn white() -> Color {
        Color {red : 255u8, green : 255u8, blue : 255u8, alpha : 1.0}
    }
    pub fn whitesmoke() -> Color {
        Color {red : 245u8, green : 245u8, blue : 245u8, alpha : 1.0}
    }
    pub fn yellow() -> Color {
        Color {red : 255u8, green : 255u8, blue : 0u8, alpha : 1.0}
    }
    pub fn yellowgreen() -> Color {
        Color {red : 154u8, green : 205u8, blue : 50u8, alpha : 1.0}
    }
}
