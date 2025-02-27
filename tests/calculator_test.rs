use data::{get_contact_list, get_course_name_list};
use tcc::calculator::{Calculator, Plan};

mod data;

fn print_plan(plan: &Plan) {
    let seed_str = plan
        .seed
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join("-");
    println!("Plan with score {}", plan.score);
    println!("Seed: {}", seed_str);

    println!("Courses:");
    for (course_name, course) in plan.course_map.iter() {
        println!("\tCourse: {}", course_name);
        for course in course {
            println!("\t\tHost: {}", course.host.team_name);
            println!("\t\tGuest:");
            for guest in course.guest_list.iter() {
                println!("\t\t\t- {}", guest.team_name);
            }
        }
    }

    println!("Walking-Path:");
    for (from, to_list) in plan.walking_path.iter() {
        print!("\t{} ", from.team_name);
        for to in to_list.iter() {
            print!(" -> {}", to.host.team_name);
        }
        print!("\n")
    }
}

#[test]
fn test_perfect_path() {
    let contact_list = get_contact_list(9);
    let course_name_list = get_course_name_list(3);
    let calculator = Calculator::new(&course_name_list, &contact_list);
    calculator.calculate();
    let plan = calculator.top_plan().unwrap();
    print_plan(&plan);
    assert_eq!(plan.course_map.len(), 3, "Number of courses should be 3");
}
