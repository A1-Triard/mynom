#![feature(never_type)]

#![no_std]

use core::error::Error as core_Error;
use core::fmt::{self, Formatter, Display};
use core::marker::PhantomData;
use either::{Either, Left, Right};

#[derive(Debug)]
pub struct UnexpectedEof;

impl Display for UnexpectedEof {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected eof")
    }
}

impl core_Error for UnexpectedEof { }

#[derive(Debug)]
pub struct ExpectedEof;

impl Display for ExpectedEof {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "expected eof")
    }
}

impl core_Error for ExpectedEof { }

#[derive(Debug)]
pub struct TagMismatch;

impl Display for TagMismatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "tag mismatch")
    }
}

impl core_Error for TagMismatch { }

#[derive(Debug)]
pub enum TagError {
    UnexpectedEof(UnexpectedEof),
    TagMismatch(TagMismatch),
}

impl Display for TagError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TagError::UnexpectedEof(e) => write!(f, "{}", e),
            TagError::TagMismatch(e) => write!(f, "{}", e),
        }
    }
}

impl core_Error for TagError { }

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

    fn map_parser<'q, U, F: FnMut(Self::Result) -> &'q [u8], Q: Parser<'q, Result=U, Error=Self::Error>>(
        self,
        f: F,
        then: Q,
    ) -> MapParser<'p, 'q, Self::Result, U, Self::Error, Self, F, Q> where Self: Sized {
        MapParser { parser: self, map: f, then, phantom: PhantomData }
    }

    fn and<U, Q: Parser<'p, Result=U, Error=Self::Error>>(
        self,
        q: Q,
    ) -> And<'p, Self::Result, U, Self::Error, Self, Q> where Self: Sized {
        And { parser: self, and: q, phantom: PhantomData }
    }

    fn and_then<U, Q: Parser<'p, Result=U, Error=Self::Error>, F: FnMut(Self::Result) -> Q>(
        self,
        f: F,
    ) -> AndThen<'p, Self::Result, U, Self::Error, Self, Q, F> where Self: Sized {
        AndThen { parser: self, map: f, phantom: PhantomData }
    }

    fn repeat<A, I: FnMut() -> A, F: FnMut(A, Self::Result) -> A>(
        self,
        count: usize,
        init: I,
        f: F,
    ) -> Repeat<'p, Self::Result, A, Self, I, F> where Self: Sized {
        Repeat { parser: self, count, init, f, phantom: PhantomData }
    }

    fn repeat_until_eof<A, I: FnMut() -> A, F: FnMut(A, Self::Result) -> A>(
        self,
        init: I,
        f: F,
    ) -> RepeatUntilEof<'p, Self::Result, A, Self, I, F> where Self: Sized {
        RepeatUntilEof { parser: self, init, f, phantom: PhantomData }
    }

    fn peek(self) -> Peek<'p, Self> where Self: Sized {
        Peek { parser: self, phantom: PhantomData }
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

impl<'p, E, P: Parser<'p, Error=E>, Q: Parser<'p, Error=E>> Parser<'p> for Either<Q, P> {
    type Result = Either<Q::Result, P::Result>;
    type Error = E;

    fn parse(&mut self, input: &'p [u8]) -> Result<(Self::Result, &'p [u8]), E> {
        match self {
            Left(q) => match q.parse(input) {
                Ok((q, r)) => Ok((Left(q), r)),
                Err(e) => Err(e),
            },
            Right(p) => match p.parse(input) {
                Ok((p, r)) => Ok((Right(p), r)),
                Err(e) => Err(e),
            },
        }
    }
}

pub struct Peek<
    'p,
    P: Parser<'p>,
> {
    parser: P,
    phantom: PhantomData<&'p ()>,
}

impl<
    'p,
    P: Parser<'p>,
> Parser<'p> for Peek<'p, P> {
    type Result = P::Result;
    type Error = P::Error;

    fn parse(&mut self, input: &'p [u8]) -> Result<(Self::Result, &'p [u8]), Self::Error> {
        match self.parser.parse(input) {
            Ok((t, _)) => Ok((t, input)),
            Err(e) => Err(e),
        }
    }
}

