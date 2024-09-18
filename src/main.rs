use std::collections::HashMap;

fn group_temperatures(temps: Vec<f64>) -> HashMap<i32, Vec<f64>> {
    let mut grouped_temps: HashMap<i32, Vec<f64>> = HashMap::new();

    for &temp in temps.iter() {
        let lower_bound = (temp / 10.0).floor() as i32 * 10;
        grouped_temps.entry(lower_bound).or_insert(Vec::new()).push(temp);
    }

    for temps in grouped_temps.values_mut() {
        temps.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    grouped_temps
}

fn main() {
    let temperatures = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];

    let result = group_temperatures(temperatures);

    for (interval, temps) in result.iter() {
        println!("[{}, {}): {:?}", interval, interval + 10, temps);
    }
}
