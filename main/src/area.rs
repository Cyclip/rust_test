// Structs
// calculate the area of a rectangle
#[derive(Debug)]

// Struct Rectangle with u32 vals width & height
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement into struct Rectangle
// Multiple impl blocks are allowed
impl Rectangle {
    // Method to calculate the area
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }

    // Method to check if another rect can fit in self
    fn can_hold(&self, rect: &Rectangle) -> bool {
        (self.width >= rect.width) && (self.height >= rect.height)
    }

    // Method to scale it by a linear scale factor
    fn scale(&mut self, scale: f32) {
        // Mutable self to modify width & height values

        // Convert width & height to f32
        let mut w: f32 = self.width as f32;
        let mut h: f32 = self.height as f32;

        w *= scale;
        h *= scale;

        self.width = w as u32;
        self.height = h as u32;
    }

    // Associated function to generate a square
    // similar to String::from() or String::new()
    // it does not take self as a parameter because it doesn't
    // require an existing struct object, it's just associated
    // with the struct.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Group as tuple for readability and it makes sense
    let mut rect1 = Rectangle{
        width: 40,
        height: 70,
    };

    let rect2 = Rectangle{
        width: 20,
        height: 50,
    };

    let rect3 = Rectangle::square(50);

    rect1.scale(1.1);

    println!(
        "Area of {:?} = {}",
        rect1,
        rect1.calculate_area()
    );

    println!(
        "{:?} can hold {:?}: {}",
        rect1,
        rect2,
        rect1.can_hold(&rect2)
    );

    println!(
        "{:?} can hold {:?}: {}",
        rect1,
        rect3,
        rect1.can_hold(&rect3)
    );
}
