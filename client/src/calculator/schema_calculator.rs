use std::collections::HashMap;
use std::vec;

use uuid::Uuid;
use web_sys::console;

use crate::calculator::{Calculator, Course, Hosting, Plan, Point, Team};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum CourseType {
    Appetizer,
    MainCourse,
    Dessert,
}

impl CourseType {
    fn rank(&self) -> usize {
        match self {
            CourseType::Appetizer => 0,
            CourseType::MainCourse => 1,
            CourseType::Dessert => 2,
        }
    }
}

struct AssignedCourse {
    course_type: CourseType,
    host: Uuid,
    guest_list: Vec<Uuid>,
}

struct Group<'a> {
    sorted_team_list: Vec<&'a Team>,
}

impl<'a> Group<'a> {
    fn new() -> Self {
        Self {
            sorted_team_list: Vec::new(),
        }
    }
}

struct SchemaCourse {
    course_type: CourseType,
    host: u8,
    guest_list: Vec<u8>,
}

struct Schema {
    plan: HashMap<u8 /* Host Id */, SchemaCourse>,
}

impl Schema {
    fn create_mapping(&self, sorted_team_list: &Vec<&Team>) -> HashMap<u8, Uuid> {
        let mut mapping = HashMap::new();

        let mut team_idx = 0 as u8;

        self.plan.keys().for_each(|idx| {
            console::debug_1(&format!("Creating mapping for host index {:?} ", idx).into());
            mapping.insert(*idx, sorted_team_list[team_idx as usize].id);
            team_idx += 1;
        });

        console::debug_1(
            &format!(
                "Size of mapping: {}, size of team list: {}",
                mapping.len(),
                sorted_team_list.len()
            )
            .into(),
        );

        mapping
    }
}

pub struct SchemaCalculator<'a> {
    schema_list: HashMap<u8 /* Number of teams */, Schema>,
    target_lat: f64,
    target_lon: f64,
    team_list: &'a Vec<Team>,
    sorted_course_list: Vec<&'a Course>,
}

