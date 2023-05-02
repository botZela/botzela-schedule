use common::schedule::PostParams;
use leptos::*;

#[component]
pub fn Header(cx: Scope, header_args: ReadSignal<PostParams>) -> impl IntoView {
    view! { cx,
        <div class="header">
            <img src="/images/ENSIAS.svg" alt="ensias_logo" />
                <p>
                    "Université Mohammed V de Rabat" <br />
                    "Ecole Nationale Supérieure d’Informatique et d’Analyse des Systèmes" <br />
                <b>
                    {move || format!("Emploi du Temps de la période 1 du Semestre 3 - {} Année", header_args.get().year)} <br />
                    {move || format!("Filiere: {} |_| Groupe : {} |_| Week : {}" , header_args.get().filiere, header_args.get().groupe, header_args.get().week)}
                </b> <br />
                    "Année universitaire : 2022-2023"
                </p>
            <img src="/images/botZela.png" alt="bot_logo" />
        </div>
    }
}
