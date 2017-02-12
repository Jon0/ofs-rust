extern crate fuse;
extern crate nanomsg;


struct Map<T> {
    hash: fn (T) -> u64,
    cmp: fn (T, T) -> bool,
    data: Vec<u64>,
}


struct Table<T> {
    rows: Vec<T>,
    maps: Vec<Map<T>>,
}


fn main() {
    let table: Table<i64>;
}