impl<'a> SchemaCalculator<'a> {
    pub fn new(
        end_point: &Point,
        team_list: &'a Vec<Team>,
        course_list: &'a Vec<Course>,
    ) -> Result<Self, String> {
        if team_list.len() < 9 {
            return Err("At least 9 teams are required in SchemaCalculator".to_string());
        }
        if course_list.len() != 3 {
            return Err("Exactly 3 courses are required in SchemaCalculator".to_string());
        }

        let mut sorted_course_list = course_list.iter().collect::<Vec<&Course>>();
        sorted_course_list.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        Ok(SchemaCalculator {
            schema_list: HashMap::from([
                (
                    9,
                    Schema {
                        plan: HashMap::from([
                            (
                                1,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 1,
                                    guest_list: vec![2, 3],
                                },
                            ),
                            (
                                4,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 4,
                                    guest_list: vec![5, 6],
                                },
                            ),
                            (
                                7,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 7,
                                    guest_list: vec![8, 9],
                                },
                            ),
                            (
                                2,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 2,
                                    guest_list: vec![6, 7],
                                },
                            ),
                            (
                                5,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 5,
                                    guest_list: vec![9, 1],
                                },
                            ),
                            (
                                8,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 8,
                                    guest_list: vec![3, 4],
                                },
                            ),
                            (
                                3,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 3,
                                    guest_list: vec![7, 5],
                                },
                            ),
                            (
                                6,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 6,
                                    guest_list: vec![1, 8],
                                },
                            ),
                            (
                                9,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 9,
                                    guest_list: vec![4, 2],
                                },
                            ),
                        ]),
                    },
                ),
                (
                    10,
                    Schema {
                        plan: HashMap::from([
                            (
                                1,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 1,
                                    guest_list: vec![2, 3],
                                },
                            ),
                            (
                                4,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 4,
                                    guest_list: vec![5, 6],
                                },
                            ),
                            (
                                7,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 7,
                                    guest_list: vec![8, 9, 10],
                                },
                            ),
                            (
                                2,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 2,
                                    guest_list: vec![6, 7],
                                },
                            ),
                            (
                                5,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 5,
                                    guest_list: vec![9, 1],
                                },
                            ),
                            (
                                8,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 8,
                                    guest_list: vec![3],
                                },
                            ),
                            (
                                10,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 10,
                                    guest_list: vec![4],
                                },
                            ),
                            (
                                3,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 3,
                                    guest_list: vec![7, 5, 10],
                                },
                            ),
                            (
                                6,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 6,
                                    guest_list: vec![1, 8],
                                },
                            ),
                            (
                                9,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 9,
                                    guest_list: vec![4, 2],
                                },
                            ),
                        ]),
                    },
                ),
                (
                    11,
                    Schema {
                        plan: HashMap::from([
                            (
                                1,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 1,
                                    guest_list: vec![2, 3],
                                },
                            ),
                            (
                                4,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 4,
                                    guest_list: vec![5, 6],
                                },
                            ),
                            (
                                7,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 7,
                                    guest_list: vec![8, 9],
                                },
                            ),
                            (
                                10,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 10,
                                    guest_list: vec![11],
                                },
                            ),
                            (
                                2,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 2,
                                    guest_list: vec![6, 7],
                                },
                            ),
                            (
                                5,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 5,
                                    guest_list: vec![9, 1],
                                },
                            ),
                            (
                                8,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 8,
                                    guest_list: vec![3, 10],
                                },
                            ),
                            (
                                11,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 11,
                                    guest_list: vec![4],
                                },
                            ),
                            (
                                3,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 3,
                                    guest_list: vec![7, 5, 11],
                                },
                            ),
                            (
                                6,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 6,
                                    guest_list: vec![1, 8],
                                },
                            ),
                            (
                                9,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 9,
                                    guest_list: vec![4, 2, 10],
                                },
                            ),
                        ]),
                    },
                ),
                (
                    12,
                    Schema {
                        plan: HashMap::from([
                            (
                                1,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 1,
                                    guest_list: vec![2, 3],
                                },
                            ),
                            (
                                4,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 4,
                                    guest_list: vec![5, 6],
                                },
                            ),
                            (
                                7,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 7,
                                    guest_list: vec![8, 9],
                                },
                            ),
                            (
                                10,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 10,
                                    guest_list: vec![11, 12],
                                },
                            ),
                            (
                                2,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 2,
                                    guest_list: vec![6, 10],
                                },
                            ),
                            (
                                5,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 5,
                                    guest_list: vec![9, 1],
                                },
                            ),
                            (
                                8,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 8,
                                    guest_list: vec![12, 4],
                                },
                            ),
                            (
                                11,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 11,
                                    guest_list: vec![3, 7],
                                },
                            ),
                            (
                                3,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 3,
                                    guest_list: vec![10, 5],
                                },
                            ),
                            (
                                6,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 6,
                                    guest_list: vec![1, 8],
                                },
                            ),
                            (
                                9,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 9,
                                    guest_list: vec![4, 11],
                                },
                            ),
                            (
                                12,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 12,
                                    guest_list: vec![7, 2],
                                },
                            ),
                        ]),
                    },
                ),
                (
                    13,
                    Schema {
                        plan: HashMap::from([
                            (
                                1,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 1,
                                    guest_list: vec![2, 3],
                                },
                            ),
                            (
                                4,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 4,
                                    guest_list: vec![5, 6],
                                },
                            ),
                            (
                                7,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 7,
                                    guest_list: vec![8, 9],
                                },
                            ),
                            (
                                10,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 10,
                                    guest_list: vec![11, 12, 13],
                                },
                            ),
                            (
                                3,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 3,
                                    guest_list: vec![6, 10],
                                },
                            ),
                            (
                                5,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 5,
                                    guest_list: vec![9, 1],
                                },
                            ),
                            (
                                8,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 8,
                                    guest_list: vec![12, 4],
                                },
                            ),
                            (
                                11,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 11,
                                    guest_list: vec![3],
                                },
                            ),
                            (
                                14,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 14,
                                    guest_list: vec![7],
                                },
                            ),
                            (
                                3,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 3,
                                    guest_list: vec![10, 5],
                                },
                            ),
                            (
                                6,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 6,
                                    guest_list: vec![1, 8, 13],
                                },
                            ),
                            (
                                9,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 9,
                                    guest_list: vec![4, 11, 14],
                                },
                            ),
                            (
                                12,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 12,
                                    guest_list: vec![7, 2],
                                },
                            ),
                        ]),
                    },
                ),
                (
                    14,
                    Schema {
                        plan: HashMap::from([
                            (
                                1,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 1,
                                    guest_list: vec![2, 3],
                                },
                            ),
                            (
                                4,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 4,
                                    guest_list: vec![5, 6],
                                },
                            ),
                            (
                                7,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 7,
                                    guest_list: vec![8, 9],
                                },
                            ),
                            (
                                10,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 10,
                                    guest_list: vec![11, 12, 13],
                                },
                            ),
                            (
                                3,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 3,
                                    guest_list: vec![6, 10],
                                },
                            ),
                            (
                                5,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 5,
                                    guest_list: vec![9, 1],
                                },
                            ),
                            (
                                8,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 8,
                                    guest_list: vec![12, 4],
                                },
                            ),
                            (
                                11,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 11,
                                    guest_list: vec![3],
                                },
                            ),
                            (
                                13,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 13,
                                    guest_list: vec![7],
                                },
                            ),
                            (
                                3,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 3,
                                    guest_list: vec![10, 5],
                                },
                            ),
                            (
                                6,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 6,
                                    guest_list: vec![1, 8, 13],
                                },
                            ),
                            (
                                9,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 9,
                                    guest_list: vec![4, 11],
                                },
                            ),
                            (
                                12,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 12,
                                    guest_list: vec![7, 2],
                                },
                            ),
                        ]),
                    },
                ),
                (
                    15,
                    Schema {
                        plan: HashMap::from([
                            (
                                1,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 1,
                                    guest_list: vec![2, 3],
                                },
                            ),
                            (
                                4,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 4,
                                    guest_list: vec![5, 6],
                                },
                            ),
                            (
                                7,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 7,
                                    guest_list: vec![8, 9],
                                },
                            ),
                            (
                                10,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 10,
                                    guest_list: vec![11, 12],
                                },
                            ),
                            (
                                13,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 13,
                                    guest_list: vec![14, 15],
                                },
                            ),
                            (
                                2,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 2,
                                    guest_list: vec![6, 13],
                                },
                            ),
                            (
                                5,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 5,
                                    guest_list: vec![9, 1],
                                },
                            ),
                            (
                                8,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 8,
                                    guest_list: vec![12, 4],
                                },
                            ),
                            (
                                11,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 11,
                                    guest_list: vec![15, 7],
                                },
                            ),
                            (
                                14,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 14,
                                    guest_list: vec![3, 10],
                                },
                            ),
                            (
                                3,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 3,
                                    guest_list: vec![13, 5],
                                },
                            ),
                            (
                                6,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 6,
                                    guest_list: vec![1, 8],
                                },
                            ),
                            (
                                9,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 9,
                                    guest_list: vec![4, 11],
                                },
                            ),
                            (
                                12,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 12,
                                    guest_list: vec![7, 14],
                                },
                            ),
                            (
                                15,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 15,
                                    guest_list: vec![10, 2],
                                },
                            ),
                        ]),
                    },
                ),
                (
                    16,
                    Schema {
                        plan: HashMap::from([
                            (
                                1,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 1,
                                    guest_list: vec![2, 3],
                                },
                            ),
                            (
                                4,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 4,
                                    guest_list: vec![5, 6],
                                },
                            ),
                            (
                                7,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 7,
                                    guest_list: vec![8, 9],
                                },
                            ),
                            (
                                10,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 10,
                                    guest_list: vec![11, 12],
                                },
                            ),
                            (
                                13,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 13,
                                    guest_list: vec![14, 15, 16],
                                },
                            ),
                            (
                                2,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 2,
                                    guest_list: vec![6, 13],
                                },
                            ),
                            (
                                5,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 5,
                                    guest_list: vec![9, 1],
                                },
                            ),
                            (
                                8,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 8,
                                    guest_list: vec![12, 4],
                                },
                            ),
                            (
                                11,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 11,
                                    guest_list: vec![15, 7],
                                },
                            ),
                            (
                                14,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 14,
                                    guest_list: vec![3],
                                },
                            ),
                            (
                                16,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 16,
                                    guest_list: vec![10],
                                },
                            ),
                            (
                                3,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 3,
                                    guest_list: vec![13, 5],
                                },
                            ),
                            (
                                6,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 6,
                                    guest_list: vec![1, 8, 16],
                                },
                            ),
                            (
                                9,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 9,
                                    guest_list: vec![4, 11],
                                },
                            ),
                            (
                                12,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 12,
                                    guest_list: vec![7, 14],
                                },
                            ),
                            (
                                15,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 15,
                                    guest_list: vec![10, 2],
                                },
                            ),
                        ]),
                    },
                ),
                (
                    17,
                    Schema {
                        plan: HashMap::from([
                            (
                                1,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 1,
                                    guest_list: vec![2, 3],
                                },
                            ),
                            (
                                4,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 4,
                                    guest_list: vec![5, 6],
                                },
                            ),
                            (
                                7,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 7,
                                    guest_list: vec![8, 9],
                                },
                            ),
                            (
                                10,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 10,
                                    guest_list: vec![11, 12],
                                },
                            ),
                            (
                                13,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 13,
                                    guest_list: vec![14, 15],
                                },
                            ),
                            (
                                16,
                                SchemaCourse {
                                    course_type: CourseType::Appetizer,
                                    host: 16,
                                    guest_list: vec![17],
                                },
                            ),
                            (
                                2,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 2,
                                    guest_list: vec![6, 13],
                                },
                            ),
                            (
                                5,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 5,
                                    guest_list: vec![9, 1],
                                },
                            ),
                            (
                                8,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 8,
                                    guest_list: vec![12, 4],
                                },
                            ),
                            (
                                11,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 11,
                                    guest_list: vec![15, 7],
                                },
                            ),
                            (
                                14,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 14,
                                    guest_list: vec![3, 16],
                                },
                            ),
                            (
                                17,
                                SchemaCourse {
                                    course_type: CourseType::MainCourse,
                                    host: 17,
                                    guest_list: vec![10],
                                },
                            ),
                            (
                                3,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 3,
                                    guest_list: vec![13, 5],
                                },
                            ),
                            (
                                6,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 6,
                                    guest_list: vec![1, 8, 17],
                                },
                            ),
                            (
                                9,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 9,
                                    guest_list: vec![4, 11, 16],
                                },
                            ),
                            (
                                12,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 12,
                                    guest_list: vec![7, 14],
                                },
                            ),
                            (
                                15,
                                SchemaCourse {
                                    course_type: CourseType::Dessert,
                                    host: 15,
                                    guest_list: vec![10, 2],
                                },
                            ),
                        ]),
                    },
                ),
            ]),
            target_lat: end_point.latitude,
            target_lon: end_point.longitude,
            team_list,
            sorted_course_list,
        })
    }
}