pub struct Repeat<
    'p,
    T,
    A,
    P: Parser<'p, Result=T>,
    I: FnMut() -> A,
    F: FnMut(A, T) -> A,
> {
    count: usize,
    parser: P,
    init: I,
    f: F,
    phantom: PhantomData<&'p ()>,
}

impl<
    'p,
    T,
    A,
    P: Parser<'p, Result=T>,
    I: FnMut() -> A,
    F: FnMut(A, T) -> A,
> Parser<'p> for Repeat<'p, T, A, P, I, F> {
    type Result = A;
    type Error = P::Error;

    fn parse(&mut self, mut input: &'p [u8]) -> Result<(A, &'p [u8]), Self::Error> {
        let mut acc = (self.init)();
        for _ in 0 .. self.count {
            match self.parser.parse(input) {
                Ok((t, r)) => {
                    acc = (self.f)(acc, t);
                    input = r;
                },
                Err(e) => return Err(e),
            }
        }
        Ok((acc, input))
    }
}

pub struct RepeatUntilEof<
    'p,
    T,
    A,
    P: Parser<'p, Result=T>,
    I: FnMut() -> A,
    F: FnMut(A, T) -> A,
> {
    parser: P,
    init: I,
    f: F,
    phantom: PhantomData<&'p ()>,
}

impl<
    'p,
    T,
    A,
    P: Parser<'p, Result=T>,
    I: FnMut() -> A,
    F: FnMut(A, T) -> A,
> Parser<'p> for RepeatUntilEof<'p, T, A, P, I, F> {
    type Result = A;
    type Error = P::Error;

    fn parse(&mut self, mut input: &'p [u8]) -> Result<(A, &'p [u8]), Self::Error> {
        let mut acc = (self.init)();
        while !input.is_empty() {
            match self.parser.parse(input) {
                Ok((t, r)) => {
                    acc = (self.f)(acc, t);
                    input = r;
                },
                Err(e) => return Err(e),
            }
        }
        Ok((acc, input))
    }
}

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

pub struct MapParser<
    'p,
    'q,
    T,
    U,
    E,
    P: Parser<'p, Result=T, Error=E>,
    F: FnMut(T) -> &'q [u8],
    Q: Parser<'q, Result=U, Error=E>,
> {
    parser: P,
    map: F,
    then: Q,
    phantom: PhantomData<(&'p (), &'q ())>,
}

impl<
    'p,
    'q,
    T,
    U,
    E,
    P: Parser<'p, Result=T, Error=E>,
    F: FnMut(T) -> &'q [u8],
    Q: Parser<'q, Result=U, Error=E>,
