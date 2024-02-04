// Constants for total seconds in one earth year
const SECONDS_IN_ONE_EARTH_YEAR: f64 = 3_15_57_600.00;

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            earth_years: (s as f64) / SECONDS_IN_ONE_EARTH_YEAR,
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

// Constants for Orbital Periods
const ORBITAL_PERIOD_MERCURY: f64 = 0.2408467;
const ORBITAL_PERIOD_VENUS: f64 = 0.61519726;
const ORBITAL_PERIOD_EARTH: f64 = 1.0;
const ORBITAL_PERIOD_MARS: f64 = 1.8808158;
const ORBITAL_PERIOD_JUPITER: f64 = 11.862615;
const ORBITAL_PERIOD_SATURN: f64 = 29.447498;
const ORBITAL_PERIOD_URANUS: f64 = 84.016846;
const ORBITAL_PERIOD_NEPTUNE: f64 = 164.79132;

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_MERCURY
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_VENUS
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_EARTH
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_MARS
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_JUPITER
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_SATURN
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_URANUS
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_NEPTUNE
    }
}
