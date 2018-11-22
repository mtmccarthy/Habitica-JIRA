
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
    DAILY,
    _WEEKLY
}

impl HTask {
    pub fn get_json<'a>(&self) -> &'a str {
        r#""#
    }

    pub fn is_valid(&self) -> bool {

        if &self.htype != &HTaskType::REWARD {
            match &self.priority {
                None => return false,
                _ => ()
            }
        }

        match &self.htype {
            HTaskType::HABIT => {

                let up = &self.up;
                match up {
                    None => return false,
                    _ => ()
                }
                let down = &self.down;
                match down {
                    None => return false,
                    _ => ()
                }


                return true;
            },
            HTaskType::DAILY => {
                let start_date = &self.start_date;
                let every_x = &self.every_x;
                let streak = &self.streak;
                let repeat = &self.repeat;
                let frequency = &self.frequency;

                match start_date {
                    None => return false,
                    _ => ()
                }
                match every_x {
                    None => return false,
                    _ => ()
                }
                match streak {
                    None => return false,
                    _ => ()
                }
                match repeat {
                    None => return false,
                    _ => ()
                }
                match frequency {
                    None => return false,
                    _ => ()
                }

                return true;
            },
            HTaskType::TODO => {
                return false;
            },
            HTaskType::REWARD => false
        }
    }
}

fn make_habit(up: bool, down: bool, prior: HTaskPriority) -> HTask {
    return HTask {
        htype: HTaskType::HABIT,
        text: String::new(),
        tags: None,
        alias: None,
        notes: None,
        date: None,
        priority: Some(prior),
        reminders: None,
        frequency: None,
        repeat: None,
        every_x: None,
        streak: None,
        start_date: None,
        up: Some(up),
        down: Some(down),
        value: None
    }
}

fn make_daily(sd: String,
              ex: i32,
              streak: i32,
              rp: HDailyRepeat,
              freq: HDailyFrequency,
              prior: HTaskPriority
) -> HTask {
    return HTask {
        htype: HTaskType::DAILY,
        text: String::new(),
        tags: None,
        alias: None,
        notes: None,
        date: None,
        priority: Some(prior),
        reminders: None,
        frequency: Some(freq),
        repeat: Some(rp),
        every_x: Some(ex),
        streak: Some(streak),
        start_date: Some(sd),
        up: None,
        down: None,
        value: None
    }
}

fn _make_empty_task() -> HTask {
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
        assert_eq!("1", get_priority_value(HTaskPriority::EASY));
        assert_eq!("1.5", get_priority_value(HTaskPriority::MEDIUM));
        assert_eq!("2.0", get_priority_value(HTaskPriority::HARD));


    }

    #[test]
    fn test_htask_is_valid() {
        let mt_task = _make_empty_task();
        let habit = make_habit(true, false, HTaskPriority::HARD);
        let repeat = HDailyRepeat {
            su: false,
            m: true,
            t: false,
            w: true,
            th: false,
            f: false,
            s: false
        };
        let daily = make_daily("asdf".to_string(),
                               0,
                               0,
                               repeat,
                               HDailyFrequency::DAILY,
                                HTaskPriority::EASY
        );
        let _todo = make_todo();
        let _reward = make_reward();

        assert_eq!(HTaskType::HABIT, habit.htype);
        assert_eq!(String::new(), habit.text);

        assert_eq!(false, mt_task.is_valid());
        assert_eq!(true, habit.is_valid());
        assert_eq!(true, daily.is_valid());




        //assert_eq!(true, daily.is_valid());
        //assert_eq!(true, todo.is_valid());
        //assert_eq!(true, reward.is_valid());

    }

    #[test]
    fn test_get_json() {
        let empty_task = _make_empty_task();

        assert_eq!(String::new(), empty_task.get_json())
    }

}