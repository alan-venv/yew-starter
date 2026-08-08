---
title: Loading Local Fonts
---

## Loading Local Fonts

For Yew applications built with Trunk, keep font files under `static/fonts` and copy the `static` directory from `index.html`.

**Example:**

```html
<link data-trunk rel="copy-dir" href="./static" />
```

Declare local fonts with `@font-face` in the global stylesheet before using them. Do not rely only on `font-family: "Font Name"` because that only works if the browser already knows the font.

Use `font-display: block` when preserving the visual identity and avoiding fallback-to-custom-font layout shifts is more important than showing fallback text immediately.

**Example:**

```css
@font-face {
    font-family: "Fira Code";
    src: url("/static/fonts/FiraCode-Regular.woff2") format("woff2");
    font-weight: 400;
    font-style: normal;
    font-display: block;
}

@font-face {
    font-family: "Fira Code";
    src: url("/static/fonts/FiraCode-SemiBold.woff2") format("woff2");
    font-weight: 600;
    font-style: normal;
    font-display: block;
}

body {
    font-family: "Fira Code", monospace;
}
```

Keep the generic fallback, such as `monospace`, even when the local font is expected to load. It preserves the intended font category if the asset path, cache, CDN, or browser loading fails.
