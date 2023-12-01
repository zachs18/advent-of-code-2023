use std::{
    any::Any,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::Hash,
    panic::{AssertUnwindSafe, RefUnwindSafe, UnwindSafe},
    str::FromStr,
};

use either::Either;
pub use itertools::Itertools;

pub trait IntoIteratorExt: IntoIterator + Sized {
    #[inline]
    fn collect_hashset(self) -> HashSet<<Self as IntoIterator>::Item>
    where
        <Self as IntoIterator>::Item: Hash + Eq,
    {
        self.into_iter().collect()
    }
    #[inline]
    fn collect_btreeset(self) -> BTreeSet<<Self as IntoIterator>::Item>
    where
        <Self as IntoIterator>::Item: Ord,
    {
        self.into_iter().collect()
    }
}
impl<I: IntoIterator> IntoIteratorExt for I {}

pub trait IntoIteratorPairExt<K, V>: IntoIterator<Item = (K, V)> + Sized {
    #[inline]
    fn collect_hashmap(self) -> HashMap<K, V>
    where
        K: Hash + Eq,
    {
        self.into_iter().collect()
    }
    #[inline]
    fn collect_btreemap(self) -> BTreeMap<K, V>
    where
        K: Ord,
    {
        self.into_iter().collect()
    }
}
impl<K, V, I: IntoIterator<Item = (K, V)>> IntoIteratorPairExt<K, V> for I {}

pub trait IntoIteratorResultExt<T, E>: IntoIterator<Item = Result<T, E>> + Sized {
    #[inline]
    fn collect_result<C: FromIterator<T>>(self) -> Result<C, E> {
        self.into_iter().collect()
    }
    #[inline]
    fn collect_result_vec(self) -> Result<Vec<T>, E> {
        self.into_iter().collect()
    }
    #[inline]
    fn collect_result_hashset(self) -> Result<HashSet<T>, E>
    where
        T: Hash + Eq,
    {
        self.into_iter().collect()
    }
    #[inline]
    fn collect_result_btreeset(self) -> Result<BTreeSet<T>, E>
    where
        T: Ord,
    {
        self.into_iter().collect()
    }
}
impl<T, E, I: IntoIterator<Item = Result<T, E>>> IntoIteratorResultExt<T, E> for I {}

pub trait IntoIteratorResultPairExt<K, V, E>:
    IntoIterator<Item = Result<(K, V), E>> + Sized
{
    #[inline]
    fn collect_result_hashmap(self) -> Result<HashMap<K, V>, E>
    where
        K: Hash + Eq,
    {
        self.into_iter().collect()
    }
    #[inline]
    fn collect_result_btreemap(self) -> Result<BTreeMap<K, V>, E>
    where
        K: Ord,
    {
        self.into_iter().collect()
    }
}

pub fn split_line_parse<T: FromStr>(line: &str) -> Result<Vec<T>, T::Err> {
    line.split_whitespace().map(T::from_str).collect()
}

/// Parse AoC input format into a vector of each indvidual input line (trimmed).
pub fn lines(input: &str) -> Vec<&str> {
    input.lines().map(str::trim).collect()
}

/// Parse AoC input format into a vector of the output of a given function called
/// on each indvidual input line (trimmed).
pub fn parse_lines<'a, T: 'a, F: FnMut(&'a str) -> T + 'a>(input: &'a str, f: F) -> Vec<T> {
    input.lines().map(str::trim).map(f).collect()
}

/// Parse AoC input format into a vector of the output of a given function called
/// on each indvidual input line (trimmed). If any call to the function fails, the
/// the whole parsing fails.
pub fn try_parse_lines<T, E>(
    input: &str,
    mut f: impl FnMut(&str) -> Result<T, E>,
) -> Result<Vec<T>, E> {
    input.lines().map(move |line| f(line.trim())).collect()
}

/// Parse AoC input format into the result of folding a given function over
/// each indvidual input line (trimmed).
pub fn fold_lines<'a, B, F: FnMut(B, &'a str) -> B>(input: &'a str, acc: B, f: F) -> B {
    input.lines().map(str::trim).fold(acc, f)
}

/// Parse AoC input format into the result of folding a given function over
/// each indvidual input line (trimmed).
pub fn try_fold_lines<'a, B, E, F: FnMut(B, &'a str) -> Result<B, E>>(
    input: &'a str,
    acc: B,
    f: F,
) -> Result<B, E> {
    input.lines().map(str::trim).try_fold(acc, f)
}

/// Parse AoC input format with groups of inputs separated by blank lines into
/// a vector of "groups", where each group is a vector of each indvidual input
/// line (trimmed) from the group.
pub fn groups(input: &str) -> Vec<Vec<&str>> {
    let lines: Vec<&str> = lines(input);
    lines
        .split(|line| line.is_empty())
        .map(|s| s.to_vec())
        .collect()
}

