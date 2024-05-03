use std::collections::HashSet;

use crate::structs::{Class, Period};

pub fn generate_schedules_by_periods(
    classes: &[Class],
    subject_codes: &[&str],
    periods: &[Period],
    current_schedule: &mut Vec<Class>,
    schedules: &mut HashSet<Vec<Class>>,
    selected_periods: &HashSet<Period>,
) {
    let all_subject_codes_included = {
        let included_subject_codes: HashSet<_> = current_schedule
            .iter()
            .map(|class| class.subject_code.as_str())
            .collect();
        included_subject_codes.len() == subject_codes.len()
    };

    if all_subject_codes_included {
        schedules.insert(current_schedule.clone());
        return;
    }

    for class in classes {
        if current_schedule
            .iter()
            .any(|c| c.subject_code == class.subject_code)
        {
            continue;
        }

        let can_schedule = class.timetable().iter().all(|class_period| {
            !selected_periods.contains(class_period) && periods.iter().any(|p| *p == *class_period)
        });

        if can_schedule {
            current_schedule.push(class.clone());
            let new_selected_periods: HashSet<_> = selected_periods
                .union(&class.timetable().iter().cloned().collect())
                .cloned()
                .collect();
            generate_schedules_by_periods(
                classes,
                subject_codes,
                periods,
                current_schedule,
                schedules,
                &new_selected_periods,
            );
            current_schedule.pop();
        }
    }
}

pub fn generate_schedules(
    classes: &[Class],
    subject_codes: &[&str],
    periods: &[Period],
) -> HashSet<Vec<Class>> {
    let mut schedules: HashSet<Vec<Class>> = HashSet::new();
    generate_schedules_by_periods(
        classes,
        subject_codes,
        periods,
        &mut Vec::new(),
        &mut schedules,
        &HashSet::new(),
    );
    schedules
}
