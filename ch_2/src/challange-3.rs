// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use std::{collections::HashMap, io};


fn main() {
  let mut company_map: HashMap<String, Vec<String,>,> = HashMap::new();


  loop {
    let mut input = String::new();

    io::stdin()
      .read_line(&mut input,)
      .expect("Please provide me with correct sequence of orders",);

    let input = input.trim();

    if input.eq_ignore_ascii_case("exit",) {
      break;
    }

    let input_parts: Vec<&str,> = input.split_whitespace().collect();

    if input_parts.len() < 2 {
      return;
    }

    match input_parts[0] {
      "Add" => {
        if input_parts[2].eq_ignore_ascii_case("Sales",)
          || input_parts[2].eq_ignore_ascii_case("Engineering",)
        {
          let log = add_employee(
            &mut company_map,
            input_parts[1].to_string(),
            input_parts[2].to_string(),
          );
          println!("{:?}\n\n", log);
        }
      },
      "Show" => {
        if input_parts[1].eq_ignore_ascii_case("Sales",) {
          let log = list_employees(&mut company_map, input_parts[1].to_string(),);
          println!("{:?}\n\n", log);
        }
      },
      "List" => {
        if input_parts[1].eq_ignore_ascii_case("All",) {
          let log = list_all_copmany(&mut company_map,);
          println!("{:?}\n\n", log);
        }
      },
      _ => (),
    }
  }
}

fn add_employee(
  company: &mut HashMap<String, Vec<String,>,>,
  name: String,
  department: String,
) -> String {
  company
    .entry(department.clone(),)
    .or_insert_with(Vec::new,)
    .push(name.clone(),);

  format!(
    "The employee with the name {}, has entered the {} department",
    name, department,
  )
}

fn list_employees(company: &mut HashMap<String, Vec<String,>,>, department: String,) -> String {
  match company.get("Sales",) {
    Some(val,) => {
      format! {"This is the full list of {} employees\n {:?}", department.clone(), val}
    },
    None => {
      format! {"The {} department you're trying to list it's employees is empty!", department.clone()}
    },
  }
}

fn list_all_copmany(company: &mut HashMap<String, Vec<String,>,>,) -> () {
  for (department, employees_list,) in company.iter() {
    println!("{:?}: {:?}", department, employees_list)
  }
}
