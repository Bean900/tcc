use data::{get_contact_list, get_course_name_list};
use env_logger::Env;
use tcc::{
    calculator::{Calculator, Course, Plan},
    contact::Contact,
};

use std::collections::{HashMap, HashSet as HeshSet};

mod data;

// START PRINT AREA
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
            println!("\t\tHost: \t{}", course.host.team_name);
            println!("\t\tGuest:");
            for guest in course.guest_list.iter() {
                println!("\t\t\t{}", guest.team_name);
            }
            println!();
        }
    }

    println!("Walking-Path:");
    for (from, to_list) in plan.walking_path.iter() {
        print!("\t{}", from.team_name);
        for to in to_list.iter() {
            print!(" -> {}", to.host.team_name);
        }
        print!("\n")
    }
}

fn print_test_params(contact_list: &Vec<Contact>, course_name_list: &Vec<String>) {
    println!(
        "Contact names: {:?}",
        contact_list
            .iter()
            .map(|c| c.team_name.as_str())
            .collect::<Vec<&str>>()
    );

    println!("Course names: {:?}", course_name_list);
}

// END PRINT AREA
// START ASSERT AREA
fn assert_number_of_guests_in_course(
    course_map: &HashMap<String, Vec<Course>>,
    number_of_guests: usize,
) {
    for (_, course_list) in course_map.iter() {
        for course in course_list {
            assert_eq!(
                course.guest_list.len(),
                number_of_guests,
                "Number of guests in course \"{}\" should be {}",
                course.host.team_name,
                number_of_guests
            );
        }
    }
}

fn assert_team_is_not_two_times_in_one_course(course_map: &HashMap<String, Vec<Course>>) {
    for (_, course_list) in course_map.iter() {
        let mut already_guest_in_course = HeshSet::new();
        let mut already_host_in_course = HeshSet::new();
        for course in course_list {
            assert!(
                !already_host_in_course.contains(&course.host.team_name),
                "Host \"{}\" is already a host in course \"{}\"",
                course.host.team_name,
                course.name
            );
            assert!(
                !already_guest_in_course.contains(&course.host.team_name),
                "Host \"{}\" is already a guest in course \"{}\"",
                course.host.team_name,
                course.name
            );
            already_host_in_course.insert(&course.host.team_name);
            for guest in course.guest_list.iter() {
                assert!(
                    !already_guest_in_course.contains(&guest.team_name),
                    "Guest \"{}\" is already a guest in course \"{}\"",
                    guest.team_name,
                    course.name
                );
                assert!(
                    !already_host_in_course.contains(&guest.team_name),
                    "Guest \"{}\" is already a host in course \"{}\"",
                    guest.team_name,
                    course.name
                );
                already_guest_in_course.insert(&guest.team_name);
            }
        }
    }
}
fn assert_team_cooks_not_two_times(course_map: &HashMap<String, Vec<Course>>) {
    let mut already_cooked = HeshSet::new();
    for (_, course_list) in course_map.iter() {
        for course in course_list {
            assert!(
                !already_cooked.contains(&course.host.team_name),
                "Host \"{}\" is already a host in another course",
                course.host.team_name
            );
            already_cooked.insert(&course.host.team_name);
        }
    }
}

fn check_course(course_map: &HashMap<String, Vec<Course>>) {
    assert_team_cooks_not_two_times(course_map);
    assert_team_is_not_two_times_in_one_course(course_map);
}
// END ASSERT AREA
// START TEST AREA

#[test]
fn test_perfect_path() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let number_of_guests = 2;
    let number_course = 3;
    let contact_list = get_contact_list(9);
    let course_name_list = get_course_name_list(number_course);

    print_test_params(&contact_list, &course_name_list);

    let calculator = Calculator::new(&course_name_list, &contact_list, None);
    calculator.calculate();
    let plan = calculator.top_plan().unwrap();
    print_plan(&plan);
    assert_eq!(plan.course_map.len(), 3, "Number of courses should be 3");
    assert_number_of_guests_in_course(&plan.course_map, number_of_guests);
    check_course(&plan.course_map);
}
//END TEST AREA
