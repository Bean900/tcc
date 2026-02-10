use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::LazyLock;
use std::vec;

#[derive(Hash, Eq, PartialEq)]
enum CourseType {
    Appetizer,
    MainCourse,
    Dessert,
}

struct Course {
    course_id: u8,
    course_type: CourseType,
    host: u8,
    guest_list: Vec<u8>,
}

struct Vector {
    angle: f64,
    team_list: Vec<u8>,
}

impl Vector {
    fn new(angle: f64) -> Self {
        Self {
            angle,
            team_list: Vec::new(),
        }
    }
}

struct VectorAssignment {
    target_lat: f64,
    target_lon: f64,
    vector_list: Vec<Vector>,
}

impl VectorAssignment {
    fn new(target_lat: f64, target_lon: f64, total_teams: u8) -> Self {
        let num_vectors = (total_teams + 8) / 9;

        let mut vector_list = Vec::new();
        for i in 0..num_vectors {
            let angle = (i as f64 * 2.0 * PI) / (num_vectors as f64);
            vector_list.push(Vector::new(angle));
        }

        Self {
            target_lat,
            target_lon,
            vector_list,
        }
    }
}

struct Schema {
    number_of_teams: u8,
    plan: HashMap<CourseType, Vec<Course>>,
}

static SCHEMA_LIST: LazyLock<Vec<Schema>> = LazyLock::new(|| {
    vec![
        Schema {
            number_of_teams: 9,
            plan: HashMap::from([
                (
                    CourseType::Appetizer,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Appetizer,
                            host: 1,
                            guest_list: vec![2, 3],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Appetizer,
                            host: 4,
                            guest_list: vec![5, 6],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Appetizer,
                            host: 7,
                            guest_list: vec![8, 9],
                        },
                    ],
                ),
                (
                    CourseType::MainCourse,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::MainCourse,
                            host: 5,
                            guest_list: vec![1, 9],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::MainCourse,
                            host: 2,
                            guest_list: vec![6, 7],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::MainCourse,
                            host: 8,
                            guest_list: vec![3, 4],
                        },
                    ],
                ),
                (
                    CourseType::Dessert,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Dessert,
                            host: 3,
                            guest_list: vec![5, 7],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Dessert,
                            host: 6,
                            guest_list: vec![1, 8],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Dessert,
                            host: 9,
                            guest_list: vec![4, 2],
                        },
                    ],
                ),
            ]),
        },
        Schema {
            number_of_teams: 10,
            plan: HashMap::from([
                (
                    CourseType::Appetizer,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Appetizer,
                            host: 1,
                            guest_list: vec![2, 3],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Appetizer,
                            host: 4,
                            guest_list: vec![5, 6],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Appetizer,
                            host: 7,
                            guest_list: vec![8, 9, 10],
                        },
                    ],
                ),
                (
                    CourseType::MainCourse,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::MainCourse,
                            host: 5,
                            guest_list: vec![1, 9],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::MainCourse,
                            host: 2,
                            guest_list: vec![7],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::MainCourse,
                            host: 8,
                            guest_list: vec![3, 4],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::MainCourse,
                            host: 10,
                            guest_list: vec![6],
                        },
                    ],
                ),
                (
                    CourseType::Dessert,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Dessert,
                            host: 3,
                            guest_list: vec![5, 7],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Dessert,
                            host: 6,
                            guest_list: vec![1, 8],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Dessert,
                            host: 9,
                            guest_list: vec![4, 2, 10],
                        },
                    ],
                ),
            ]),
        },
        Schema {
            number_of_teams: 11,
            plan: HashMap::from([
                (
                    CourseType::Appetizer,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Appetizer,
                            host: 1,
                            guest_list: vec![2, 3],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Appetizer,
                            host: 4,
                            guest_list: vec![5, 6, 11],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Appetizer,
                            host: 7,
                            guest_list: vec![8, 9, 10],
                        },
                    ],
                ),
                (
                    CourseType::MainCourse,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::MainCourse,
                            host: 5,
                            guest_list: vec![1, 9],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::MainCourse,
                            host: 2,
                            guest_list: vec![7, 6],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::MainCourse,
                            host: 8,
                            guest_list: vec![3, 4],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::MainCourse,
                            host: 10,
                            guest_list: vec![11],
                        },
                    ],
                ),
                (
                    CourseType::Dessert,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Dessert,
                            host: 3,
                            guest_list: vec![5, 7, 11],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Dessert,
                            host: 6,
                            guest_list: vec![1, 8],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Dessert,
                            host: 9,
                            guest_list: vec![4, 2, 10],
                        },
                    ],
                ),
            ]),
        },
        Schema {
            number_of_teams: 12,
            plan: HashMap::from([
                (
                    CourseType::Appetizer,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Appetizer,
                            host: 1,
                            guest_list: vec![2, 3],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Appetizer,
                            host: 4,
                            guest_list: vec![5, 6],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Appetizer,
                            host: 7,
                            guest_list: vec![8, 9],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::Appetizer,
                            host: 10,
                            guest_list: vec![11, 12],
                        },
                    ],
                ),
                (
                    CourseType::MainCourse,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::MainCourse,
                            host: 5,
                            guest_list: vec![1, 10],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::MainCourse,
                            host: 2,
                            guest_list: vec![7, 12],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::MainCourse,
                            host: 8,
                            guest_list: vec![3, 4],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::MainCourse,
                            host: 11,
                            guest_list: vec![6, 9],
                        },
                    ],
                ),
                (
                    CourseType::Dessert,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Dessert,
                            host: 3,
                            guest_list: vec![7, 11],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Dessert,
                            host: 6,
                            guest_list: vec![1, 4],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Dessert,
                            host: 9,
                            guest_list: vec![2, 10],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::Dessert,
                            host: 12,
                            guest_list: vec![5, 8],
                        },
                    ],
                ),
            ]),
        },
        Schema {
            number_of_teams: 13,
            plan: HashMap::from([
                (
                    CourseType::Appetizer,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Appetizer,
                            host: 1,
                            guest_list: vec![2, 3],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Appetizer,
                            host: 4,
                            guest_list: vec![5, 6],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Appetizer,
                            host: 7,
                            guest_list: vec![8, 9, 13],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::Appetizer,
                            host: 10,
                            guest_list: vec![11, 12],
                        },
                    ],
                ),
                (
                    CourseType::MainCourse,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::MainCourse,
                            host: 5,
                            guest_list: vec![1],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::MainCourse,
                            host: 2,
                            guest_list: vec![7, 12],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::MainCourse,
                            host: 8,
                            guest_list: vec![3, 4],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::MainCourse,
                            host: 11,
                            guest_list: vec![6, 9],
                        },
                        Course {
                            course_id: 5,
                            course_type: CourseType::MainCourse,
                            host: 13,
                            guest_list: vec![10],
                        },
                    ],
                ),
                (
                    CourseType::Dessert,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Dessert,
                            host: 3,
                            guest_list: vec![7, 11],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Dessert,
                            host: 6,
                            guest_list: vec![1, 4, 13],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Dessert,
                            host: 9,
                            guest_list: vec![2, 10],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::Dessert,
                            host: 12,
                            guest_list: vec![5, 8],
                        },
                    ],
                ),
            ]),
        },
        Schema {
            number_of_teams: 14,
            plan: HashMap::from([
                (
                    CourseType::Appetizer,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Appetizer,
                            host: 1,
                            guest_list: vec![2, 3],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Appetizer,
                            host: 4,
                            guest_list: vec![5, 6, 13],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Appetizer,
                            host: 7,
                            guest_list: vec![8, 9, 14],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::Appetizer,
                            host: 10,
                            guest_list: vec![11, 12],
                        },
                    ],
                ),
                (
                    CourseType::MainCourse,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::MainCourse,
                            host: 5,
                            guest_list: vec![1, 10],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::MainCourse,
                            host: 2,
                            guest_list: vec![7, 12],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::MainCourse,
                            host: 8,
                            guest_list: vec![3, 4],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::MainCourse,
                            host: 11,
                            guest_list: vec![6, 9],
                        },
                        Course {
                            course_id: 5,
                            course_type: CourseType::MainCourse,
                            host: 13,
                            guest_list: vec![14],
                        },
                    ],
                ),
                (
                    CourseType::Dessert,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Dessert,
                            host: 3,
                            guest_list: vec![7, 11, 13],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Dessert,
                            host: 6,
                            guest_list: vec![1, 4, 14],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Dessert,
                            host: 9,
                            guest_list: vec![2, 10],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::Dessert,
                            host: 12,
                            guest_list: vec![5, 8],
                        },
                    ],
                ),
            ]),
        },
        Schema {
            number_of_teams: 15,
            plan: HashMap::from([
                (
                    CourseType::Appetizer,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Appetizer,
                            host: 1,
                            guest_list: vec![2, 3],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Appetizer,
                            host: 4,
                            guest_list: vec![5, 6],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Appetizer,
                            host: 7,
                            guest_list: vec![8, 9],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::Appetizer,
                            host: 10,
                            guest_list: vec![11, 12],
                        },
                        Course {
                            course_id: 5,
                            course_type: CourseType::Appetizer,
                            host: 13,
                            guest_list: vec![14, 15],
                        },
                    ],
                ),
                (
                    CourseType::MainCourse,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::MainCourse,
                            host: 5,
                            guest_list: vec![10, 13],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::MainCourse,
                            host: 2,
                            guest_list: vec![7, 15],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::MainCourse,
                            host: 8,
                            guest_list: vec![3, 4],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::MainCourse,
                            host: 11,
                            guest_list: vec![6, 9],
                        },
                        Course {
                            course_id: 5,
                            course_type: CourseType::MainCourse,
                            host: 14,
                            guest_list: vec![1, 12],
                        },
                    ],
                ),
                (
                    CourseType::Dessert,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Dessert,
                            host: 3,
                            guest_list: vec![7, 11],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Dessert,
                            host: 6,
                            guest_list: vec![1, 13],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Dessert,
                            host: 9,
                            guest_list: vec![10, 14],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::Dessert,
                            host: 12,
                            guest_list: vec![2, 5],
                        },
                        Course {
                            course_id: 5,
                            course_type: CourseType::Dessert,
                            host: 15,
                            guest_list: vec![4, 8],
                        },
                    ],
                ),
            ]),
        },
        Schema {
            number_of_teams: 16,
            plan: HashMap::from([
                (
                    CourseType::Appetizer,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Appetizer,
                            host: 1,
                            guest_list: vec![2, 3, 16],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Appetizer,
                            host: 4,
                            guest_list: vec![5, 6],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Appetizer,
                            host: 7,
                            guest_list: vec![8, 9],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::Appetizer,
                            host: 10,
                            guest_list: vec![11, 12],
                        },
                        Course {
                            course_id: 5,
                            course_type: CourseType::Appetizer,
                            host: 13,
                            guest_list: vec![14, 15],
                        },
                    ],
                ),
                (
                    CourseType::MainCourse,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::MainCourse,
                            host: 5,
                            guest_list: vec![10, 13],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::MainCourse,
                            host: 2,
                            guest_list: vec![7],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::MainCourse,
                            host: 8,
                            guest_list: vec![3, 4],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::MainCourse,
                            host: 11,
                            guest_list: vec![6, 9],
                        },
                        Course {
                            course_id: 5,
                            course_type: CourseType::MainCourse,
                            host: 14,
                            guest_list: vec![1, 12],
                        },
                        Course {
                            course_id: 6,
                            course_type: CourseType::MainCourse,
                            host: 16,
                            guest_list: vec![15],
                        },
                    ],
                ),
                (
                    CourseType::Dessert,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Dessert,
                            host: 3,
                            guest_list: vec![7, 11],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Dessert,
                            host: 6,
                            guest_list: vec![1, 13],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Dessert,
                            host: 9,
                            guest_list: vec![10, 14, 16],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::Dessert,
                            host: 12,
                            guest_list: vec![2, 5],
                        },
                        Course {
                            course_id: 5,
                            course_type: CourseType::Dessert,
                            host: 15,
                            guest_list: vec![4, 8],
                        },
                    ],
                ),
            ]),
        },
        Schema {
            number_of_teams: 17,
            plan: HashMap::from([
                (
                    CourseType::Appetizer,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Appetizer,
                            host: 1,
                            guest_list: vec![2, 3, 16],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Appetizer,
                            host: 4,
                            guest_list: vec![5, 6, 17],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Appetizer,
                            host: 7,
                            guest_list: vec![8, 9],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::Appetizer,
                            host: 10,
                            guest_list: vec![11, 12],
                        },
                        Course {
                            course_id: 5,
                            course_type: CourseType::Appetizer,
                            host: 13,
                            guest_list: vec![14, 15],
                        },
                    ],
                ),
                (
                    CourseType::MainCourse,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::MainCourse,
                            host: 5,
                            guest_list: vec![10, 13],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::MainCourse,
                            host: 2,
                            guest_list: vec![7],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::MainCourse,
                            host: 8,
                            guest_list: vec![3, 4],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::MainCourse,
                            host: 11,
                            guest_list: vec![6, 9],
                        },
                        Course {
                            course_id: 5,
                            course_type: CourseType::MainCourse,
                            host: 14,
                            guest_list: vec![1, 12],
                        },
                        Course {
                            course_id: 6,
                            course_type: CourseType::MainCourse,
                            host: 16,
                            guest_list: vec![15, 17],
                        },
                    ],
                ),
                (
                    CourseType::Dessert,
                    vec![
                        Course {
                            course_id: 1,
                            course_type: CourseType::Dessert,
                            host: 3,
                            guest_list: vec![7, 11],
                        },
                        Course {
                            course_id: 2,
                            course_type: CourseType::Dessert,
                            host: 6,
                            guest_list: vec![1, 13],
                        },
                        Course {
                            course_id: 3,
                            course_type: CourseType::Dessert,
                            host: 9,
                            guest_list: vec![10, 14, 16],
                        },
                        Course {
                            course_id: 4,
                            course_type: CourseType::Dessert,
                            host: 12,
                            guest_list: vec![2, 5, 17],
                        },
                        Course {
                            course_id: 5,
                            course_type: CourseType::Dessert,
                            host: 15,
                            guest_list: vec![4, 8],
                        },
                    ],
                ),
            ]),
        },
    ]
});