> Parser<'p> for MapParser<'p, 'q, T, U, E, P, F, Q> {
    type Result = U;
    type Error = E;

    fn parse(&mut self, input: &'p [u8]) -> Result<(U, &'p [u8]), E> {
        match self.parser.parse(input) {
            Ok((t, r)) => match self.then.parse((self.map)(t)) {
                Ok((x, _)) => Ok((x, r)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

pub struct And<
    'p,
    T,
    U,
    E,
    P: Parser<'p, Result=T, Error=E>,
    Q: Parser<'p, Result=U, Error=E>,
> {
    parser: P,
    and: Q,
    phantom: PhantomData<&'p ()>,
}

impl<
    'p,
    T,
    U,
    E,
    P: Parser<'p, Result=T, Error=E>,
    Q: Parser<'p, Result=U, Error=E>,
> Parser<'p> for And<'p, T, U, E, P, Q> {
    type Result = (T, U);
    type Error = E;

    fn parse(&mut self, input: &'p [u8]) -> Result<((T, U), &'p [u8]), E> {
        match self.parser.parse(input) {
            Ok((t, r)) => match self.and.parse(r) {
                Ok((x, r)) => Ok(((t, x), r)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
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

pub struct Take(usize);

impl<'p> Parser<'p> for Take {
    type Result = &'p [u8];
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(&'p [u8], &'p [u8]), UnexpectedEof> {
        if input.len() < self.0 {
            Err(UnexpectedEof)
        } else {
            Ok((&input[.. self.0], &input[self.0 ..]))
        }
    }
}

pub fn take(n: usize) -> Take { Take(n) }

pub struct Tag<'a>(&'a [u8]);

impl<'p, 'a> Parser<'p> for Tag<'a> {
    type Result = ();
    type Error = TagError;

    fn parse(&mut self, input: &'p [u8]) -> Result<((), &'p [u8]), TagError> {
        if input.len() < self.0.len() {
            Err(TagError::UnexpectedEof(UnexpectedEof))
        } else if &input[.. self.0.len()] != self.0 {
            Err(TagError::TagMismatch(TagMismatch))
        } else {
            Ok(((), &input[self.0.len() ..]))
        }
    }
}

pub fn tag(x: &[u8]) -> Tag<'_> { Tag(x) }

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

pub struct U16Le(());

impl<'p> Parser<'p> for U16Le {
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

pub fn u16_le() -> U16Le { U16Le(()) }

pub struct U32Le(());

impl<'p> Parser<'p> for U32Le {
    type Result = u32;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(u32, &'p [u8]), UnexpectedEof> {
        if input.len() < 4 {
            Err(UnexpectedEof)
        } else {
            Ok((u32::from_le_bytes(*input[.. 4].as_array().unwrap()), &input[4 ..]))
        }
    }
}

pub fn u32_le() -> U32Le { U32Le(()) }

pub struct U64Le(());

impl<'p> Parser<'p> for U64Le {
    type Result = u64;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(u64, &'p [u8]), UnexpectedEof> {
        if input.len() < 8 {
            Err(UnexpectedEof)
        } else {
            Ok((u64::from_le_bytes(*input[.. 8].as_array().unwrap()), &input[8 ..]))
        }
    }
}

pub fn u64_le() -> U64Le { U64Le(()) }

pub struct I8(());

impl<'p> Parser<'p> for I8 {
    type Result = i8;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(i8, &'p [u8]), UnexpectedEof> {
        if input.is_empty() {
            Err(UnexpectedEof)
        } else {
            Ok((input[0].cast_signed(), &input[1 ..]))
        }
    }
}

pub fn i8() -> I8 { I8(()) }

pub struct I16Le(());

impl<'p> Parser<'p> for I16Le {
    type Result = i16;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(i16, &'p [u8]), UnexpectedEof> {
        if input.len() < 2 {
            Err(UnexpectedEof)
        } else {
            Ok((i16::from_le_bytes(*input[.. 2].as_array().unwrap()), &input[2 ..]))
        }
    }
}

pub fn i16_le() -> I16Le { I16Le(()) }

pub struct I32Le(());

impl<'p> Parser<'p> for I32Le {
    type Result = i32;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(i32, &'p [u8]), UnexpectedEof> {
        if input.len() < 4 {
            Err(UnexpectedEof)
        } else {
            Ok((i32::from_le_bytes(*input[.. 4].as_array().unwrap()), &input[4 ..]))
        }
    }
}

pub fn i32_le() -> I32Le { I32Le(()) }

pub struct I64Le(());

impl<'p> Parser<'p> for I64Le {
    type Result = i64;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(i64, &'p [u8]), UnexpectedEof> {
        if input.len() < 8 {
            Err(UnexpectedEof)
        } else {
            Ok((i64::from_le_bytes(*input[.. 8].as_array().unwrap()), &input[8 ..]))
        }
    }
}

pub fn i64_le() -> I64Le { I64Le(()) }

pub struct F32Le(());

impl<'p> Parser<'p> for F32Le {
    type Result = f32;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(f32, &'p [u8]), UnexpectedEof> {
        if input.len() < 4 {
            Err(UnexpectedEof)
        } else {
            Ok((f32::from_le_bytes(*input[.. 4].as_array().unwrap()), &input[4 ..]))
        }
    }
}

pub fn f32_le() -> F32Le { F32Le(()) }

pub struct F64Le(());

impl<'p> Parser<'p> for F64Le {
    type Result = f64;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(f64, &'p [u8]), UnexpectedEof> {
        if input.len() < 8 {
            Err(UnexpectedEof)
        } else {
            Ok((f64::from_le_bytes(*input[.. 8].as_array().unwrap()), &input[8 ..]))
        }
    }
}

pub fn f64_le() -> F64Le { F64Le(()) }

pub struct Eof(());

impl<'p> Parser<'p> for Eof {
    type Result = ();
    type Error = ExpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<((), &'p [u8]), ExpectedEof> {
        if !input.is_empty() {
            Err(ExpectedEof)
        } else {
            Ok(((), input))
        }
    }
}

pub fn eof() -> Eof { Eof(()) }

pub struct U16Be(());

impl<'p> Parser<'p> for U16Be {
    type Result = u16;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(u16, &'p [u8]), UnexpectedEof> {
        if input.len() < 2 {
            Err(UnexpectedEof)
        } else {
            Ok((u16::from_be_bytes(*input[.. 2].as_array().unwrap()), &input[2 ..]))
        }
    }
}

pub fn u16_be() -> U16Be { U16Be(()) }

pub struct U32Be(());

impl<'p> Parser<'p> for U32Be {
    type Result = u32;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(u32, &'p [u8]), UnexpectedEof> {
        if input.len() < 4 {
            Err(UnexpectedEof)
        } else {
            Ok((u32::from_be_bytes(*input[.. 4].as_array().unwrap()), &input[4 ..]))
        }
    }
}

pub fn u32_be() -> U32Be { U32Be(()) }

pub struct U64Be(());

impl<'p> Parser<'p> for U64Be {
    type Result = u64;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(u64, &'p [u8]), UnexpectedEof> {
        if input.len() < 8 {
            Err(UnexpectedEof)
        } else {
            Ok((u64::from_be_bytes(*input[.. 8].as_array().unwrap()), &input[8 ..]))
        }
    }
}

pub fn u64_be() -> U64Be { U64Be(()) }

pub struct I16Be(());

impl<'p> Parser<'p> for I16Be {
    type Result = i16;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(i16, &'p [u8]), UnexpectedEof> {
        if input.len() < 2 {
            Err(UnexpectedEof)
        } else {
            Ok((i16::from_be_bytes(*input[.. 2].as_array().unwrap()), &input[2 ..]))
        }
    }
}

pub fn i16_be() -> I16Be { I16Be(()) }

pub struct I32Be(());

impl<'p> Parser<'p> for I32Be {
    type Result = i32;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(i32, &'p [u8]), UnexpectedEof> {
        if input.len() < 4 {
            Err(UnexpectedEof)
        } else {
            Ok((i32::from_be_bytes(*input[.. 4].as_array().unwrap()), &input[4 ..]))
        }
    }
}

pub fn i32_be() -> I32Be { I32Be(()) }

pub struct I64Be(());

impl<'p> Parser<'p> for I64Be {
    type Result = i64;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(i64, &'p [u8]), UnexpectedEof> {
        if input.len() < 8 {
            Err(UnexpectedEof)
        } else {
            Ok((i64::from_be_bytes(*input[.. 8].as_array().unwrap()), &input[8 ..]))
        }
    }
}

pub fn i64_be() -> I64Be { I64Be(()) }

pub struct F32Be(());

impl<'p> Parser<'p> for F32Be {
    type Result = f32;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(f32, &'p [u8]), UnexpectedEof> {
        if input.len() < 4 {
            Err(UnexpectedEof)
        } else {
            Ok((f32::from_be_bytes(*input[.. 4].as_array().unwrap()), &input[4 ..]))
        }
    }
}

pub fn f32_be() -> F32Be { F32Be(()) }

pub struct F64Be(());

impl<'p> Parser<'p> for F64Be {
    type Result = f64;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(f64, &'p [u8]), UnexpectedEof> {
        if input.len() < 8 {
            Err(UnexpectedEof)
        } else {
            Ok((f64::from_be_bytes(*input[.. 8].as_array().unwrap()), &input[8 ..]))
        }
    }
}

pub fn f64_be() -> F64Be { F64Be(()) }

pub struct U16Ne(());

impl<'p> Parser<'p> for U16Ne {
    type Result = u16;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(u16, &'p [u8]), UnexpectedEof> {
        if input.len() < 2 {
            Err(UnexpectedEof)
        } else {
            Ok((u16::from_ne_bytes(*input[.. 2].as_array().unwrap()), &input[2 ..]))
        }
    }
}

pub fn u16_ne() -> U16Ne { U16Ne(()) }

pub struct U32Ne(());

impl<'p> Parser<'p> for U32Ne {
    type Result = u32;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(u32, &'p [u8]), UnexpectedEof> {
        if input.len() < 4 {
            Err(UnexpectedEof)
        } else {
            Ok((u32::from_ne_bytes(*input[.. 4].as_array().unwrap()), &input[4 ..]))
        }
    }
}

pub fn u32_ne() -> U32Ne { U32Ne(()) }

pub struct U64Ne(());

impl<'p> Parser<'p> for U64Ne {
    type Result = u64;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(u64, &'p [u8]), UnexpectedEof> {
        if input.len() < 8 {
            Err(UnexpectedEof)
        } else {
            Ok((u64::from_ne_bytes(*input[.. 8].as_array().unwrap()), &input[8 ..]))
        }
    }
}

pub fn u64_ne() -> U64Ne { U64Ne(()) }

pub struct I16Ne(());

impl<'p> Parser<'p> for I16Ne {
    type Result = i16;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(i16, &'p [u8]), UnexpectedEof> {
        if input.len() < 2 {
            Err(UnexpectedEof)
        } else {
            Ok((i16::from_ne_bytes(*input[.. 2].as_array().unwrap()), &input[2 ..]))
        }
    }
}

pub fn i16_ne() -> I16Ne { I16Ne(()) }

pub struct I32Ne(());

impl<'p> Parser<'p> for I32Ne {
    type Result = i32;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(i32, &'p [u8]), UnexpectedEof> {
        if input.len() < 4 {
            Err(UnexpectedEof)
        } else {
            Ok((i32::from_ne_bytes(*input[.. 4].as_array().unwrap()), &input[4 ..]))
        }
    }
}

pub fn i32_ne() -> I32Ne { I32Ne(()) }

pub struct I64Ne(());

impl<'p> Parser<'p> for I64Ne {
    type Result = i64;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(i64, &'p [u8]), UnexpectedEof> {
        if input.len() < 8 {
            Err(UnexpectedEof)
        } else {
            Ok((i64::from_ne_bytes(*input[.. 8].as_array().unwrap()), &input[8 ..]))
        }
    }
}

pub fn i64_ne() -> I64Ne { I64Ne(()) }

pub struct F32Ne(());

impl<'p> Parser<'p> for F32Ne {
    type Result = f32;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(f32, &'p [u8]), UnexpectedEof> {
        if input.len() < 4 {
            Err(UnexpectedEof)
        } else {
            Ok((f32::from_ne_bytes(*input[.. 4].as_array().unwrap()), &input[4 ..]))
        }
    }
}

pub fn f32_ne() -> F32Ne { F32Ne(()) }

pub struct F64Ne(());

impl<'p> Parser<'p> for F64Ne {
    type Result = f64;
    type Error = UnexpectedEof;

    fn parse(&mut self, input: &'p [u8]) -> Result<(f64, &'p [u8]), UnexpectedEof> {
        if input.len() < 8 {
            Err(UnexpectedEof)
        } else {
            Ok((f64::from_ne_bytes(*input[.. 8].as_array().unwrap()), &input[8 ..]))
        }
    }
}

pub fn f64_ne() -> F64Ne { F64Ne(()) }

pub struct Error<E, F: FnMut() -> E> {
    err: F,
}

impl<'p, E, F: FnMut() -> E> Parser<'p> for Error<E, F> {
    type Result = !;
    type Error = E;

    fn parse(&mut self, _input: &'p [u8]) -> Result<(!, &'p [u8]), E> {
        Err((self.err)())
    }
}

pub fn error<E, F: FnMut() -> E>(f: F) -> Error<E, F> { Error { err: f } }

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

    #[test]
    fn map_parser() {
        let res = take(2).map_err(|_| ())
            .map_parser(|x| x, (u16le().map_err(|_| ()), eof().map_err(|_| ()))).parse(&[2, 0, 3, 4]);
        assert!(res.is_ok());
        assert_eq!(res.ok().unwrap(), ((2u16, ()), &[3u8, 4][..]));
    }
}
