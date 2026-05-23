#![feature(never_type)]

#![no_std]

use core::error::Error;
use core::fmt::{self, Formatter, Display};
use core::marker::PhantomData;

#[derive(Debug)]
pub struct UnexpectedEof;

impl Display for UnexpectedEof {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected eof")
    }
}

impl Error for UnexpectedEof { }

pub trait Parser<'p> {
    type Result;
    type Error;

    fn parse(&mut self, input: &'p [u8]) -> Result<(Self::Result, &'p [u8]), Self::Error>;

    fn map<U, F: FnMut(Self::Result) -> U>(
        self,
        f: F,
    ) -> Map<'p, Self::Result, U, Self, F> where Self: Sized {
        Map { parser: self, map: f, phantom: PhantomData }
    }

    fn map_err<X, F: FnMut(Self::Error) -> X>(
        self,
        f: F,
    ) -> MapErr<'p, Self::Error, X, Self, F> where Self: Sized {
        MapErr { parser: self, map: f, phantom: PhantomData }
    }

    fn map_res<U, F: FnMut(Self::Result) -> Result<U, Self::Error>>(
        self,
        f: F,
    ) -> MapRes<'p, Self::Result, U, Self, F> where Self: Sized {
        MapRes { parser: self, map: f, phantom: PhantomData }
    }

    fn and_then<U, Q: Parser<'p, Result=U, Error=Self::Error>, F: FnMut(Self::Result) -> Q>(
        self,
        f: F,
    ) -> AndThen<'p, Self::Result, U, Self::Error, Self, Q, F> where Self: Sized {
        AndThen { parser: self, map: f, phantom: PhantomData }
    }
}

macro_rules! impl_parser_for_tuple {
    (
        $($T:ident),+$(,)?
    ) => {
        impl<'p, E, $($T: Parser<'p, Error=E>,)+> Parser<'p> for ($($T,)+) {
            type Result = ($($T::Result,)+);
            type Error = E;

            fn parse(&mut self, input: &'p [u8]) -> Result<(Self::Result, &'p [u8]), Self::Error> {
                let mut x = input;
                #[allow(non_snake_case)]
                let ($($T,)+) = self;
                $(
                    #[allow(non_snake_case)]
                    let ($T, r) = $T.parse(x)?;
                    x = r;
                )+
                Ok((($($T,)+), x))
            }
        }
    };
}

impl_parser_for_tuple!(A);
impl_parser_for_tuple!(A, B);
impl_parser_for_tuple!(A, B, C);
impl_parser_for_tuple!(A, B, C, D);
impl_parser_for_tuple!(A, B, C, D, F);
impl_parser_for_tuple!(A, B, C, D, F, G);
impl_parser_for_tuple!(A, B, C, D, F, G, H);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O, P);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_parser_for_tuple!(A, B, C, D, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);

pub struct Map<'p, T, U, P: Parser<'p, Result=T>, F: FnMut(T) -> U> {
    parser: P,
    map: F,
    phantom: PhantomData<&'p ()>,
}

impl<'p, T, U, P: Parser<'p, Result=T>, F: FnMut(T) -> U> Parser<'p> for Map<'p, T, U, P, F> {
    type Result = U;
    type Error = P::Error;

    fn parse(&mut self, input: &'p [u8]) -> Result<(U, &'p [u8]), Self::Error> {
        match self.parser.parse(input) {
            Ok((t, r)) => Ok(((self.map)(t), r)),
            Err(e) => Err(e),
        }
    }
}

pub struct MapRes<'p, T, U, P: Parser<'p, Result=T>, F: FnMut(T) -> Result<U, P::Error>> {
    parser: P,
    map: F,
    phantom: PhantomData<&'p ()>,
}

impl<
    'p,
    T,
    U,
    P: Parser<'p, Result=T>,
    F: FnMut(T) -> Result<U, P::Error>
> Parser<'p> for MapRes<'p, T, U, P, F> {
    type Result = U;
    type Error = P::Error;

    fn parse(&mut self, input: &'p [u8]) -> Result<(U, &'p [u8]), Self::Error> {
        match self.parser.parse(input) {
            Ok((t, r)) => match (self.map)(t) {
                Ok(x) => Ok((x, r)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

pub struct MapErr<'p, E, X, P: Parser<'p, Error=E>, F: FnMut(E) -> X> {
    parser: P,
    map: F,
    phantom: PhantomData<&'p ()>,
}

impl<'p, E, X, P: Parser<'p, Error=E>, F: FnMut(E) -> X> Parser<'p> for MapErr<'p, E, X, P, F> {
    type Result = P::Result;
    type Error = X;

    fn parse(&mut self, input: &'p [u8]) -> Result<(Self::Result, &'p [u8]), X> {
        self.parser.parse(input).map_err(|x| (self.map)(x))
    }
}

pub struct AndThen<
    'p,
    T,
    U,
    E,
    P: Parser<'p, Result=T, Error=E>,
    Q: Parser<'p, Result=U, Error=E>,
    F: FnMut(T) -> Q,
> {
    parser: P,
    map: F,
    phantom: PhantomData<&'p ()>,
}

impl<
    'p,
    T,
    U,
    E,
    P: Parser<'p, Result=T, Error=E>,
    Q: Parser<'p, Result=U, Error=E>,
    F: FnMut(T) -> Q,
> Parser<'p> for AndThen<'p, T, U, E, P, Q, F> {
    type Result = U;
    type Error = E;

    fn parse(&mut self, input: &'p [u8]) -> Result<(U, &'p [u8]), E> {
        match self.parser.parse(input) {
            Ok((t, r)) => match (self.map)(t).parse(r) {
                Ok((x, r)) => Ok((x, r)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

pub struct Consume(());

impl<'p> Parser<'p> for Consume {
    type Result = &'p [u8];
    type Error = !;

    fn parse(&mut self, input: &'p [u8]) -> Result<(&'p [u8], &'p [u8]), !> {
        Ok((input, &input[input.len() ..]))
    }
}

pub fn consume() -> Consume { Consume(()) }

pub struct U8(());

impl<'p> Parser<'p> for U8 {
    type Result = u8;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(u8, &'p [u8]), UnexpectedEof> {
        if input.is_empty() {
            Err(UnexpectedEof)
        } else {
            Ok((input[0], &input[1 ..]))
        }
    }
}

pub fn u8() -> U8 { U8(()) }

pub struct U16le(());

impl<'p> Parser<'p> for U16le {
    type Result = u16;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(u16, &'p [u8]), UnexpectedEof> {
        if input.len() < 2 {
            Err(UnexpectedEof)
        } else {
            Ok((u16::from_le_bytes(*input[.. 2].as_array().unwrap()), &input[2 ..]))
        }
    }
}

pub fn u16le() -> U16le { U16le(()) }

#[cfg(test)]
mod tests {
    use core::num::NonZero;
    use super::*;

    fn non_zero_u16le<'p>() -> impl Parser<'p, Result=Option<NonZero<u16>>, Error=UnexpectedEof> {
        u16le().map(NonZero::new)
    }

    #[test]
    fn it_works() {
        let res = non_zero_u16le().parse(&[2, 0]);
        assert!(res.is_ok());
        assert_eq!(res.ok().unwrap(), (NonZero::new(2u16), &[][..]));
    }

    #[test]
    fn consume_all() {
        let res = consume().parse(&[2, 0]);
        assert!(res.is_ok());
        assert_eq!(res.ok().unwrap(), (&[2, 0][..], &[][..]));
    }
}
