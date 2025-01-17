use leptos::prelude::*;
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Params, Default)]
pub struct PostParams {
    pub year: Option<String>,
    pub filiere: Option<String>,
    pub groupe: Option<String>,
    pub week: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Seance {
    pub class: String,
    pub name: String,
    pub module_num: Option<i32>,
    pub professor: Option<String>,
    #[serde(alias = "otherFilieres")]
    pub other_filieres: Vec<String>,
}

pub type Days = Vec<Vec<Option<Seance>>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub year: String,
    pub filiere: String,
    pub week: Option<String>,
    pub days: Days,
}
