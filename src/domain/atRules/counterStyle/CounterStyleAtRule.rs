// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


macro_rules! accessor
{
    (#[$doc: meta] $name: tt $ident: ident: $ty: ty = !) => {
        #[$doc]
        pub fn $ident(&self) -> Option<&$ty>
        {
            self.$ident.as_ref()
        }
    };

    (#[$doc: meta] $name: tt $ident: ident: $ty: ty = $initial: expr) => {
        #[$doc]
        pub fn $ident(&self) -> Cow<$ty>
        {
            if let Some(ref value) = self.$ident
            {
                Cow::Borrowed(value)
            }
            else
            {
                Cow::Owned($initial)
            }
        }
    }
}

macro_rules! counter_style_descriptors
{
    (
        $( #[$doc: meta] $name: tt $ident: ident / $gecko_ident: ident: $ty: ty = $initial: tt )+
    ) => {
        /// An @counter-style rule
        #[derive(Clone, Debug)]
        pub struct CounterStyleAtRule
        {
            name: CustomIdent,
            $(
                #[$doc]
                $ident: Option<$ty>,
            )+
        }

        impl CounterStyleAtRule
        {
            fn empty(name: CustomIdent) -> Self
            {
                Self
                {
                    name: name,
                    $(
                        $ident: None,
                    )+
                }
            }

            /// Get the name of the counter style rule.
            pub fn name(&self) -> &CustomIdent
            {
                &self.name
            }

            $(
                accessor!(#[$doc] $name $ident: $ty = $initial);
            )+
        }

        impl<'a, 'b, 'i> DeclarationParser<'i> for CounterStyleRuleParser<'a, 'b>
        {
            type Declaration = ();
            
            type Error = SelectorParseError<'i, StyleParseError<'i>>;
			
            fn parse_value<'t>(&mut self, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<(), ParseError<'i>> {
                match_ignore_ascii_case!
                {
                	&*name,
                    $(
                        $name =>
                        {
                            // DeclarationParser also calls parse_entirely  so we’d normally not need to, but in this case we do because we set the value as a side effect rather than returning it.
                            let value = input.parse_entirely(|i| Parse::parse(self.context, i))?;
                            self.rule.$ident = Some(value)
                        }
                    )*
                    _ => return Err(SelectorParseError::UnexpectedIdent(name.clone()).into())
                }
                Ok(())
            }
        }

        impl ToCss for CounterStyleAtRule
        {
            fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
            {
                dest.write_str("@counter-style ")?;
                self.name.to_css(dest)?;
                dest.write_str(" {\n")?;
                $(
                    if let Some(ref value) = self.$ident
                    {
                        dest.write_str(concat!("  ", $name, ": "))?;
                        ToCss::to_css(value, dest)?;
                        dest.write_str(";\n")?;
                    }
                )+
                dest.write_str("}")
            }
        }
        
        impl CounterStyleAtRule
        {
			/// Parse the body (inside `{}`) of an @counter-style rule
			pub(crate) fn parse_body<'i, 't, R: ParseErrorReporter>(name: CustomIdent, context: &ParserContext,error_context: &ParserErrorContext<R>, input: &mut Parser<'i, 't>) -> Result<CounterStyleAtRule, ParseError<'i>>
			{
				let start = input.current_source_location();
				
				let mut rule = CounterStyleAtRule::empty(name);
				
				{
					let parser = CounterStyleRuleParser
					{
						context,
						rule: &mut rule,
					};
					
					let mut iter = DeclarationListParser::new(input, parser);
					while let Some(declaration) = iter.next()
					{
						if let Err(preciseParseError) = declaration
						{
							return Err(CustomParseError::UnsupportedCounterStyleDescriptorDeclaration(preciseParseError));
						}
					}
				}
				
				use self::System::*;
				
				let error = match *rule.system()
				{
					ref system @ Cyclic | ref system @ Fixed { .. } | ref system @ Symbolic | ref system @ Alphabetic | ref system @ Numeric if rule.symbols.is_none() =>
					{
						Some(CustomParseError::InvalidCounterStyleWithoutSymbols(system))
					}
					
					ref system @ Alphabetic | ref system @ Numeric if rule.symbols().unwrap().0.len() < 2 =>
					{
						Some(CustomParseError::InvalidCounterStyleNotEnoughSymbols(system))
					}
					
					Additive if rule.additive_symbols.is_none() =>
					{
						Some(CustomParseError::InvalidCounterStyleWithoutAdditiveSymbols)
					}
					
					Extends(_) if rule.symbols.is_some() =>
					{
						Some(CustomParseError::InvalidCounterStyleExtendsWithSymbols)
					}
					
					Extends(_) if rule.additive_symbols.is_some() =>
					{
						Some(CustomParseError::InvalidCounterStyleExtendsWithAdditiveSymbols)
					}
					
					_ => None
				};
				
				if let Some(error) = error
				{
					Err(ParseError::Custom(error))
				}
				else
				{
					Ok(rule)
				}
			}
        }
    }
}

counter_style_descriptors!
{
    /// https://drafts.csswg.org/css-counter-styles/#counter-style-system
    "system" system / eCSSCounterDesc_System: System = {
        System::Symbolic
    }

    /// https://drafts.csswg.org/css-counter-styles/#counter-style-negative
    "negative" negative / eCSSCounterDesc_Negative: Negative = {
        Negative(Symbol::String("-".to_owned()), None)
    }

    /// https://drafts.csswg.org/css-counter-styles/#counter-style-prefix
    "prefix" prefix / eCSSCounterDesc_Prefix: Symbol = {
        Symbol::String("".to_owned())
    }

    /// https://drafts.csswg.org/css-counter-styles/#counter-style-suffix
    "suffix" suffix / eCSSCounterDesc_Suffix: Symbol = {
        Symbol::String(". ".to_owned())
    }

    /// https://drafts.csswg.org/css-counter-styles/#counter-style-range
    "range" range / eCSSCounterDesc_Range: Ranges = {
        Ranges(Vec::new())  // Empty Vec represents 'auto'
    }

    /// https://drafts.csswg.org/css-counter-styles/#counter-style-pad
    "pad" pad / eCSSCounterDesc_Pad: Pad = {
        Pad(0, Symbol::String("".to_owned()))
    }

    /// https://drafts.csswg.org/css-counter-styles/#counter-style-fallback
    "fallback" fallback / eCSSCounterDesc_Fallback: Fallback = {
        Fallback(CustomIdent("decimal".to_string()))
    }

    /// https://drafts.csswg.org/css-counter-styles/#descdef-counter-style-symbols
    "symbols" symbols / eCSSCounterDesc_Symbols: Symbols = !

    /// https://drafts.csswg.org/css-counter-styles/#descdef-counter-style-additive-symbols
    "additive-symbols" additive_symbols / eCSSCounterDesc_AdditiveSymbols: AdditiveSymbols = !

    /// https://drafts.csswg.org/css-counter-styles/#counter-style-speak-as
    "speak-as" speak_as / eCSSCounterDesc_SpeakAs: SpeakAs = {
        SpeakAs::Auto
    }
}
