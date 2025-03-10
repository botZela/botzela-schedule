use common::schedule;
use leptos::prelude::*;
use leptos_router::components::Form;
use leptos_router::hooks::use_query;

#[component]
pub fn Header() -> impl IntoView {
    let params = use_query::<schedule::PostParams>();

    let body = move || {
        params.with(|params| {
            params.clone().unwrap_or(schedule::PostParams {
                year: Some("1A".to_owned()),
                week: Some("S22".to_owned()),
                groupe: Some("G1".to_owned()),
                filiere: Some("2IA".to_owned()),
            })
        })
    };

    let year_vec = ["1A", "2A", "3A"];

    let fl_vec = [
        "2IA", "2SCL", "BI&A", "GD", "GL", "IDF", "IDSIT", "SSI", "SSE",
    ];
    let grp_vec = ["G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8"];

    let week_vec = ["S16", "S17", "S22"];

    view! {
        <div class="header">
            <img src="/images/ENSIAS.svg" alt="ensias_logo"/>
            <Form method="GET" action="">
                <p>
                    "Université Mohammed V de Rabat" <br/>
                    "Ecole Nationale Supérieure d’Informatique et d’Analyse des Systèmes"
                    <br/>
                    <b>
                        "Emploi du Temps de la période 1 du Semestre 3 - " "Année "
                        <select name="year" id="year" oninput="this.form.requestSubmit()">
                            {year_vec
                                .iter()
                                .map(|&year| {
                                    view! {
                                        <option
                                            value=year
                                            selected=move || body().year == Some(year.to_string())
                                        >
                                            {year}
                                        </option>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </select> <br/> "Filiere:  "
                        <select name="filiere" id="filiere" oninput="this.form.requestSubmit()">
                            {fl_vec
                                .iter()
                                .map(|&fl| {
                                    view! {
                                        <option
                                            value=fl
                                            selected=move || body().filiere == Some(fl.to_string())
                                        >
                                            {fl}
                                        </option>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </select> " |_| " "Groupe:"
                        <select name="groupe" id="groupe" oninput="this.form.requestSubmit()">
                            {grp_vec
                                .iter()
                                .map(|&grp| {
                                    view! {
                                        <option
                                            value=grp
                                            selected=move || body().groupe == Some(grp.to_string())
                                        >
                                            {grp}
                                        </option>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </select> " |_| " " Week : "
                        <select name="week" id="week" oninput="this.form.requestSubmit()">
                            {week_vec
                                .iter()
                                .map(|&week| {
                                    view! {
                                        <option
                                            value=week
                                            selected=move || body().week == Some(week.to_string())
                                        >
                                            {week}
                                        </option>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </select>
                    </b> <br/> "Année universitaire : 2022-2023"
                </p>
            </Form>
            <img src="/images/botZela.png" alt="bot_logo"/>
        </div>
    }
}
