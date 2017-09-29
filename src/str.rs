// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


// From https://github.com/servo/servo/blob/faf5b1f797c243c5036973a191d3abae36de4138/components/style/str.rs

/// Returns true if a given string has a given prefix with case-insensitive match.
pub(crate) fn starts_with_ignore_ascii_case(string: &str, prefix: &str) -> bool
{
	string.len() >= prefix.len() && string.as_bytes()[0..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
}
