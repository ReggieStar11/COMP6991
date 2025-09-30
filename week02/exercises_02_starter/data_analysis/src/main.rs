const ENROLMENTS_PATH: &str = "enrolments.psv";

use std::collections::HashMap;

use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Data {
    course_code: String,
    student_id: i32,
    name: String,
    program: String,
    plan: String,
    wam: f64,
    session: String,
    birthdate: String,
    sex: String,
}
fn main() {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'|') // Set delimiter to pipe character for PSV
        .has_headers(false) // Your data doesn't have headers
        .from_path(ENROLMENTS_PATH)
        .expect("Failed to open file");

    let records: Vec<Data> = rdr
        .deserialize()
        .map(|result| result.expect("Failed to parse record"))
        .collect();

    let mut unique_students: HashMap<i32, &Data> = HashMap::new();

    for students in &records {
        unique_students.insert(students.student_id, students);
    }
    let uniq_students: Vec<&Data> = unique_students.values().copied().collect();
    
    
    println!("Number of students: {}", uniq_students.len());
    // Group records by course code
    let grouped = group_by_course(&records);

    let mut common = String::new();
    let mut uncommon = String::new();
    let mut common_num: usize = 0;
    let mut uncommon_num: usize = usize::MAX;  // Start with max value

    for (course, students) in &grouped {
        if students.len() > common_num.try_into().unwrap() {
            common_num = students.len();
            common = course.clone();
        }

        if students.len() < uncommon_num.try_into().unwrap() {
            uncommon_num = students.len();
            uncommon = course.clone();
        }

    }
    println!("Most common course: {} with {} students", common, common_num);
    println!("Least common course: {} with {} students", uncommon, uncommon_num);

    let average_wam: f64 = unique_students
    .values()
    .map(|student| student.wam)
    .sum::<f64>() / unique_students.len() as f64;

    println!("Average WAM: {:.2}", average_wam);


}

fn group_by_course(records: &[Data]) -> HashMap<String, Vec<&Data>> {
    let mut hashmap: HashMap<String, Vec<&Data>> = HashMap::new();

    for item in records {
        hashmap
            .entry(item.course_code.clone())
            .or_insert_with(Vec::new)
            .push(item); // Push the reference
    }

    hashmap
}
