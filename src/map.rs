struct Map<T> {
    hash: fn (T) -> u64,
    cmp: fn (T, T) -> bool,
    data: Vec<u64>,
}


struct Table<T> {
    rows: Vec<T>,
    maps: Vec<Map<T>>,
}
