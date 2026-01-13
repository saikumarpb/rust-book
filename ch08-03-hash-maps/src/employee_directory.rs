use std::{collections::HashMap, io};

pub fn employee_directory_program() {
    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
    // for example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then, let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("> ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            ["Add", name, "to", department] => {
                company
                    .entry(department.to_string())
                    .or_insert_with(Vec::new)
                    .push(name.to_string());

                println!("Added {name} to {department}");
            }

            ["List", department] if *department != "All" => {
                if let Some(employees) = company.get(*department) {
                    let mut sorted_employees = employees.clone();
                    sorted_employees.sort();

                    println!("{department}:");
                    for name in sorted_employees {
                        println!("    {name}")
                    }
                } else {
                    println!("No such department")
                }
            }

            ["List", "All"] => {
                let mut departments: Vec<_> = company.keys().collect();
                departments.sort();

                for dept in departments {
                    let mut employees = company[dept].clone(); // we used direct call instead of get because we know dept definitely exists
                    employees.sort();

                    println!("{dept}:");
                    for name in employees {
                        println!("    {name}")
                    }
                }
            }

            _ => println!("Invalid command"),
        }
    }
}
