extern crate html5ever;
extern crate regex;
extern crate xmltree;
extern crate itertools;

use self::html5ever::parse_document;
use self::html5ever::rcdom::{RcDom, Handle};
use self::html5ever::tendril::TendrilSink;

use self::regex::Regex;

use self::itertools::join;

pub mod optnode;
pub mod optnodes;
pub mod html;

use ir;

pub fn convert_file(contents: &str) -> ir::Document {
    fn fix_non_enclosing_tags(contents: &str) -> String {
        Regex::new(r"<img((.|\n)*?)/{0}>")
            .unwrap()
            .replace_all(contents, "<img$1 />")
            .into_owned()
    }
    let contents = fix_non_enclosing_tags(contents);

    let dom = parse_document(RcDom::default(), Default::default()).one(contents);

    let doc = html::convert_dom(&dom.document);

    panic!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn nothing() {
    }
}
