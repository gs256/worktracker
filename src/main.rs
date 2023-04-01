fn main() {
    let test_text = "1 20\n2 10\n\n2h 30m\n3:43 \n\n12h 20m\n14h 10\n\n11 40m\n1h 20m";
    let time_spans = text_to_time_spans(test_text);

    for time_span in &time_spans {
        println!(
            "{:>7} - {:>7} => {}",
            time_span.start.to_string(),
            time_span.stop.to_string(),
            time_span.duration().to_string()
        )
    }

    println!();

    let total_time = calculate_total_time(time_spans);
    println!("Total time: {}h {}m", total_time.hours, total_time.minutes);
}

#[derive(Debug)]
pub struct TimeStamp {
    pub hours: i32,
    pub minutes: i32,
}

impl TimeStamp {
    pub fn from_minutes(minutes: i32) -> TimeStamp {
        let hours = minutes / 60;

        return TimeStamp {
            hours: hours,
            minutes: minutes - hours * 60,
        };
    }

    pub fn to_minutes(&self) -> i32 {
        return self.hours * 60 + self.minutes;
    }

    pub fn to_string(&self) -> String {
        if self.hours != 0 && self.minutes != 0 {
            return format!("{}h {}m", self.hours, self.minutes);
        } else if self.hours == 0 {
            return format!("{}m", self.minutes);
        } else {
            return format!("{}h", self.hours);
        }
    }
}

#[derive(Debug)]
pub struct TimeSpan {
    pub start: TimeStamp,
    pub stop: TimeStamp,
}

impl TimeSpan {
    pub fn duration(&self) -> TimeStamp {
        let mut stop_h = self.stop.hours;
        let stop_m = self.stop.minutes;

        while stop_h < self.start.hours {
            stop_h += 12;
        }

        let total_minutes = (stop_h * 60 + stop_m) - self.start.to_minutes();
        return TimeStamp::from_minutes(total_minutes);
    }
}

fn calculate_total_time(time_spans: Vec<TimeSpan>) -> TimeStamp {
    let mut total_minutes = 0;

    for time_span in time_spans {
        total_minutes += time_span.duration().to_minutes();
    }

    let result = TimeStamp::from_minutes(total_minutes);
    return result;
}

fn text_to_time_spans(text: &str) -> Vec<TimeSpan> {
    let timestamp_strings: Vec<&str> = text.lines().collect();
    let timestamp_pairs = collect_timestamp_pairs(timestamp_strings);
    let mut time_spans: Vec<TimeSpan> = vec![];

    for pair in timestamp_pairs {
        let start = parse_timestamp(pair.0).unwrap();
        let stop = parse_timestamp(pair.1).unwrap();

        let time_span = TimeSpan {
            start: start,
            stop: stop,
        };

        time_spans.push(time_span)
    }

    return time_spans;
}

fn collect_timestamp_pairs(lines: Vec<&str>) -> Vec<(&str, &str)> {
    let mut timestamp_pairs: Vec<(&str, &str)> = vec![];
    let mut i = 0;

    while i < lines.len() - 1 {
        let first = lines[i].trim();
        let second = lines[i + 1].trim();
        if first.len() == 0 {
            i += 1;
            continue;
        }
        if second.len() == 0 {
            println!("line {}: timestamp '{first}' has no pair", i);
            i += 1;
            continue;
        }
        timestamp_pairs.push((first, second));
        i += 2;
    }

    return timestamp_pairs;
}

fn parse_timestamp(string: &str) -> Option<TimeStamp> {
    let string = string.trim();

    if string.len() == 0 {
        return None;
    }

    let numbers: Vec<&str> = string
        .split([' ', '\t', 'h', 'm', 'H', 'M', ':'])
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .collect();

    if numbers.len() != 2 {
        return None;
    }

    let hours = numbers[0].parse::<i32>();
    let minutes = numbers[1].parse::<i32>();

    if hours.is_err() || minutes.is_err() {
        return None;
    }

    let hours = hours.unwrap();
    let minutes = minutes.unwrap();

    if hours < 0 || minutes < 0 {
        return None;
    }

    let timestamp = TimeStamp {
        hours: hours,
        minutes: minutes,
    };

    return Some(timestamp);
}
