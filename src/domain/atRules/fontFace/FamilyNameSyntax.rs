// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FamilyNameSyntax
{
	/// The family name was specified in a quoted form, e.g. "Font Name" or 'Font Name'.
	Quoted,
	
	/// The family name was specified in an unquoted form as a sequence of identifiers.
	/// The `String` is the serialization of the sequence of identifiers.
	Identifiers(String),
}
