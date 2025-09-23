use incodoc_to_ansi::*;

use md_to_incodoc::parse_md_to_incodoc;
use incodoc::output::doc_out;
use incodoc::reference_doc::REF_DOC;
use incodoc::parsing::parse;

const INPUT: &str =
"
This is a test. This is another sentence.
This is on another line.

[this is a **bold** link](url)

```rust
let x = 0;
for i in 0..10 {
    println!(\"{}\", yay);
}
```

- test
  - test
  - test
- test
  test
  test
- ```rust
  let x = 0;
  let y = 1;
  ```
- test
- A | B
  --|--
  0 | 1
C | D
--|--
2 | 3
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    // let doc = parse(REF_DOC).unwrap();

    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");

    println!("{}", doc_to_ansi_string(&doc));
}
