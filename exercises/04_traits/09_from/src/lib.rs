// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

// impl WrappingU32
// {
//     fn from(num : u32) -> WrappingU32
//     {
//         WrappingU32 { value: num }
//     }
// }

impl From<u32> for WrappingU32 // tell rust how to convert u32 to WrappingU32 and the reverse convert
{
    fn from(num : u32) -> WrappingU32
    {
        WrappingU32 { value: num }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