impl<'a> Calculator for SchemaCalculator<'a> {
    fn calculate(&self) -> Plan {
        let group_list = self.assign_groups();
        let course_list = self.assign_courses(&group_list);

        self.map_to_plan(course_list)
    }
}

impl<'a> SchemaCalculator<'a> {
    fn map_to_plan(&self, assigned_course_list: Vec<AssignedCourse>) -> Plan {
        let course_mapping = HashMap::from([
            (CourseType::Appetizer, self.sorted_course_list[0].id),
            (CourseType::MainCourse, self.sorted_course_list[1].id),
            (CourseType::Dessert, self.sorted_course_list[2].id),
        ]);

        let mut hosting_list = HashMap::new();
        let mut walking_path = HashMap::new();

        for assigned_course in assigned_course_list {
            let hosting_id = Uuid::new_v4();
            hosting_list.insert(
                hosting_id,
                Hosting {
                    course: course_mapping
                        .get(&assigned_course.course_type)
                        .expect("Course type not found in mapping")
                        .clone(),
                    host: assigned_course.host,
                    guest_list: assigned_course.guest_list.clone(),
                },
            );
            walking_path
                .entry(assigned_course.host)
                .or_insert_with(Vec::new)
                .push(hosting_id);
            for guest in assigned_course.guest_list {
                walking_path
                    .entry(guest)
                    .or_insert_with(Vec::new)
                    .push(hosting_id);
            }
        }

        Plan {
            hosting_list,
            walking_path,
        }
    }

