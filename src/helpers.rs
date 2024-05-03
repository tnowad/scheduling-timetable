pub fn filter_classes(classes: &[Class], subject_codes: &[&str], days: &[i32]) -> Vec<Class> {
    classes
        .iter()
        .filter(|class| {
            subject_codes.contains(&class.subject_code.as_str())
                && class
                    .timetable()
                    .iter()
                    .any(|period| days.contains(&period.day))
        })
        .cloned()
        .collect()
}

