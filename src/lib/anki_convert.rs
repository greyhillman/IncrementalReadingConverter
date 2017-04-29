pub mod group_lines;
pub mod ir_to_anki;
pub mod html_to_ir;
pub mod ir;

fn convert_file_debug(file_type: &str, contents: &str) -> String {
    match file_type {
        "html" | "xhtml" => format!("{:?}", html_to_ir::convert_file(contents)),
        _ => panic!("Filetype not supported."),
    }
}

pub fn convert_file(debug: bool, file_type: &str, contents: &str) -> String {
    if debug {
        return convert_file_debug(file_type, contents);
    }

    match file_type {
        "html" | "xhtml" => {
            let doc = html_to_ir::convert_file(contents);
            ir_to_anki::convert(doc)
        }
        _ => panic!("Filetype not supported."),
    }
}
