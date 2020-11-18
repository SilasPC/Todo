
use serde_derive::{Serialize,Deserialize};
use chrono::NaiveDate;

#[derive(Serialize,Deserialize,Default)]
pub struct Data {
    pub order: Order,
    pub tasks: Vec<Task>,
}
impl Data {
    pub fn print(&self) {
        if self.tasks.len() == 0 {
            println!("No notes to show");
            return
        }
        let longest_digit = 1+(self.tasks.len() as f32).log10().floor() as usize;
        let longest_note = self.tasks.iter()
            .map(|t| t.note.chars().count())
            .max()
            .unwrap();
        let s = " ".repeat(longest_note);
        for (i, task) in self.tasks.iter().enumerate() {
            let digit_len = 1+((i+1) as f32).log10().floor() as usize;
            let date = if let Some(d) = task.date.0 {
                d.format("%d/%m").to_string()
            } else {
                "".into()
            };
            let digit_spacing = &s[..(longest_digit-digit_len)];
            let date_spacing = &s[..(longest_note - task.note.chars().count())];
            println!("[{}{}] {}{} {}",digit_spacing,i+1,task.note,date_spacing,date)
        }
    }
    pub fn set_order(&mut self, order: Order) {
        self.tasks.sort_by(order.cmp_fn());
        self.order = order;
    }
    pub fn insert(&mut self, item: Task) {
        self.tasks.push(item);
        self.tasks.sort_by(self.order.cmp_fn());
    }
}

#[derive(Serialize,Deserialize)]
pub struct Task {
    pub pri: u8,
    pub note: String,
    pub date: IncOpt
}

#[derive(Serialize,Deserialize,PartialEq,Eq)]
pub struct IncOpt (pub Option<NaiveDate>);
impl From<Option<NaiveDate>> for IncOpt {
    fn from(o:Option<NaiveDate>) -> IncOpt {
        IncOpt(o)
    }
}
impl From<NaiveDate> for IncOpt {
    fn from(o:NaiveDate) -> IncOpt {
        IncOpt(Some(o))
    }
}
impl PartialOrd for IncOpt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        if self.0.is_none() && other.0.is_some() {
            Some(Ordering::Greater)
        } else if self.0.is_some() && other.0.is_none() {
            Some(Ordering::Less)
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}
impl Ord for IncOpt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

#[derive(Serialize,Deserialize,Clone,Copy)]
pub enum Order {
    DatePri,
}
impl Default for Order {
    fn default() -> Self {Self::DatePri}
}

type TB<'r> = (usize,&'r Task);
impl Order {
    fn cmp_fn(&self) -> impl FnMut(&Task,&Task) -> std::cmp::Ordering {
        use Order::*;
        match self {
            DatePri => |a: &Task, b: &Task| 
                a.date.cmp(&b.date) // inc date
                    .then(b.pri.cmp(&a.pri)) // dec pri
                    .then(a.note.cmp(&b.note)) // inc lex
        }
    }
}