#[derive(Debug, PartialEq)]
pub struct Clock {
    min: i32,
}

impl Clock {
    // Constructs a new Clock instance
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;

        Clock {
            min: Self::custom_rem_euclid(total_minutes, 1440), // 1440 minutes in a day
        }
    }

    // Adds minutes to the clock
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.min + minutes)
    }

    pub fn to_string(&self) -> String {
        let hrs = Self::format_time_object(self.min / 60);
        let minutes = Self::format_time_object(self.min % 60);

        String::from(format!("{}:{}", hrs, minutes))
    }

    fn format_time_object(time: i32) -> String {
        let mut formatted_string = String::from(&time.to_string());

        if formatted_string.len() == 1 {
            formatted_string = String::from("0") + &formatted_string;
        }

        formatted_string
    }

    // Computes the Euclidean remainder
    fn custom_rem_euclid(dividend: i32, divisor: i32) -> i32 {
        let remainder = dividend % divisor;

        if remainder < 0 {
            remainder + divisor
        } else {
            remainder
        }
    }
}
