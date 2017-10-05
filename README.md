# css

[css] is a rust crate that implements a CSS stylesheet parser and serializer. It uses the [cssparser](https://docs.rs/crate/cssparser/) crate. It wraps source code originally forked of the style source code from [servo](https://github.com/servo/servo). The great efforts and achievements of those at Mozilla that have built Servo are very much appreciated.

To get going add a dependency to Cargo.toml, eg:-

```toml
[dependencies]
css = "*"
```

And then add some simple code, say to `lib.rs` of the form:-

```rust
extern crate css;
use ::css::Stylesheet;

// Parse
let stylesheetUtf8String: String = ...;
let stylesheet = Stylesheet::parse(&stylesheetUtf8String).expect("Wasn't a valid stylesheet");

// Serialize

use ::std::fmt::Write;

let mut destination: Write = ...
const include_source_urls: bool = false;
stylesheet.to_css(&mut destination, include_source_urls).expect("Wrote out a stylesheet");
```


## Purpose

I've built this crate to allow me to create Rust-ified variants of Autoprefixer and PurifyCSS. Why? Because my [cordial](https://github.com/lemonrock/cordial) web server runs facing the internet, and, given the nature of NodeJS code, it's not acceptable to have it as a dependency. It can also be used to evaluate `@media`, `@viewport` and `calc() / var() / attr()` rules and expressions.

It is, therefore, quite incomplete. I'd welcome patches to add support for parsing common (and less common) CSS properties.

Please note that unlike a web browser [css] does not ignore what it does not understand. This is deliberate; it makes it far easier to catch assumptions in CSS code. Please patch the source if there's something you think should be parsed.

This crate will break compatibility regularly and often as it matures. It also pays no attention whatsoever to semver. At some point, if I persist with it, it should settle down and then be suitable for a more considered approach.



## Licensing

The license for this project is Mozilla Public License 2.0 (MPL-2.0). The code forked from Servo is also licensed under Mozilla Public License 2.0. The Servo Developers, part of the Mozilla Foundation, retain copyright on the unaltered style source code, and also its implementation approach, forked from Servo.

There is a temporary fork of the selectors crate (and its dependency servo_arc), also from Servo, in src/selectors/temporary_fork (from "https://github.com/servo/servo", rev = "4f984a6428a0f497e311a0800efa55166c15aac6"). This will be removed once a newer version of the selectors crate (such as 0.19.0) is published to [crates.io](https://crates.io/).


## CSS Simplifications for Size


### Selectors

* Deduplicates lists of selectors


### @media

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
* Optimise css colors to the smallest size, whether hex, rgb, or a name


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

/*

Analysis of autoprefixer rules still needed

Needed (Prefixes, either property names or property values)
css-animation
	-webkit- UC Browser for Android
transforms2d
	-webkit- UC Browser for Android
transforms3d
	-webkit- UC Browser for Android
	backface-visibility needs a prefix for Safari 9 / 10
css-filters
	-webkit- UC Browser, Samsung Internet, older Android Browser
user-select-none
	-ms-, -moz-, -webkit- but not -o-
font-feature
	-webkit- UC Browser for Android and older Samsung
font-kerning
	-webkit- for iOS Safari, surprisingly
css-hyphens
	-ms-, -webkit-
	-broken on new blink (chrome, opera)
css3-tabsize
	-moz- required
css-sticky

Needed (Selectors)
css-selection
	- the ::selection psuedo-element needs a -moz- prefix
css-placeholder
	- the ::selection psuedo-element needs a -ms- and -webkit- prefix, but is a bit messy: see http://caniuse.com/#search=css-placeholder&feature_sort=alpha
fullscreen
	-ms-, -moz-, -webkit- prefixes
	- the :fullscreen and ::backdrop selectors (note the single colon for :fullscreen is correct)
	- given the current status is quite a mess, probably best avoided
intrinsic-width
	- a bit of a mess
text-decoration
	- prefix with -webkit; avoid text-decoration-skip and text-decoration-style
	(prefix text-decoration-skip with -webkit- and covert values to none and skip); blink browsers only support objects and ink
text-size-adjust
	- prefix with -moz and -webkit
css-masks
	- prefix with -moz and -webkit
css-clip-path
	- prefix with -webkit
css-boxdecorationbreak
	- prefix with -webkit
css-logical-props
	- prefix with -webkit, -moz
css-appearance
	- prefix with -webkit, -moz
css-writing-mode
	- prefix with -webkit
css-read-only-write
	- prefix ':read-write' selector with -moz
	- prefix ':read-only' selector with -moz
text-emphasis
	- prefix with -webkit
css-grid
	- prefix with -ms
css-any-link
	- prefix with -moz, -webkit for the :any-link CSS pseudo-class

Complex mess not readily supportable
multicolumn
	- lots of bugs, some historic, some of which can't be fixed with a simple autoprefixer
flexbox
	- only older syntax supported by UC Browser for Android, and with -webkit- prefix
	- IE 11 has a large number of bugs making flexbox effectively unsable
css-crisp-edges
	- need -moz- prefix but only supports crisp-edges, however, chrome, with official support, only supports pixelated...
	- need -ms- for IE111, but IE11 is quite broken
	- need -webkit- for UC Browser and a non-standard value, too

Not enough support
css-snappoints
	- not supported in chrome, so not interesting
css-regions
	- not supported in chrome or firefox
css-image-set
	- prefix with -webkit; needs parsing of the background property to properly implement
css-cross-fade
	- prefix with -webkit; needs parsing of data

Viewport
	css-deviceadaptation
		- add -ms- prefix; remove all but width and height
		- for older opera mobile, -o- prefix, remove all but orientation

Media Queries
	css-media-resolution
		- need -webkit- prefix
		- IE11 only supports the dpi unit


Non-Standard
	- Firefox, Chrome and Safari support background-img-opts / -webkit-background-clip: text


Not Needed
border-radius
css-boxshadow
css-transitions
css-gradients
css3-boxsizing
expressions
border-image
css3-cursors-newer
css3-cursors-grab
pointer
object-fit
css-shapes
text-overflow
css-text-align-last
css-unicode-bidi

Not enough support
css-filter-function
css-element-function	needs -moz-
css-backdrop-filter		needs -webkit-, but only supported by Safari

*/
