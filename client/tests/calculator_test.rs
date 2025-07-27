use std::{
    collections::{HashMap, HashSet as HeshSet},
    thread,
    time::{self, Duration},
};

mod data;

use chrono::NaiveTime;
use data::{get_contact_list, get_course_list};
use tcc::storage::{mapper::Plan, AddressData, ContactData, CourseData, MeetingPointData};
use tcc::{calculator::Calculator, storage::mapper::Hosting};
use uuid::Uuid;

use crate::data::{get_cook_and_run, real_live_data};

// START PRINT AREA
fn print_plan(plan: &Plan) {
    println!("Plan with score {}", plan.greatest_distance);

    println!("Hostings:");
    for hosting_list in &plan.hosting_list {
        println!("\tCourse: {}", hosting_list.course.name);
        println!("\tHost: {}", hosting_list.host.team_name);
        println!("\tGuest:");
        for guest in hosting_list.guest_list.iter() {
            println!("\t\t{}", guest.team_name);
        }
    }

    println!("Courses:");
    for (contact, hosting_list) in &plan.walking_path {
        let mut path_url = "https://routing.openstreetmap.de/?".to_string();

        println!("\tTeam: {}", contact.team_name);
        for host in hosting_list {
            path_url.push_str(&format!(
                "&loc={}%2C{}",
                host.host.address.latitude, host.host.address.longitude
            ));
            println!("\t\tCourse: {}", host.course.name);
            println!("\t\tHost: \t{}", host.host.team_name);
            println!("\t\tGuest:");
            for guest in host.guest_list.iter() {
                println!("\t\t\t{}", guest.team_name);
            }
        }
        println!("URL: {}&loc=50.5500%2C9.6787", path_url);
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
fn assert_number_of_guests_in_course(walkin_path: &HashMap<ContactData, Vec<Hosting>>) {
    let course_host_map: HashMap<Uuid /* Course Id */, Vec<Hosting>> = walkin_path
        .values()
        .flat_map(|hosting_list| {
            hosting_list
                .iter()
                .map(|h| (h.course.id, h.clone()))
                .collect::<Vec<(Uuid, Hosting)>>()
        })
        .fold(HashMap::new(), |mut acc, (course_id, hosting)| {
            acc.entry(course_id).or_insert_with(Vec::new).push(hosting);
            acc
        });
    for hosting_list in course_host_map.values() {
        let mut current_number_of_overhang = 0;
        let number_of_guests = hosting_list
            .iter()
            .map(|h| h.guest_list.len())
            .sum::<usize>();

        let number_of_hosts = hosting_list.len();
        let base_number_of_guests = number_of_guests / number_of_hosts;
        let number_of_overhang_guests = number_of_guests % number_of_hosts;

        for hosting in hosting_list {
            let guest_count = hosting.guest_list.len();
            assert!(
                guest_count >= base_number_of_guests,
                "Hosting \"{}\" of {} hostings has fewer guests ({}) than the minimum required ({}) of a total of {} guests",
                hosting.host.team_name,
                number_of_hosts,
                guest_count,
                base_number_of_guests,
                number_of_guests,
            );

            if guest_count > base_number_of_guests {
                assert!(
                    guest_count == base_number_of_guests + 1,
                    "Hosting \"{}\" has too many guests ({})",
                    hosting.host.team_name,
                    guest_count
                );
                current_number_of_overhang += 1;
            }
        }

        assert!(
            current_number_of_overhang == number_of_overhang_guests,
            "Number of overhang guests should be {} but was {}",
            number_of_overhang_guests,
            current_number_of_overhang
        );
    }
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

fn check_course(walkin_path: &HashMap<ContactData, Vec<Hosting>>) {
    assert_number_of_guests_in_course(walkin_path);
    assert_team_cooks_not_two_times(walkin_path);
    assert_team_is_not_two_times_in_one_course(walkin_path);
}
// END ASSERT AREA
// START TEST AREA

fn run_calculation(calculator: &mut Calculator) {
    calculator.calculate();

    let start_time = time::Instant::now();
    thread::sleep(Duration::from_millis(10000));
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
}

#[test]
fn test_team_of_nine() {
    let number_of_guests = 9;
    let number_course = 3;
    let contact_list = get_contact_list(number_of_guests);
    let course_list = get_course_list(number_course);

    print_test_params(&contact_list, &course_list);

    let cook_and_run = get_cook_and_run(
        contact_list.clone(),
        course_list.clone(),
        None,
        Some(MeetingPointData {
            name: "Irish Pub".to_string(),
            time: NaiveTime::from_hms_opt(18, 0, 0).expect("Expect time"),
            address: AddressData {
                address: "Peterstor 1, 36037 Fulda".to_string(),
                latitude: 50.5516,
                longitude: 9.6790,
            },
        }),
        None,
    );

    let calculator = Calculator::new(&cook_and_run);
    assert!(
        calculator.is_ok(),
        "Calculator was not created successfully: {}",
        calculator.err().unwrap()
    );
    let mut calculator = calculator.unwrap();
    run_calculation(&mut calculator);

    let plan_data = calculator.get_top_plan();

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
    assert_number_of_guests_in_course(&plan.walking_path);
    check_course(&plan.walking_path);
}

#[test]
fn test_team_real() {
    println!("Start test_team_real");
    let number_of_guests = 35;
    let number_course = 3;
    let contact_list = real_live_data();
    let course_list = get_course_list(number_course);
    let course_with_more_hosts = Some(course_list[1].id);

    print_test_params(&contact_list, &course_list);

    let cook_and_run = get_cook_and_run(
        contact_list.clone(),
        course_list.clone(),
        course_with_more_hosts,
        None,
        Some(MeetingPointData {
            name: "Irish Pub".to_string(),
            time: NaiveTime::from_hms_opt(18, 0, 0).expect("Expect time"),
            address: AddressData {
                address: "Peterstor 1, 36037 Fulda".to_string(),
                latitude: 50.5500,
                longitude: 9.6787,
            },
        }),
    );

    let calculator = Calculator::new(&cook_and_run);
    assert!(
        calculator.is_ok(),
        "Calculator was not created successfully: {}",
        calculator.err().unwrap()
    );
    let mut calculator = calculator.unwrap();
    run_calculation(&mut calculator);

    let plan_data = calculator.get_top_plan();

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
    check_course(&plan.walking_path);
}
