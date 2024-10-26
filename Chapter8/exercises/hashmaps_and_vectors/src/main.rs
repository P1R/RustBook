use std::{io, collections::{hash_map::Entry, HashMap}};

fn main() {
    println!("Welcome to NetHunters company users and departments manager");
    let mut company = HashMap::new();

    loop {
        let mut option = String::new();
        print_menu();
        io::stdin().read_line(&mut option).expect("Error on Reading the option");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error is {:?}", e);
                continue
            },
        };

        println!("you selected {}", &option);
        match option {
            1 => list_department(&company),  
            2 => add_employee(&mut company),  
            3 => remove_employee(&mut company),  
            4 => remove_department(&mut company),  
            5 => list_all_company(&company),  
            _ => {
                println!("Goodbye ;)");
                break
            },
        }

    }

}

fn print_menu(){
    println!("");
    println!("please select one of the following options:");
    println!("1. list department");
    println!("2. Add employee");
    println!("3. remove employee");
    println!("4. remove department");
    println!("5. list all company");
    println!("Any other number: Exit");
}

fn list_department(company: &HashMap<String, Vec<String>>) { 
    let mut department = String::new();
    println!("Input the department name:");
    io::stdin().read_line(&mut department).expect("Error on Reading the option");
   
    match company.get(&department) {
        Some(d) => {
            println!("the employees in department {} are:", department.trim());
            for employee in d {
                println!("{}", employee.trim());
            }
        },
        None => println!("the department: {department} doesn't exists")
    };
}

fn list_all_company(company: &HashMap<String, Vec<String>>){
    println!("this is the list of all the company:");
    // in future avoid duplication of code and call list_department()
    for department in company.clone().into_keys() {
        match company.get(&department) {
            Some(d) => {
                println!("the employees in department {} are:", department.trim());
                for employee in d {
                    println!("{}", employee.trim());
                }
            },
            None => println!("the department: {department} doesn't exists")
        };
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>){
    let mut department = String::new();
    let mut employee = String::new();

    println!("Input the department name:");
    io::stdin().read_line(&mut department).expect("Error on Reading department");
    println!("Input the employee name:");
    io::stdin().read_line(&mut employee).expect("Error on Reading employee");

    match company.entry(department) {
        Entry::Occupied(mut v) => {
            let emplys = v.get_mut();
            add_ordered(emplys, &employee); 
        },
        Entry::Vacant(s) => {
            let mut employees = Vec::new();
            add_ordered(&mut employees, &employee);
            s.insert(employees);
        },
    };
}

fn remove_employee(company: &mut HashMap<String, Vec<String>>){
    let mut department = String::new();
    let mut employee = String::new();

    println!("Input the department name:");
    io::stdin().read_line(&mut department).expect("Error on Reading department");
    println!("Input the employee name:");
    io::stdin().read_line(&mut employee).expect("Error on Reading employee");

    match company.entry(department){
        Entry::Occupied(mut v) => {
            let emplys = v.get_mut();
            remove_ordered(emplys, &employee); 
        },
        Entry::Vacant(s) => println!("Error: department does not exists:{:#?}", s),
    }
}

fn remove_department(company: &mut HashMap<String, Vec<String>>){
    let mut department = String::new();
    println!("Input the department name:");
    io::stdin().read_line(&mut department).expect("Error on Reading department");

    match company.entry(department.clone()) {
        Entry::Occupied(_v) => {
            match company.remove(&department) {
                Some(_s) => println!("the department {} was deleted!",
                                    department.trim()),
                None => println!("Error removing department {department}"),
            };
        },
        Entry::Vacant(_) => println!("department not found!"),
    }
}

fn add_ordered(employees: &mut Vec<String>, new_employee: &String) {
    match employees.binary_search(&new_employee) {
        Ok(e) => println!("employee {} already registered",
                          employees[e].trim()),
        Err(_) => {
            employees.push(new_employee.to_string());
            employees.sort();
        },
    }
}

fn remove_ordered(employees: &mut Vec<String>, employee: &String) {
    match employees.binary_search(&employee) {
        Ok(index) => {
            employees.remove(index);
            println!("employee {employee} got removed! ");
        },
        Err(e) => println!("employee does not exists: Error {:?}", e), 
    };
}
