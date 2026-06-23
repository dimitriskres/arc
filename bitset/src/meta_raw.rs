
use crate::{bit, flat_raw};

pub const fn get_word_count(metas: & [usize], words: & [usize]) -> usize
{
    return match metas
    {
        [_] => words.len(),
        [_, word_count, ..] => * word_count,
        [] => unreachable!()
    };
}

pub const fn get_capacity(metas: & [usize], words: & [usize]) -> usize
{
    return get_word_count(metas, words) * bit::SIZE;
}

pub const fn get_index_at_level(index: usize, level: usize) -> usize
{
    return index >> (bit::MOVE * level);
}

pub const fn get_word_index_at_level(index: usize, level: usize) -> usize 
{
    return get_index_at_level(index, level + 1);
}

pub const fn get_rune_index_at_level(index: usize, level: usize) -> usize
{
    return get_index_at_level(index, level) & (bit::SIZE - 1);
}

pub unsafe fn get_unchecked(metas: & [usize], words: & [usize], index: usize) -> bool
{
    debug_assert!(index < get_capacity(metas, words));

    let rune = unsafe { flat_raw::get_unchecked(words, index) };

    return rune;
}

pub unsafe fn insert_unchecked(metas: & [usize], words: & mut [usize], mut index: usize)
{
    debug_assert!(index < get_capacity(metas, words));

    let mut level = 0;

    let mut meta = 0;

    loop
    {
        let word_index = flat_raw::get_word_index(index);

        let rune_index = flat_raw::get_rune_index(index);

        let word = unsafe { words.get_unchecked_mut(meta + word_index) };

        let zero = * word == 0;

        * word = unsafe { bit::insert_unchecked(* word, rune_index) };

        if ! zero
        {
            break;
        };

        level += 1;

        if level == metas.len()
        {
            break;
        };

        index = word_index;

        meta = * unsafe { metas.get_unchecked(level) };
    };
}

pub unsafe fn remove_unchecked(metas: & [usize], words: & mut [usize], mut index: usize)
{
    debug_assert!(index < get_capacity(metas, words));

    let mut level = 0;

    let mut meta = 0;

    loop
    {
        let word_index = flat_raw::get_word_index(index);

        let rune_index = flat_raw::get_rune_index(index);

        let word = unsafe { words.get_unchecked_mut(meta + word_index) };

        if * word == 0
        {
            break;
        };

        * word = unsafe { bit::remove_unchecked(* word, rune_index) };

        if * word != 0
        {
            break;
        };

        level += 1;

        if level == metas.len()
        {
            break;
        };

        index = word_index;

        meta = * unsafe { metas.get_unchecked(level) };
    };
}

pub unsafe fn fill_with_start_unchecked(metas: & [usize], words: & mut [usize], mut start: usize)
{
    debug_assert!(metas.len() > 0);
    debug_assert!(start < get_capacity(metas, words));

    let mut level = 0;

    let mut meta = 0;

    loop
    {
        level += 1;

        let start_word_index = flat_raw::get_word_index(start);

        let start_rune_index = flat_raw::get_rune_index(start);

        let mut span_start_word_index = meta + start_word_index;

        let span_limit_word_index = metas.get(level).copied().unwrap_or(words.len());

        if start_rune_index != 0
        {
            let word = unsafe { words.get_unchecked_mut(span_start_word_index) };

            * word = unsafe { bit::fill_upper_at_unchecked(* word, start_rune_index) };

            span_start_word_index += 1; // at boundary -> do not fill word
        };

        unsafe { flat_raw::span_with_word_range_unchecked(words, span_start_word_index, span_limit_word_index, bit::FULL) };

        if level == metas.len()
        {
            break;
        };

        start = start_word_index;

        meta = span_limit_word_index;
    };
}

