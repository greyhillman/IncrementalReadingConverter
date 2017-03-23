fn multi_to_single(lines: &str) -> String {
    lines.to_string()
        .lines()
        .fold(String::new(), |mut acc, mut x| {
            acc = acc.trim().to_string();
            x = x.trim();
            match acc.pop() {
                Some('-') => acc + &x,
                Some(c) => {
                    acc.push(c);
                    acc + " " + &x
                }
                None => acc + &x,
            }
        })
        .trim()
        .to_string()
}

pub fn group_lines_file(contents: &str) -> String {
    contents.lines()
        .map(|x| x.trim())
        .fold(String::new(), |acc, x| acc + &x + "\n")
        .split("\n\n")
        .map(multi_to_single)
        .filter(|x| !x.is_empty())
        .fold(String::new(), |acc, x| acc + &x + "\n\n")
        .trim_right()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_to_empty() {
        assert!(multi_to_single("").is_empty());
    }

    #[test]
    fn single_to_single() {
        assert_eq!(multi_to_single("abc"), "abc".to_string());
    }

    #[test]
    fn double() {
        assert_eq!(multi_to_single("Hello\nthere"), "Hello there".to_string());
    }

    #[test]
    fn triple() {
        assert_eq!(multi_to_single("a\nb\nc"), "a b c".to_string());
    }

    #[test]
    fn double_hypen() {
        assert_eq!(multi_to_single("a-\nb"), "ab".to_string());
    }

    #[test]
    fn triple_hyphen() {
        assert_eq!(multi_to_single("a-\nb-\nc"), "abc".to_string());
    }

    #[test]
    fn indented_par() {
        assert_eq!(multi_to_single("a\n   b   \nc"), "a b c".to_string());
    }

    #[test]
    fn indented_par_to_line_space() {
        let contents = "   abc\n\ndef\n\n   gh";
        assert_eq!(group_lines_file(contents), "abc\n\ndef\n\ngh".to_string());
    }

    #[test]
    fn indented_pars_with_line_sep() {
        let contents = "    abc\n    def\n\n$ a = b $\n\n    thus";
        assert_eq!(group_lines_file(contents),
                   "abc def\n\n$ a = b $\n\nthus".to_string());
    }

    #[test]
    fn empty_line_space() {
        let contents = "    abc\n     \n$$a$$";
        let result = "abc\n\n$$a$$".to_string();
        assert_eq!(group_lines_file(contents), result);
    }

    #[test]
    fn single_par() {
        let contents = "This is a line\nthat is follow-\ned by another line.";
        let result = "This is a line that is followed by another line.".to_string();
        assert_eq!(group_lines_file(contents), result);
    }

    #[test]
    fn two_par() {
        let contents = "This\n\nis\n\nspart-\na!";
        let result = "This\n\nis\n\nsparta!".to_string();
        assert_eq!(group_lines_file(contents), result);
    }

}
