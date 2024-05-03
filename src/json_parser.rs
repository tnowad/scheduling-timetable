
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
            })
        })
        .collect();
    Ok(classes)
}
