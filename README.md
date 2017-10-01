# css

[css] is a rust crate that implements a CSS stylesheet parser and serializer. It uses the [cssparser](https://docs.rs/crate/cssparser/) crate. It wraps source code originally forked of the style source code from [servo](https://github.com/servo/servo). The great efforts and achievements of those at Mozilla that have built Servo are very much appreciated.


## Licensing

The license for this project is Mozilla Public License 2.0 (MPL-2.0). The code forked from Servo is also licensed under Mozilla Public License 2.0. The Servo Developers, part of the Mozilla Foundation, retain copyright on the unaltered style source code, and also its implementation approach, forked from Servo.


## CSS Simplifications for Size


### Selectors

* Deduplicates lists of selectors


### @media

* calc() nodes are reduced to simple assignments except for lengths
* aspect-ratios are normalized using Euclid's algorithm
* resolutions are reduced to either pixel densities (eg a DPI of 96 => a density of 1) or DPI from dots-per-centimetre (which is always larger)
* Keyframe percentage of '100%' is written as 'to'


### @font-face

* font feature settings have duplicated setting names removed
* font feature settings are written as '0' and '1' instead of 'on' and 'off'


### @viewport

* user-zoom values are normalized to 'zoom' and 'fixed'


### Miscellany

* Angles are always converted to radians (this may need to be reviewed)
* Times are always converted to seconds (this may need to be reviewed)


## CSS Corrections and Upgrades

* Unprefixed lengths are given a dimension of `px`
* The `aural` media type is converted to `speech`
* The non-standard [-webkit-device-pixel-ratio set of media queries](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/-webkit-device-pixel-ratio) are converted into `resolution` 


## TODO


### Property declarations and similar declarations

* Do not specify a dimension if the value is 0 (eg for width)
* Can omit final ';' in a list of property declarations (valid in W3C parser in any event)


### @-rules

* Eliminate -webkit-keyframes and -moz-keyframes from resultant rules
* Eliminate -moz-document?
* Optimise [@viewport](https://developer.mozilla.org/en-US/docs/Web/CSS/@viewport) by removing duplicated rules, forcing definitions to shorthand forms, etc
* Output prefixes -ms-, -o- for @viewport, and remove unsupported properties (orientation only for Opera, width and height only for IE11 / Edge)
* Support page
* Output page, stripping size property, removing duplicates, etc
* Support parsing @counter-style Symbol::Image()


### Media Queries

* Support Level 4 operator syntax (eg `>=`)
* Support parsing in a boolean context where a value has been omitted and defaults to zero (0) or none, eg for grid, color, etc
* Support compressing CSS by omitting default values of zero (0) or not() when serializing
* Support calc() for aspect-ratio

[css]: https://github.com/lemonrock/css "css GitHub page"
