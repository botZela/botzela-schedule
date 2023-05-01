use crate::routes::schedule::PostParams;
use mongodb::{bson::doc, Database};
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
                            (Some(t), None | Some(_)) | (None, Some(t)) => Some(t),
                            _ => None,
                        })
                        .collect()
                })
                .collect(),
            ..self
        }
    }

    pub async fn fetch(database: &Database, payload: &PostParams) -> Option<Schedule> {
        let collection = database.collection::<Schedule>("ensias-schedules");

        let filter =
            doc! { "year": &payload.year, "filiere": &payload.filiere, "week": &payload.week};
        let fl = collection.find_one(filter, None).await.unwrap_or(None);

        let filter =
            doc! { "year": &payload.year, "filiere": &payload.groupe, "week": &payload.week};
        let grp = collection.find_one(filter, None).await.unwrap_or(None);

        match (fl, grp) {
            (Some(fl), Some(grp)) => Some(fl.merge(grp)),
            _ => None,
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
