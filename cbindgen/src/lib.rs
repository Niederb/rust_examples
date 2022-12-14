#[repr(C)]
pub struct MyPoint {
    x: u8, 
    y: u32,
    z: u8,
}

#[no_mangle]
pub extern "C" fn give_me_the_point() -> MyPoint {
    MyPoint { x: 5, y: 10, z: 4 }
}

#[no_mangle]
pub extern "C" fn square_all_the_things(i: i32) -> i32 {
    i * i
}

