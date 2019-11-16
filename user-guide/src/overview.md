# Overview

The `mdbook-html` project is an alternate backend for [`mdbook`][mdbook], a
program for compiling a set of linked Markdown files into a book-like format
viewable in the browser.

Out of the box, `mdbook-html` should be a drop-in replacement for the default
HTML renderer and *Just Work*.

Some reasons you may want to use `mdbook-html`:

- You want to add a small tweak to the page without needing to maintain a fork
  of the entire [`index.hbs`][index-hbs]
- You want to write your own templates to get tighter control over how a
  document is laid out

[mdbook]: https://github.com/rust-lang-nursery/mdBook
[index-hbs]: https://github.com/rust-lang/mdBook/blob/master/src/theme/index.hbs