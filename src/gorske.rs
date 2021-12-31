pub const BIGMAC_PRICE: f32 = 6.40; // Price of a Single Big Mac, in AUD
const GORSKE_TOTAL: i32 = 30000; //Total number of Gorske eaten by our Hero

pub fn get_gorske() -> f32 {
    let g: f32 = BIGMAC_PRICE * GORSKE_TOTAL as f32;
    g
}

pub fn calculate_cost(hours: f32, hourly_rate: f32, staff_count: i32) -> f32 {
    // Calculates our wasted cost

    let cost = hours * hourly_rate as f32 * staff_count as f32 * 52 as f32;
    cost
 
}

pub fn calculate_hourly_salary(yearly_salary: i32) -> f32 {
    // Converts a yearly salary into a hourly salary
    let hourly: f32 = yearly_salary as f32 / 52.0 / 40.0;
    hourly
}

pub fn calculate_gorkse_cost(hours: f32, hourly_rate: f32, staff_count: i32) -> f32 {
    calculate_cost(hours, hourly_rate, staff_count) / get_gorske()
}


pub fn calculate_bigmac_cost(hours: f32, hourly_rate: f32, staff_count: i32) -> f32 {
    calculate_cost(hours, hourly_rate, staff_count) / BIGMAC_PRICE
}