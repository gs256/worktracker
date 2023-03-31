fn main() {
    let test_text = "1 20\n2 10\n\n2h 30m\n3:43 \n\n12h 20m\n14h 10\n\n11 40m\n1h 20m";
    let time_spans = text_to_time_spans(test_text);
    println!("{:#?}", time_spans);
}

#[derive(Debug)]
pub struct TimeStamp {
    hours: i32,
    minutes: i32,
}

#[derive(Debug)]
pub struct TimeSpan {
    start: TimeStamp,
    stop: TimeStamp,
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
