use incodoc_to_ansi::*;

use md_to_incodoc::parse_md_to_incodoc;
use incodoc::output::doc_out;
use incodoc::reference_doc::REF_DOC;
use incodoc::parsing::parse;

const INPUT: &str =
"
# H1

### H3

This is a test. This is another sentence.
This is on another line.

[this is a **bold** link](url)

### H3

```rust
let x = 0;
for i in 0..10 {
    println!(\"{}\", yay);
}
```

- test
  1. test
  1. A | B
     --|--
     0 | 1
  1. test
  1. test
  1. test
  1. test
  1. test
  1. test
  1. test
  1. test
  1. test
- test
  - [ ] test
  - [x]
    A | B
    --|--
    0 | 1
  - [ ] test
- test
  test
  - A | B
    --|--
    0 | 1
  - ```rust
    let x = 0;
    let y = 1;
    ```
  - A | B
    --|--
    0 | 1
  test
- ```rust
  let x = 0;
  let y = 1;
  ```
  test
- test
- A | B
  --|--
  0 | 1
C | D | E
--|--|--
2 | *3* | ~~4~~
**5** | ***6*** | `let x = 0;`

test line

+++
nav l0
  link link text $ url
  nav l1
    link link text $ url
    nav l2a
      link link text $ url
      link link text $ url
    end
    nav l2b
      link link text $ url
    end
  end
end
+++
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    // let doc = parse(REF_DOC).unwrap();

    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");

    println!("{}", doc_to_ansi_string(&doc));
}
