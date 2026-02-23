use std::collections::HashMap;

use uuid::Uuid;

use super::{CourseData, HostingData, PlanData, TeamData};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Hosting {
    pub id: Uuid,
    pub course: CourseData,
    pub host: TeamData,
    pub guest_list: Vec<TeamData>,
}

impl Hosting {
    fn from_hosting_data(
        hosting_data: &HostingData,
        course_list: &Vec<CourseData>,
        team_list: &Vec<TeamData>,
    ) -> Self {
        Hosting {
            id: hosting_data.id,
            course: find_course(hosting_data.name, course_list)
                .expect("Expect course")
                .clone(),
            host: find_team(hosting_data.host, team_list)
                .expect("Expect team")
                .clone(),
            guest_list: hosting_data
                .guest_list
                .iter()
                .map(|&g| find_team(g, team_list).expect("Expect team").clone())
                .collect(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Plan {
    pub hosting_list: Vec<Hosting>,
    pub walking_path: HashMap<TeamData, Vec<Hosting>>,
}

impl Plan {
    pub fn from_plan_data(
        plan_data: &PlanData,
        course_list: &Vec<CourseData>,
        team_list: &Vec<TeamData>,
    ) -> Self {
        let hosting_list: Vec<Hosting> = plan_data
            .hosting_list
            .iter()
            .map(|h| Hosting::from_hosting_data(h, course_list, team_list))
            .collect();
        let walking_path: HashMap<TeamData, Vec<Hosting>> = plan_data
            .walking_path
            .iter()
            .map(|(&team_id, hosting_ids)| {
                let team = find_team(team_id, team_list).expect("Expect team").clone();
                let hostings: Vec<Hosting> = hosting_ids
                    .iter()
                    .map(|&hosting_id| {
                        let host: Hosting = find_hosting(hosting_id, &hosting_list)
                            .expect("Expect hosting")
                            .clone();
                        host
                    })
                    .collect();
                (team, hostings)
            })
            .collect();

        Plan {
            hosting_list,
            walking_path,
        }
    }
}

fn find_team(id: Uuid, team_list: &Vec<TeamData>) -> Option<&TeamData> {
    for team in team_list.iter() {
        if team.id == id {
            return Some(team);
        }
    }
    None
}

fn find_course(id: Uuid, course_list: &Vec<CourseData>) -> Option<&CourseData> {
    for course in course_list.iter() {
        if course.id == id {
            return Some(course);
        }
    }
    None
}

fn find_hosting(id: Uuid, hosting_list: &Vec<Hosting>) -> Option<&Hosting> {
    for hosting in hosting_list.iter() {
        if hosting.id == id {
            return Some(hosting);
        }
    }
    None
}
