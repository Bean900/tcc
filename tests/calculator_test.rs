use data::{get_contact_list, get_course_name_list};
use env_logger::Env;
use tcc::{
    calculator::{Calculator, Course, Plan},
    contact::Contact,
};

use core::num;
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

    /*
    println!("Walking-Path:");
    for (from, to_list) in plan.walking_path.iter() {
        println!("{}:\n\t", from.team_name);
        for to in to_list.iter() {
            print!(" -> {}", to.host.team_name);
        }
        print!("\n")
    }*/
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
    number_of_course: usize,
    number_of_guests: usize,
) {
    let base_number_of_guests = number_of_guests / number_of_course - 1;
    let number_of_overhang_guests = number_of_guests % number_of_course;

    for (_, course_list) in course_map.iter() {
        let mut seen_people = HeshSet::new();
        let mut number_of_overhang = 0;
        for course in course_list {
            assert!(
                !seen_people.contains(&course.host.team_name),
                "Host \"{}\" is already participating in another course",
                course.host.team_name
            );

            seen_people.insert(&course.host.team_name);
            for guest in course.guest_list.iter() {
                assert!(
                    !seen_people.contains(&guest.team_name),
                    "Guest \"{}\" is already participating in another course",
                    guest.team_name
                );
                seen_people.insert(&guest.team_name);
            }
            assert!(
                course.guest_list.len() >= base_number_of_guests
                    && course.guest_list.len() < base_number_of_guests + 1,
                "Number of guests in course \"{}\" should be less than {} and greater or equal to {} but was {}",
                course.host.team_name,
                base_number_of_guests + 1,
                base_number_of_guests,
                course.guest_list.len()
            );
            if course.guest_list.len() == base_number_of_guests + 1 {
                number_of_overhang += 1;
            }

            assert!(
                course.guest_list.len() <= base_number_of_guests + 1,
                "Number of guests in course \"{}\" should be less than or equal to {} but was {}",
                course.host.team_name,
                base_number_of_guests + 1,
                course.guest_list.len()
            );
        }
        assert!(
            number_of_overhang == number_of_overhang_guests,
            "Number of overhang guests should be {} but was {}",
            number_of_overhang_guests,
            number_of_overhang
        );
        assert!(
            number_of_guests == seen_people.len(),
            "Number of guests should be {} but was {}",
            number_of_guests,
            seen_people.len()
        );
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

fn check_course(
    course_map: &HashMap<String, Vec<Course>>,
    course_with_more_hosts: Option<&String>,
) {
    assert_team_cooks_not_two_times(course_map);
    assert_team_is_not_two_times_in_one_course(course_map);
    let mut number_of_hosts = None;
    for (course_name, course_list) in course_map.iter() {
        if course_with_more_hosts.is_some()
            && course_with_more_hosts.expect("Expect course name for course with more hosts")
                == course_name
        {
            if number_of_hosts.is_some() {
                assert!(
                    number_of_hosts.expect("Expect number of hosts") == course_list.len() + 1,
                    "Overhang course \"{}\" has not exaclty {} but {} hosts",
                    course_name,
                    number_of_hosts.expect("Expect number of hosts"),
                    course_list.len() + 1
                );
            } else {
                number_of_hosts = Some(course_list.len() - 1);
            }
        } else {
            if number_of_hosts.is_some() {
                assert!(
                    number_of_hosts.expect("Expect number of hosts") == course_list.len(),
                    "Course \"{}\" has not exaclty {} but {} hosts",
                    course_name,
                    number_of_hosts.expect("Expect number of hosts"),
                    course_list.len() + 1
                );
            } else {
                number_of_hosts = Some(course_list.len());
            }
        }
    }
}
// END ASSERT AREA
// START TEST AREA

#[test]
fn test_team_of_nine() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let number_of_guests = 9;
    let number_course = 3;
    let contact_list = get_contact_list(number_of_guests);
    let course_name_list = get_course_name_list(number_course);

    print_test_params(&contact_list, &course_name_list);

    let calculator = Calculator::new(&course_name_list, &contact_list, None);
    calculator.calculate();
    let plan = calculator.top_plan().unwrap();
    print_plan(&plan);
    assert_eq!(
        plan.course_map.len(),
        number_course,
        "Number of courses should be {}",
        number_course,
    );
    assert_number_of_guests_in_course(&plan.course_map, number_course, number_of_guests);
    check_course(&plan.course_map, None);
    //Plan with score 3762.0062854856706
    //Seed: 31-154-93-147-18-76-38-47-87-69-187-250-155-10-125-119-21-255-59-67-24-2-129-2-125-26-228-252-245-254-63-2-166-63-163-84-44-118-149-196-215-81-125-254-177-119-218-207-111-184
    /*
    Walking-Path:
    Team 2:
     -> Team 1 -> Team 9 -> Team 2
    Team 8:
     -> Team 4 -> Team 8 -> Team 7
    Team 3:
     -> Team 5 -> Team 3 -> Team 6
    Team 6:
     -> Team 9 -> Team 6 -> Team 3
    Team 1:
     -> Team 6 -> Team 7 -> Team 1
    Team 7:
     -> Team 2 -> Team 7 -> Team 3
    Team 9:
     -> Team 1 -> Team 2 -> Team 9
    Team 4:
     -> Team 8 -> Team 4 -> Team 5
    Team 5:
     -> Team 5 -> Team 4 -> Team 8
      */
}

#[test]
fn test_team_of_ten() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let number_of_guests = 10;
    let number_course = 3;
    let contact_list = get_contact_list(number_of_guests);
    let course_name_list = get_course_name_list(number_course);
    let course_with_more_hosts = Some(&course_name_list[1]);

    print_test_params(&contact_list, &course_name_list);

    let calculator = Calculator::new(&course_name_list, &contact_list, course_with_more_hosts);
    calculator.calculate();
    let plan = calculator.top_plan().unwrap();
    print_plan(&plan);
    assert_eq!(
        plan.course_map.len(),
        number_course,
        "Number of courses should be {}",
        number_course,
    );
    assert_number_of_guests_in_course(&plan.course_map, number_course, number_of_guests);
    check_course(&plan.course_map, course_with_more_hosts);
}
//END TEST AREA
