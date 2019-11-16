# Themes

The `mdbook-html` renderer figures out how to render a book based on a set of
conventions and standard concepts. These are bundled into something referred
to as a *Theme*.

A *Theme* will typically be a set of files on disk, with the general directory
structure looking something like this:

- theme/
  - layouts/
    - chapter.hbs
    - print.hbs
  - partials/
    - head.hbs
    - header.hbs
    - sidebar.hbs
    - content.hbs
    - footer.hbs
  - static/
    - js/
      - my_custom_script.js
    - css/
      - custom_stylesheet.css

If the book contains a `themes/` directory `mdbook-html` will automatically
load everything inside. These files take preference over the default theme, so
overriding (for example) the `head.hbs` partial is just a case of adding your
own `theme/partials/head.hbs` file.

# Layout

When rendering a particular page, `mdbook-html` will choose the top-level
template from the `layouts/` directory based on the page *Kind*.

The `chapter.hbs` template defines how a normal chapter will be rendered, and
the `print.hbs` template will be used when generating a printable version of
the entire document.

<details>
<summary>The default <code>chapter.hbs</code> template</summary>

```html
{{#include ../../default-theme/layouts/chapter.hbs}}
```
</details>

<details>
<summary>The default <code>print.hbs</code> template</summary>

```html
{{#include ../../default-theme/layouts/print.hbs}}
```
</details>

# Partials

Any file within the `partials/` directory will be accessible as a [Handlebars
Partial][hbs-partial] during the rendering process. Defining a custom partial
allows theme developers to reuse common code and hook into the document
rendering process at pre-defined locations.

# Static Files

All files inside the `static/` directory are copied directly to the output
location without being processed by `mdbook-html` in any way. This is typically
used for static assets like icons, fonts, CSS, and JavaScript.

[hbs-partial]: https://handlebarsjs.com/#partials