pub unsafe fn drop_with_start_unchecked(metas: & [usize], words: & mut [usize], mut start: usize)
{
    debug_assert!(metas.len() > 0);
    debug_assert!(start < get_capacity(metas, words));

    let mut level = 0;

    let mut meta = 0;

    loop
    {
        level += 1;

        let mut start_word_index = flat_raw::get_word_index(start);

        let start_rune_index = flat_raw::get_rune_index(start);

        let mut span_start_word_index = meta + start_word_index;

        let span_limit_word_index = metas.get(level).copied().unwrap_or(words.len());

        if start_rune_index != 0
        {
            let word = unsafe { words.get_unchecked_mut(span_start_word_index) };

            * word = unsafe { bit::drop_upper_at_unchecked(* word, start_rune_index) };

            span_start_word_index += 1; // at boundary -> do not drop the entire word

            start_word_index += (* word != 0) as usize; // at boundary and word nonzero -> skip the parent
        };

        unsafe { flat_raw::span_with_word_range_unchecked(words, span_start_word_index, span_limit_word_index, bit::ZERO) };

        if level == metas.len()
        {
            break;
        };

        start = start_word_index;

        meta = span_limit_word_index;
    };
}

pub unsafe fn fill_with_limit_unchecked(metas: & [usize], words: & mut [usize], mut limit: usize)
{
    debug_assert!(metas.len() > 0);
    debug_assert!(limit <= get_capacity(metas, words));

    let mut level = 0;

    let mut meta = 0;

    loop
    {
        level += 1;

        let mut limit_word_index = flat_raw::get_word_index(limit);

        let limit_rune_index = flat_raw::get_rune_index(limit);

        let span_start_word_index = meta;

        let span_limit_word_index = meta + limit_word_index;

        unsafe { flat_raw::span_with_word_range_unchecked(words, span_start_word_index, span_limit_word_index, bit::FULL) };

        if limit_rune_index != 0
        {
            let word = unsafe { words.get_unchecked_mut(span_limit_word_index) };

            * word = unsafe { bit::fill_lower_at_unchecked(* word, limit_rune_index) };

            limit_word_index += 1; // at boundary -> fill parent
        };

        if level == metas.len()
        {
            break;
        };

        limit = limit_word_index;

        meta = * unsafe { metas.get_unchecked(level) };
    };
}

pub unsafe fn drop_with_limit_unchecked(metas: & [usize], words: & mut [usize], mut limit: usize)
{
    debug_assert!(metas.len() > 0);
    debug_assert!(limit <= get_capacity(metas, words));

    let mut level = 0;

    let mut meta = 0;

    loop
    {
        level += 1;

        let mut limit_word_index = flat_raw::get_word_index(limit);

        let limit_rune_index = flat_raw::get_rune_index(limit);

        let span_start_word_index = meta;

        let span_limit_word_index = meta + limit_word_index;

        unsafe { flat_raw::span_with_word_range_unchecked(words, span_start_word_index, span_limit_word_index, bit::ZERO) };

        if limit_rune_index != 0
        {
            let word = unsafe { words.get_unchecked_mut(span_limit_word_index) };

            * word = unsafe { bit::drop_lower_at_unchecked(* word, limit_rune_index) };

            limit_word_index += (* word == 0) as usize; // at boundary and word zero -> drop the parent
        };

        if level == metas.len()
        {
            break;
        };

        limit = limit_word_index;

        meta = * unsafe { metas.get_unchecked(level) };
    };
}

pub unsafe fn fill_with_range_unchecked(metas: & [usize], words: & mut [usize], mut start: usize, mut limit: usize)
{
    debug_assert!(metas.len() > 0);
    debug_assert!(start < limit);
    debug_assert!(limit <= get_capacity(metas, words));

    let mut level = 0;

    let mut meta = 0;

    loop
    {
        level += 1;

        let start_word_index = flat_raw::get_word_index(start);

        let start_rune_index = flat_raw::get_rune_index(start);

        let mut limit_word_index = flat_raw::get_word_index(limit);

        let limit_rune_index = flat_raw::get_rune_index(limit);

        if start_word_index == limit_word_index
        {
            let word = unsafe { words.get_unchecked_mut(meta + start_word_index) };

            let zero = * word == 0;

            * word = unsafe { bit::fill_range_unchecked(* word, start_rune_index, limit_rune_index) };

            if ! zero
            {
                break;
            };

            limit_word_index += 1; // at boundary -> fill parent
        }
        else
        {
            let mut span_start_word_index = meta + start_word_index;

            let span_limit_word_index = meta + limit_word_index;

            if start_rune_index != 0
            {
                let word = unsafe { words.get_unchecked_mut(span_start_word_index) };

                * word = unsafe { bit::fill_upper_at_unchecked(* word, start_rune_index) };

                span_start_word_index += 1; // at boundary -> do not fill word
            };

            unsafe { flat_raw::span_with_word_range_unchecked(words, span_start_word_index, span_limit_word_index, bit::FULL) };

            if limit_rune_index != 0
            {
                let word = unsafe { words.get_unchecked_mut(span_limit_word_index) };

                * word = unsafe { bit::fill_lower_at_unchecked(* word, limit_rune_index) };

                limit_word_index += 1; // at boundary -> fill parent
            };
        };

        if level == metas.len()
        {
            break;
        };

        start = start_word_index;

        limit = limit_word_index;

        meta = * unsafe { metas.get_unchecked(level) };
    };
}

