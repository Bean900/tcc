use chrono::{Local, NaiveTime};
use tcc::storage::{AddressData, ContactData, CookAndRunData, CourseData, MeetingPointData};
use uuid::Uuid;

pub fn get_contact_list(number_of_contacts: usize) -> Vec<ContactData> {
    //These addresses are randomly generated and do not correspond to known addresses
    let contact_data = vec![
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 1".to_string(),
            address: AddressData {
                address: "Theodor-Stern-Kai 7, 60596 Frankfurt am Main".to_string(),
                latitude: 50.09523,
                longitude: 8.66144,
            },
            mail: "team1@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 2".to_string(),
            address: AddressData {
                address: "Mainzer Landstraße 50, 60325 Frankfurt am Main".to_string(),
                latitude: 50.11092,
                longitude: 8.68212,
            },
            mail: "team2@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 3".to_string(),
            address: AddressData {
                address: "Zeil 106, 60313 Frankfurt am Main".to_string(),
                latitude: 50.11552,
                longitude: 8.68417,
            },
            mail: "team3@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 4".to_string(),
            address: AddressData {
                address: "Friedrich-Ebert-Anlage 49, 60308 Frankfurt am Main".to_string(),
                latitude: 50.11222,
                longitude: 8.65119,
            },
            mail: "team4@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 5".to_string(),
            address: AddressData {
                address: "Bockenheimer Landstraße 24, 60323 Frankfurt am Main".to_string(),
                latitude: 50.11667,
                longitude: 8.66972,
            },
            mail: "team5@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 6".to_string(),
            address: AddressData {
                address: "Schillerstraße 30, 60313 Frankfurt am Main".to_string(),
                latitude: 50.11417,
                longitude: 8.67861,
            },
            mail: "team6@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 7".to_string(),
            address: AddressData {
                address: "Kaiserstraße 62, 60329 Frankfurt am Main".to_string(),
                latitude: 50.10722,
                longitude: 8.66972,
            },
            mail: "team7@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 8".to_string(),
            address: AddressData {
                address: "Taunusanlage 12, 60325 Frankfurt am Main".to_string(),
                latitude: 50.11333,
                longitude: 8.66972,
            },
            mail: "team8@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 9".to_string(),
            address: AddressData {
                address: "Berliner Straße 72, 60311 Frankfurt am Main".to_string(),
                latitude: 50.11111,
                longitude: 8.68333,
            },
            mail: "team9@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 10".to_string(),
            address: AddressData {
                address: "Konrad-Adenauer-Straße 7, 60313 Frankfurt am Main".to_string(),
                latitude: 50.11389,
                longitude: 8.68278,
            },
            mail: "team10@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 11".to_string(),
            address: AddressData {
                address: "Neue Mainzer Straße 52, 60311 Frankfurt am Main".to_string(),
                latitude: 50.11028,
                longitude: 8.68278,
            },
            mail: "team11@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 12".to_string(),
            address: AddressData {
                address: "Große Eschenheimer Straße 43, 60313 Frankfurt am Main".to_string(),
                latitude: 50.11611,
                longitude: 8.68222,
            },
            mail: "team12@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 13".to_string(),
            address: AddressData {
                address: "Oeder Weg 15, 60318 Frankfurt am Main".to_string(),
                latitude: 50.12028,
                longitude: 8.68333,
            },
            mail: "team13@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 14".to_string(),
            address: AddressData {
                address: "Eschersheimer Landstraße 55, 60322 Frankfurt am Main".to_string(),
                latitude: 50.11833,
                longitude: 8.68222,
            },
            mail: "team14@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 15".to_string(),
            address: AddressData {
                address: "Fahrgasse 89, 60311 Frankfurt am Main".to_string(),
                latitude: 50.11056,
                longitude: 8.68444,
            },
            mail: "team15@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 16".to_string(),
            address: AddressData {
                address: "Hanauer Landstraße 126, 60314 Frankfurt am Main".to_string(),
                latitude: 50.11083,
                longitude: 8.70111,
            },
            mail: "team16@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 17".to_string(),
            address: AddressData {
                address: "Weserstraße 17, 60329 Frankfurt am Main".to_string(),
                latitude: 50.10639,
                longitude: 8.66944,
            },
            mail: "team17@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
        ContactData {
            id: Uuid::new_v4(),
            team_name: "Team 18".to_string(),
            address: AddressData {
                address: "Schäfergasse 20, 60313 Frankfurt am Main".to_string(),
                latitude: 50.11583,
                longitude: 8.68222,
            },
            mail: "team18@test.de".to_string(),
            phone_number: "1234".to_string(),
            members: 2,
            diets: vec![],
            needs_check: false,
            notes: vec![],
        },
    ];

    if number_of_contacts > contact_data.len() {
        panic!("Number of contacts must be greater than or equal to the number of contact data");
    }

    contact_data[..number_of_contacts].to_vec()
}

pub fn get_course_list(number_of_courses: usize) -> Vec<CourseData> {
    let course_list = vec![
        CourseData {
            id: Uuid::new_v4(),
            name: "Amuse-Bouche".to_string(),
            time: NaiveTime::from_hms_opt(01, 0, 0).expect("Expect time"),
        },
        CourseData {
            id: Uuid::new_v4(),
            name: "appetizer".to_string(),
            time: NaiveTime::from_hms_opt(03, 0, 0).expect("Expect time"),
        },
        CourseData {
            id: Uuid::new_v4(),
            name: "main course".to_string(),
            time: NaiveTime::from_hms_opt(06, 0, 0).expect("Expect time"),
        },
        CourseData {
            id: Uuid::new_v4(),
            name: "dessert".to_string(),
            time: NaiveTime::from_hms_opt(09, 0, 0).expect("Expect time"),
        },
        CourseData {
            id: Uuid::new_v4(),
            name: "Digestif & Petit Fours".to_string(),
            time: NaiveTime::from_hms_opt(13, 0, 0).expect("Expect time"),
        },
    ];

    match number_of_courses {
        1 => vec![course_list[2].clone()],
        2..5 => course_list[1..number_of_courses + 1].to_vec(),
        5 => course_list,
        _ => panic!("Number of courses must be between 1 and 5"),
    }
}

pub fn get_cook_and_run(
    contact_list: Vec<ContactData>,
    course_list: Vec<CourseData>,
    course_with_more_hosts: Option<Uuid>,
    start_point: Option<MeetingPointData>,
    end_point: Option<MeetingPointData>,
) -> CookAndRunData {
    CookAndRunData {
        id: Uuid::new_v4(),
        name: "Some test cook and run".to_string(),
        created: Local::now().to_utc(),
        edited: Local::now().to_utc(),
        occur: Local::now().to_utc().date_naive(),
        contact_list: contact_list.clone(),
        course_list: course_list.clone(),
        invite_allowed: false,
        course_with_more_hosts: course_with_more_hosts,
        is_in_cloud: false,
        plan_text: None,
        invite_text: None,
        start_point: start_point,
        end_point: end_point,
        top_plan: None,
    }
}
