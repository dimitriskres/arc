use std::{ptr, usize};

use crate::bit;

pub const fn get_capacity(words: & [usize]) -> usize
{
    return words.len() * bit::SIZE;
}

pub const fn get_word_index(index: usize) -> usize
{
    return index / bit::SIZE;
}

pub const fn get_rune_index(index: usize) -> usize
{
    return index % bit::SIZE;
}

pub const fn get_index(word_index: usize, rune_index: usize) -> usize
{
    return word_index * bit::SIZE + rune_index;
}

pub unsafe fn get_unchecked(words: & [usize], index: usize) -> bool
{
    debug_assert!(index < get_capacity(words));

    let word_index = get_word_index(index);

    let rune_index = get_rune_index(index);

    let word = * unsafe { words.get_unchecked(word_index)};

    let rune = unsafe { bit::get_unchecked(word, rune_index) };

    return rune;
}

pub unsafe fn insert_unchecked(words: & mut [usize], index: usize)
{
    debug_assert!(index < get_capacity(words));

    let word_index = get_word_index(index);

    let rune_index = get_rune_index(index);

    let word = unsafe { words.get_unchecked_mut(word_index) };

    * word = unsafe { bit::insert_unchecked(* word, rune_index) };
}

pub unsafe fn remove_unchecked(words: & mut [usize], index: usize)
{
    debug_assert!(index < get_capacity(words));

    let word_index = get_word_index(index);

    let rune_index = get_rune_index(index);

    let word = unsafe { words.get_unchecked_mut(word_index) };

    * word = unsafe { bit::remove_unchecked(* word, rune_index) };
}

pub(crate) unsafe fn span_with_word_range_unchecked(words: & mut [usize], start_word_index: usize, limit_word_index: usize, value: usize)
{
    debug_assert!(start_word_index <= limit_word_index);
    debug_assert!(limit_word_index <= words.len());

    let count = limit_word_index - start_word_index;

    let words_ptr = unsafe { words.as_mut_ptr().add(start_word_index) };

    unsafe { ptr::write_bytes(words_ptr, value as u8, count) };
}

pub unsafe fn fill_with_start_unchecked(words: & mut [usize], start: usize)
{
    debug_assert!(start < get_capacity(words));

    let mut start_word_index = get_word_index(start);

    let start_rune_index = get_rune_index(start);

    if start_rune_index != 0
    {
        let word = unsafe { words.get_unchecked_mut(start_word_index) };

        * word = unsafe { bit::fill_upper_at_unchecked(* word, start_rune_index) };

        start_word_index += 1;
    };

    let limit_word_index = words.len();

    unsafe { span_with_word_range_unchecked(words, start_word_index, limit_word_index, bit::FULL) };
}

pub unsafe fn drop_with_start_unchecked(words: & mut [usize], start: usize)
{
    debug_assert!(start < get_capacity(words));

    let mut start_word_index = get_word_index(start);

    let start_rune_index = get_rune_index(start);

    if start_rune_index != 0
    {
        let word = unsafe { words.get_unchecked_mut(start_word_index) };

        * word = unsafe { bit::drop_upper_at_unchecked(* word, start_rune_index) };

        start_word_index += 1;
    };

    let limit_word_index = words.len();

    unsafe { span_with_word_range_unchecked(words, start_word_index, limit_word_index, bit::ZERO) };
}

pub unsafe fn fill_with_limit_unchecked(words: & mut [usize], limit: usize)
{
    debug_assert!(limit <= get_capacity(words));

    let start_word_index = 0;

    let limit_word_index = get_word_index(limit);

    let limit_rune_index = get_rune_index(limit);

    unsafe { span_with_word_range_unchecked(words, start_word_index, limit_word_index, bit::FULL) };

    if limit_rune_index != 0
    {
        let word = unsafe { words.get_unchecked_mut(limit_word_index) };

        * word = unsafe { bit::fill_lower_at_unchecked(* word, limit_rune_index) };
    };
}

