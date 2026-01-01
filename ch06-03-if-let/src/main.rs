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

fn main() {
    let config_max = Some(3u8);

    // For ex: We want to execute code if the value is the Some variant.
    match config_max {
        Some(max) => println!("The max is configured to be {max}"),
        _ => (), // Since match construct is exhaustive, this boiler plate code to satisfy the compiler needs to be added.
    }

    // The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.
    // The syntax if let takes a pattern and an expression separated by an equal sign
    // The code in the if let block only runs if the value matches the pattern.
    if let Some(max) = config_max {
        println!("The max is configured to be {max}");
    }

    // Using if let means less typing, less indentation, and less boilerplate code.
    // However, you lose the exhaustive checking match enforces that ensures that you aren’t forgetting to handle any cases.
    // if let is like a syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.

    // We can include an else with an if let.
    // The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else.

    let coin = Coin::Penny;
    let mut count = 0;

    match &coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}"),
        _ => count += 1,
    }

    // we could use an if let and else expression, like this:

    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {state:?}")
    } else {
        count += 1
    }

    let coin = Coin::Quarter(UsState::Atlanta);

    if let Some(str) = coin.describe_state_quarter() {
        println!("{str}");
    }

    // When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle.
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alaska => year >= 1819,
            UsState::Albama => year >= 1959,
            UsState::Atlanta => year >= 1845,
        }
    }
}

impl Coin {
    fn describe_state_quarter(&self) -> Option<String> {
        // We could also take advantage of the fact that expressions produce a value either to produce the state from the if let or to return early
        let state = if let Self::Quarter(state) = self {
            state
        } else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old for America"))
        } else {
            Some(format!("{state:?} is pretty new for America"))
        }
    }

    fn describe_state_quarter_using_let_else(&self) -> Option<String> {
        // Rust has let...else. The let...else syntax takes a pattern on the left side and an expression on the right, very similar to if let,
        // but it does not have an if branch, only an else branch.
        // If the pattern matches, it will bind the value from the pattern in the outer scope.
        // If the pattern does not match, the program will flow into the else arm
        let Self::Quarter(state) = self else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old for America"))
        } else {
            Some(format!("{state:?} is pretty new for America"))
        }
    }
}