pub unsafe fn drop_with_range_unchecked(metas: & [usize], words: & mut [usize], mut start: usize, mut limit: usize)
{
    debug_assert!(metas.len() > 0);
    debug_assert!(start < limit);
    debug_assert!(limit <= get_capacity(metas, words));

    let mut level = 0;

    let mut meta = 0;

    loop
    {
        level += 1;

        let mut start_word_index = flat_raw::get_word_index(start);

        let start_rune_index = flat_raw::get_rune_index(start);

        let mut limit_word_index = flat_raw::get_word_index(limit);

        let limit_rune_index = flat_raw::get_rune_index(limit);

        if start_word_index == limit_word_index
        {
            let word = unsafe { words.get_unchecked_mut(meta + start_word_index) };

            * word = unsafe { bit::drop_range_unchecked(* word, start_rune_index, limit_rune_index) };

            if * word != 0
            {
                break;
            };
            
            limit_word_index += 1; // at boundary and word zero -> drop the parent
        }
        else
        {
            let mut span_start_word_index = meta + start_word_index;

            let span_limit_word_index = meta + limit_word_index;

            if start_rune_index != 0
            {
                let word = unsafe { words.get_unchecked_mut(span_start_word_index) };

                * word = unsafe { bit::drop_upper_at_unchecked(* word, start_rune_index) };

                span_start_word_index += 1; // at boundary -> do not drop the entire word

                start_word_index += (* word != 0) as usize; // at boundary and word nonzero -> skip the parent
            };

            unsafe { flat_raw::span_with_word_range_unchecked(words, span_start_word_index, span_limit_word_index, bit::ZERO) };

            if limit_rune_index != 0
            {
                let word = unsafe { words.get_unchecked_mut(span_limit_word_index) };

                * word = unsafe { bit::drop_lower_at_unchecked(* word, limit_rune_index) };

                limit_word_index += (* word == 0) as usize; // at boundary and word zero -> drop the parent
            };
        };

        if level == metas.len()
        {
            break;
        };

        if start_word_index == limit_word_index 
        { 
            break; 
        };

        start = start_word_index;

        limit = limit_word_index;

        meta = * unsafe { metas.get_unchecked(level) };
    };
}

unsafe fn next_raw_unchecked(metas: & [usize], words: & [usize], mut meta: usize, start_word_index: usize, limit_word_index: usize) -> usize
{
    let mut level = metas.len() - 1;

    let mut word_index = start_word_index;

    let mut word;

    loop
    {
        if word_index == limit_word_index
        {
            return usize::MAX;
        };

        word = * unsafe { words.get_unchecked(word_index) };

        if word != 0
        {
            break;
        };

        word_index += 1;
    };

    word_index -= meta;

    let mut index;

    loop 
    {
        let rune_index = unsafe { bit::next_unchecked(word) };

        index = flat_raw::get_index(word_index, rune_index);

        if level == 0
        {
            break;
        };

        level -= 1;

        meta = * unsafe { metas.get_unchecked(level) };

        word_index = index;

        word = * unsafe { words.get_unchecked(meta + word_index) };
    };

    return index;
}

pub unsafe fn next_unchecked(metas: & [usize], words: & [usize]) -> usize
{
    debug_assert!(metas.len() > 0);

    let level = metas.len() - 1;

    let meta = * unsafe { metas.get_unchecked(level) };

    let start_word_index = meta;

    let limit_word_index = words.len();

    let index = unsafe { next_raw_unchecked(metas, words, meta, start_word_index, limit_word_index) };

    return index;
}

