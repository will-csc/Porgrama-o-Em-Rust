fn main() {
    struct SomeData (
        i32,
        f32,
        char,
        (u8,u8,u8,u8,u8),
    );
    let data = SomeData (
        10_000_000,
        183.19,
        'Q',
        (9, 0, 250, 60, 200),
    );
    print!("{}, {}, {}, {}",
    data.2, data.0, data.1, data.3.2);
}