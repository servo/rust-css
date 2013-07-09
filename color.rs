/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::float::round;
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

    let r = round(255.0*hue_to_rgb(m1, m2, h + 1.0/3.0) as c_double);;
    let g = round(255.0*hue_to_rgb(m1, m2, h) as c_double);
    let b = round(255.0*hue_to_rgb(m1, m2, h - 1.0/3.0) as c_double);

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
        for only_colors.split_iter(',').advance |s| {
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
        for only_vals.split_iter(',').advance |s| {
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
        for only_vals.split_iter(',').advance |s| {
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
        for only_vals.split_iter(',').advance |s| {
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

    // The 16 basic css colors
    pub fn black() -> Color {
        Color {red : 0u8, green : 0u8, blue : 0u8, alpha : 1.0}
    }
    pub fn silver() -> Color {
        Color {red : 192u8, green : 192u8, blue : 192u8, alpha : 1.0}
    }
    pub fn gray() -> Color {
        Color {red : 128u8, green : 128u8, blue : 128u8, alpha : 1.0}
    }
    pub fn white() -> Color {
        Color {red : 255u8, green : 255u8, blue : 255u8, alpha : 1.0}
    }
    pub fn maroon() -> Color {
        Color {red : 128u8, green : 0u8, blue : 0u8, alpha : 1.0}
    }
    pub fn red() -> Color { 
        Color {red : 255u8, green : 0u8, blue : 0u8, alpha : 1.0}
    }
    pub fn purple() -> Color {
        Color {red : 128u8, green : 0u8, blue : 128u8, alpha : 1.0}
    }
    pub fn fuchsia() -> Color {
        Color {red : 255u8, green : 0u8, blue : 255u8, alpha : 1.0}
    }
    pub fn green() -> Color { 
        Color {red : 0u8, green : 128u8, blue : 0u8, alpha : 1.0}
    }
    pub fn lime() -> Color {
        Color {red : 0u8, green : 255u8, blue : 0u8, alpha : 1.0}
    }
    pub fn olive() -> Color {
        Color {red : 128u8, green : 128u8, blue : 0u8, alpha : 1.0}
    }
    pub fn yellow() -> Color {
        Color {red : 255u8, green : 255u8, blue : 0u8, alpha : 1.0}
    }
    pub fn navy() -> Color {
        Color {red : 0u8, green : 0u8, blue : 128u8, alpha : 1.0}
    }
    pub fn blue() -> Color {
        Color {red : 0u8, green : 0u8, blue : 255u8, alpha : 1.0}
    }
    pub fn teal() -> Color {
        Color {red : 0u8, green : 128u8, blue : 128u8, alpha : 1.0}
    }
    pub fn aqua() -> Color {
        Color {red : 0u8, green : 255u8, blue : 255u8, alpha : 1.0}
    }


    // The other 130 css colors
    // TODO
}
