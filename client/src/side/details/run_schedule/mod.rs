use std::collections::HashMap;

use uuid::Uuid;

use crate::storage::{CourseData, HostingData, MeetingPointData, PlanData, TeamData};

pub mod run_schedule;

#[derive(Debug, Clone, PartialEq)]
pub struct Team {
    pub name: String,
    address: String,
    mail: Option<String>,
    phone: Option<String>,
    members: Option<u8>,
    diets: Option<String>,
}

impl Team {
    fn new(team_data: &TeamData) -> Self {
        Team {
            name: team_data.name.clone(),
            address: team_data.address.address.clone(),
            mail: team_data.mail.clone(),
            phone: team_data.phone.clone(),
            members: team_data.members,
            diets: team_data.diets.clone(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Course {
    name: String,
    time: String,
}

impl Course {
    fn new(course_data: &CourseData) -> Self {
        Course {
            name: course_data.name.clone(),
            time: course_data.time.format("%H:%M").to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MeetingPoint {
    name: String,
    time: String,
    address: String,
}

impl MeetingPoint {
    fn new(meeting_point_data: &MeetingPointData) -> Self {
        MeetingPoint {
            name: meeting_point_data.name.clone(),
            time: meeting_point_data.time.format("%H:%M").to_string(),
            address: meeting_point_data.address.address.clone(),
        }
    }
}

pub struct ProjectSchedule {
    team_map: HashMap<Uuid, Team>,
    course_map: HashMap<Uuid, Course>,
    start_point: Option<MeetingPoint>,
    end_point: Option<MeetingPoint>,
}

impl ProjectSchedule {
    pub fn new(
        team_data_list: &Vec<TeamData>,
        course_list: &Vec<CourseData>,
        start_point: &Option<MeetingPointData>,
        end_point: &Option<MeetingPointData>,
    ) -> Self {
        let team_map = team_data_list
            .iter()
            .map(|t| (t.id, Team::new(t)))
            .collect::<std::collections::HashMap<Uuid, Team>>();
        let course_map = course_list
            .iter()
            .map(|c| (c.id, Course::new(c)))
            .collect::<std::collections::HashMap<Uuid, Course>>();

        let start_meeting_point = start_point.as_ref().map(|s| MeetingPoint::new(s));
        let end_meeting_point = end_point.as_ref().map(|e| MeetingPoint::new(e));

        let project_schedule = ProjectSchedule {
            team_map,
            course_map,
            start_point: start_meeting_point,
            end_point: end_meeting_point,
        };

        project_schedule
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Schedule {
    pub host: Team,
    guest_list: Vec<Team>,
    walking_path: Vec<(Course, Team)>,
    start_point: Option<MeetingPoint>,
    end_point: Option<MeetingPoint>,
}

impl Schedule {
    pub fn default(
        has_start: bool,
        has_end: bool,
        number_of_courses: u8,
        number_of_guests: u8,
        host_course: u8,
        with_mail: bool,
        with_phone: bool,
        with_members: bool,
        with_diets: bool,
    ) -> Self {
        let host_name = "[Host Team Name]".to_string();
        let guest_name = "[Guest Team Name]".to_string();
        let address = "[Address]".to_string();
        let mail = Some("[Mail]".to_string());
        let phone = Some("[Phone]".to_string());
        let members = Some(2);
        let diets = Some("[Diets]".to_string());
        let course_name = "[Course Name]".to_string();
        let end_point_name = "[Point Name]".to_string();

        let host = Team {
            name: host_name.clone(),
            address: address.clone(),
            mail: if with_mail { mail.clone() } else { None },
            phone: if with_phone { phone.clone() } else { None },
            members: if with_members { members } else { None },
            diets: if with_diets { diets.clone() } else { None },
        };

        let guest_list = (0..number_of_guests)
            .map(|_| Team {
                name: guest_name.clone(),
                address: address.clone(),
                mail: if with_mail { mail.clone() } else { None },
                phone: if with_phone { phone.clone() } else { None },
                members: if with_members { members } else { None },
                diets: if with_diets { diets.clone() } else { None },
            })
            .collect::<Vec<Team>>();

        let walking_path = (0..number_of_courses)
            .map(|i| {
                let course = Course {
                    name: course_name.clone(),
                    time: "[HH:MM]".to_string(),
                };
                let team = if i + 1 == host_course {
                    host.clone()
                } else {
                    guest_list[0].clone()
                };
                (course, team)
            })
            .collect::<Vec<(Course, Team)>>();

        let start_point = if has_start {
            Some(MeetingPoint {
                name: "Start Point".to_string(),
                time: "[HH:MM]".to_string(),
                address: address.clone(),
            })
        } else {
            None
        };

        let end_point = if has_end {
            Some(MeetingPoint {
                name: end_point_name.clone(),
                time: "[HH:MM]".to_string(),
                address: address.clone(),
            })
        } else {
            None
        };

        Schedule {
            host,
            guest_list,
            walking_path,
            start_point,
            end_point,
        }
    }

    pub fn new(team_id: Uuid, plan: &PlanData, project_schedule: &ProjectSchedule) -> Self {
        let hosting_map = plan
            .hosting_list
            .iter()
            .map(|h| (h.id, h))
            .collect::<std::collections::HashMap<Uuid, &HostingData>>();

        let hosting_data = hosting_map
            .iter()
            .find(|(k, v)| v.host == team_id)
            .expect("Expect to find hosting for team")
            .1;

        Schedule {
            host: project_schedule
                .team_map
                .get(&hosting_data.host)
                .expect("Expect host team")
                .clone(),
            guest_list: hosting_data
                .guest_list
                .iter()
                .map(|g| {
                    project_schedule
                        .team_map
                        .get(g)
                        .expect("Expect guest team")
                        .clone()
                })
                .collect(),
            walking_path: plan
                .walking_path
                .get(&hosting_data.host)
                .expect("Expect to find walking path for host")
                .iter()
                .map(|c| {
                    let hosting = hosting_map.get(c).expect("Expect hosting data");

                    let course = project_schedule
                        .course_map
                        .get(&hosting.name)
                        .expect("Expect course")
                        .clone();
                    let team = project_schedule
                        .team_map
                        .get(&hosting.host)
                        .expect("Expect host team")
                        .clone();
                    (course, team)
                })
                .collect(),
            start_point: project_schedule.start_point.clone(),
            end_point: project_schedule.end_point.clone(),
        }
    }

    pub fn new_map(plan: &PlanData, project_schedule: &ProjectSchedule) -> HashMap<Uuid, Self> {
        let hosting_map = plan
            .hosting_list
            .iter()
            .map(|h| (h.id, h))
            .collect::<std::collections::HashMap<Uuid, &HostingData>>();
        let mut schedule_map = HashMap::new();
        for hosting_data in plan.hosting_list.iter() {
            let schedule = Schedule {
                host: project_schedule
                    .team_map
                    .get(&hosting_data.host)
                    .expect("Expect host team")
                    .clone(),
                guest_list: hosting_data
                    .guest_list
                    .iter()
                    .map(|g| {
                        project_schedule
                            .team_map
                            .get(g)
                            .expect("Expect guest team")
                            .clone()
                    })
                    .collect(),
                walking_path: plan
                    .walking_path
                    .get(&hosting_data.host)
                    .expect("Expect to find walking path for host")
                    .iter()
                    .map(|c| {
                        let hosting = hosting_map.get(c).expect("Expect hosting data");

                        let course = project_schedule
                            .course_map
                            .get(&hosting.name)
                            .expect("Expect course")
                            .clone();
                        let team = project_schedule
                            .team_map
                            .get(&hosting.host)
                            .expect("Expect host team")
                            .clone();
                        (course, team)
                    })
                    .collect(),
                start_point: project_schedule.start_point.clone(),
                end_point: project_schedule.end_point.clone(),
            };
            schedule_map.insert(hosting_data.host, schedule);
        }

        schedule_map
    }
}
