use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container mt-16 mx-48 grid grid-cols-3">
            <div class="col-span-1">
                <h1 class="">{" hola "}</h1>
            </div>
            <div class="col-span-2">
                <h1 class="">{" hola "}</h1>
            </div>
        </div>
    }
}
