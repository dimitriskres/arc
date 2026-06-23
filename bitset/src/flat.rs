use std::fmt::Debug;

use crate::{bit, flat_raw, like::BitSetLike};

#[derive(Clone)]
pub struct BitSet
{
    words: Box<[usize]>
}

impl Debug for BitSet
{
    fn fmt(& self, f: & mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        return f.debug_set().entries(self.iter()).finish();
    }
}

impl BitSet
{
    pub unsafe fn from_raw(words: Box<[usize]>) -> Self
    {
        return Self { words };
    }

    pub unsafe fn new_unchecked(count: usize, value: bool) -> Self
    {
        debug_assert!(count > 0);

        let limit = count - 1;

        let word_count = flat_raw::get_word_index(limit) + 1;

        let fill = if value { usize::MAX } else { 0 };

        let mut words = vec![fill; word_count].into_boxed_slice();

        if value
        {
            let limit_word_index = flat_raw::get_word_index(limit);

            let word = unsafe { words.get_unchecked_mut(limit_word_index) };

            let limit_rune_index = flat_raw::get_rune_index(limit) + 1;

            * word = unsafe { bit::drop_upper_at_unchecked(* word, limit_rune_index) };
        };

        return unsafe { Self::from_raw(words) };
    }
}

impl BitSetLike for BitSet
{
    fn new(count: usize, value: bool) -> Self
    {
        assert!(count > 0);

        return unsafe { Self::new_unchecked(count, value) };
    }

    fn words(& self) -> & [usize] 
    {
        return & self.words;
    }

    fn capacity(& self) -> usize 
    {
        return flat_raw::get_capacity(& self.words);    
    }

    unsafe fn get_unchecked(& self, index: usize) -> bool 
    {
        return unsafe { flat_raw::get_unchecked(& self.words, index) };
    }

    unsafe fn insert_unchecked(& mut self, index: usize) 
    {
        unsafe { flat_raw::insert_unchecked(& mut self.words, index) };
    }

    unsafe fn remove_unchecked(& mut self, index: usize) 
    {
        unsafe { flat_raw::remove_unchecked(& mut self.words, index) };
    }

    unsafe fn fill_with_start_unchecked(& mut self, start: usize) 
    {
        unsafe { flat_raw::fill_with_start_unchecked(& mut self.words, start) };
    }

    unsafe fn drop_with_start_unchecked(& mut self, start: usize) 
    {
        unsafe { flat_raw::drop_with_start_unchecked(& mut self.words, start) };
    }

    unsafe fn fill_with_limit_unchecked(& mut self, limit: usize) 
    {
        unsafe { flat_raw::fill_with_limit_unchecked(& mut self.words, limit) };
    }

    unsafe fn drop_with_limit_unchecked(& mut self, limit: usize) 
    {
        unsafe { flat_raw::drop_with_limit_unchecked(& mut self.words, limit) };
    }

    unsafe fn fill_with_range_unchecked(& mut self, start: usize, limit: usize) 
    {
        unsafe { flat_raw::fill_with_range_unchecked(& mut self.words, start, limit) };
    }

    unsafe fn drop_with_range_unchecked(& mut self, start: usize, limit: usize) 
    {
        unsafe { flat_raw::drop_with_range_unchecked(& mut self.words, start, limit) };
    }

    unsafe fn next_unchecked(& self) -> usize
    {
        return unsafe { flat_raw::next_unchecked(& self.words) };
    }

    unsafe fn next_with_start_unchecked(& self, start: usize) -> usize
    {
        return unsafe { flat_raw::next_with_start_unchecked(& self.words, start) };
    }

    unsafe fn next_with_limit_unchecked(& self, limit: usize) -> usize
    {
        return unsafe { flat_raw::next_with_limit_unchecked(& self.words, limit) };
    }

    unsafe fn next_with_range_unchecked(& self, start: usize, limit: usize) -> usize
    {
        return unsafe { flat_raw::next_with_range_unchecked(& self.words, start, limit) };
    }

    type Iterator<'a> = flat_raw::Iterator<'a>;

    fn iter_none<'a>(&'a self) -> Self::Iterator<'a> 
    {
        return Self::Iterator::empty();
    }

    fn iter<'a>(&'a self) -> Self::Iterator<'a>
    {
        return unsafe { Self::Iterator::new(& self.words) };
    }

    unsafe fn iter_with_start_unchecked<'a>(&'a self, start: usize) -> Self::Iterator<'a>
    {
        return unsafe { Self::Iterator::new_with_start(& self.words, start) };    
    }

    type IteratorWithLimit<'a> = flat_raw::IteratorWithLimit<'a>;

    fn iter_with_limit_none<'a>(&'a self) -> Self::IteratorWithLimit<'a> 
    {
        return Self::IteratorWithLimit::empty();
    }

    unsafe fn iter_with_limit_unchecked<'a>(&'a self, limit: usize) -> Self::IteratorWithLimit<'a>
    {
        return unsafe { Self::IteratorWithLimit::new(& self.words, limit) };
    }

    unsafe fn iter_with_range_unchecked<'a>(&'a self, start: usize, limit: usize) -> Self::IteratorWithLimit<'a>
    {
        return unsafe { Self::IteratorWithLimit::new_with_start(& self.words, start, limit) };    
    }
}