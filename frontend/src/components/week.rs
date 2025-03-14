use super::day::*;
use crate::SHOW_BRANCH;
use leptos::prelude::*;

#[component]
fn WeekHeader() -> impl IntoView {
    view! {
        <tr>
            <th class="jour"></th>
            <Show
                when=move || SHOW_BRANCH
                fallback=|| {
                    view! {}
                }
            >

                <th class="filiere">"Filière"</th>
            </Show>
            <th class="class">"Class"</th>
            <th class="matiere">"9h00 à 10h40"</th>
            <th class="class">"Class"</th>
            <th class="matiere">"11h00 à 12h40"</th>
            <th class="delimiter">""</th>
            <th class="class">"Class"</th>
            <th class="matiere">"14h00 à 15h40"</th>
            <th class="class">"Class"</th>
            <th class="matiere">"16h00 à 17H40"</th>
        </tr>
    }
}

#[component]
pub fn Week(days: Option<common::schedule::Days>) -> impl IntoView {
    let days_name = ["Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi"];

    let days = match days {
        Some(x) if !x.is_empty() => x,
        _ => vec![vec![None; 4]; 6],
    };

    let days_view: Vec<_> = days_name
        .iter()
        .zip(days.iter())
        .map(|(&day, seances)| view! { <Day day seances=seances.clone()/> })
        .collect();

    view! {
        <div class="body">
            <table>
                <WeekHeader/>
                {days_view}
            </table>
        </div>
    }
}
