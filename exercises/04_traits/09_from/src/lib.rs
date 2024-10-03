// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

impl From<i32> for WrappingU32 {
    fn from(val: i32) -> Self {
        WrappingU32 { value: u32::try_from(val).expect("Can't be negative") }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