    fn calculate_angle(&self, team: &Team) -> f64 {
        let delta_lon = team.point.longitude - self.target_lon;
        let delta_lat = team.point.latitude - self.target_lat;
        delta_lat.atan2(delta_lon)
    }

    fn calculate_distance(&self, point: &Point) -> f64 {
        let delta_lon = point.longitude - self.target_lon;
        let delta_lat = point.latitude - self.target_lat;
        (delta_lon.powi(2) + delta_lat.powi(2)).sqrt()
    }

    fn assign_groups(&self) -> Vec<Group<'a>> {
        let mut teams_with_angles: Vec<(&Team, f64)> = self
            .team_list
            .into_iter()
            .map(|team| {
                let angle = self.calculate_angle(&team);
                (team, angle)
            })
            .collect();

        teams_with_angles.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let mut group_list = vec![];

        for i in 0..(teams_with_angles.len() / 9) {
            let is_last_group = i == teams_with_angles.len() / 9 - 1;

            let mut group = Group::new();

            let num_teams_in_group = if is_last_group {
                9 + (teams_with_angles.len() % 9)
            } else {
                9
            };

            //Add teams between i*9 and i*9+num_teams_in_group to group.team_list
            let mut team_list = teams_with_angles[i * 9..i * 9 + num_teams_in_group]
                .iter()
                .map(|(team, _angle)| (*team, self.calculate_distance(&team.point)))
                .collect::<Vec<(&Team, f64)>>();

            team_list.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            group
                .sorted_team_list
                .extend_from_slice(&team_list.iter().map(|t| t.0).collect::<Vec<&Team>>());

            group_list.push(group);
        }

