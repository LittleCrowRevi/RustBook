use crate::utilities::input_util;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        let area_self = self.area();
        let area_rect2 = rect2.area();
        area_rect2 < area_self
    }
}

pub fn structs() {
    let rect1 = Rectangle {
        width: input_util::read_num_input(String::from("Input Rectangle width! "),
                                          String::from("Input a number!")) as u32,
        height: input_util::read_num_input(String::from("Input Rectangle height! "),
                                           String::from("Input a number!")) as u32,
    };
    dbg!(&rect1);

    println!("The area of your rectangle is {} square pixels.", rect1.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}