This code contains a lot of information, so let's go over it line by line. To obtain the user input and print the result as output, we need to bring io input/output library into scope. The io library comes from the standard library, known as std.
use std::io;
By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude (khúc dạo đầu), and you can see everything in it in the standard library documentation.

If a type you want to use isn't in the prelude, you have to bring that type into scope explicitly with a use statement. Using the std::io library provides you a number of usefull features, including the ability to accept user input

If a type you want isn't in the prelude. You have to bring this type into scope explicitly with a use statement. Using std::io library provides you a number of useful features, including the ability to accept user input.
As you saw in the chapter 1, the main function is the entry point into the program:
fn main() {

The fn syntax declares a new function, the parentheses, (), indicate there are no parameters; and the curly bracket, {, starts the body of the function.

As you also learned in Chaper 1, println! is a macro that prints a string to the screen:

> println!("Guess sthe number!");
> println!("Please input your guess.");

This code is printing a prompt stating what the game is and requesting input from the user.

### Storing Values with Variables

Next, we'll create a variable too store the user input, like this:

> let mut guess = String::new();

Now the program is getting interesting! There's a lot going on in this little line. We use the let statement to craete the variable. Here's another example:

> let apples = 5;

This line created a variable named apples and binds it to the value 5. In Rust, variables are immutable by default, meaning once we give the variable a value, the value won't change. We'll be discussing this concept in detail in the Variable and Mutability section in chapter 3. To make a variable mutable, we add mut before the variable name.

> let apples = 5 // immutable
> let mut bananas = 5 // mutable

> The syntax // starts a comment that continues until the end of the line. Rust ignores everything in comments. We'll discuss comment in more detail in chapter 3.