        group_list
    }

    fn assign_courses(&self, groups: &Vec<Group<'a>>) -> Vec<AssignedCourse> {
        let mut assigned_course_list = vec![];

        for group in groups {
            console::log_1(&format!("Group size: {}", group.sorted_team_list.len()).into());
            let schema = self
                .schema_list
                .get(&(group.sorted_team_list.len() as u8))
                .expect("No schema found for group size");

            let mapping = schema.create_mapping(&group.sorted_team_list);
            schema.plan.values().for_each(|schema_course| {
                console::debug_1(
                    &format!(
                        "Host: {:?}, Guest list for host {:?}: {:?}",
                        schema_course.host, schema_course.guest_list, mapping
                    )
                    .into(),
                );
                let host = mapping
                    .get(&schema_course.host)
                    .expect("Host index not found in mapping")
                    .clone();
                let guest_list = schema_course
                    .guest_list
                    .iter()
                    .map(|guest_idx| {
                        mapping
                            .get(guest_idx)
                            .expect("Guest index not found in mapping")
                            .clone()
                    })
                    .collect();
                let assigned_course = AssignedCourse {
                    course_type: schema_course.course_type,
                    host,
                    guest_list,
                };
                assigned_course_list.push(assigned_course);
            });
        }

        assigned_course_list.sort_by(|a, b| a.course_type.rank().cmp(&b.course_type.rank()));

        assigned_course_list
    }
}
