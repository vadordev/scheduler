use chrono;
use serde::Deserialize;
use std::collections::HashMap;
use toml::de::from_str;

pub struct SingleShift {
    pub start: chrono::NaiveTime,
    pub end: chrono::NaiveTime,
}

pub fn parse_shifts(shifts: String) -> Option<HashMap<String, SingleShift>> {
    match from_str::<ShiftsToml>(shifts.as_str()) {
        Ok(shifts) => {
            let shifts = shifts.shifts;

            let mut return_shifts = HashMap::with_capacity(shifts.len());

            for single_shift in shifts {
                let (name, parsed_shift) = match parse_single_shift(&single_shift) {
                    Ok(parsed_shift) => parsed_shift,
                    Err(_) => return None,
                };
                return_shifts.insert(name, parsed_shift);
            }

            Some(return_shifts)
        }
        Err(_) => None,
    }
}

fn parse_single_shift(shift: &SerializedSingleShift) -> chrono::ParseResult<(String, SingleShift)> {
    let parse = chrono::NaiveTime::parse_from_str;
    let fmt = "%H:%M";
    let start = parse(shift.start.as_str(), &fmt)?;
    let end = parse(shift.end.as_str(), &fmt)?;

    Ok((shift.name.clone(), SingleShift { start, end }))
}

#[derive(Deserialize)]
struct SerializedSingleShift {
    name: String,
    start: String,
    end: String,
}

#[derive(Deserialize)]
struct ShiftsToml {
    shifts: Vec<SerializedSingleShift>,
}

// enum Subject {
//     Everyone,
//     User(userid),
// }

// struct Constraint {
//     subject: Subject,
// }
