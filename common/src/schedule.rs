use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct PostParams {
    pub year: String,
    pub filiere: String,
    pub groupe: String,
    pub week: String,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub year: String,
    pub filiere: String,
    pub week: Option<String>,
    pub days: Vec<Vec<Option<Seance>>>,
}