pub unsafe fn drop_with_limit_unchecked(words: & mut [usize], limit: usize)
{
    debug_assert!(limit <= get_capacity(words));

    let start_word_index = 0;

    let limit_word_index = get_word_index(limit);

    let limit_rune_index = get_rune_index(limit);

    unsafe { span_with_word_range_unchecked(words, start_word_index, limit_word_index, bit::ZERO) };

    if limit_rune_index != 0
    {
        let word = unsafe { words.get_unchecked_mut(limit_word_index) };

        * word = unsafe { bit::drop_lower_at_unchecked(* word, limit_rune_index) };
    };
}

pub unsafe fn fill_with_range_unchecked(words: & mut [usize], start: usize, limit: usize)
{
    debug_assert!(start < limit);
    debug_assert!(limit <= get_capacity(words));

    let mut start_word_index = get_word_index(start);

    let start_rune_index = get_rune_index(start);

    let limit_word_index = get_word_index(limit);

    let limit_rune_index = get_rune_index(limit);

    if start_word_index == limit_word_index
    {
        let word = unsafe { words.get_unchecked_mut(start_word_index) };

        * word = unsafe { bit::fill_range_unchecked(* word, start_rune_index, limit_rune_index) };

        return; // single word case; nothing else to do
    };

    if start_rune_index != 0
    {
        let word = unsafe { words.get_unchecked_mut(start_word_index) };

        * word = unsafe { bit::fill_upper_at_unchecked(* word, start_rune_index) };

        start_word_index += 1;
    };

    unsafe { span_with_word_range_unchecked(words, start_word_index, limit_word_index, bit::FULL) };

    if limit_rune_index != 0
    {
        let word = unsafe { words.get_unchecked_mut(limit_word_index) };

        * word = unsafe { bit::fill_lower_at_unchecked(* word, limit_rune_index) };
    };
}

pub unsafe fn drop_with_range_unchecked(words: & mut [usize], start: usize, limit: usize)
{
    debug_assert!(start < limit);
    debug_assert!(limit <= get_capacity(words));

    let mut start_word_index = get_word_index(start);

    let start_rune_index = get_rune_index(start);

    let limit_word_index = get_word_index(limit);

    let limit_rune_index = get_rune_index(limit);

    if start_word_index == limit_word_index
    {
        let word = unsafe { words.get_unchecked_mut(start_word_index) };

        * word = unsafe { bit::drop_range_unchecked(* word, start_rune_index, limit_rune_index) };

        return; // single word case; nothing else to do
    };

    if start_rune_index != 0
    {
        let word = unsafe { words.get_unchecked_mut(start_word_index) };

        * word = unsafe { bit::drop_upper_at_unchecked(* word, start_rune_index) };

        start_word_index += 1;
    };

    unsafe { span_with_word_range_unchecked(words, start_word_index, limit_word_index, bit::ZERO) };

    if limit_rune_index != 0
    {
        let word = unsafe { words.get_unchecked_mut(limit_word_index) };

        * word = unsafe { bit::drop_lower_at_unchecked(* word, limit_rune_index) };
    };
}

pub unsafe fn next_unchecked(words: & [usize]) -> usize
{
    debug_assert!(words.len() > 0);

    let mut word_index = 0;

    let mut word;

    loop
    {
        word = unsafe { * words.get_unchecked(word_index) };

        if word != 0
        {
            break;
        };

        word_index += 1;

        if word_index == words.len()
        {
            return usize::MAX;
        }; 
    };

    let rune_index = unsafe { bit::next_unchecked(word) };

    let index = get_index(word_index, rune_index);

    return index;
}

pub unsafe fn next_with_start_unchecked(words: & [usize], start: usize) -> usize
{
    debug_assert!(words.len() > 0);
    debug_assert!(start < get_capacity(words));

    let mut word_index = get_word_index(start);

    let mut word;

    let start_rune_index = get_rune_index(start);

    word = unsafe { * words.get_unchecked(word_index) };

    word = unsafe { bit::drop_lower_at_unchecked(word, start_rune_index) };

    while word == 0
    {
        word_index += 1;

        if word_index == words.len()
        {
            return usize::MAX;
        };

        word = unsafe { * words.get_unchecked(word_index) };
    };

    let rune_index = unsafe { bit::next_unchecked(word) };

    let index = get_index(word_index, rune_index);

    return index;
}

