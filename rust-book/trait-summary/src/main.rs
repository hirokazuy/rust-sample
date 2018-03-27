extern crate trait_summary;
use trait_summary::{Tweet, Summarizable};

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of precipitaion is {}%.",
                self.high_temp, self.low_temp, self.chance_of_precipitation)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    let wf = WeatherForecast {
        high_temp: 20.0,
        low_temp: 7.3,
        chance_of_precipitation: 30.0,
    };
    println!("1 new weather forecast: {}", wf.summary());
}
