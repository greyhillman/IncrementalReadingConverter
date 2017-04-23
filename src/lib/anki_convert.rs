pub mod group_lines;
pub mod ir_to_anki;
pub mod html_to_ir;

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
        "html" | "xhtml" => html_to_ir::convert_file(contents).handle(),
        _ => panic!("Filetype not supported."),
    }
}
