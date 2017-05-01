extern crate html5ever;
extern crate regex;
extern crate xmltree;
extern crate itertools;

use self::html5ever::parse_document;
use self::html5ever::rcdom::RcDom;
use self::html5ever::tendril::TendrilSink;

pub mod html;
mod optimize;
mod convert;

use ir;

pub fn convert_file(contents: &str) -> ir::Document {
    let dom = parse_document(RcDom::default(), Default::default()).one(contents);

    let doc = html::convert_dom(&dom.document);

    match doc {
        Ok(doc) => doc.convert(),
        Err(err) => {
            println!("Failed to convert document: Reason:\n{}", err);
            return ir::Document::new();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn body(content: &str) -> String {
        format!("<html><body>{}</body></html>", content)
    }

    #[test]
    fn empty() {
        let content = "";
        let result = ir::Document::new();
        assert_eq!(convert_file(content), result);
    }

    #[test]
    fn empty_body() {
        let content = &body("");
        let result = ir::Document::new();
        assert_eq!(convert_file(content), result);
    }

    #[test]
    fn only_head() {
        let content = "<html><head></head></html>";
        let result = ir::Document::new();
        assert_eq!(convert_file(content), result);
    }

    #[test]
    fn head_with_script() {
        let content = "<html><head><script></script></head></html>";
        let result = ir::Document::new();
        assert_eq!(convert_file(content), result);
    }

    #[test]
    fn head_with_style() {
        let content = "<html><head><style></style></head></html>";
        let result = ir::Document::new();
        assert_eq!(convert_file(content), result);
    }

    #[test]
    fn par() {
        let content = &body("<p>Text</p>");
        let result = ir::Document::new()
            .add(ir::IR::par(ir::TextBlock::new()
                             .add(ir::Text::text("Text"))));
        assert_eq!(convert_file(content), result);
    }

    #[test]
    fn img() {
        let content = &body("<img src=\"image.png\" />");
        let result = ir::Document::new()
            .add(ir::IR::img("image.png"));
        assert_eq!(convert_file(content), result);
    }

    #[test]
    fn pre() {
        let content = &body("<pre><span>x</span> = <span>1</span>\n\nx</pre>");
        let result = ir::Document::new()
            .add(ir::IR::pre("x = 1\n\nx"));
        assert_eq!(convert_file(content), result);
    }
}
