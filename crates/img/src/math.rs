pub fn idx_to_xy(idx: usize, width: usize) -> (usize, usize) {
    (idx % width, idx / width)
}

pub fn xy_to_idx(xy: (usize, usize), width: usize) -> usize {
    let (x, y) = xy;
    y * width + x
}
