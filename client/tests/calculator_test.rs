use std::{
    collections::{HashMap, HashSet as HeshSet},
    thread,
    time::{self, Duration},
};

mod data;

use std::sync::Once;

use chrono::Local;
use data::{get_contact_list, get_course_list};
use tcc::{
    calculator::generate_cook_and_run_plan,
    storage::{mapper::Plan, ContactData, CookAndRunData, CourseData},
};
use tcc::{calculator::Calculator, storage::mapper::Hosting};
use uuid::Uuid;

use crate::data::get_cook_and_run;

static INIT: Once = Once::new();

// START PRINT AREA
fn print_plan(plan: &Plan) {
    println!("Plan with score {}", plan.greatest_distance);

    println!("Courses:");
    for (contact, hosting_list) in &plan.walking_path {
        println!("\tTeam: {}", contact.team_name);
        for host in hosting_list {
            println!("\t\tCourse: \t{}", host.course.name);
            println!("\t\tHost: \t{}", host.host.team_name);
            println!("\t\tGuest:");
            for guest in host.guest_list.iter() {
                println!("\t\t\t{}", guest.team_name);
            }
            println!();
        }
    }
}

fn print_test_params(contact_list: &Vec<ContactData>, course_list: &Vec<CourseData>) {
    println!(
        "Contact names: {:?}",
        contact_list
            .iter()
            .map(|c| c.team_name.as_str())
            .collect::<Vec<&str>>()
    );

    println!(
        "Course names: {:?}",
        course_list
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<&str>>()
    );
}

// END PRINT AREA
// START ASSERT AREA
fn assert_number_of_guests_in_course(
    walkin_path: &HashMap<ContactData, Vec<Hosting>>,
    number_of_guests: usize,
    course_with_more_hosts: Option<Uuid>,
) {
    let base_number_of_guests = number_of_guests
        / walkin_path
            .values()
            .next()
            .expect("Expect at least one element")
            .len()
        - 1;
    let number_of_overhang_guests = number_of_guests
        % walkin_path
            .values()
            .next()
            .expect("Expect at least one element")
            .len();

    assert!(
        course_with_more_hosts.is_none() && number_of_overhang_guests == 0,
        "No course with more hosts is set. So there are no overhang allowed"
    );
    let mut current_number_of_overhang = 0;

    for (_, host_list) in walkin_path.iter() {
        for hosting in host_list {
            if hosting.guest_list.len() != base_number_of_guests {
                assert!(
                    number_of_overhang_guests == 0,
                    "No course with overhang allowed"
                );
                assert!(
                    hosting
                        .course
                        .id
                        .eq(&course_with_more_hosts.expect("Expect course with overhang")),
                    "Expect only course \"{}\" to have overhang. But found overhang in \"{}\"",
                    course_with_more_hosts.expect("Expect course with overhang"),
                    hosting.course.id
                );
                assert!(
                    hosting.guest_list.len() == base_number_of_guests + 1,
                    "Base number of guests in course \"{}\" can only be one higher",
                    hosting.course.name
                );
                current_number_of_overhang += 1;
            }
        }
    }

    assert!(
        current_number_of_overhang == number_of_overhang_guests,
        "Number of overhang guests should be {} but was {}",
        number_of_overhang_guests,
        current_number_of_overhang
    );
}

fn assert_team_is_not_two_times_in_one_course(walkin_path: &HashMap<ContactData, Vec<Hosting>>) {
    let mut course_hosting_contact_map = HashMap::new();
    for (_, hosting_list) in walkin_path.iter() {
        for hosting in hosting_list {
            let hosting_map = course_hosting_contact_map
                .entry(hosting.course.id)
                .or_insert_with(HashMap::new);

            let list_of_guest = hosting_map.entry(hosting.id).or_insert(Vec::new());
            if list_of_guest.is_empty() {
                list_of_guest.extend(hosting.guest_list.iter().map(|c| c.id));
            } else {
                assert!(
                    hosting
                        .guest_list
                        .iter()
                        .all(|c| list_of_guest.contains(&c.id)),
                    "Guest should be always the same"
                );

                assert!(
                    list_of_guest.len() == hosting.guest_list.len(),
                    "Number of guest should be identical"
                );
            }
        }
    }
    for (course, hosting_map) in course_hosting_contact_map {
        let mut seen_contacts = HeshSet::new();
        for (_, contact_list) in hosting_map {
            for contact in contact_list {
                assert!(
                    !seen_contacts.contains(&contact),
                    "Contact \"{}\" was already seen in course \"{}\"",
                    contact,
                    course
                );
                seen_contacts.insert(contact);
            }
        }
    }
}
fn assert_team_cooks_not_two_times(walkin_path: &HashMap<ContactData, Vec<Hosting>>) {
    for (contact, hosting_list) in walkin_path {
        let mut already_cooking = false;
        for hosting in hosting_list {
            if contact.id.eq(&hosting.host.id) {
                assert!(
                    !already_cooking,
                    "Contact \"{}\" is already hosting one cooking",
                    contact.team_name
                );
                already_cooking = true;
            }
        }
        assert!(
            already_cooking,
            "Contact \"{}\" is not hosting",
            contact.team_name
        );
    }
}

fn check_course(
    walkin_path: &HashMap<ContactData, Vec<Hosting>>,
    course_with_more_hosts: Option<Uuid>,
    number_of_guests: usize,
) {
    assert_number_of_guests_in_course(walkin_path, number_of_guests, course_with_more_hosts);
    assert_team_cooks_not_two_times(walkin_path);
    assert_team_is_not_two_times_in_one_course(walkin_path);
}
// END ASSERT AREA
// START TEST AREA
/*
fn run_calculation(calculator: &mut Calculator) {
    calculator.calculate();

    let start_time = time::Instant::now();
    while calculator.get_top_plan().is_none() {
        // Wait until a plan is available
        thread::sleep(Duration::from_millis(100));
    }
    calculator.stop();

    if start_time.elapsed().as_secs() == 0 {
        println!(
            "Calculation took {} milliseconds!",
            start_time.elapsed().as_millis()
        );
    } else {
        println!(
            "Calculation took {} seconds!",
            start_time.elapsed().as_secs()
        );
    }
}*/

#[test]
fn test_team_of_nine() {
    let number_of_guests = 9;
    let number_course = 3;
    let contact_list = get_contact_list(number_of_guests);
    let course_list = get_course_list(number_course);

    print_test_params(&contact_list, &course_list);

    let cook_and_run =
        get_cook_and_run(contact_list.clone(), course_list.clone(), None, None, None);

    let plan_data = generate_cook_and_run_plan(cook_and_run);

    let plan_data = plan_data.expect("Expect plan");
    let plan = Plan::to_plan(&plan_data, &course_list, &contact_list);

    print_plan(&plan);
    assert_eq!(
        plan.walking_path
            .values()
            .next()
            .expect("Expect at least one")
            .len(),
        number_course,
        "Number of courses should be {}",
        number_course,
    );
    assert_number_of_guests_in_course(&plan.walking_path, number_of_guests, None);
    check_course(&plan.walking_path, None, number_of_guests);
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