pub unsafe fn next_with_limit_unchecked(words: & [usize], mut limit: usize) -> usize
{
    debug_assert!(words.len() > 0);
    debug_assert!(limit > 0);
    debug_assert!(limit <= get_capacity(words));

    limit -= 1; // inclusive

    let limit_word_index = get_word_index(limit);

    let limit_rune_index = get_rune_index(limit) + 1; // exclusive

    let mut word_index = 0;

    let mut word;

    loop
    {
        word = unsafe { * words.get_unchecked(word_index) };

        if word_index == limit_word_index
        {
            word = unsafe { bit::drop_upper_at_unchecked(word, limit_rune_index) };

            if word != 0
            {
                break;
            };

            return usize::MAX;
        };

        if word != 0
        {
            break;
        };

        word_index += 1;
    };

    let rune_index = unsafe { bit::next_unchecked(word) };

    let index = get_index(word_index, rune_index);

    return index;
}

pub unsafe fn next_with_range_unchecked(words: & [usize], start: usize, mut limit: usize) -> usize
{
    debug_assert!(words.len() > 0);
    debug_assert!(start < limit);
    debug_assert!(limit <= get_capacity(words));

    limit -= 1; // inclusive

    let mut word_index = get_word_index(start);

    let mut word;

    let start_rune_index = get_rune_index(start);

    let limit_word_index = get_word_index(limit);

    let limit_rune_index = get_rune_index(limit) + 1; // exclusive

    word = unsafe { * words.get_unchecked(word_index) };

    if word_index == limit_word_index
    {
        word = unsafe { bit::drop_range_unchecked(word, start_rune_index, limit_rune_index) };

        if word == 0
        {
            return usize::MAX;
        };
    }
    else
    {
        word = unsafe { bit::drop_lower_at_unchecked(word, start_rune_index) };

        while word == 0
        {
            word_index += 1;

            word = unsafe { * words.get_unchecked(word_index) };

            if word_index == limit_word_index
            {
                word = unsafe { bit::drop_upper_at_unchecked(word, limit_rune_index) };

                if word != 0
                {
                    break;
                };

                return usize::MAX;
            };
        };
    };

    let rune_index = unsafe { bit::next_unchecked(word) };

    let index = get_index(word_index, rune_index);

    return index;
}

pub struct Iterator<'a>
{
    words: &'a [usize],
    state: usize,
    cache: usize,
}

impl<'a> Default for Iterator<'a>
{
    fn default() -> Self
    {
        return Self::empty();
    }
}

