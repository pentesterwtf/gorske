use crate::{gorske::*, text_input::TextInput};
use yew::prelude::*;

/// Used to send a Message notifying a state change from UI functions to code
///
/// i.e. a text field is updated, which will send a StateChangeMessage, which is handled
/// by the framework to process
pub enum StateChangeMessage {
    SetHours(String),
    SetStaffCount(String),
    SetAverageSalary(String),
}

#[derive(Debug, Default)]
pub struct App {
    hours: String,
    staff_count: String,
    average_yearly_salary: String,
}

impl App {
    /// Regenerates the Gorske cost with a given input
    ///
    /// Used for UI rendering
    fn regenerate_gorske_cost(&self) -> String {
        if self.hours.is_empty() {
            return "Cost/Year in Gorske Unit".to_string();
        }

        let hours = self.hours.parse::<f32>().unwrap_or(0.0);
        let average_yearly_salary = self.average_yearly_salary.parse::<i32>().unwrap_or(0);
        let hourly_rate = calculate_hourly_salary(average_yearly_salary);
        let staff_count = self.staff_count.parse::<i32>().unwrap_or(0);
        let g_cost = calculate_gorske_cost(hours, hourly_rate, staff_count);
        format!("Gorske cost: {} GU / year", g_cost)
    }

    /// Regenerates the BigMac cost with a given input
    ///
    /// Used for UI rendering
    fn regenerate_bigmac_cost(&self) -> String {
        if self.hours.is_empty() {
            return "Cost/Year in BigMacs".to_string();
        }

        let hours = self.hours.parse::<f32>().unwrap_or(0.0);
        let average_yearly_salary = self.average_yearly_salary.parse::<i32>().unwrap_or(0);
        let hourly_rate = calculate_hourly_salary(average_yearly_salary);
        let staff_count = self.staff_count.parse::<i32>().unwrap_or(0);
        let g_cost = calculate_bigmac_cost(hours, hourly_rate, staff_count);
        format!("BigMac Cost: {} BigMacs / year", g_cost)
    }

    /// Regenerates the Actual cost with a given input
    ///
    /// Used for UI rendering
    fn regenerate_actual_cost(&self) -> String {
        if self.hours.is_empty() {
            return "Cost/Year in Australian Dollarydoos".to_string();
        }

        let hours = self.hours.parse::<f32>().unwrap_or(0.0);
        let average_yearly_salary = self.average_yearly_salary.parse::<i32>().unwrap_or(0);
        let hourly_rate = calculate_hourly_salary(average_yearly_salary);
        let staff_count = self.staff_count.parse::<i32>().unwrap_or(0);
        let g_cost = calculate_cost(hours, hourly_rate, staff_count);
        format!("Dollar cost: ${} / year", g_cost)
    }

    fn get_bigmac_string(&self) -> String {
        format!(
            "The current price of a Big Mac is ${} AUD.",
            crate::gorske::BIGMAC_PRICE
        )
    }
    fn get_gorske_string(&self) -> String {
        format!("One (1) Gorske Unit is ${} AUD.", get_gorske())
    }
}

impl Component for App {
    type Message = StateChangeMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            hours: "10".into(),
            staff_count: "1".into(),
            average_yearly_salary: "100000".into(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            StateChangeMessage::SetHours(hours) => self.hours = hours,
            StateChangeMessage::SetStaffCount(staff_count) => self.staff_count = staff_count,
            StateChangeMessage::SetAverageSalary(average_yearly_salary) => {
                self.average_yearly_salary = average_yearly_salary
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let change_hours = ctx.link().callback(StateChangeMessage::SetHours);
        let change_staff_count = ctx.link().callback(StateChangeMessage::SetStaffCount);
        let change_average_salary = ctx.link().callback(StateChangeMessage::SetAverageSalary);
        html! {

            <main>
            <h1> {"Gorske Calculator"} </h1>
            <div class="heading">
            {"Donald A. Gorske (born November 28, 1953) is an American world record holder known as the \"Big Mac enthusiast\". "}
            <br />
            {" He is best known for having eaten 30,000 Big Mac hamburgers from the U.S. fast food chain McDonald's in his lifetime, winning him a place in the Guinness Book of Records."}
            </div>
            <div class="intro">
            <h2> {" Why"} </h2>
            {"Sometimes, organisations think a thing doesn't really cost them that much. Use this handy calculator to do the math and see the actual cost in units such as Gorske, Dollars, and McDonalds BigMacs."}
            <h2> {"Assumptions"} </h2>
            <ul>
                <li>{{self.get_bigmac_string()}}</li>
                <li>{{self.get_gorske_string()}}</li>
                <li> {"Working week is 40 hours"} </li>
                <li> {"FTE are working 52 weeks a year"} </li>
                <li> {"Time wasted is calculated as: (Staff * Average hourly salary * Wasted hours every week)"} </li>
                <li>{"You're trying to piss off someone by pointing out how they're being cheap, and how much it's actually costing them"} </li>
            </ul>
            </div>
                <div class="entry">
                    <div>
                        {"Enter in time wasted:"}
                        <div class="footnote">
                            {"(Give an estimate for a week, in hours)"}
                        </div>
                    </div>
                    <div>
                        <TextInput on_change= {change_hours} value={{self.hours.clone()}} />
                    </div>

                    <div>
                    {"Enter in average yearly salary for FTE:"}

                     <TextInput on_change= {change_average_salary} value={{self.average_yearly_salary.clone()}} />

                    <div>
                    {"Enter in number of FTE affected:"}
                     <TextInput on_change= {change_staff_count} value={{self.staff_count.clone()}} />
                    </div>
                </div>

                </div>
                <div class="readout">
                    <div>
                        {self.regenerate_gorske_cost()}
                    </div>
                    <div>
                        {self.regenerate_bigmac_cost()}
                </div>
                <div>
                    {self.regenerate_actual_cost()}
                </div>
                </div>
            </main>
        }
    }
}
