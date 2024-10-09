<!---
SPDX-FileCopyrightText: 2020-2021 Robin Krahl <robin.krahl@ireas.org>
SPDX-License-Identifier: CC0-1.0
-->

# Unreleased

## Breaking Changes

- Introduce the `IntoBoxedElement` trait and use it for the `push` and
  `element` methods of `Document`, `LinearLayout` and `TableLayoutRow`.
- Support setting the line thickness and color for `FramedElement` and
  `FrameCellDecorator`:
  - Add the `LineStyle` struct.
  - Add the `FramedElement::with_line_style` and
    `FrameCellDecorator::with_line_style` constructors.
  - Remove the `style` argument from `CellDecorator::decorate_cell`.
  - Change the `Style` argument for `Area::draw_line` to `LineStyle`.
  - Change `Element::framed` to take the line style as an argument.
  - Add the `prepare_cell` method to `CellDecorator`.
  - Add the `row_height` argument to `CellDecorator::decorate_cell` and make it
    return the total row height.
- Fix the line height calculations for multi-style paragraphs:
  - Introduce the `fonts::Metrics` struct and the `Font::metrics` and
    `Style::metrics` methods.
  - Calculate the maximum line metrics in `Paragraph::render`.
  - Change `Area::text_section` to take `Metrics` instead of `Style`.
- Refactor the `render` module:
  - Change the `Layer`, `Area` and `TextSection` lifetimes.
  - Store a reference to the current `Page` in `Layer`.
  - Accept a point iterator instead of a point vector in `Area::draw_line`.
- Remove the `From<Position>` implementation for `printpdf::Point`.

## Non-Breaking Changes

- Implement `std::iter::Extend` for `Document`, `LinearLayout`,
  `UnorderedList`, `OrderedList` and `TableLayoutRow`.
- Implement `std::iter::FromIterator` for `UnorderedList` and `OrderedList`.
- Add the `minimal` example that produces a minimal PDF document.
- Add the `Layer::next` and `Area::next_layer` methods for accessing the next
  layer of a page.
- Remove left bearing from the first character of a string for consistent
  alignment with different font sizes.
- Add `set_creation_date` and `set_modification_date` methods to `Document` and
  `with_creation_date` and `with_modification_date` to `Renderer`.
- Add basic test suite.
- Add the `UserSpacePosition` and `LayerPosition` structs to the `render`
  module.
- Cache per-layer settings (fill color, outline color, outline thickness) and
  per-text-section settings (font family and size).

## Bug Fixes

- Return an error if a paragraph overflows.
- Use the ascent instead of the glyph height for vertical positioning of text.

# v0.2.0 (2021-06-17)

This release improves the font handling, adds support for embedding images and
contains many small improvements and bugfixes.

Thanks to Alexander Dean-Kennedy for implementing the images support and to
Scott Steele for contributing a bug fix.

## Breaking Changes

- Improve the font handling:
  - Make `FontFamily` generic over the font data type.
  - Make the fields of the `FontFamily` struct public.
  - Load the PDF font in `Renderer::load_font` from bytes instead of a path.
  - Separate font loading and font caching:
    - Replace the `load_font` and `load_font_family` methods of the `FontCache`
      struct with `add_font` and `add_font_family`, and the `load_font_family`
      method of `Document` with `add_font_family`.
    - Add the `FontData::load` method and the `fonts::load_from_files`
      function.
    - Change the arguments of the `FontCache::new` and `Decorator::new`
      methods.
  - Make the `Document::new`, `Document::add_font_family`, `FontCache::new`,
    `FontCache::add_font`, `FontCache::add_font_family` and `Font::new` methods
    infallible.
  - Add support for built-in fonts.
    - Added the `Error::UnsupportedEncoding` variant.
    - Change the return type of the `Area::print_str` and
      `TextSection::print_str` methods to return a `Result`.
- Move the `FontCache` instance used during the rendering process to the new
  `Context` struct.
- Remove the `Document::set_margins` method (use a `PageDecorator` instead).
- Replace the `PdfprintError` variant of `ErrorKind` with `PdfError` and
  `PdfIndexError`.
- Change the return type of `render::Area::text_section` from `Result<_, ()>`
  to  `Option<_>`.
- Bump the MSRV to 1.45.0.
- Move `Alignment` struct out of `elements` module.

## Non-Breaking Changes

- Add the `StyledCow<'s>` struct that contains a `Cow<'s, str>` with a `Style`
  annotation.
- Derive `Copy` for `StyledStr`.
- Add support for hyphenation (enabled by the `hyphenation` feature).
- Add the `PageBreak` element.
- Implement `From<Vec<StyledString>>` for `Paragraph`.
- Add the `PageDecorator` trait, the `SimplePageDecorator` implementation and
  the `Document::set_page_decorator` method to allow customization of all
  document pages.
- Add support for kerning and add the `Font::kerning` and `Font::glyph_ids`
  methods.
- Add the `error::Context` trait for easier error generation.
- Add support for `Image` as a possible insertable element.

## Bug Fixes

- Always use the configured paper size when adding new pages to a `Document`.

# v0.1.1 (2020-10-16)

This patch release adds some trait implementations and utility functions and
improves the crate documentation.

Thanks to Matteo Bertini for contributions.

- Implement `From<&String>` for `StyledString`.
- Derive `Add`, `AddAssign`, `Sub` and `SubAssign` for `Position` and `Size`.
- Implement `From<Position>` for `printpdf::Point`.
- Add `split_horizontally` method to `Area`.
- Add `width` method to `StyledString` and `StyledStr`.
- Add `font_cache` method to `Document`.

# v0.1.0 (2020-10-15)

Initial release with support for formatted text, text wrapping and basic
shapes.
