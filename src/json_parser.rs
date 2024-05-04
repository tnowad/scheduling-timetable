use serde_json::Value;

use regex::Regex;

use crate::{
    helpers::get_period_number,
    structs::{Class, Period, PeriodTime, Session},
};

const PERIOD_TIMES: [PeriodTime; 13] = [
    PeriodTime {
        period: 1,
        start_time: "07:00",
        end_time: "07:50",
    },
    PeriodTime {
        period: 2,
        start_time: "07:50",
        end_time: "08:40",
    },
    PeriodTime {
        period: 3,
        start_time: "09:00",
        end_time: "09:50",
    },
    PeriodTime {
        period: 4,
        start_time: "09:50",
        end_time: "10:40",
    },
    PeriodTime {
        period: 5,
        start_time: "10:40",
        end_time: "11:30",
    },
    PeriodTime {
        period: 6,
        start_time: "13:00",
        end_time: "13:50",
    },
    PeriodTime {
        period: 7,
        start_time: "13:50",
        end_time: "14:40",
    },
    PeriodTime {
        period: 8,
        start_time: "15:00",
        end_time: "15:50",
    },
    PeriodTime {
        period: 9,
        start_time: "15:50",
        end_time: "16:40",
    },
    PeriodTime {
        period: 10,
        start_time: "16:40",
        end_time: "17:30",
    },
    PeriodTime {
        period: 11,
        start_time: "17:40",
        end_time: "18:30",
    },
    PeriodTime {
        period: 12,
        start_time: "18:30",
        end_time: "19:20",
    },
    PeriodTime {
        period: 13,
        start_time: "19:20",
        end_time: "20:10",
    },
];

pub fn parse_period_string(session: &str) -> Vec<Period> {
    let re = Regex::new(r"Thứ (\d+),từ (\d+:\d+) (?:đến|đến\s+(\d+:\d+)),Ph (\S+),GV (.+),(\d+/\d+/\d+)(?: đến (\d+/\d+/\d+))?").unwrap();

    session
        .split("<hr>")
        .flat_map(|session| {
            re.captures_iter(session).flat_map(|cap| {
                let day = cap[1].parse::<i32>().unwrap();
                let start_time = &cap[2];
                let end_time = cap.get(3).map_or("", |m| m.as_str());
                let start_period = get_period_number(start_time, &PERIOD_TIMES).unwrap();
                let end_period = get_period_number(end_time, &PERIOD_TIMES).unwrap_or(start_period);
                (start_period..=end_period).map(move |period| Period { period, day })
            })
        })
        .collect()
}

pub fn parse_sessions(schedule: &str) -> Vec<Session> {
    schedule
        .split("<br>")
        .map(|session| Session {
            periods: parse_period_string(session),
        })
        .collect()
}

pub fn parse_json(json_str: &str) -> Result<Vec<Class>, serde_json::Error> {
    let root: Value = serde_json::from_str(json_str)?;
    let classes: Vec<Class> = root["data"]["ds_nhom_to"]
        .as_array()
        .unwrap_or_else(|| panic!("Expected ds_nhom_to to be an array"))
        .iter()
        .filter_map(|item| {
            let class_data = item.as_object()?;
            let subject_name = class_data["ma_mon"].as_str().and_then(|ma_mon| {
                root["data"]["ds_mon_hoc"]
                    .as_array()?
                    .iter()
                    .find(|mon_hoc| mon_hoc["ma"].as_str() == Some(ma_mon))
                    .and_then(|mon_hoc| mon_hoc["ten"].as_str())
            })?;

            Some(Class {
                class_id: class_data["id_to_hoc"].as_str()?.to_owned(),
                subject_id: class_data["id_mon"].as_str()?.to_owned(),
                subject_code: class_data["ma_mon"].as_str()?.to_owned(),
                subject_name: subject_name.to_owned(),
                group: class_data["nhom_to"].as_str()?.to_owned(),
                registered_count: class_data["sl_dk"].as_i64().unwrap_or(0) as i32,
                permitted_count: class_data["sl_cp"].as_i64().unwrap_or(0) as i32,
                remaining_count: class_data["sl_cl"].as_i64().unwrap_or(0) as i32,
                sessions: parse_sessions(class_data["tkb"].as_str().expect("")),
            })
        })
        .collect();
    Ok(classes)
}
