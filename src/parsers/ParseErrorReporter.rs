// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A generic trait for an error reporter.
pub trait ParseErrorReporter
{
	/// Called when the style engine detects an error.
	///
	/// Returns the current input being parsed, the source location it was reported from, and a message.
	/// was ContextualParseError
	fn report_error(&self, location: SourceLocation, error: ());
}
