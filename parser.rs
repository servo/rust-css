/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/**
Constructs a list of css style rules from a token stream
*/

// TODO: fail according to the css spec instead of failing when things
// are not as expected

use util::DataStream;
use netsurfcss::stylesheet::{CssStylesheet, CssStylesheetParams, CssStylesheetParamsVersion1, css_stylesheet_create};
use netsurfcss::types::CssLevel21;
use netsurfcss::CssResult;
use wapcaplet::LwcString;
use extra::url::Url;
use netsurfcss::stylesheet::CssUrlResolutionFn;

fn default_params(url: Url) -> CssStylesheetParams {
    let resolve: CssUrlResolutionFn = resolve_url;
    CssStylesheetParams {
        params_version: CssStylesheetParamsVersion1,
        level: CssLevel21,
        charset: ~"UTF-8",
        url: url.to_str(),
        title: ~"FIXME-css-title",
        allow_quirks: false,
        inline_style: false,
        resolve: Some(resolve),
        import: None,
        color: None,
        font: None,
    }
}

// This takes a DataStreamFactory instead of a DataStream because
// servo's DataStream contains a comm::Port, which is not sendable,
// so DataStream is an @fn which can't be sent to the lexer task.
// So the DataStreamFactory gives the caller an opportunity to create
// the data stream from inside the lexer task.
pub fn parse_stylesheet(url: Url, input: DataStream) -> CssStylesheet {
    let params = default_params(url);
    let mut sheet = css_stylesheet_create(&params);

    loop {
        match input() {
            Some(data) => {
                sheet.append_data(data);
            }
            None => break
        }
    }
    sheet.data_done();
    sheet
}

pub fn parse_style_attribute(url: Url, data: &str) -> CssStylesheet {
    let mut params = default_params(url);
    params.inline_style = true;
    let mut sheet = css_stylesheet_create(&params);
    sheet.append_data(data.as_bytes());
    sheet.data_done();
    sheet
}

fn resolve_url(_base: &str, _rel: &LwcString) -> CssResult<LwcString> {
    fail!(~"resolving url");
}
