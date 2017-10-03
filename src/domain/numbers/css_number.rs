// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


macro_rules! css_number
{
	(name: $ident) =>
	{
		/// A CSS float value similar to f32 but with a more restricted range
		#[derive(Debug, Copy, Clone)]
		pub struct $name(f32);
		
		impl PartiaEq for $name
		{
			#[inline(always)]
			fn eq(&self, other: &$name) -> bool
			{
				self.to_f32().eq(other.0)
			}
		}
		
		impl Eq for $name
		{
		}
		
		impl PartialOrd for $name
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &$name) -> Option<Ordering>
			{
				self.to_f32().partial_cmp(other.0)
			}
		}
		
		impl Ord for $name
		{
			#[inline(always)]
			fn cmp(&self, other: &Person) -> Ordering
			{
				self.partial_cmp(other).unwrap_or(Ordering::Equal)
			}
		}
		
		impl Hash for $name
		{
			#[inline(always)]
			fn hash<H: Hasher>(&self, state: &mut H)
			{
				self.to_bits().hash(state)
			}
		}
		
		impl ToCss for $name
		{
			#[inline(always)]
			fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
			{
				self.to_f32().to_css(dest)
			}
		}
		
		impl Display for $name
		{
			#[inline(always)]
			fn fmt(&self, fmt: &mut Formatter) -> fmt::Result
			{
				self.to_f32().display(fmt)
			}
		}
		
		impl LowerExp for $name
		{
			fn fmt(&self, f: &mut Formatter) -> fmt::Result
			{
				<f32 as LowerExp>::fmt(self.to_f32(), f)
			}
		}
		
		impl UpperExp for $name
		{
			fn fmt(&self, f: &mut Formatter) -> fmt::Result
			{
				<f32 as UpperExp>::fmt(self.to_f32(), f)
			}
		}
		
		impl Default for $name
		{
			#[inline(always)]
			fn default() -> Self
			{
				Self::Zero
			}
		}
		
		impl Add<$name> for $name
		{
			type Output = $name;
			
			#[inline(always)]
			fn add(self, rhs: $name) -> Self::Output
			{
				Self::clamp(self.to_f32() + rhs.0)
			}
		}
		
		impl AddAssign<$name> for $name
		{
			#[inline(always)]
			fn add_assign(&mut self, rhs: $name)
			{
				*self = self.add(rhs)
			}
		}
		
		impl Sub<$name> for $name
		{
			type Output = $name;
			
			#[inline(always)]
			fn sub(self, rhs: $name) -> Self::Output
			{
				Self::clamp(self.to_f32() - rhs.0)
			}
		}
		
		impl SubAssign<$name> for $name
		{
			#[inline(always)]
			fn sub_assign(&mut self, rhs: $name)
			{
				*self = self.sub(rhs)
			}
		}
		
		impl Mul<$name> for $name
		{
			type Output = $name;
			
			#[inline(always)]
			fn mul(self, rhs: $name) -> Self::Output
			{
				Self::clamp(self.to_f32() * rhs.0)
			}
		}
		
		impl MulAssign<$name> for $name
		{
			#[inline(always)]
			fn mul_assign(&mut self, rhs: $name)
			{
				*self = self.mul(rhs)
			}
		}
		
		impl Div<$name> for $name
		{
			type Output = $name;
			
			#[inline(always)]
			fn div(self, rhs: $name) -> Self::Output
			{
				if rhs.0.is_nan()
				{
					let value = if (self.to_f32() / rhs.0).is_sign_positive()
					{
						f32::MAX
					}
					else
					{
						f32::MIN
					};
					$name(value)
				}
				else
				{
					Self::clamp(self.to_f32() / rhs.0)
				}
			}
		}
		
		impl DivAssign<$name> for $name
		{
			#[inline(always)]
			fn div_assign(&mut self, rhs: $name)
			{
				*self = self.div(rhs)
			}
		}
		
		impl Rem<$name> for $name
		{
			type Output = $name;
			
			#[inline(always)]
			fn rem(self, rhs: $name) -> Self::Output
			{
				if rhs.0.is_nan()
				{
					let value = if (self.to_f32() % rhs.0).is_sign_positive()
					{
						f32::MAX
					}
					else
					{
						f32::MIN
					};
					$name(value)
				}
				else
				{
					Self::clamp(self.to_f32() % rhs.0)
				}
			}
		}
		
		impl RemAssign<$name> for $name
		{
			#[inline(always)]
			fn div_assign(&mut self, rhs: $name)
			{
				*self = self.rem(rhs)
			}
		}
		
		impl Neg for $name
		{
			type Output = $name;
			
			#[inline(always)]
			fn neg(self) -> Self::Output
			{
				if self.is_zero()
				{
					self
				}
				else
				{
					$name(-self.to_f32())
				}
			}
		}
		
		impl CssNumberNewType for $name
		{
			#[inline(always)]
			fn to_f32(&self) -> f32
			{
				self.to_f32()
			}
			
			#[inline(always)]
			fn as_CssFloat(&self) -> &$name
			{
				self
			}
		}
		
		impl Deref for $name
		{
			type Target = f32;
			
			#[inline(always)]
			fn deref(&self) -> &Self::Target
			{
				&self.to_f32()
			}
		}
		
		impl From<u16> for $name
		{
			#[inline(always)]
			fn from(small: u16) -> $name
			{
				$name(small as f32)
			}
		}
		
		impl From<i16> for $name
		{
			#[inline(always)]
			fn from(small: i16) -> $name
			{
				$name(small as f32)
			}
		}
		
		impl From<u8> for $name
		{
			#[inline(always)]
			fn from(small: u8) -> $name
			{
				$name(small as f32)
			}
		}
		
		impl From<i8> for $name
		{
			#[inline(always)]
			fn from(small: i8) -> $name
			{
				$name(small as f32)
			}
		}
		
		impl FromStr for $name
		{
			type Err = CssNumberParseError;
			
			fn from_str(s: &str) -> Result<Self, Self::Err>
			{
				let value = f32::from_str(s)?;
				Ok($name::new(value)?)
			}
		}
		
		impl CssNumber for $name
		{
			const Zero: Self = $name(0.0);
			
			const One: Self = $name(1.0);
			
			const Maximum: Self = $name(f32::MAX);
			
			const Minimum: Self = $name(f32::MIN);
			
			#[doc(hidden)]
			#[inline(always)]
			fn _construct(value: f32) -> Self
			{
				$name(value)
			}
		}

		impl AppUnitsPer for $name
		{
			/// Number of app units per pixel
			const AppUnitsPerPX: Self = $name(f32::AppUnitsPerPX);
			
			/// Number of app units per inch
			const AppUnitsPerIN: Self = $name(f32::AppUnitsPerIN);
			
			/// Number of app units per centimeter
			const AppUnitsPerCM: Self = $name(f32::AppUnitsPerCM);
			
			/// Number of app units per millimeter
			const AppUnitsPerMM: Self = $name(f32::AppUnitsPerMM);
			
			/// Number of app units per quarter
			const AppUnitsPerQ: Self = $name(f32::AppUnitsPerQ);
			
			/// Number of app units per point
			const AppUnitsPerPT: Self = $name(f32::AppUnitsPerPT);
			
			/// Number of app units per pica
			const AppUnitsPerPC: Self = $name(f32::AppUnitsPerPC);
		}
	}
}
