fn runner() {
    let width1 = 78;
    let height1 = 589;

    println!(
        "The area of the rectangle is {} squared pixels.",
        area(width1, height1)
    );

    let rect1 = (width1, height1);

    println!(
        "The area when using a tuple is {} squared pixels.",
        tuple_area(rect1)
    );

    let rect2 = Rectangle {
        width: width1,
        height: height1,
    };

    println!(
        "The area when using a rectangle struct is {} squared pixels.",
        rect_area(&rect2)
    );

    // debug a struct
    println!("Here is our rectangle \n {:#?}", rect2);

    println!(
        "The area when using a struct method is {} squared pixels.",
        rect2.area()
    );

    let width2 = 678;
    let height2 = 44;

    let rect3 = Rectangle {
        height: height2,
        width: width2,
    };

    println!(
        "Rectangle {:?} can hold {:?}: {}",
        rect2,
        rect3,
        rect2.can_hold(&rect3)
    );

    let squ = Rectangle::square(height2);

    println!(
        "The square is of size: {} and area: {}",
        height2,
        squ.area()
    );

    //create an instance of a struct
    let mut user1 = User {
        email: String::from("me@me.com"),
        username: String::from("superbike_z"),
        active: true,
        sign_in_count: 1,
    };

    // access instance variables
    println!(
        "Uname, email, active, sign_count: {} {} {} {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    // update mutable instance variables using dot notation
    user1.sign_in_count = 2;

    println!("Updated sign in count: {}", user1.sign_in_count);

    let user2 = build_user(String::from("her@her.com"), String::from("dino_c"));

    println!("User2 name is: {}", user2.username);

    //update user2 with new instance
    let updated_user2 = User {
        active: false,
        sign_in_count: { user2.sign_in_count + 1 },
        // use .. syntax to set variables to the values from the other struct
        ..user2
    };

    println!(
        "Update user2 values active: {}, sign_in_count: {}",
        updated_user2.active, updated_user2.sign_in_count
    );

    let black = Col(0, 0, 0);

    println!("Black is R: {}, G: {}, B: {}", black.0, black.1, black.2);

    let origin = Point(0, 0, 0);

    println!(
        "Origins is point x: {}, y: {}, z: {}",
        origin.0, origin.1, origin.2
    );
}
// sample struct for a user type
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        // init shorthand when variables and fields have the same name
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

//tuple structs

struct Col(i32, i32, i32);
struct Point(i32, i32, i32);

// area calculator
//
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rect_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // a single variable method using the self type
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // a multivariable struct method
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    // an associated function commonly used as a constructor
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
