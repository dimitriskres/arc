
pub trait BitSetLike
{
    fn new(count: usize, value: bool) -> Self;

    fn words(& self) -> & [usize];

    fn capacity(& self) -> usize;

    unsafe fn get_unchecked(& self, index: usize) -> bool;

    fn get(& self, index: usize) -> bool
    {
        if index >= self.capacity()
        {
            return false;
        };

        return unsafe { self.get_unchecked(index) };
    }

    unsafe fn insert_unchecked(& mut self, index: usize);

    fn insert(& mut self, index: usize)
    {
        if index >= self.capacity()
        {
            return;
        };

        unsafe { self.insert_unchecked(index) };
    }

    unsafe fn remove_unchecked(& mut self, index: usize);

    fn remove(& mut self, index: usize)
    {
        if index >= self.capacity()
        {
            return;
        };

        unsafe { self.remove_unchecked(index) };
    }

    unsafe fn fill_with_start_unchecked(& mut self, start: usize);

    fn fill_with_start(& mut self, start: usize)
    {
        if start >= self.capacity()
        {
            return;
        };

        unsafe { self.fill_with_start_unchecked(start) };
    }

    unsafe fn drop_with_start_unchecked(& mut self, start: usize);

    fn drop_with_start(& mut self, start: usize)
    {
        if start >= self.capacity()
        {
            return;
        };

        unsafe { self.drop_with_start_unchecked(start) };
    }

    unsafe fn fill_with_limit_unchecked(& mut self, limit: usize);

    fn fill_with_limit(& mut self, limit: usize)
    {
        let limit = self.capacity().min(limit);

        unsafe { self.fill_with_limit_unchecked(limit) };
    }

    unsafe fn drop_with_limit_unchecked(& mut self, limit: usize);

    fn drop_with_limit(& mut self, limit: usize)
    {
        let limit = self.capacity().min(limit);

        unsafe { self.drop_with_limit_unchecked(limit) };
    }

    unsafe fn fill_with_range_unchecked(& mut self, start: usize, limit: usize);

    fn fill_with_range(& mut self, start: usize, limit: usize)
    {
        let capacity = self.capacity();

        let limit = limit.min(capacity);

        let start = start.min(limit);

        if start == limit
        {
            return;
        };

        unsafe { self.fill_with_range_unchecked(start, limit) };
    }

    unsafe fn drop_with_range_unchecked(& mut self, start: usize, limit: usize);

    fn drop_with_range(& mut self, start: usize, limit: usize)
    {
        let capacity = self.capacity();

        let limit = limit.min(capacity);

        let start = start.min(limit);

        if start == limit
        {
            return;
        };

        unsafe { self.drop_with_range_unchecked(start, limit) };
    }

    unsafe fn next_unchecked(& self) -> usize;

    fn next(& self) -> Option<usize>
    {
        let index = unsafe { self.next_unchecked() };

        if index == usize::MAX
        {
            return None;
        };

        return Some(index);
    }

    unsafe fn next_with_start_unchecked(& self, start: usize) -> usize;

    fn next_with_start(& self, start: usize) -> Option<usize>
    {
        if start >= self.capacity()
        {
            return None;
        };

        let index = unsafe { self.next_with_start_unchecked(start) };

        if index == usize::MAX
        {
            return None;
        };

        return Some(index);
    }

    unsafe fn next_with_limit_unchecked(& self, limit: usize) -> usize;

    fn next_with_limit(& self, limit: usize) -> Option<usize>
    {
        let limit = self.capacity().min(limit);

        if limit == 0
        {
            return None;
        };

        let index = unsafe { self.next_with_limit_unchecked(limit) };

        if index == usize::MAX
        {
            return None;
        };

        return Some(index);
    }

    unsafe fn next_with_range_unchecked(& self, start: usize, limit: usize) -> usize;

    fn next_with_range(& self, start: usize, limit: usize) -> Option<usize>
    {
        let capacity = self.capacity();

        let limit = limit.min(capacity);

        let start = start.min(limit);

        if start == limit
        {
            return None;
        };

        let index = unsafe { self.next_with_range_unchecked(start, limit) };

        if index == usize::MAX
        {
            return None;
        };

        return Some(index);
    }

    type Iterator<'a>: Iterator<Item = usize>
    where 
        Self: 'a;

    fn iter_none<'a>(&'a self) -> Self::Iterator<'a>;

    fn iter<'a>(&'a self) -> Self::Iterator<'a>;

    unsafe fn iter_with_start_unchecked<'a>(&'a self, start: usize) -> Self::Iterator<'a>;

    fn iter_with_start<'a>(&'a self, start: usize) -> Self::Iterator<'a>
    {
        if start >= self.capacity()
        {
            return self.iter_none();
        };

        return unsafe { self.iter_with_start_unchecked(start) };    
    }

    type IteratorWithLimit<'a>: Iterator<Item = usize>
    where 
        Self: 'a;

    fn iter_with_limit_none<'a>(&'a self) -> Self::IteratorWithLimit<'a>;

    unsafe fn iter_with_limit_unchecked<'a>(&'a self, limit: usize) -> Self::IteratorWithLimit<'a>;

    fn iter_with_limit<'a>(&'a self, limit: usize) -> Self::IteratorWithLimit<'a>
    {
        let limit = self.capacity().min(limit);

        if limit == 0
        {
            return self.iter_with_limit_none();
        };

        return unsafe { self.iter_with_limit_unchecked(limit) };
    }

    unsafe fn iter_with_range_unchecked<'a>(&'a self, start: usize, limit: usize) -> Self::IteratorWithLimit<'a>;

    fn iter_with_range<'a>(&'a self, start: usize, limit: usize) -> Self::IteratorWithLimit<'a>
    {
        let limit = self.capacity().min(limit);

        if limit == 0
        {
            return self.iter_with_limit_none();
        };

        let start = start.min(limit);

        if start == limit
        {
            return self.iter_with_limit_none();
        };

        return unsafe { self.iter_with_range_unchecked(start, limit) };    
    }
}