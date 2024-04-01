struct WeatherData(&'static str, f32, u32);

fn print_weekly(weekly: [WeatherData; 7]) {
    println!("Weekly weather:");
    for data in weekly.iter() {
        println!("Day: {}, Temperature: {:.1}°C, Humidity: {:.1}%", data.0, data.1, data.2);
    }
}

fn main() {
    let weekly: [WeatherData; 7] = [
        WeatherData("Monday", 26.25, 60),
        WeatherData("Tuesday", 26.5, 62),
        WeatherData("Wednesday", 25.75, 58),
        WeatherData("Thursday", 24.9, 55),
        WeatherData("Friday", 25.5, 57),
        WeatherData("Saturday", 27.0, 63),
        WeatherData("Sunday", 27.5, 65),
    ];
    print_weekly(weekly);
}

