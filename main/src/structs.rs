#![allow(unused)]
// Struct test

struct Color(u8, u8, u8);

struct Object3D {
    id: String,
    position: [i32; 3],
    size: [i32; 3],
    color: Color
}

fn new_object(id: &str, position: [i32; 3], size: [i32; 3], color: Color) -> Object3D {
    Object3D {
        id: String::from(id),
        position,
        size,
        color
    }
}

fn main() {
    let mut obj1 = new_object(
        "obj1",
        [0, 0, 0],
        [3, 3, 3],
        Color(250, 0, 51)
    );

    let mut obj2 = Object3D {
        id: String::from("obj2"),
        position: [10, 0, 0],
        ..obj1
    };

    obj1.position = [1, 0, 0];

    println!("Position obj1: {:?}", obj1.position);
    println!("Position obj2: {:?}", obj2.position);
}
