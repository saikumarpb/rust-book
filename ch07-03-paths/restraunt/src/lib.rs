mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn eat_at_restraunt() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist()
}

// Observe that front_of_house isn’t public, because the eat_at_restaurant function is defined in the same module as front_of_house
// (that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant.

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order() // Using super allows us to reference an item that we know is in the parent module
    }

    fn cook_order() {}

    // We think the back_of_house module and the deliver_order function are likely to stay in the same relationship (within the same module as siblings)
    // to each other and get moved together should we decide to reorganize the crate’s module tree.
    // Therefore, we used super so that we’ll have fewer places to update code in the future if this code gets moved to a different module.

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Mangoes"),
            }
        }
    }

    // Since back_of_house::Breakfast has a private field, the struct must provide a public associated function (like summer()) to create its instance.
    // Without this public constructor, we cannot create a Breakfast object inside eat_at_restaurant(), because we cannot directly set the private seasonal_fruit field from outside the module.

    // Key point:
    // Private fields can only be set from inside the module, so expose a public constructor when the struct needs to be created externally.

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn eat_breakfast_at_restraunt() {
    // // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Now we change the toast
    meal.toast = String::from("Wheat");

    // meal.seasonal_fruit = String::from("Apple"); // gives compile error : field `seasonal_fruit` of struct `Breakfast` is private

    println!("I'd like to eat {} toast please", meal.toast);

    let appetizer1 = back_of_house::Appetizer::Soup;
    let appetizer2 = back_of_house::Appetizer::Salad;
}
