// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A struct to hold a simplified `<length>` or `<percentage>` expression.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CalcLengthOrPercentage
{
	pub clamping_mode: AllowedNumericType,
	pub absolute: Option<AbsoluteLength>,
	pub vw: Option<CSSFloat>,
	pub vh: Option<CSSFloat>,
	pub vmin: Option<CSSFloat>,
	pub vmax: Option<CSSFloat>,
	pub em: Option<CSSFloat>,
	pub ex: Option<CSSFloat>,
	pub ch: Option<CSSFloat>,
	pub rem: Option<CSSFloat>,
	pub percentage: Option<Percentage>,
}

impl ToCss for CalcLengthOrPercentage
{
	/// https://drafts.csswg.org/css-values/#calc-serialize
	#[allow(unused_assignments)]
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use ::num_traits::Zero;
		
		let mut first_value = true;
		macro_rules! first_value_check
		{
            ($val:expr) => {
                if !first_value
                {
                    dest.write_str(if $val < Zero::zero()
                    {
                        " - "
                    }
                    else
                    {
                        " + "
                    })?;
                }
                else if $val < Zero::zero()
                {
                    dest.write_str("-")?;
                }
                first_value = false;
            };
        }
		
		macro_rules! serialize
		{
            ( $( $val:ident ),* ) => {
                $(
                    if let Some(val) = self.$val
                    {
                        first_value_check!(val);
                        val.abs().to_css(dest)?;
                        dest.write_str(stringify!($val))?;
                    }
                )*
            };
        }
		
		macro_rules! serialize_abs
		{
            ( $( $val:ident ),+ ) => {
                $(
                    if let Some(AbsoluteLength::$val(v)) = self.absolute
                    {
                        first_value_check!(v);
                        AbsoluteLength::$val(v.abs()).to_css(dest)?;
                    }
                )+
            };
        }
		
		dest.write_str("calc(")?;
		
		// NOTE(emilio): Percentages first because of web-compat problems, see: https://github.com/w3c/csswg-drafts/issues/1731
		if let Some(val) = self.percentage
		{
			first_value_check!(val.0);
			val.abs().to_css(dest)?;
		}
		
		// NOTE(emilio): The order here is very intentional, and alphabetic per the spec linked above.
		serialize!(ch);
		serialize_abs!(Cm);
		serialize!(em, ex);
		serialize_abs!(In);
		serialize_abs!(Mm, Pc, Pt, Px, Q);
		serialize!(rem, vh, vmax, vmin, vw);
		
		dest.write_str(")")
	}
}
