
pub type Scalar = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Object<X, Y>
{
    pub x: X,
    pub y: Y
}

impl<X, Y> crate::model::Tiny for Object<X, Y>
where
    X: crate::model::Tiny,
    Y: crate::model::Tiny
{

}

pub trait CodecLike
{
    type X;
    type Y;
    type I;

    fn encode(& self, x: Self::X, y: Self::Y) -> Self::I;

    fn decode_to_x(& self, i: Self::I) -> Self::X;

    fn decode_to_y(& self, i: Self::I) -> Self::Y;

    fn decode(& self, i: Self::I) -> (Self::X, Self::Y);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObjectCodec<X, Y>
{
    phantom: std::marker::PhantomData<(X, Y)>
}

impl<X, Y> ObjectCodec<X, Y>
{
    pub fn new() -> Self
    {
        return Self { phantom: std::marker::PhantomData };
    }
}

impl<X, Y> CodecLike for ObjectCodec<X, Y>
{
    type X = X;
    type Y = Y;
    type I = Object<Self::X, Self::Y>;

    fn encode(& self, x: Self::X, y: Self::Y) -> Self::I
    {
        return Object { x, y };
    }

    fn decode_to_x(& self, i: Self::I) -> Self::X
    {
        return i.x;
    }

    fn decode_to_y(& self, i: Self::I) -> Self::Y
    {
        return i.y;
    }

    fn decode(& self, i: Self::I) -> (Self::X, Self::Y)
    {
        return (i.x, i.y);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UniformCodec
{
    c: Scalar,
    d: fastdiv::PrecomputedDivU32
}

impl UniformCodec
{
    pub fn new(c: Scalar) -> Self
    {
        let d = fastdiv::FastDiv::precompute_div(c);

        return Self { c, d };
    }

    pub fn c(& self) -> Scalar
    {
        return self.c;
    }
}

impl CodecLike for UniformCodec
{
    type X = Scalar;
    type Y = Scalar;
    type I = Scalar;

    fn encode(& self, x: Self::X, y: Self::Y) -> Self::I
    {
        return x * self.c + y;
    }

    fn decode_to_x(& self, i: Self::I) -> Self::X
    {
        return fastdiv::FastDiv::fast_div(i, self.d);
    }

    fn decode_to_y(& self, i: Self::I) -> Self::Y
    {
        return fastdiv::FastDiv::fast_mod(i, self.d, self.c);
    }

    fn decode(& self, i: Self::I) -> (Self::X, Self::Y)
    {
        return (self.decode_to_x(i), self.decode_to_y(i));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SparseCodec
{
    offsets: Box<[Scalar]>,
    indexes: Box<[Scalar]>,
}

impl SparseCodec
{
    pub fn new<F>(len: usize, get: F) -> Self
    where 
        F: Fn(usize) -> usize
    {
        let mut offsets = Vec::with_capacity(len + 1);

        let mut indexes = Vec::new();

        let mut offset = 0;

        for x in 0..len
        {
            offsets.push(offset);

            let count = get(x);

            indexes.extend(std::iter::repeat(x as Scalar).take(count));

            offset += count as Scalar;
        };

        offsets.push(offset);

        let offsets = offsets.into_boxed_slice();

        let indexes = indexes.into_boxed_slice();

        return Self { offsets, indexes };
    }

    pub fn count_x(& self) -> usize
    {
        return self.offsets.len() - 1;
    }

    pub fn count_y(& self, x: Scalar) -> usize
    {
        let x = x as usize;

        return self.offsets[x + 1] as usize - self.offsets[x] as usize;
    }

    pub fn count_i(& self) -> usize
    {
        return self.indexes.len();
    }
}

impl CodecLike for SparseCodec
{
    type X = Scalar;
    type Y = Scalar;
    type I = Scalar;

    fn encode(& self, x: Self::X, y: Self::Y) -> Self::I
    {
        return self.offsets[x as usize] + y;
    }

    fn decode_to_x(& self, i: Self::I) -> Self::X
    {
        return self.indexes[i as usize];
    }

    fn decode_to_y(& self, i: Self::I) -> Self::Y
    {
        let x = self.decode_to_x(i);

        return i - self.offsets[x as usize];
    }

    fn decode(& self, i: Self::I) -> (Self::X, Self::Y)
    {
        let x = self.decode_to_x(i);

        return (x, i - self.offsets[x as usize]);
    }
}