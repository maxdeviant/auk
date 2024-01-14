# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added `WithChildren` trait.
- `HtmlElement`: Added `text_content_mut` method.
- Added docs for HTML elements.
- `HtmlElement`: Added new attribute methods:
  - `max`
  - `target`
  - `value`
- Added `Visitor` trait.
- Added `MutVisitor` trait.
- Added `HtmlElementRenderer`.

### Deprecated

- Deprecated `HtmlElement::render_to_string` in favor of `HtmlElementRenderer::render_to_string`.

## [0.2.0] - 2024-01-06

### Changed

- Changed `HtmlElement` fields to private.

## [0.1.0] - 2024-01-06

- Initial release.

[unreleased]: https://github.com/maxdeviant/auk/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/maxdeviant/auk/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/maxdeviant/auk/releases/tag/v0.1.0
