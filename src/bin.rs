use incodoc_to_ansi::*;

use md_to_incodoc::parse_md_to_incodoc;
use incodoc::output::doc_out;
use incodoc::reference_doc::REF_DOC;
use incodoc::parsing::parse;

const INPUT: &str =
"
This is a test. This is another sentence.
This is on another line.

- A | B
  --|--
  0 | 1
- C | D
  --|--
  2 | 3
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    let doc = parse(REF_DOC).unwrap();
    println!("{}", doc_to_ansi_string(&doc));
    // let mut output = String::new();
    // doc_out(&doc, &mut output);
    // println!("{output}");
}
