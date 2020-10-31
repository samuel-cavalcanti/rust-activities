pub mod employee_control {
    use std::{collections::HashMap, vec::Vec};

    struct Employee {
        name: String,
        department: String,
    }
    pub fn start() {
        println!("starting !!");

        let mut map = HashMap::new();

        loop {
            let input_string = input("add employee names to a department in a company".to_string());

            let (employee_name, department_name) = parse_input_string(input_string);

            let employee = Employee {
                name: employee_name,
                department: department_name,
            };

            map.insert(employee.name.clone(), employee);

            print_employees(&map);
        }
    }

    fn input(mensage: String) -> String {
        let mut user_input = String::new();

        println!("{}", mensage);

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("error on read line");

        return user_input;
    }

    fn parse_input_string(input_string: String) -> (String, String) {
        let input_string = input_string.replace("\n", "");

        let splited_strings = input_string.split(" ").collect::<Vec<&str>>();
        return (
            String::from(splited_strings[1]),
            String::from(splited_strings[3]),
        );
    }

    fn print_employees(employees: &HashMap<String, Employee>) {
        let mut array: Vec<&Employee> = employees.values().collect::<Vec<&Employee>>();

        array.sort_by_key(|a| a.name.clone());
        for employee in array {
            println!(
                "employee {} department {}",
                employee.name, employee.department
            )
        }
    }
}
