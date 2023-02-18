struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle{
        Rectangle {
             width: size,
             height: size
        }
    }
}

fn main() {
    let mut user1 = User{
        email: String::from("bogdan@mail.com"),
        username: String::from("bogdan123"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("walace123");

    let user2 = build_user(
        String::from("kyle@mail.com"),
        String::from("kyle123") 
    );

    let user3 = User{
        email: String::from("james@mail.com"),
        username: String::from("james123"),
        ..user2
    };

    //Tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    //Calculate area without struct
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area1(width1, height1)
    );

    //Calculate area with tuple
    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area2(rect)
    );

    //Calculate area with struct
    let rect = Rectangle{
        width: 30,
        height: 50
    };
    println!(
        "The area of the rectangle is {} square pixels",
        area3(&rect)
    );
    println!("rect: {:#?}", rect);

    //Methods
    println!(
        "The area of the rectangle is {} square pixels",
        rect.area()
    );

    //Methods with parameters
    let rect = Rectangle{
        width: 30,
        height: 50
    };

    let rect1 = Rectangle{
        width: 20,
        height: 40
    };

    let rect2 = Rectangle{
        width: 40,
        height: 50
    };

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    //Associative functions
    let rect3 = Rectangle::square(25);


}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area1(width: u32, height: u32) -> u32{
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}

fn area3(rect: &Rectangle) -> u32{
    rect.width * rect.height
}