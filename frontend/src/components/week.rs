use super::day::*;
use crate::SHOW_BRANCH;
use common::schedule::Seance;
use leptos::*;

#[component]
fn WeekHeader(cx: Scope) -> impl IntoView {
    view! { cx,
        <tr>
            <th class="jour"></th>
            <Show
                when=move || SHOW_BRANCH
                fallback=|_| {
                    view! { cx,  }
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
pub fn Week(cx: Scope, days: Vec<Vec<Option<Seance>>>) -> impl IntoView {
    let days_name = vec!["Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi"];

    let days_view: Vec<_> = days_name
        .iter()
        .zip(days.iter())
        .map(|(&day, &ref seances)| view! { cx, <Day day seances=seances.clone()/> })
        .collect();

    view! { cx,
        <div class="body">
            <table>
                {if days_view.len() == 0 {
                    view! { cx, <p ce>"Not Found"</p> }
                        .into_view(cx)
                } else {
                    view! { cx,
                        <WeekHeader/>
                        {days_view}
                    }
                        .into_view(cx)
                }}
            </table>
        </div>
    }
}
