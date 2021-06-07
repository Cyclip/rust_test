#![allow(unused)]
// Struct test

struct Object3D {
    id: String,
    position: [i32; 3],
    size: [i32; 3],
    color: [u8; 3]
}

fn new_object(id: &str, position: [i32; 3], size: [i32; 3], color: [u8; 3]) -> Object3D {
    Object3D {
        id: String::from(id),
        position,
        size,
        color
    }
}

fn main() {
    let mut table = new_object(
        "table",
        [0, 0, 0],
        [3, 3, 3],
        [250, 0, 51]
    );
}
