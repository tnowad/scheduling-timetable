#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Period {
    pub period: i32,
    pub day: i32,
}

pub struct PeriodTime {
    pub period: i32,
    pub start_time: &'static str,
    pub end_time: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Session {
    pub(crate) periods: Vec<Period>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Class {
    pub class_id: String,
    pub subject_id: String,
    pub subject_code: String,
    pub subject_name: String,
    pub group: String,
    pub registered_count: i32,
    pub permitted_count: i32,
    pub remaining_count: i32,
    pub sessions: Vec<Session>,
}

impl Class {
    pub fn timetable(&self) -> Vec<Period> {
        self.sessions
            .iter()
            .flat_map(|session| session.periods.iter().cloned())
            .collect()
    }
}