pub unsafe fn next_with_start_raw_unchecked(metas: & [usize], words: & [usize], start: usize, limit_word_index: usize) -> usize
{
    let mut level = 0;

    let mut meta = 0;

    let mut word;

    let mut word_index = start;

    loop
    {
        let rune_index = flat_raw::get_rune_index(word_index);

        word_index = flat_raw::get_word_index(word_index);
        
        word = * unsafe { words.get_unchecked(meta + word_index) };

        word = unsafe { bit::drop_lower_at_unchecked(word, rune_index) };

        if word != 0
        {
            break;
        };

        level += 1;

        word_index += 1;

        if level == metas.len()
        {
            word_index += meta;

            loop
            {
                if word_index == limit_word_index
                {
                    return usize::MAX;
                };

                word = * unsafe { words.get_unchecked(word_index) };

                if word != 0
                {
                    break;
                };

                word_index += 1;
            };

            word_index -= meta;

            level -= 1;

            break;
        };

        let save = word_index + meta;

        meta = * unsafe { metas.get_unchecked(level) };

        if save == meta
        {
            return usize::MAX;
        };
    };

    let mut index;

    loop 
    {
        let rune_index = unsafe { bit::next_unchecked(word) };

        index = flat_raw::get_index(word_index, rune_index);

        if level == 0
        {
            break;
        };

        level -= 1;

        meta = * unsafe { metas.get_unchecked(level) };

        word_index = index;

        word = * unsafe { words.get_unchecked(meta + word_index) };
    };

    return index;
}

pub unsafe fn next_with_start_unchecked(metas: & [usize], words: & [usize], start: usize) -> usize
{
    debug_assert!(metas.len() > 0);
    debug_assert!(start < get_capacity(metas, words));

    let limit_word_index = words.len();

    let index = unsafe { next_with_start_raw_unchecked(metas, words, start, limit_word_index) };

    return index;
}

pub unsafe fn next_with_limit_unchecked(metas: & [usize], words: & [usize], limit: usize) -> usize
{
    debug_assert!(metas.len() > 0);
    debug_assert!(limit > 0);
    debug_assert!(limit <= get_capacity(metas, words));

    let level = metas.len() - 1;

    let meta = * unsafe { metas.get_unchecked(level) };

    let start_word_index = meta;

    let limit_word_index = meta + get_word_index_at_level(limit - 1, level) + 1;

    let index = unsafe { next_raw_unchecked(metas, words, meta, start_word_index, limit_word_index) };

    if index >= limit
    {
        return usize::MAX;
    };

    return index;
}

pub unsafe fn next_with_range_unchecked(metas: & [usize], words: & [usize], start: usize, limit: usize) -> usize
{
    debug_assert!(start < limit);
    debug_assert!(limit <= get_capacity(metas, words));

    let level = metas.len() - 1;

    let meta = * unsafe { metas.get_unchecked(level) };

    let limit_word_index = meta + get_word_index_at_level(limit - 1, level) + 1;

    let index = unsafe { next_with_start_raw_unchecked(metas, words, start, limit_word_index) };

    if index >= limit
    {
        return usize::MAX;
    };

    return index;
}

pub struct Iterator<'a, const N: usize>
{
    metas: &'a [usize],
    words: &'a [usize],
    level: usize,
    cache: [usize; N],
    state: usize,
    index: usize
}

impl<const N: usize> Default for Iterator<'_ , N>
{
    fn default() -> Self
    {
        return Self::empty();
    }
}

impl<'a, const N: usize> Iterator<'a, N>
{
    unsafe fn from_raw(metas: &'a [usize], words: &'a [usize], level: usize, cache: [usize; N], state: usize, index: usize) -> Self
    {
        return Self { metas, words, level, cache, state, index };
    }

    pub fn empty() -> Self
    {
        let metas = & [0];

        let words = & [0];

        return unsafe { Self::new_unchecked(metas, words) };
    }

    pub unsafe fn new_unchecked(metas: &'a [usize], words: &'a [usize]) -> Self
    {
        debug_assert!(metas.len() > 0);

        let level = 0;

        let cache = [0; N];

        let state = 0;

        let index = 0;

        return unsafe { Self::from_raw(metas, words, level, cache, state, index) };
    }