/// Parse AoC input format with groups of inputs as chunks of a number of lines into
/// a vector of "groups", where each group is a vector of each indvidual input
/// line (trimmed) from the group.
pub fn chunks(input: &str, chunk_size: usize) -> Vec<Vec<&str>> {
    assert!(chunk_size > 0);
    let lines: Vec<&str> = lines(input);
    lines.chunks(chunk_size).map(|s| s.to_vec()).collect()
}

/// Parse AoC input format with groups of inputs separated by blank lines into
/// a vector of "groups", where each group is a vector of the output of a given
/// function called on each indvidual input line (trimmed) from the group.
pub fn parse_groups<'a, T: 'a, F: FnMut(&'a str) -> T>(input: &'a str, mut f: F) -> Vec<Vec<T>> {
    let lines: Vec<&str> = lines(input);
    lines
        .split(|line| line.is_empty())
        .map(|s| s.iter().copied().map(&mut f).collect_vec())
        .collect()
}

/// Parse AoC input format with groups of inputs as chunks of a number of lines into
/// a vector of "groups", where each group is a vector of the output of a given
/// function called on each indvidual input line (trimmed) from the group.
pub fn parse_chunks<'a, T: 'a, F: FnMut(&'a str) -> T>(
    input: &'a str,
    chunk_size: usize,
    mut f: F,
) -> Vec<Vec<T>> {
    assert!(chunk_size > 0);
    let lines: Vec<&str> = lines(input);
    lines
        .chunks(chunk_size)
        .map(|s| s.iter().copied().map(&mut f).collect())
        .collect()
}

/// Parse AoC input format with groups of inputs separated by blank lines into
/// the result of folding a given function over each group of (trimmed) lines.
pub fn fold_groups<'a, B, F: FnMut(B, &[&'a str]) -> B>(input: &'a str, mut acc: B, mut f: F) -> B {
    let lines: Vec<&str> = lines(input);
    for group in lines.split(|line| line.is_empty()) {
        acc = f(acc, group);
    }
    acc
}

/// Parse AoC input format with groups of inputs as chunks of a number of lines into
/// a vector of "groups", where each group is a vector of the output of a given
/// function called on each indvidual input line (trimmed) from the group.
pub fn fold_chunks<'a, B, F: FnMut(B, &[&'a str]) -> B>(
    input: &'a str,
    chunk_size: usize,
    mut acc: B,
    mut f: F,
) -> B {
    assert!(chunk_size > 0);
    let lines: Vec<&str> = lines(input);
    for chunk in lines.chunks(chunk_size) {
        acc = f(acc, chunk);
    }
    acc
}

/// Parse AoC input format with groups of inputs separated by blank lines into
/// a vector of the result of folding a given function over each indvidual input
/// line (trimmed) of a group.
pub fn groups_fold_lines<'a, B: Clone, F: FnMut(B, &'a str) -> B>(
    input: &'a str,
    acc: B,
    mut f: F,
) -> Vec<B> {
    let lines: Vec<&str> = lines(input);
    lines
        .split(|line| line.is_empty())
        .map(|s| s.iter().copied().fold(acc.clone(), &mut f))
        .collect()
}

/// Parse AoC input format with groups of inputs separated by blank lines into
/// a vector of the result of folding a given function over each indvidual input
/// line (trimmed) of a group.
pub fn chunks_fold_lines<'a, B: Clone, F: FnMut(B, &'a str) -> B>(
    input: &'a str,
    chunk_size: usize,
    acc: B,
    mut f: F,
) -> Vec<B> {
    assert!(chunk_size > 0);
    let lines: Vec<&str> = lines(input);
    lines
        .chunks(chunk_size)
        .map(|s| s.iter().copied().fold(acc.clone(), &mut f))
        .collect()
}

/// requires a and b are sorted ascending
/// Performs a sort of merge-sort-like search.
/// Returns the first common item found, from the first slice.
/// May return None if a pair is incomparable.
pub fn common_item<'a, T: PartialOrd>(a: &'a [T], b: &[T]) -> Option<&'a T> {
    let mut i = 0;
    let mut j = 0;
    loop {
        let a_item = a.get(i)?;
        let b_item = b.get(j)?;
        match a_item.partial_cmp(b_item)? {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Equal => return Some(a_item),
            std::cmp::Ordering::Greater => j += 1,
        }
    }
}

/// requires a and b are sorted ascending
/// Performs a sort of merge-sort-like search
/// Returns the first common item found, from the first slice.
/// May return None if a pair is incomparable.
pub fn common_item3<'a, T: PartialOrd>(a: &'a [T], b: &[T], c: &[T]) -> Option<&'a T> {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    loop {
        let ab_item;
        loop {
            let a_item = a.get(i)?;
            let b_item = b.get(j)?;
            match a_item.partial_cmp(b_item)? {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Equal => {
                    ab_item = a_item;
                    break;
                }
                std::cmp::Ordering::Greater => j += 1,
            }
        }
        let c_item = c.get(k)?;
        match ab_item.partial_cmp(c_item)? {
            std::cmp::Ordering::Less => {
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Equal => return Some(ab_item),
            std::cmp::Ordering::Greater => k += 1,
        }
    }
}

