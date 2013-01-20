use n;

pub enum StylesheetOrigin {
    OriginUA,
    OriginUser,
    OriginAuthor
}

impl StylesheetOrigin {
    fn to_net(&self) -> n::ll::t::css_origin {
        match *self {
            OriginUA => n::ll::t::CSS_ORIGIN_UA,
            OriginUser => n::ll::t::CSS_ORIGIN_USER,
            OriginAuthor => n::ll::t::CSS_ORIGIN_AUTHOR
        }
    }
}