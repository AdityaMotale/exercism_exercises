#[derive(Debug, PartialEq)]
pub struct Clock {
    mins: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hrs = Self::handle_hrs_overflow(hours);
        let mins = Self::handle_minutes_overflow(minutes);

        Clock {
            mins: (hrs * 60) + mins,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            mins: self.mins + minutes,
        }
    }

    pub fn to_string(&self) -> String {
        let hrs = Self::get_time_in_format(Self::handle_hrs_overflow(self.mins / 60));
        let minutes = Self::get_time_in_format(self.mins % 60);

        String::from(format!("{}:{}", hrs, minutes))
    }

    fn handle_hrs_overflow(mut hr: i32) -> i32 {
        if hr < 0 {
            if hr < -24 {
                hr = hr % -24;
            }

            return 24 + hr;
        }

        if hr < 24 {
            return hr;
        }

        hr = hr % 24;

        if hr == 24 {
            hr = 0;
        }

        hr
    }

    fn handle_minutes_overflow(mut min: i32) -> i32 {
        // 24 * 60, maximum minutes in a day
        let MINUTES = 1440;

        if min < 0 {
            if min < -1440 {
                min = min % -1440;
                println!("{}", min);

                if min < -1440 {
                    return min + MINUTES;
                }

                return min;
            }

            return min;
        }

        min = min % MINUTES;

        min
    }

    fn get_time_in_format(time: i32) -> String {
        let mut formatted_string = String::from(&time.to_string());

        if formatted_string.len() == 1 {
            formatted_string = String::from("0") + &formatted_string;
        }

        formatted_string
    }
}
