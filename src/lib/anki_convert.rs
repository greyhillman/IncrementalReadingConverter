pub mod group_lines;
pub mod ir_to_anki;
pub mod html_to_ir;

pub fn convert_file(debug: bool, file_type: &str, contents: &str) -> String {
    match file_type {
        "html" | "xhtml" => {
            if debug {
                html_to_ir::convert_file(contents)
            } else {
                ir_to_anki::convert_file(&html_to_ir::convert_file(contents))
            }
        }
        _ => panic!("Filetype not supported."),
    }
}
