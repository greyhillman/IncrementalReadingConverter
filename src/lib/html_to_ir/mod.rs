extern crate html5ever;
extern crate regex;
extern crate xmltree;
extern crate itertools;

use self::html5ever::parse_document;
use self::html5ever::rcdom::RcDom;
use self::html5ever::tendril::TendrilSink;

pub mod html;

use ir;

pub fn convert_file(contents: &str) -> ir::Document {
    let dom = parse_document(RcDom::default(), Default::default()).one(contents);

    let doc = html::convert_dom(&dom.document);

    let doc = match doc {
        Ok(doc) => doc,
        Err(err) => {
            println!("Failed to convert document: Reason:\n{}", err);
            return ir::Document::new();
        }
    };

    let _ = doc.convert();

    panic!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn nothing() {}
}
