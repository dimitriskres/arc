
pub const SIZE: usize = usize::BITS as usize;

pub const FULL: usize = usize::MAX;

pub const ZERO: usize = usize::MIN;

pub const MOVE: usize = SIZE.trailing_zeros() as usize;

pub const unsafe fn get_mask_unchecked(index: usize) -> usize
{
    debug_assert!(index < SIZE);

    return 1 << index;
}

pub const unsafe fn get_unchecked(word: usize, index: usize) -> bool
{
    return word & unsafe { get_mask_unchecked(index) } != 0;
}

pub const unsafe fn insert_unchecked(word: usize, index: usize) -> usize
{
    return word | unsafe { get_mask_unchecked(index) };
}

pub const unsafe fn remove_unchecked(word: usize, index: usize) -> usize
{
    return word & ! unsafe { get_mask_unchecked(index) };
}

// lower

pub const unsafe fn get_lower_mask_unchecked(count: usize) -> usize
{
    debug_assert!(count < SIZE);

    return ! (FULL << count);
}

// lower fill

pub const unsafe fn fill_lower_unchecked(word: usize, count: usize) -> usize
{
    return word | unsafe { get_lower_mask_unchecked(count) };
}

pub const unsafe fn fill_lower_at_unchecked(word: usize, index: usize) -> usize
{
    let count = index;

    return unsafe { fill_lower_unchecked(word, count) };
}

pub const unsafe fn fill_outside_lower_unchecked(word: usize, count: usize) -> usize
{
    return word | ! unsafe { get_lower_mask_unchecked(count) };
}

pub const unsafe fn fill_outside_lower_at_unchecked(word: usize, index: usize) -> usize
{
    let count = index;

    return unsafe { fill_outside_lower_unchecked(word, count) };
}

// lower drop

pub const unsafe fn drop_lower_unchecked(word: usize, count: usize) -> usize
{
    return word & ! unsafe { get_lower_mask_unchecked(count) };
}

pub const unsafe fn drop_lower_at_unchecked(word: usize, index: usize) -> usize
{
    let count = index;

    return unsafe { drop_lower_unchecked(word, count) };
}

pub const unsafe fn drop_outside_lower_unchecked(word: usize, count: usize) -> usize
{
    return word & unsafe { get_lower_mask_unchecked(count) };
}

pub const unsafe fn drop_outside_lower_at_unchecked(word: usize, index: usize) -> usize
{
    let count = index;

    return unsafe { drop_outside_lower_unchecked(word, count) };
}

// upper

pub const unsafe fn get_upper_mask_unchecked(count: usize) -> usize
{
    debug_assert!(count < SIZE);

    return ! (FULL >> count);
}

// upper fill

pub const unsafe fn fill_upper_unchecked(word: usize, count: usize) -> usize
{
    return word | unsafe { get_upper_mask_unchecked(count) };
}

pub const unsafe fn fill_upper_at_unchecked(word: usize, index: usize) -> usize
{
    debug_assert!(index <= SIZE);

    let count = SIZE - index;

    return unsafe { fill_upper_unchecked(word, count) };
}

pub const unsafe fn fill_outside_upper_unchecked(word: usize, count: usize) -> usize
{
    return word | ! unsafe { get_upper_mask_unchecked(count) };
}

pub const unsafe fn fill_outside_upper_at_unchecked(word: usize, index: usize) -> usize
{
    debug_assert!(index <= SIZE);

    let count = SIZE - index;

    return unsafe { fill_outside_upper_unchecked(word, count) };
}

// upper drop

pub const unsafe fn drop_upper_unchecked(word: usize, count: usize) -> usize
{
    return word & ! unsafe { get_upper_mask_unchecked(count) };
}

pub const unsafe fn drop_upper_at_unchecked(word: usize, index: usize) -> usize
{
    debug_assert!(index <= SIZE);

    let count = SIZE - index;

    return unsafe { drop_upper_unchecked(word, count) };
}

pub const unsafe fn drop_outside_upper_unchecked(word: usize, count: usize) -> usize
{
    return word & unsafe { get_upper_mask_unchecked(count) };
}

pub const unsafe fn drop_outside_upper_at_unchecked(word: usize, index: usize) -> usize
{
    debug_assert!(index <= SIZE);

    let count = SIZE - index;

    return unsafe { drop_outside_upper_unchecked(word, count) };
}

// range

pub const unsafe fn get_range_mask_unchecked(start: usize, limit: usize) -> usize
{
    debug_assert!(limit <= SIZE);
    debug_assert!(start < limit);

    let lower_count = start;

    let upper_count = SIZE - limit;

    return (FULL << lower_count) & (FULL >> upper_count);
}

// range fill

pub const unsafe fn fill_range_unchecked(word: usize, start: usize, limit: usize) -> usize
{
    return word | unsafe { get_range_mask_unchecked(start, limit) };
}

pub const unsafe fn fill_outside_range_unchecked(word: usize, start: usize, limit: usize) -> usize
{
    return word | ! unsafe { get_range_mask_unchecked(start, limit) };
}

// range drop

pub const unsafe fn drop_range_unchecked(word: usize, start: usize, limit: usize) -> usize
{
    return word & ! unsafe { get_range_mask_unchecked(start, limit) };
}

pub const unsafe fn drop_outside_range_unchecked(word: usize, start: usize, limit: usize) -> usize
{
    return word & unsafe { get_range_mask_unchecked(start, limit) };
}

// next

pub const unsafe fn next_unchecked(word: usize) -> usize
{
    debug_assert!(word != 0);

    return word.trailing_zeros() as usize;
}

pub const unsafe fn clip_unchecked(word: usize) -> usize
{
    debug_assert!(word != ZERO);

    return word & (word - 1);
}