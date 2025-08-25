use std::num::NonZeroUsize;

pub(crate) fn idx_to_xy(idx: usize, width: NonZeroUsize) -> (usize, usize) {
    (idx % width, idx / width)
}

pub(crate) fn xy_to_idx(xy: (usize, usize), width: NonZeroUsize) -> usize {
    let (x, y) = xy;
    let width: usize = width.into();
    y * width + x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn idx_to_xy_basic() {
        assert_eq!(idx_to_xy(0, 1.try_into().unwrap()), (0, 0));
        assert_eq!(idx_to_xy(0, 100.try_into().unwrap()), (0, 0));
        assert_eq!(idx_to_xy(10, 100.try_into().unwrap()), (10, 0));
        assert_eq!(idx_to_xy(10, 1.try_into().unwrap()), (0, 9));
        assert_eq!(idx_to_xy(10, 3.try_into().unwrap()), (1, 2));
        assert_eq!(idx_to_xy(10, 10.try_into().unwrap()), (0, 1));
    }

    #[test]
    fn xy_to_idx_basic() {
        assert_eq!(xy_to_idx((0, 0), 1.try_into().unwrap()), 0);
        assert_eq!(xy_to_idx((0, 0), 100.try_into().unwrap()), 0);
        assert_eq!(xy_to_idx((10, 0), 100.try_into().unwrap()), 10);
        assert_eq!(xy_to_idx((0, 9), 1.try_into().unwrap()), 10);
        assert_eq!(xy_to_idx((1, 2), 3.try_into().unwrap()), 10);
        assert_eq!(xy_to_idx((0, 1), 10.try_into().unwrap()), 10);
    }
}
