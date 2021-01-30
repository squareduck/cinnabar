// use nom;
// use nom::branch::alt;
// use nom::character::complete::one_of;
// use nom::combinator::recognize;
// use nom::sequence::pair;
// use nom_locate::{position, LocatedSpan};

// const ALPHA_DOWNCASE: &str = "abcdefghigklmnopqrstuvwxyz";
// const SYMBOL_GLYPHS: &str = "_-+*/<!>?=:^";
// const DIGIT: &str = "0123456789";

// type Span<'a> = LocatedSpan<&'a str>;

// fn symbol_initial(i: Span) -> nom::IResult<Span, char> {
//     alt((one_of(ALPHA_DOWNCASE), one_of(SYMBOL_GLYPHS)))(i)
// }

// fn symbol_rest(i: Span) -> nom::IResult<Span, char> {
//     alt((one_of(ALPHA_DOWNCASE), one_of(SYMBOL_GLYPHS), one_of(DIGIT)))(i)
// }

// fn value_symbol<'a>(
//     i: Span<'a>,
//     context: &'a mut Context,
// ) -> nom::IResult<Span<'a>, (&'a mut Context, Span<'a>, Value)> {
//     let (_, span) = position(i)?;
//     let (rest, name) = recognize(pair(symbol_initial, symbol_rest))(i)?;

//     let id = &context.symbols.put(&name);
//     Ok((rest, (context, span, Value::Symbol(*id))))
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn symbol() {
//         let mut ctx = Context::default();
//         let (r, (_, _, value_a)) = value_symbol(Span::new("a? nce"), &mut ctx).unwrap();
//         let (_, (_, _, value_b)) = value_symbol(Span::new("b?"), &mut ctx).unwrap();
//         let (_, (_, _, value_c)) = value_symbol(Span::new("a?"), &mut ctx).unwrap();

//         assert_eq!(&&r, " nce");

//         assert_eq!(
//             value_a,
//             Value::Symbol(ctx.symbols.get_by_name("a?").unwrap())
//         );

//         assert_eq!(
//             value_b,
//             Value::Symbol(ctx.symbols.get_by_name("b?").unwrap())
//         );

//         assert_eq!(
//             value_c,
//             Value::Symbol(ctx.symbols.get_by_name("a?").unwrap())
//         );

//         assert!(value_a != value_b);
//         assert!(value_a == value_c);
//     }
// }
