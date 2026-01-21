pub struct Guess {
    value: i32,
}

impl Guess {
    // Code outside the guessing_game module must use the Guess::new function to create an instance of Guess,
    // thereby ensuring that there’s no way for a Guess to have a value that hasn’t been checked by the conditions in the Guess::new function.

    /**
     * Value should be between 1 and 100
     */
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100. Got : {value}");
        }

        Guess { value }
    }

    // This kind of method is sometimes called a getter because its purpose is to get some data from its fields and return it.
    // This public method is necessary because the value field of the Guess struct is private.
    // It’s important that the value field be private so that code using the Guess struct is not allowed to set value directly:
    pub fn value(&self) -> i32 {
        self.value
    }
}
