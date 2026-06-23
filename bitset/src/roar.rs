use std::usize;

use roaring;

use crate::like::BitSetLike;

pub struct Iterator<'a>
{
    iter: roaring::bitmap::Iter<'a>
}

impl<'a> Iterator<'a>
{
    pub fn new(iter: roaring::bitmap::Iter<'a>) -> Self 
    {
        return Self { iter };
    }
}

impl<'a> std::iter::Iterator for Iterator<'a>
{
    type Item = usize;

    fn next(& mut self) -> Option<Self::Item> 
    {
        return self.iter.next().map(|x| x as usize);
    }
}

impl BitSetLike for roaring::bitmap::RoaringBitmap
{
    fn new(count: usize, value: bool) -> Self 
    {
        let mut this = Self::new();

        if value
        {
            this.insert_range(0..count as u32);
        };

        return this;
    }

    fn words(& self) -> & [usize] 
    {
        panic!("RoaringBitmap does not expose its internal words");
    }

    fn capacity(& self) -> usize 
    {
        return u32::MAX as usize;
    }

    unsafe fn get_unchecked(& self, index: usize) -> bool 
    {
        return self.contains(index as u32);
    }

    unsafe fn insert_unchecked(& mut self, index: usize) 
    {
        self.insert(index as u32);
    }

    unsafe fn remove_unchecked(& mut self, index: usize) 
    {
        self.remove(index as u32);
    }

    unsafe fn fill_with_start_unchecked(& mut self, start: usize) 
    {
        self.insert_range(start as u32..u32::MAX);
    }

    unsafe fn drop_with_start_unchecked(& mut self, start: usize) 
    {
        self.remove_range(start as u32..u32::MAX);
    }

    unsafe fn fill_with_limit_unchecked(& mut self, limit: usize) 
    {
        self.insert_range(0..limit as u32);
    }

    unsafe fn drop_with_limit_unchecked(& mut self, limit: usize) 
    {
        self.remove_range(0..limit as u32);
    }

    unsafe fn fill_with_range_unchecked(& mut self, start: usize, limit: usize) 
    {
        self.insert_range(start as u32..limit as u32);
    }

    unsafe fn drop_with_range_unchecked(& mut self, start: usize, limit: usize) 
    {
        self.remove_range(start as u32..limit as u32);
    }

    unsafe fn next_unchecked(& self) -> usize 
    {
        return BitSetLike::iter(self).next().unwrap_or(usize::MAX);
    }

    unsafe fn next_with_start_unchecked(& self, start: usize) -> usize 
    {
        return BitSetLike::iter_with_start(self, start).next().unwrap_or(usize::MAX);
    }

    unsafe fn next_with_limit_unchecked(& self, limit: usize) -> usize 
    {
        return BitSetLike::iter_with_limit(self, limit).next().unwrap_or(usize::MAX);
    }

    unsafe fn next_with_range_unchecked(& self, start: usize, limit: usize) -> usize 
    {
        return BitSetLike::iter_with_range(self, start, limit).next().unwrap_or(usize::MAX);
    }

    type Iterator<'a> = Iterator<'a>
    where 
        Self: 'a;

    fn iter_none<'a>(&'a self) -> Self::Iterator<'a> 
    {
        return Self::Iterator::new(self.range(0..0));    
    }

    fn iter<'a>(&'a self) -> Self::Iterator<'a> 
    {
        return Self::Iterator::new(self.iter());
    }

    unsafe fn iter_with_start_unchecked<'a>(&'a self, start: usize) -> Self::Iterator<'a> 
    {
        return Self::Iterator::new(self.range(start as u32..));
    }

    type IteratorWithLimit<'a> = Self::Iterator<'a>
    where 
        Self: 'a;

    fn iter_with_limit_none<'a>(&'a self) -> Self::IteratorWithLimit<'a>
    {
        return self.iter_none();    
    }
    unsafe fn iter_with_limit_unchecked<'a>(&'a self, limit: usize) -> Self::Iterator<'a> 
    {
        return Self::Iterator::new(self.range(..limit as u32));
    }

    unsafe fn iter_with_range_unchecked<'a>(&'a self, start: usize, limit: usize) -> Self::Iterator<'a> 
    {
        return Self::Iterator::new(self.range(start as u32..limit as u32));
    }
}