use data::{get_contact_list, get_course_name_list};
use tcc::calculator::{Calculator, Plan};

mod data;

fn get_calculator(number_of_contacts: usize, number_of_courses: usize) -> Calculator {
    let contact_list = get_contact_list(number_of_contacts);
    let course_name_list = get_course_name_list(number_of_courses);
    Calculator::new(course_name_list, contact_list)
}

fn print_plan(plan: Plan) {
    let seed_str = plan
        .seed
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join("-");
    println!("Plan with score {}", plan.score);
    println!("Seed: {}", seed_str);

    println!("Route per Person:");
}

#[test]
fn test_perfect_path() {
    let calculator = get_calculator(9, 3);
    calculator.calculate();
    calculator.top_plan();
}
