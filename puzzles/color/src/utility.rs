
pub fn get_edges<Rng>(rng: & mut Rng, size: arc::codec::Scalar, density: f64) -> Box<[Box<[arc::codec::Scalar]>]>
where 
    Rng: rand::Rng,
{
    let mut edges = vec![vec![]; size as usize];

    for i in 0..size
    {
        for j in (i + 1)..size
        {
            if rand::RngExt::random::<f64>(rng) < density
            {
                edges[i as usize].push(j);
                edges[j as usize].push(i);
            };
        };
    };

    return edges.into_iter().map(Vec::into_boxed_slice).collect()
}