impl<'a> Iterator<'a>
{
    unsafe fn from_raw(words: &'a [usize], state: usize, cache: usize) -> Self
    {
        return Self { words, state, cache };
    }

    pub fn empty() -> Self
    {
        let words = & [0];

        let state = words.len() - 1;

        let cache = 0;

        return unsafe { Self::from_raw(words, state, cache) };
    }

    pub unsafe fn new(words: &'a [usize]) -> Self
    {
        debug_assert!(words.len() > 0);

        let state = 0;

        let cache = unsafe { * words.get_unchecked(0) };

        return unsafe { Self::from_raw(words, state, cache) };
    }

    pub unsafe fn new_with_start(words: &'a [usize], start: usize) -> Self
    {
        debug_assert!(words.len() > 0);
        debug_assert!(start < get_capacity(words));

        let state = get_word_index(start);

        let mut cache = unsafe { * words.get_unchecked(state) };

        let start_rune_index = get_rune_index(start);

        cache = unsafe { bit::drop_lower_at_unchecked(cache, start_rune_index) };

        return unsafe { Self::from_raw(words, state, cache) };
    }
}

impl<'a> std::iter::Iterator for Iterator<'a>
{
    type Item = usize;

    fn next(& mut self) -> Option<Self::Item>
    {
        if self.cache == 0
        {
            let mut word_index = self.state;

            let mut word;

            loop
            {
                word_index += 1;

                if word_index == self.words.len()
                {
                    return None;
                };

                word = unsafe { * self.words.get_unchecked(word_index) };

                if word != 0
                {
                    break;
                };
            };

            self.state = word_index;

            self.cache = word;
        };

        let rune_index = unsafe { bit::next_unchecked(self.cache) };

        self.cache = unsafe { bit::clip_unchecked(self.cache) };

        let index = get_index(self.state, rune_index);

        return Some(index);
    }
}

pub struct IteratorWithLimit<'a>
{
    words: &'a [usize],
    state: usize,
    cache: usize,
    limit_word_index: usize,
    limit_rune_index: usize
}

impl<'a> Default for IteratorWithLimit<'a>
{
    fn default() -> Self
    {
        return Self::empty();
    }
}

impl<'a> IteratorWithLimit<'a>
{
    unsafe fn from_raw(words: &'a [usize], state: usize, cache: usize, limit_word_index: usize, limit_rune_index: usize) -> Self
    {
        return Self { words, state, cache, limit_word_index, limit_rune_index };
    }

    pub fn empty() -> Self
    {
        let words = & [0];

        let state = words.len() - 1;

        let cache = 0;

        let limit_word_index = state;

        let limit_rune_index = 0;

        return unsafe { Self::from_raw(words, state, cache, limit_word_index, limit_rune_index) };
    }

    pub unsafe fn new(words: &'a [usize], mut limit: usize) -> Self
    {
        debug_assert!(words.len() > 0);
        debug_assert!(limit > 0);
        debug_assert!(limit <= get_capacity(words));

        limit -= 1; // inclusive

        let limit_word_index = get_word_index(limit);

        let limit_rune_index = get_rune_index(limit) + 1; // exclusive

        let state = 0;

        let mut cache = unsafe { * words.get_unchecked(0) };

        if state == limit_word_index
        {
            cache = unsafe { bit::drop_upper_at_unchecked(cache, limit_rune_index) };
        };

        return unsafe { Self::from_raw(words, state, cache, limit_word_index, limit_rune_index) };
    }

    pub unsafe fn new_with_start(words: &'a [usize], start: usize, mut limit: usize) -> Self
    {
        debug_assert!(words.len() > 0);
        debug_assert!(start < limit);
        debug_assert!(limit <= get_capacity(words));

        limit -= 1; // inclusive

        let limit_word_index = get_word_index(limit);

        let limit_rune_index = get_rune_index(limit) + 1; // exclusive

        let state = get_word_index(start);

        let mut cache = unsafe { * words.get_unchecked(state) };

        let start_rune_index = get_rune_index(start);

        cache = unsafe { bit::drop_lower_at_unchecked(cache, start_rune_index) };

        if state == limit_word_index
        {
            cache = unsafe { bit::drop_upper_at_unchecked(cache, limit_rune_index) };
        };

        return unsafe { Self::from_raw(words, state, cache, limit_word_index, limit_rune_index) };
    }
}

impl<'a> std::iter::Iterator for IteratorWithLimit<'a>
{
    type Item = usize;

    fn next(& mut self) -> Option<Self::Item>
    {
        if self.cache == 0
        {
            let mut word_index = self.state;

            let mut word;

            loop
            {
                if word_index == self.limit_word_index
                {
                    return None;
                };
                
                word_index += 1;

                word = unsafe { * self.words.get_unchecked(word_index) };

                if word_index == self.limit_word_index
                {
                    word = unsafe { bit::drop_upper_at_unchecked(word, self.limit_rune_index) };

                    if word != 0
                    {
                        break;
                    };

                    return None;
                };

                if word != 0
                {
                    break;
                };
            };

            self.state = word_index;

            self.cache = word;
        };

        let rune_index = unsafe { bit::next_unchecked(self.cache) };

        self.cache = unsafe { bit::clip_unchecked(self.cache) };

        let index = get_index(self.state, rune_index);

        return Some(index);
    }
}