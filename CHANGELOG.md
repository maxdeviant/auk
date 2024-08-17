# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `HtmlElement`: Added new attribute methods:
  - `for_`
  - `type_`

### Removed

- Removed `text` function for construction `TextElement`s.
  - Use `TextElement::new` instead.

## [0.4.0] - 2024-08-10

### Added

- Added `With` trait.
- Added `tbody` element.
- `HtmlElement`: Added new attribute methods:
  - `alt`
  - `crossorigin`
  - `loading`
- `HtmlElementRenderer`: Added `html` getter.
- `Element`, `HtmlElement`, and `TextElement` now implement `Clone`.

### Changed

- `HtmlElement`: Made fields `pub`.
- `HtmlElement`: Made `attr` method `pub`.
- `HtmlElement`: Made `is_void` method `pub`.
- `TextElement`: Made fields `pub`.
- `WithChildren`: Replaced `children_mut` with `extend`.
- `HtmlElementRenderer`: `render_to_string` now omits empty attributes.

## [0.3.0] - 2024-01-13

### Added

- Added `Element` enum.
- Added `TextElement` struct.
- Added `WithChildren` trait.
- `HtmlElement`: Added new attribute methods:
  - `max`
  - `target`
  - `value`
- Added docs for HTML elements.
- Added `Visitor` trait.
- Added `MutVisitor` trait.
- Added `HtmlElementRenderer`.

### Removed

- `HtmlElement`: Removed `text_content` method in favor of `child`.
- `HtmlElement`: Removed `render_to_string` method in favor of `HtmlElementRenderer::render_to_string`.

## [0.2.0] - 2024-01-06

### Changed

- Changed `HtmlElement` fields to private.

## [0.1.0] - 2024-01-06

- Initial release.

[unreleased]: https://github.com/maxdeviant/auk/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/maxdeviant/auk/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/maxdeviant/auk/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/maxdeviant/auk/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/maxdeviant/auk/releases/tag/v0.1.0