pub struct SingleFunction<F, A> {
    inner: Option<Either<F, std::thread::Result<(A, Option<A>)>>>,
}

impl<F, A, T> SingleFunction<F, A>
where
    T: Into<Option<A>>,
    F: FnOnce(&str) -> (A, T) + UnwindSafe,
{
    pub fn new(f: F) -> Self {
        Self {
            inner: Some(Either::Left(f)),
        }
    }

    pub fn part_1<'a>(&'a mut self) -> impl for<'b> FnOnce(&'b str) -> &'a A + 'a {
        |input| {
            let output = match self.inner.take().unwrap() {
                Either::Left(runner) => {
                    let output = std::panic::catch_unwind(|| runner(input));
                    let output = output.map(|(a, b)| (a, b.into()));
                    output
                }
                Either::Right(output) => output,
            };
            self.inner = Some(Either::Right(output));
            match self.inner.as_ref().unwrap().as_ref().right().unwrap() {
                Ok((p1, _p2)) => p1,
                Err(_) => panic!("solution function panicked"),
            }
        }
    }
    pub fn part_2<'a>(&'a mut self) -> impl for<'b> FnOnce(&'b str) -> &'a A + 'a {
        |input| {
            let output = match self.inner.take().unwrap() {
                Either::Left(runner) => {
                    let output = std::panic::catch_unwind(|| runner(input));
                    let output = output.map(|(a, b)| (a, b.into()));
                    output
                }
                Either::Right(output) => output,
            };
            self.inner = Some(Either::Right(output));
            match self.inner.as_ref().unwrap().as_ref().right().unwrap() {
                Ok((_p1, Some(p2))) => p2,
                Ok((_p1, None)) => panic!("solution function only gave part 1"),
                Err(_) => panic!("solution function panicked"),
            }
        }
    }
}

struct MyOnce<F, R> {
    inner: Either<F, std::thread::Result<R>>,
}

impl<F, R> MyOnce<F, R> {
    fn new(callable: F) -> Self {
        Self {
            inner: Either::Left(callable),
        }
    }

    fn call_once<U>(&mut self, input: U, name: &str) -> &R
    where
        F: FnOnce(U) -> R,
    {
        self.try_call_once(input)
            .unwrap_or_else(|_| panic!("{name} panicked"))
    }

    fn try_call_once<U>(&mut self, input: U) -> Result<&R, &Box<dyn Any + Send>>
    where
        F: FnOnce(U) -> R,
    {
        if self.inner.is_left() {
            replace_with::replace_with_or_abort(&mut self.inner, |inner| {
                let callable = inner.left().unwrap();
                Either::Right(std::panic::catch_unwind(AssertUnwindSafe(|| {
                    callable(input)
                })))
            })
        }
        match &self.inner {
            Either::Right(value) => value.as_ref(),
            _ => unreachable!(),
        }
    }
}

pub struct PreParsed<ParseFn, Parsed, Part1Fn, Part2Fn, Part1Ret, Part2Ret> {
    parser: MyOnce<ParseFn, Parsed>,
    part_1: MyOnce<Part1Fn, Part1Ret>,
    part_2: MyOnce<Part2Fn, Part2Ret>,
}

impl<PF, P, F1, F2, R1, R2> PreParsed<PF, P, F1, F2, R1, R2>
where
    PF: FnOnce(&str) -> P,
    F1: FnOnce(&P) -> R1 + UnwindSafe,
    F2: FnOnce(&P) -> R2 + UnwindSafe,
    P: RefUnwindSafe,
{
    pub fn new(parser: PF, part_1: F1, part_2: F2) -> Self {
        Self {
            parser: MyOnce::new(parser),
            part_1: MyOnce::new(part_1),
            part_2: MyOnce::new(part_2),
        }
    }

    pub fn part_1<'a>(&'a mut self) -> impl for<'b> FnOnce(&'b str) -> &'a R1 + 'a {
        |input| {
            let parsed = self.parser.call_once(input, "parser");
            self.part_1.call_once(parsed, "part 1 solution function")
        }
    }

    pub fn part_2<'a>(&'a mut self) -> impl for<'b> FnOnce(&'b str) -> &'a R2 + 'a {
        |input| {
            let parsed = self.parser.call_once(input, "parser");
            self.part_2.call_once(parsed, "part 2 solution function")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{groups, lines};

    #[test]
    fn test_list() {
        assert_eq!(
            lines("a\nb\nc\n\nd\ne\n"),
            vec!["a", "b", "c", "", "d", "e"],
        );
    }

    #[test]
    fn test_list_of_lists() {
        assert_eq!(
            groups("a\nb\nc\n\nd\ne\n"),
            vec![vec!["a", "b", "c"], vec!["d", "e"]],
        );
    }
}
