use std::fmt::Debug;

use crate::{bit, flat_raw, like::BitSetLike, meta_raw};

const METAS: usize = 8;

unsafe fn drop_padding(metas: & [usize], words: & mut [usize])
{
    let mut index = meta_raw::get_capacity(metas, words) - 1;

    let mut level = 0;

    loop
    {
        let meta = * unsafe { metas.get_unchecked(level) };

        let word_index = flat_raw::get_word_index(index);

        let rune_index = flat_raw::get_rune_index(index) + 1;
        
        let word = unsafe { words.get_unchecked_mut(meta + word_index) };

        * word = unsafe { bit::drop_upper_at_unchecked(* word, rune_index) };

        level += 1;

        if level == metas.len()
        {
            break;
        };

        index = word_index;
    };
}

#[derive(Clone)]
pub struct BitSet
{
    metas: [usize; METAS],
    words: Box<[usize]>,
    depth: usize
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
    pub unsafe fn from_raw(metas: [usize; METAS], words: Box<[usize]>, depth: usize) -> Self
    {
        return Self { metas, words, depth };
    }

    pub unsafe fn new_with_depth_unchecked(count: usize, value: bool, depth: usize) -> Self
    {
        debug_assert!(count > 0);
        debug_assert!(depth > 0);
        debug_assert!(depth <= METAS);

        let mut metas = [0; METAS];

        let mut word_count = count;

        let mut word_count_total = 0;

        let mut level = 0;

        loop
        {
            level += 1;

            word_count = flat_raw::get_word_index(word_count + bit::SIZE - 1);

            word_count_total += word_count;

            metas[level] = word_count_total;

            if level == depth
            {
                break;
            };
        };

        let fill = if value { usize::MAX } else { 0 };

        let mut words = vec![fill; word_count_total].into_boxed_slice();

        if value
        {
            if count < meta_raw::get_capacity(& metas[..depth], & words)
            {
                unsafe { meta_raw::drop_with_start_unchecked(& metas[..depth], & mut words, count) };
            };

            unsafe { drop_padding(& metas[..depth], & mut words) };
        };

        return Self { metas, words, depth };
    }

    pub fn new_with_depth(count: usize, value: bool, depth: usize) -> Self
    {
        assert!(count > 0, "cannot create a meta bitset with no words");
        assert!(depth > 0, "cannot create a meta bitset with no depth");
        assert!(depth <= METAS, "cannot create a meta bitset with more than {} levels", METAS);
        
        return unsafe { Self::new_with_depth_unchecked(count, value, depth) };
    }

    pub fn new_with_limit(count: usize, value: bool, limit: usize) -> Self
    {
        debug_assert!(limit > 0);

        let mut depth = 0;

        let mut word_count = count;

        loop
        {
            depth += 1;

            word_count = (word_count + bit::SIZE - 1) / bit::SIZE;

            if depth == METAS || word_count <= limit
            {
                break;
            };
        };

        return Self::new_with_depth(count, value, depth);
    }

    pub fn depth(&self) -> usize
    {
        return self.depth;
    }
}

impl BitSetLike for BitSet
{
    fn new(count: usize, value: bool) -> Self
    {
        return Self::new_with_limit(count, value, 256);
    }

    fn words(& self) -> & [usize] 
    {
        return & self.words[..self.metas[1]];
    }

    fn capacity(& self) -> usize 
    {
        return meta_raw::get_capacity(& self.metas[..self.depth], & self.words);
    }

    unsafe fn get_unchecked(& self, index: usize) -> bool 
    {
        return unsafe { meta_raw::get_unchecked(& self.metas[..self.depth], & self.words, index) };
    }

    unsafe fn insert_unchecked(& mut self, index: usize) 
    {
        unsafe { meta_raw::insert_unchecked(& self.metas[..self.depth], & mut self.words, index) };
    }

    unsafe fn remove_unchecked(& mut self, index: usize) 
    {
        unsafe { meta_raw::remove_unchecked(& self.metas[..self.depth], & mut self.words, index) };
    }

    unsafe fn fill_with_start_unchecked(& mut self, start: usize) 
    {
        unsafe { meta_raw::fill_with_start_unchecked(& self.metas[..self.depth], & mut self.words, start) };
    }

    unsafe fn drop_with_start_unchecked(& mut self, start: usize) 
    {
        unsafe { meta_raw::drop_with_start_unchecked(& self.metas[..self.depth], & mut self.words, start) };
    }

    unsafe fn fill_with_limit_unchecked(& mut self, limit: usize) 
    {
        unsafe { meta_raw::fill_with_limit_unchecked(& self.metas[..self.depth], & mut self.words, limit) };
    }

    unsafe fn drop_with_limit_unchecked(& mut self, limit: usize) 
    {
        unsafe { meta_raw::drop_with_limit_unchecked(& self.metas[..self.depth], & mut self.words, limit) };
    }

    unsafe fn fill_with_range_unchecked(& mut self, start: usize, limit: usize) 
    {
        unsafe { meta_raw::fill_with_range_unchecked(& self.metas[..self.depth], & mut self.words, start, limit) };
    }

    unsafe fn drop_with_range_unchecked(& mut self, start: usize, limit: usize) 
    {
        unsafe { meta_raw::drop_with_range_unchecked(& self.metas[..self.depth], & mut self.words, start, limit) };
    }

    unsafe fn next_unchecked(& self) -> usize
    {
        return unsafe { meta_raw::next_unchecked(& self.metas[..self.depth], & self.words) };
    }

    unsafe fn next_with_start_unchecked(& self, start: usize) -> usize
    {
        return unsafe { meta_raw::next_with_start_unchecked(& self.metas[..self.depth], & self.words, start) };
    }

    unsafe fn next_with_limit_unchecked(& self, limit: usize) -> usize
    {
        return unsafe { meta_raw::next_with_limit_unchecked(& self.metas[..self.depth], & self.words, limit) };
    }

    unsafe fn next_with_range_unchecked(& self, start: usize, limit: usize) -> usize
    {
        return unsafe { meta_raw::next_with_range_unchecked(& self.metas[..self.depth], & self.words, start, limit) };
    }

    type Iterator<'a> = meta_raw::Iterator<'a, METAS>;

    fn iter_none<'a>(&'a self) -> Self::Iterator<'a> 
    {
        return Self::Iterator::empty();
    }

    fn iter<'a>(&'a self) -> Self::Iterator<'a>
    {
        return unsafe { Self::Iterator::new_unchecked(& self.metas[..self.depth], & self.words) };
    }

    unsafe fn iter_with_start_unchecked<'a>(&'a self, start: usize) -> Self::Iterator<'a>
    {
        return unsafe { Self::Iterator::new_with_start_unchecked(& self.metas[..self.depth], & self.words, start) };
    }

    type IteratorWithLimit<'a> = meta_raw::IteratorWithLimit<'a, METAS>;

    fn iter_with_limit_none<'a>(&'a self) -> Self::IteratorWithLimit<'a> 
    {
        return Self::IteratorWithLimit::empty();
    }

    unsafe fn iter_with_limit_unchecked<'a>(&'a self, limit: usize) -> Self::IteratorWithLimit<'a>
    {
        return unsafe { Self::IteratorWithLimit::new_unchecked(& self.metas[..self.depth], & self.words, limit) };
    }

    unsafe fn iter_with_range_unchecked<'a>(&'a self, start: usize, limit: usize) -> Self::IteratorWithLimit<'a>
    {
        return unsafe { Self::IteratorWithLimit::new_with_range_unchecked(& self.metas[..self.depth], & self.words, start, limit) };
    }
}