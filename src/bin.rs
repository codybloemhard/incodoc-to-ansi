use incodoc_to_ansi::*;
use incodoc_to_ansi::config::*;

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
This is `inline code` inside a line.

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

footnote [^0]
another [^longernoteid]

[^0]: footnote def

[^longernoteid]:
  line 0.
  line 1.
  `{ code }`
  line 2.
  line 3.

> this is a quote
> with some **bold** in it
> > [!NOTE]
> > another quote

- test
  - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
  - bbbbb
  - cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc
- ddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
  - test
  - test

aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa

# h1

aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa

## h2

aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa

```rust
let xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx = 000000000000000000000000000000000000000000000000000000000000000000000;
```

A | B
--|--
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb

- `loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong`
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    // let doc = parse(REF_DOC).unwrap();
    // let doc = parse_md_to_incodoc(&simpleio::read_file_into_string("/home/cody/git/linux-rice/README.md").unwrap());

    // let mut output = String::new();
    // doc_out(&doc, &mut output);
    // println!("{output}");

    let cs = term_size::dimensions().unwrap_or((80, 0)).0;
    let conf = Config {
        width: cs,
        nav_config: NavConfig {
            link_indent: 3,
            sub_indent: 3,
            pre_description_newlines: 1,
            post_description_newlines: 1,
            pre_link_newlines: 1,
        },
        section_config: SectionConfig {
            paragraph_indent: 2,
            section_indent: 2,
        },
        headed_section_config: HeadedSectionConfig {
            pre_heading_newlines: 1,
            post_heading_newlines: 2,
        },
        blockquote_config: BlockquoteConfig {
            pre_quote_newlines: 1,
        },
        code_block_config: CodeBlockConfig {
            indent: 0,
            pre_code_block_newlines: 1,
        },
        list_config: ListConfig {
            pre_item_newlines: 1,
        },
        table_config: TableConfig {
            pre_table_newlines: 1,
        },
    };
    println!("{}", doc_to_ansi_string(&doc, &conf));
}

