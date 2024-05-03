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

pub fn get_period_number(time: &str, periods: &[PeriodTime]) -> Option<i32> {
    periods
        .iter()
        .find(|&period| time >= period.start_time && time <= period.end_time)
        .map(|period| period.period)
}

pub fn draw_schedule(schedule: &[Class]) {
    let mut timetable: Vec<Vec<String>> = vec![vec!["".to_string(); 8]; 14];

    let mut short_names = HashMap::new();
    let mut short_name_counters = HashMap::new();
    for class in schedule.iter() {
        let first_letter = class.subject_name.chars().next().unwrap();
        let group_number = &class.group;
        let counter = short_name_counters
            .entry((first_letter, group_number))
            .or_insert(1);
        let short_name = format!("{}{:02}", first_letter, counter);
        short_names.insert(&class.class_id, short_name.clone());
        *counter += 1;

        println!(
            "Short Name - {}, Class - {}, Group - {}, Subject Code - {}, Subject Name - {}",
            short_name, class.class_id, class.group, class.subject_code, class.subject_name
        );
    }

    for class in schedule.iter() {
        for period in class.timetable().iter() {
            timetable[period.period as usize][period.day as usize] =
                short_names[&class.class_id].clone();
        }
    }

    println!("Timetable:");

    (2..=7).for_each(|i| {
        timetable[0][i] = i.to_string();
    });

    (1..=13).for_each(|i| {
        timetable[i][0] = i.to_string();
    });

    for row in timetable.iter() {
        for cell in row.iter() {
            print!("{:3} ", cell);
        }
        println!();
    }
}
