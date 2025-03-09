use tcc::contact::Contact;
pub fn get_contact_list(number_of_contacts: usize) -> Vec<Contact> {
    //These addresses are randomly generated and do not correspond to known addresses
    let contact_data = vec![
        Contact::new(
            0,
            "Team 1",
            "Theodor-Stern-Kai 7, 60596 Frankfurt am Main",
            50.09523,
            8.66144,
        ),
        Contact::new(
            1,
            "Team 2",
            "Mainzer Landstraße 50, 60325 Frankfurt am Main",
            50.11092,
            8.68212,
        ),
        Contact::new(
            2,
            "Team 3",
            "Zeil 106, 60313 Frankfurt am Main",
            50.11552,
            8.68417,
        ),
        Contact::new(
            3,
            "Team 4",
            "Friedrich-Ebert-Anlage 49, 60308 Frankfurt am Main",
            50.11222,
            8.65119,
        ),
        Contact::new(
            4,
            "Team 5",
            "Bockenheimer Landstraße 24, 60323 Frankfurt am Main",
            50.11667,
            8.66972,
        ),
        Contact::new(
            5,
            "Team 6",
            "Schillerstraße 30, 60313 Frankfurt am Main",
            50.11417,
            8.67861,
        ),
        Contact::new(
            6,
            "Team 7",
            "Kaiserstraße 62, 60329 Frankfurt am Main",
            50.10722,
            8.66972,
        ),
        Contact::new(
            7,
            "Team 8",
            "Taunusanlage 12, 60325 Frankfurt am Main",
            50.11333,
            8.66972,
        ),
        Contact::new(
            8,
            "Team 9",
            "Berliner Straße 72, 60311 Frankfurt am Main",
            50.11111,
            8.68333,
        ),
        Contact::new(
            9,
            "Team 10",
            "Konrad-Adenauer-Straße 7, 60313 Frankfurt am Main",
            50.11389,
            8.68278,
        ),
        Contact::new(
            10,
            "Team 11",
            "Neue Mainzer Straße 52, 60311 Frankfurt am Main",
            50.11028,
            8.68278,
        ),
        Contact::new(
            11,
            "Team 12",
            "Große Eschenheimer Straße 43, 60313 Frankfurt am Main",
            50.11611,
            8.68222,
        ),
        Contact::new(
            12,
            "Team 13",
            "Oeder Weg 15, 60318 Frankfurt am Main",
            50.12028,
            8.68333,
        ),
        Contact::new(
            13,
            "Team 14",
            "Eschersheimer Landstraße 55, 60322 Frankfurt am Main",
            50.11833,
            8.68222,
        ),
        Contact::new(
            14,
            "Team 15",
            "Fahrgasse 89, 60311 Frankfurt am Main",
            50.11056,
            8.68444,
        ),
        Contact::new(
            15,
            "Team 16",
            "Hanauer Landstraße 126, 60314 Frankfurt am Main",
            50.11083,
            8.70111,
        ),
        Contact::new(
            16,
            "Team 17",
            "Weserstraße 17, 60329 Frankfurt am Main",
            50.10639,
            8.66944,
        ),
        Contact::new(
            17,
            "Team 18",
            "Schäfergasse 20, 60313 Frankfurt am Main",
            50.11583,
            8.68222,
        ),
    ];

    if number_of_contacts > contact_data.len() {
        panic!("Number of contacts must be greater than or equal to the number of contact data");
    }

    contact_data[..number_of_contacts].to_vec()
}

pub fn get_course_name_list(number_of_courses: usize) -> Vec<String> {
    let course_list = vec![
        "Amuse-Bouche".to_string(),
        "appetizer".to_string(),
        "main course".to_string(),
        "dessert".to_string(),
        "Digestif & Petit Fours".to_string(),
    ];

    match number_of_courses {
        1 => vec![course_list[2].clone()],
        2..5 => course_list[1..number_of_courses + 1].to_vec(),
        5 => course_list,
        _ => panic!("Number of courses must be between 1 and 5"),
    }
}
