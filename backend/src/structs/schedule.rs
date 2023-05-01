use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Seance {
    pub class: String,
    pub name: String,
    pub module_num: Option<i32>,
    pub professor: Option<String>,
    #[serde(alias = "otherFilieres")]
    pub other_filieres: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub year: String,
    pub filiere: String,
    pub week: Option<String>,
    pub days: Vec<Vec<Option<Seance>>>,
}

impl Schedule {
    pub fn merge(self, second: Self) -> Self {
        Self {
            days: self
                .days
                .into_iter()
                .zip(second.days.into_iter())
                .map(|(r, l)| {
                    r.into_iter()
                        .zip(l.into_iter())
                        .map(|(x, y)| match (x, y) {
                            (Some(t), None) | (None, Some(t)) => Some(t),
                            (Some(t), Some(_)) => Some(t),
                            _ => None,
                        })
                        .collect()
                })
                .collect(),
            ..self
        }
    }
}

fn get_module(seance: Option<Seance>) -> Option<Seance> {
    match seance {
        Some(mut x) => {
            let re = Regex::new(r"M\d\.(\d)").unwrap();
            let num: i32 = re
                .captures(&x.name)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap_or(0);
            x.module_num = Some(num);
            Some(x)
        }
        _ => None,
    }
}

pub fn gen_module_num(days: Vec<Vec<Option<Seance>>>) -> Vec<Vec<Option<Seance>>> {
    days.into_iter()
        .map(|day| day.into_iter().map(get_module).collect())
        .collect()
}
