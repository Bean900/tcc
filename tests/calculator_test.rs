use data::{get_contact_list, get_course_name_list};
use tcc::calculator::{Calculator, Plan};

mod data;

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
    let contact_list = get_contact_list(9);
    let course_name_list = get_course_name_list(3);
    let calculator = Calculator::new(&course_name_list, &contact_list);
    calculator.calculate();
    calculator.top_plan();
}
