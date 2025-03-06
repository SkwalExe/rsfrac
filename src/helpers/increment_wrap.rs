use num_traits::{NumOps, One};

pub(crate) fn decrement_wrap<N>(num: &mut N, wrap_around: N)
where
    N: NumOps + Copy + One,
{
    *num = (*num + wrap_around - N::one()) % wrap_around;
}

pub(crate) fn increment_wrap<N>(num: &mut N, wrap_around: N)
where
    N: NumOps + Copy + One,
{
    *num = (*num + N::one()) % wrap_around;
}
