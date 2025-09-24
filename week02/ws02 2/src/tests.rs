#[cfg(test)]
use crate::*;

#[test]
pub fn test_most_least_used_stations() {
    let data = data_preprocessing().unwrap();
    // There are two least used stations, both with 170 total uses. You can return either of them.
    assert!(matches!(
        most_least_used_stations(&data, TimeOfDay::Morning),
        Some((min, max)) if
            (min == "Bowral" && max == "Central")
            || (min == "Lithgow" && max == "Central")
    ));
}

#[test]
pub fn test_search_station_busiest_times_of_day() {
    let data = data_preprocessing().unwrap();
    assert_eq!(
        sorted(vec![
            (TimeOfDay::Morning, 374980),
            (TimeOfDay::Midday, 284220),
            (TimeOfDay::Evening, 424160),
            (TimeOfDay::Midnight, 167710)
        ]),
        sorted(search_station_busiest_times_of_day(&data, "Central").unwrap())
    );
}

#[test]
pub fn test_search_station_busiest_year() {
    let data = data_preprocessing().unwrap();
    assert_eq!(
        Some("2020".to_string()),
        search_station_busiest_year(&data, "Central")
    );
}

#[test]
pub fn test_find_largest_yearly_utilisation_increase() {
    let data = data_preprocessing().unwrap();
    assert_eq!(
        Some("Town Hall".to_string()),
        find_largest_yearly_utilisation_increase(&data)
    );
}

#[test]
pub fn test_find_biggest_percentage_change() {
    let data = data_preprocessing().unwrap();
    assert_eq!(
        Some("Olympic Park".to_string()),
        find_biggest_percentage_change(&data)
    );
}

#[test]
pub fn test_find_north_most_station() {
    let data = data_preprocessing().unwrap();
    assert_eq!(
        Some("Singleton".to_string()),
        find_north_most_station(&data)
    );
}

#[test]
pub fn test_find_south_most_station() {
    let data = data_preprocessing().unwrap();
    assert_eq!(
        Some("Bomaderry".to_string()),
        find_south_most_station(&data)
    );
}

#[test]
pub fn test_find_east_most_station() {
    let data = data_preprocessing().unwrap();
    assert_eq!(
        Some("Newcastle Interchange".to_string()),
        find_east_most_station(&data)
    );
}

#[test]
pub fn test_find_west_most_station() {
    let data = data_preprocessing().unwrap();
    assert_eq!(Some("Bathurst".to_string()), find_west_most_station(&data));
}

#[cfg(test)]
fn sorted<T: Ord>(mut vec: Vec<T>) -> Vec<T> {
    vec.sort();
    vec
}

#[test]
pub fn test_find_closest_stations() {
    let data = data_preprocessing().unwrap();
    assert_eq!(
        Some(sorted(vec![
            "Martin Place".to_string(),
            "St James".to_string()
        ])),
        find_two_closest_stations(&data).map(|(a, b)| sorted(vec![a, b]))
    );
}

#[test]
pub fn test_find_furthest_stations() {
    let data = data_preprocessing().unwrap();
    assert_eq!(
        Some(sorted(vec![
            "East Maitland".to_string(),
            "Goulburn".to_string()
        ])),
        find_two_furthest_stations(&data).map(|(a, b)| sorted(vec![a, b]))
    );
}
