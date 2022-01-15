// Price of a Single Big Mac, in AUD
pub const BIGMAC_PRICE: f32 = 6.40;
// Total number of BigMacs eaten by Our Hero
// Whcih gives us one Gorske unit
const GORSKE_TOTAL: f32 = 30000.0;

/// Gets us a const for Gorske unit
pub fn get_gorske() -> f32 {
    BIGMAC_PRICE * GORSKE_TOTAL
}

/// Calculates our wasted cost in Dollarydoos
pub fn calculate_cost(hours: f32, hourly_rate: f32, staff_count: f32) -> f32 {
    hours * hourly_rate * staff_count as f32 * 52.0
}

/// Converts a yearly salary into a hourly salary
pub fn calculate_hourly_salary(yearly_salary: f32) -> f32 {
    yearly_salary as f32 / 52.0 / 40.0
}

/// Calculates a given cost and outputs in Gorske
pub fn calculate_gorske_cost(hours: f32, hourly_rate: f32, staff_count: f32) -> f32 {
    calculate_cost(hours, hourly_rate, staff_count) / get_gorske()
}

/// Calculates a given cost and outputs in BigMac's
pub fn calculate_bigmac_cost(hours: f32, hourly_rate: f32, staff_count: f32) -> f32 {
    calculate_cost(hours, hourly_rate, staff_count) / BIGMAC_PRICE
}
