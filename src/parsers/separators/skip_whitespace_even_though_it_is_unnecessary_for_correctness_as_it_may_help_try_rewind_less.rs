// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


fn skip_whitespace_even_though_it_is_unnecessary_for_correctness_as_it_may_help_try_rewind_less<'i, 't>(input: &mut Parser<'i, 't>)
{
	input.skip_whitespace();
}
