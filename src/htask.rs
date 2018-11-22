
use std::option::{Option};


#[derive(Debug, PartialEq)]
pub struct HTaskCommon {
    text: String,
    tags: Option<Vec<String>>,
    alias: Option<String>,
    reminders: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct HTaskHabit {
    common: HTaskCommon,
    up: bool,
    down: bool,
    prior: HTaskPriority
}

#[derive(Debug, PartialEq)]
pub struct HTaskDaily {
    common: HTaskCommon,
    start_date: String,
    every_x: i32,
    streak: i32,
    repeat: HDailyRepeat,
    frequency: HDailyFrequency,
    prior: HTaskPriority
}

#[derive(Debug, PartialEq)]
pub struct HTaskTODO {
    common: HTaskCommon,
    date: Option<String>,
    prior: HTaskPriority
}

#[derive(Debug, PartialEq)]
pub struct HTaskReward {
    common: HTaskCommon,
    value: i32
}


#[derive(Debug, PartialEq, Copy)]
struct HDailyRepeat {
    su: bool,
    m: bool,
    t: bool,
    w: bool,
    th: bool,
    f: bool,
    s: bool
}

impl Clone for HDailyRepeat {
    fn clone(&self) -> HDailyRepeat{ *self }
}

#[derive(Debug, PartialEq)]
enum UserAttribute {
    _String,
    _INT,
    _PER,
    _CON
}

#[derive(Debug, PartialEq)]
enum HTaskPriority {
    TRIVIAL,
    EASY,
    MEDIUM,
    HARD
}

#[derive(Debug, PartialEq)]
enum HDailyFrequency {
    DAILY,
    WEEKLY
}

trait HTask {
    fn get_json<'a>() -> &'a str;
}

fn make_htask_common(text: &String) -> HTaskCommon{
    return HTaskCommon {
        text: text.clone(),
        tags: None,
        alias: None,
        reminders: None
    }
}

fn make_habit(text: &String, up: bool, down: bool, prior: HTaskPriority) -> HTaskHabit {
    return HTaskHabit {
        common: make_htask_common(text),
        up,
        down,
        prior
    }
}

fn make_daily(text: &String,
              sd: String,
              ex: i32,
              streak: i32,
              rp: HDailyRepeat,
              freq: HDailyFrequency,
              prior: HTaskPriority
) -> HTaskDaily {
    return HTaskDaily {
        common: make_htask_common(text),
        start_date: sd,
        every_x: ex,
        streak,
        repeat: rp,
        frequency: freq,
        prior
    }
}


fn make_todo(text:&String, prior: HTaskPriority) -> HTaskTODO {
    return HTaskTODO{
        common: make_htask_common(text),
        date: None,
        prior
    }
}

fn make_reward(text: &String, value: i32) -> HTaskReward {
    return HTaskReward {
        common: make_htask_common(text),
        value
    }
}

fn get_priority_value<'a>(pri: HTaskPriority) -> &'a str {

    match pri {
        HTaskPriority::TRIVIAL => "0.1",
        HTaskPriority::EASY => "1",
        HTaskPriority::MEDIUM => "1.5",
        HTaskPriority::HARD => "2.0"
    }

}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_get_priority_value() {
        assert_eq!("0.1", get_priority_value(HTaskPriority::TRIVIAL));
        assert_eq!("1", get_priority_value(HTaskPriority::EASY));
        assert_eq!("1.5", get_priority_value(HTaskPriority::MEDIUM));
        assert_eq!("2.0", get_priority_value(HTaskPriority::HARD));


    }

    #[test]
    fn test_make_tasks() {
        let text = "Sample".to_string();

        let common = HTaskCommon {
            text: text.clone(),
            tags: None,
            alias: None,
            reminders: None
        };
        assert_eq!(common, make_htask_common(&text));

        let habit = HTaskHabit {
            common: make_htask_common(&text),
            up: true,
            down: false,
            prior: HTaskPriority::HARD
        };
        assert_eq!(habit, make_habit(&text, true, false, HTaskPriority::HARD));

        let start_date = "a start date".to_string();
        let repeat = HDailyRepeat {
            su: false,
            m: true,
            t: false,
            w: false,
            th: false,
            f: false,
            s: false
        };
        let weekly_frequency = HDailyFrequency::WEEKLY;
        assert_ne!(weekly_frequency, HDailyFrequency::DAILY);
        let daily = HTaskDaily {
            common: make_htask_common(&text),
            start_date: start_date.clone(),
            every_x: 0,
            streak: 0,
            repeat: repeat.clone(),
            frequency: HDailyFrequency::DAILY,
            prior: HTaskPriority::HARD
        };
        assert_eq!(daily,
                   make_daily(&text,
                              start_date,
                              0,
                              0,
                              repeat,
                              HDailyFrequency::DAILY,
                              HTaskPriority::HARD));

        let todo = HTaskTODO {
            common: make_htask_common(&text),
            date: None,
            prior: HTaskPriority::HARD
        };

        assert_eq!(todo, make_todo(&text, HTaskPriority::HARD));

        let reward = HTaskReward {
            common: make_htask_common(&text),
            value: 0
        };

        assert_eq!(reward, make_reward(&text, 0))

    }

    #[test]
    fn test_get_json() {
    }

}