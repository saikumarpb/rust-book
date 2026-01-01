
#[derive(Debug)]
enum UsState {
    Albama,
    Alaska,
    Atlanta,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,  //  An arm has two parts: a pattern and some code.
            Coin::Nickel => 5, // => operator that separates the pattern and the code to run.
            Coin::Dime => {
                // Use `{}` to run multiple lines in a `match` arm, and when `{}` is used the trailing comma becomes optional;
                // without `{}` only a single expression is allowed and a comma is required.
                println!("Lucky penny!");
                10
            }
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }
}

fn main() {
    // The match Control Flow Construct

    // `match` is a control flow construct that lets you compare a value against patterns.
    // Each pattern is checked in order until one matches.
    // Once a pattern matches, the corresponding code block executes.
    // Patterns can include literal values (like numbers or strings).
    // Patterns can also include variable names to capture matched values.
    // Wildcards (`_`) can be used to match anything.
    // `match` supports many other pattern types for powerful control flow.

    // match prevents unhandled enum variants and makes code safer.
    // If any case is missing, the Rust compiler throws an error at compile time.

    // When the match expression executes, it compares the resultant value against the pattern of each arm, in order.
    // If a pattern matches the value, the code associated with that pattern is executed.
    // If that pattern doesn’t match the value, execution continues to the next arm

    // The code associated with each arm is an expression, and
    // the resultant value of the expression in the matching arm is the value that gets returned for the entire match expression.

    let coin = Coin::Quarter(UsState::Atlanta);
    coin.value_in_cents();

    let five_plus_one = plus_one(Some(5));
    let none_plus_one = plus_one(None);

    // Rust knows that we didn’t cover every possible case and even knows which pattern we forgot!
    // Matches in Rust are exhaustive: We must exhaust every last possibility in order for the code to be valid.

    // Catch-All Patterns and the _ Placeholder

    // Using enums, we can also take special actions for a few particular values,
    // but for all other values take one default action.

    let dice_roll = 2;

    match dice_roll {
        1 => add_fancy_hat(),
        5 => remove_fancy_hat(),
        other => move_player(other), // covers every other possible value, This catch-all pattern meets the requirement that match must be exhaustive.
    }

    //  Note that we have to put the catch-all arm last because the patterns are evaluated in order.
    //  If we had put the catch-all arm earlier, the other arms would never run, so Rust will warn us if we add arms after a catch-all!

    match dice_roll {
        other => move_player(other),
        1 => add_fancy_hat(),
        5 => remove_fancy_hat(),
    }

    // Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern:
    // _ is a special pattern that matches any value and does not bind to that value.
    //This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.

    match dice_roll {
        1 => add_fancy_hat(),
        5 => remove_fancy_hat(),
        _ => re_roll(), // we’re explicitly ignoring all other values in the last arm; we haven’t forgotten anything.
    }

    match dice_roll {
        1 => add_fancy_hat(),
        other => remove_fancy_hat(), // warns unused variable: other
    }

    // we’ll change the rules of the game one more time so that nothing else happens on your turn if you roll anything other than a 1 or a 5.
    // We can express that by using the unit value (the empty tuple type

    match dice_roll {
        1 => add_fancy_hat(),
        5 => remove_fancy_hat(),
        _ => (),
    }

    // In next lesson we’re going to move on to the if let syntax, which can be useful in situations where the match expression is a bit wordy.
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("Value of None + 1 = None");
            None
        }
        Some(x) => {
            println!("Value of {x} + 1 = {}", x + 1);
            Some(x + 1)
        }
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(dice_roll: u8) {}
fn re_roll() {}