    pub unsafe fn new_with_start_unchecked(metas: &'a [usize], words: &'a [usize], start: usize) -> Self
    {
        debug_assert!(metas.len() > 0);
        debug_assert!(start < get_capacity(metas, words));

        let mut level = metas.len() - 1;

        let mut cache = [0; N];

        let index = get_word_index_at_level(start, level) + 1;

        loop
        {
            let meta = * unsafe { metas.get_unchecked(level) };

            let word_index = get_word_index_at_level(start, level);

            let rune_index = get_rune_index_at_level(start, level) + (level != 0) as usize;

            let mut word = * unsafe { words.get_unchecked(meta + word_index) };

            word = if rune_index == bit::SIZE { 0 } else { unsafe { bit::drop_lower_at_unchecked(word, rune_index) } };

            * unsafe { cache.get_unchecked_mut(level) } = word;

            if level == 0
            {
                break;
            };

            level -= 1;
        };

        let state = get_index_at_level(start, level + 1);

        return unsafe { Self::from_raw(metas, words, level, cache, state, index) };
    }    
}

impl<'a, const N: usize> std::iter::Iterator for Iterator<'a, N>
{
    type Item = usize;

    fn next(& mut self) -> Option<Self::Item> 
    {
        let mut word;

        loop 
        {
            word = unsafe { self.cache.get_unchecked_mut(self.level) };

            if * word != 0
            {
                break;
            };

            self.level += 1;

            if self.level == self.metas.len()
            {
                self.level -= 1;

                let meta = * unsafe { self.metas.get_unchecked(self.level) };

                let mut word_index = meta + self.index;

                * word = loop
                {
                    if word_index == self.words.len()
                    {
                        return None;
                    };

                    let temp = * unsafe { self.words.get_unchecked(word_index) };

                    if temp != 0
                    {
                        break temp;
                    };

                    word_index += 1;
                };

                self.state = word_index - meta;

                self.index = self.state + 1;
                
                break;
            };

            self.state = flat_raw::get_word_index(self.state);
        };

        let mut index;

        loop
        {
            let rune_index = unsafe { bit::next_unchecked(* word) };

            * word = unsafe { bit::clip_unchecked(* word) };

            index = flat_raw::get_index(self.state, rune_index);

            if self.level == 0
            {
                break;
            };

            self.level -= 1;

            self.state = index;

            word = unsafe { self.cache.get_unchecked_mut(self.level) };

            let meta = * unsafe { self.metas.get_unchecked(self.level) };

            * word = * unsafe { self.words.get_unchecked(meta + self.state) };
        };

        return Some(index);
    }
}

pub struct IteratorWithLimit<'a, const N: usize>
{
    inner: Iterator<'a, N>,
    limit: usize
}

impl<'a, const N: usize> Default for IteratorWithLimit<'a, N>
{
    fn default() -> Self
    {
        return Self::empty();
    }
}

impl<'a, const N: usize> IteratorWithLimit<'a, N>
{
    unsafe fn from_raw(inner: Iterator<'a, N>, limit: usize) -> Self
    {
        return Self { inner, limit };
    }

    pub fn empty() -> Self
    {
        let inner = Iterator::empty();

        return unsafe { Self::from_raw(inner, 0) };
    }

    pub unsafe fn new_unchecked(metas: &'a [usize], words: &'a [usize], limit: usize) -> Self
    {
        debug_assert!(metas.len() > 0);

        let level = metas.len() - 1;

        let meta = * unsafe { metas.get_unchecked(level) };

        let limit_word_index = meta + get_word_index_at_level(limit - 1, level) + 1;

        let words = & words[..limit_word_index];

        let inner = unsafe { Iterator::new_unchecked(metas, words) };

        return unsafe { Self::from_raw(inner, limit) };
    }

    pub unsafe fn new_with_range_unchecked(metas: &'a [usize], words: &'a [usize], start: usize, limit: usize) -> Self
    {
        debug_assert!(metas.len() > 0);
        debug_assert!(start < limit);
        debug_assert!(limit <= get_capacity(metas, words));

        let level = metas.len() - 1;

        let meta = * unsafe { metas.get_unchecked(level) };

        let limit_word_index = meta + get_word_index_at_level(limit - 1, level) + 1;

        let words = & words[..limit_word_index];

        let inner = unsafe { Iterator::new_with_start_unchecked(metas, words, start) };

        return unsafe { Self::from_raw(inner, limit) };
    }
}

impl<'a, const N: usize> std::iter::Iterator for IteratorWithLimit<'a, N>
{
    type Item = usize;

    fn next(& mut self) -> Option<Self::Item> 
    {
        let index = self.inner.next()?;

        if index >= self.limit
        {
            return None;
        };

        return Some(index);
    }
}