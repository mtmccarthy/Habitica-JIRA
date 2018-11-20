
use std::option::{Option};

#[derive(Debug, PartialEq)]
pub struct HTask {
    htype: HTaskType,
    text: String,
    tags: Option<Vec<String>>,
    alias: Option<String>,
    notes: Option<String>,
    date: Option<String>,
    priority: Option<HTaskPriority>,
    reminders: Option<Vec<String>>,
    frequency: Option<HDailyFrequency>,
    repeat: Option<HDailyRepeat>,
    every_x: Option<i32>,
    streak: Option<i32>,
    start_date: Option<String>,
    up: Option<bool>,
    down: Option<bool>,
    value: Option<i32>


}

#[derive(Debug, PartialEq)]
struct HDailyRepeat {
    su: bool,
    m: bool,
    t: bool,
    w: bool,
    th: bool,
    f: bool,
    s: bool
}

#[derive(Debug, PartialEq)]
enum HTaskType {
    HABIT,
    DAILY,
    TODO,
    REWARD
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
    _DAILY,
    _WEEKLY
}

impl HTask {
    pub fn get_json<'a>(&self) -> &'a str {
        r#"{

        }"#
    }

    pub fn is_valid(&self) -> bool {
        match self.htype {
            HTaskType::HABIT => false,
            HTaskType::DAILY => false,
            HTaskType::TODO => false,
            HTaskType::REWARD => false
        }
    }
}

fn make_habit() -> HTask {
    return HTask {
        htype: HTaskType::HABIT,
        text: String::new(),
        tags: None,
        alias: None,
        notes: None,
        date: None,
        priority: None,
        reminders: None,
        frequency: None,
        repeat: None,
        every_x: None,
        streak: None,
        start_date: None,
        up: None,
        down: None,
        value: None
    }
}

fn make_daily() -> HTask {
    return HTask {
        htype: HTaskType::DAILY,
        text: String::new(),
        tags: None,
        alias: None,
        notes: None,
        date: None,
        priority: None,
        reminders: None,
        frequency: None,
        repeat: None,
        every_x: None,
        streak: None,
        start_date: None,
        up: None,
        down: None,
        value: None
    }
}

fn make_todo() -> HTask {
    return HTask {
        htype: HTaskType::TODO,
        text: String::new(),
        tags: None,
        alias: None,
        notes: None,
        date: None,
        priority: None,
        reminders: None,
        frequency: None,
        repeat: None,
        every_x: None,
        streak: None,
        start_date: None,
        up: None,
        down: None,
        value: None
    }
}

fn make_reward() -> HTask {
    return HTask {
        htype: HTaskType::REWARD,
        text: String::new(),
        tags: None,
        alias: None,
        notes: None,
        date: None,
        priority: None,
        reminders: None,
        frequency: None,
        repeat: None,
        every_x: None,
        streak: None,
        start_date: None,
        up: None,
        down: None,
        value: None
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
        assert_eq!("1.0", get_priority_value(HTaskPriority::EASY));
        assert_eq!("1.5", get_priority_value(HTaskPriority::MEDIUM));
        assert_eq!("2.0", get_priority_value(HTaskPriority::HARD));


    }

    #[test]
    fn test_htask_is_valid() {
        let habit = make_habit();
        let daily = make_daily();
        let todo = make_todo();
        let reward = make_reward();

        assert_eq!(HTaskType::HABIT, habit.htype);
        assert_eq!(String::new(), habit.text);
        assert_eq!(None, habit.tags);
        assert_eq!(None, habit.alias);
        assert_eq!(None, habit.notes);
        assert_eq!(None, habit.date);
        assert_eq!(None, habit.priority);
        assert_eq!(None, habit.reminders);
        assert_eq!(None, habit.frequency);
        assert_eq!(None, habit.repeat);
        assert_eq!(None, habit.every_x);
        assert_eq!(None, habit.streak);
        assert_eq!(None, habit.start_date);
        assert_eq!(None, habit.up);
        assert_eq!(None, habit.down);
        assert_eq!(None, habit.value);

        assert_eq!(true, habit.is_valid());
        assert_eq!(true, daily.is_valid());
        assert_eq!(true, todo.is_valid());
        assert_eq!(true, reward.is_valid());

    }

    #[test]
    fn test_get_json() {
        let habit = make_habit();

        assert_eq!(String::new(), habit.get_json())
    }

}